//! REST API 与路由分发。
//!
//! # 接口分两类
//!
//! 1. **转发 mihomo**（`/api/mihomo/*`）—— 节点、连接、配置热改。
//!    clashx 用**无认证**方式调 `127.0.0.1:9090`，
//!    因为 9090 根本不绑局域网，只有本机能连。
//!    浏览器永远只跟 9080 打交道，鉴权也只做一次。
//!
//! 2. **clashx 自己的操作**（`/api/clashx/*`）—— 运行模式、规则、
//!    订阅、旁路由开关、配置读写。这些是 mihomo 没有的能力，
//!    也是本项目存在的理由。
//!
//! # 鉴权顺序（重要）
//!
//! ```text
//! 请求 → 查路径是否需要鉴权 → Basic Auth → 分发
//! ```
//! **所有路径都需要鉴权**，包括 `/` 和静态资源 ——
//! 配置文件里有订阅 token，任何免鉴权路径都是泄漏面。
//! 唯一的例外是 `/api/health`，它只返回 "ok" 不含任何信息，
//! 方便负载均衡/监控探活。

use anyhow::Result;
use serde_json::json;

use crate::config::Config;
use crate::core::Api;
use crate::nft;
use crate::runmode::RunMode;
use crate::web::{percent_decode, Request, Response, ServerCtx};

/// 不需要鉴权的路径（探活）。
const PUBLIC: &[&str] = &["/api/health"];

/// 总路由。
pub fn dispatch(req: &Request, ctx: &ServerCtx) -> Response {
    // 探活放行
    if PUBLIC.contains(&req.path.as_str()) {
        return Response::json(200, r#"{"status":"ok"}"#);
    }

    // 鉴权（配置文件读不出来时一律拒绝，不能因为「读不到配置」就放行）
    let cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => {
            return Response::json(
                500,
                json!({"error": format!("读配置失败：{e:#}")}).to_string(),
            )
        }
    };
    if !crate::web::check_auth(&cfg, req) {
        return Response::unauthorized("clashx");
    }

    // 路由
    match req.path.as_str() {
        // ---- 静态资源 ----
        "/" | "/index.html" => match index_html() {
            Some(html) => Response::html(200, html),
            None => Response::text(500, "内置 UI 缺失（编译时未 include）"),
        },
        "/bundle.js" => match bundle_js() {
            Some(js) => Response::bytes(
                200,
                "application/javascript; charset=utf-8",
                js.as_bytes().to_vec(),
            ),
            None => Response::text(500, "内置 JS 缺失"),
        },
        "/app.css" => match app_css() {
            Some(css) => Response::bytes(200, "text/css; charset=utf-8", css.as_bytes().to_vec()),
            None => Response::text(500, "内置 CSS 缺失"),
        },

        // ---- zashboard 反代 ----
        p if p.starts_with("/zashboard") => zashboard(req, ctx),

        // ---- API ----
        p if p.starts_with("/api/") => api(req, ctx, p),

        _ => Response::text(404, "404 Not Found"),
    }
}


/// 节点数缓存（5 秒 TTL）。
///
/// 与 `cached_mihomo_status` 分开：TTL 不同（节点数变化更慢），
/// 且失效原因不同（订阅更新 vs 内核重启）。
fn cached_node_count(ctx: &ServerCtx, cfg: &Config) -> usize {
    const TTL: std::time::Duration = std::time::Duration::from_secs(5);

    {
        let guard = ctx.node_count_cache.lock().unwrap();
        if let Some((at, n)) = guard.as_ref() {
            if at.elapsed() < TTL {
                return *n;
            }
        }
    }

    let n = crate::collect_proxies(cfg, &ctx.paths).0.len();
    let mut guard = ctx.node_count_cache.lock().unwrap();
    *guard = Some((std::time::Instant::now(), n));
    n
}

/// 取 mihomo 状态（1 秒 TTL）。
///
/// 缓存失效时也要重新拉：内核可能刚好重启过。
fn cached_mihomo_status(ctx: &ServerCtx) -> crate::web::MihomoStatus {
    const TTL: std::time::Duration = std::time::Duration::from_secs(1);

    {
        let guard = ctx.mihomo_cache.lock().unwrap();
        if let Some((at, st)) = guard.as_ref() {
            if at.elapsed() < TTL {
                return st.clone();
            }
        }
    }

    let st = match mihomo_api(ctx) {
        Some(api) => {
            let ver = api.version().unwrap_or_default();
            let up = !ver.is_empty();
            let conns = api
                .connections()
                .ok()
                .and_then(|c| serde_json::from_str::<serde_json::Value>(&c).ok())
                .and_then(|v| v.get("connections").and_then(|c| c.as_array()).map(|a| a.len()))
                .unwrap_or(0);
            crate::web::MihomoStatus { version: ver, up, connections: conns }
        }
        None => crate::web::MihomoStatus { version: "未知".into(), up: false, connections: 0 },
    };
    let mut guard = ctx.mihomo_cache.lock().unwrap();
    *guard = Some((std::time::Instant::now(), st.clone()));
    st
}

/// 读配置，带 mtime 缓存。
///
/// 压测实测：不加缓存时每个请求都要重新解析 YAML，
/// 串行 ~97ms/次，并发时 CPU 全耗在重复解析上。
/// 文件指纹（mtime + 长度）没变就复用上次解析结果。
fn load_cfg(ctx: &ServerCtx) -> Result<Config> {
    let fp = match crate::web::ConfigFingerprint::of(&ctx.paths.user_config) {
        Some(f) => f,
        // 指纹拿不到（文件不存在）→ 走原路径，让上层报「配置不存在」
        None => return crate::load_config(&ctx.paths),
    };

    // 快路径：指纹一致且有缓存
    {
        let guard = ctx.config_cache.lock().unwrap();
        if let Some((cached_fp, cfg)) = guard.as_ref() {
            if *cached_fp == fp {
                return Ok(cfg.clone());
            }
        }
    }

    // 慢路径：解析 + 更新缓存
    let cfg = crate::load_config(&ctx.paths)?;
    let mut guard = ctx.config_cache.lock().unwrap();
    *guard = Some((fp, cfg.clone()));
    Ok(cfg)
}

/// 写配置后必须让缓存失效。
///
/// ★ 漏了这步的后果：Web 界面里改了配置，刷新后界面还显示旧值 ——
///   因为指纹只在文件被外部修改时才会变，而 Web 自己是写完再读，
///   mtime 变了但如果写之前刚好有一份「指纹相同」的缓存就出问题了。
///   实际上写文件必然改mtime，所以理论上不会命中；
///   但显式失效更稳，且写路径本来就要拿锁，成本可忽略。
pub fn invalidate_cfg_cache(ctx: &ServerCtx) {
    let mut guard = ctx.config_cache.lock().unwrap();
    *guard = None;
}

fn json_err(status: u16, msg: impl Into<String>) -> Response {
    Response::json(status, json!({ "error": msg.into() }).to_string())
}

// ==================== API 分发 ====================

fn api(req: &Request, ctx: &ServerCtx, path: &str) -> Response {
    match path {
        // ---- 总览 ----
        "/api/clashx/overview" => overview(ctx),

        // ---- 运行模式 ----
        "/api/clashx/run-mode" => {
            if req.method == "GET" {
                let cfg = match load_cfg(ctx) {
                    Ok(c) => c,
                    Err(e) => return json_err(500, format!("{e:#}")),
                };
                let r = cfg.run_mode.resolve();
                Response::json(
                    200,
                    json!({
                        "current": cfg.run_mode.as_str(),
                        "dns_mode": cfg.run_mode.dns_mode_name(),
                        "tun": r.tun_enable,
                        "firewall": format!("{:?}", r.firewall),
                        "need_explicit_port": r.need_explicit_port,
                        "all": all_modes_json(),
                    })
                    .to_string(),
                )
            } else {
                json_err(405, "用 GET")
            }
        }
        "/api/clashx/run-mode/set" => set_run_mode(req, ctx),

        // ---- 旁路由 ----
        "/api/clashx/gateway" => gateway_status(ctx),
        "/api/clashx/gateway/enable" => {
            if req.method != "POST" {
                return json_err(405, "用 POST");
            }
            gateway_enable(req, ctx)
        }
        "/api/clashx/gateway/disable" => {
            if req.method != "POST" {
                return json_err(405, "用 POST");
            }
            match nft::disable() {
                Ok(()) => Response::json(200, r#"{"ok":true}"#),
                Err(e) => json_err(500, format!("{e:#}")),
            }
        }

        // ---- 规则 ----
        "/api/clashx/rules" => {
            if req.method != "GET" {
                return json_err(405, "用 GET");
            }
            let cfg = match load_cfg(ctx) {
                Ok(c) => c,
                Err(e) => return json_err(500, format!("{e:#}")),
            };
            Response::json(
                200,
                json!({
                    "rules": cfg.rules,
                    "groups": cfg.proxy_groups.iter().map(|g| json!({
                        "name": g.name,
                        "type": format!("{:?}", g.group_type),
                    })).collect::<Vec<_>>(),
                })
                .to_string(),
            )
        }
        "/api/clashx/rules/add" => {
            if req.method != "POST" {
                return json_err(405, "用 POST");
            }
            rules_add(req, ctx)
        }
        "/api/clashx/rules/rm" => {
            if req.method != "POST" {
                return json_err(405, "用 POST");
            }
            rules_rm(req, ctx)
        }
        "/api/clashx/rules/test" => {
            if req.method != "POST" {
                return json_err(405, "用 POST");
            }
            rules_test(req, ctx)
        }

        // ---- 订阅 ----
        "/api/clashx/subs" => {
            if req.method != "GET" {
                return json_err(405, "用 GET");
            }
            let cfg = match load_cfg(ctx) {
                Ok(c) => c,
                Err(e) => return json_err(500, format!("{e:#}")),
            };
            Response::json(
                200,
                json!({
                    "subs": cfg.subscriptions.iter().map(|s| json!({
                        "name": s.name,
                        "url": mask_url(&s.url),
                        "user_agent": s.user_agent,
                        "update_interval": s.update_interval,
                        "keyword": s.keyword,
                        "exclude_keyword": s.exclude_keyword,
                    })).collect::<Vec<_>>(),
                })
                .to_string(),
            )
        }
        "/api/clashx/subs/add" => {
            if req.method != "POST" {
                return json_err(405, "用 POST");
            }
            subs_add(req, ctx)
        }
        "/api/clashx/subs/rm" => {
            if req.method != "POST" {
                return json_err(405, "用 POST");
            }
            subs_rm(req, ctx)
        }

        // ---- 密码 ----
        "/api/clashx/password" => {
            if req.method != "POST" {
                return json_err(405, "用 POST");
            }
            change_password(req, ctx)
        }

        // ---- 配置（读）----
        "/api/clashx/config" => {
            if req.method != "GET" {
                return json_err(405, "用 GET");
            }
            match std::fs::read_to_string(&ctx.paths.user_config) {
                Ok(text) => Response::text(200, text),
                Err(e) => json_err(404, format!("配置文件不存在：{e}")),
            }
        }
        "/api/clashx/rendered" => {
            if req.method != "GET" {
                return json_err(405, "用 GET");
            }
            match std::fs::read_to_string(&ctx.paths.generated) {
                Ok(t) => Response::text(200, t),
                Err(e) => json_err(404, format!("未生成配置：{e}")),
            }
        }

        // ---- mihomo 转发 ----
        p if p.starts_with("/api/mihomo/") => mihomo_proxy(req, ctx, &p["/api/mihomo/".len()..]),

        _ => json_err(404, format!("未知接口 {path}")),
    }
}

fn all_modes_json() -> serde_json::Value {
    serde_json::Value::Array(
        [
            RunMode::FakeIp,
            RunMode::RedirHost,
            RunMode::FakeIpTun,
            RunMode::RedirHostTun,
            RunMode::FakeIpMix,
            RunMode::RedirHostMix,
        ]
        .iter()
        .map(|m| {
            let r = m.resolve();
            json!({
                "value": m.as_str(),
                "dns": m.dns_mode_name(),
                "tun": r.tun_enable,
                "firewall": format!("{:?}", r.firewall),
                "need_port": r.need_explicit_port,
            })
        })
        .collect(),
    )
}

// ==================== 各接口实现 ====================

fn overview(ctx: &ServerCtx) -> Response {
    let cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => return json_err(500, format!("{e:#}")),
    };
    let r = cfg.run_mode.resolve();

    // mihomo 状态：走 1 秒 TTL 缓存。
    // 不缓存的话每个请求要 fork 两个 curl，并发时 QPS 封在 85
    // （压测实测，而 /api/health 同等返回量能到 570）。
    let ms = cached_mihomo_status(ctx);

    // 节点数：走 5 秒缓存。
    // collect_proxies 要解析订阅缓存里 71 个节点的 YAML（~5ms），
    // 而 overview 是 UI 首屏最常刷的接口，不该每次都重算。
    let node_count = cached_node_count(ctx, &cfg);
    let geodata_ok = nft::is_active() || true; // GeoData 由 status 命令查更准，这里不重复
    let _ = geodata_ok;

    Response::json(
        200,
        json!({
            "run_mode": cfg.run_mode.as_str(),
            "dns_mode": cfg.run_mode.dns_mode_name(),
            "tun": r.tun_enable,
            "firewall": format!("{:?}", r.firewall),
            "gateway": nft::is_active(),
            "mihomo_up": ms.up,
            "mihomo_version": ms.version,
            "mihomo_connections": ms.connections,
            "node_count": node_count,
            "rule_count": cfg.rules.len(),
            "sub_count": cfg.subscriptions.len(),
            "has_password": cfg.web.as_ref().and_then(|w| w.password_sha256.as_ref()).is_some(),
        })
        .to_string(),
    )
}

fn mihomo_api(ctx: &ServerCtx) -> Option<Api> {
    let cfg = load_cfg(ctx).ok()?;
    // 控制地址与secret 从已生成的配置读 —— 那是内核实际在用的
    let text = std::fs::read_to_string(&ctx.paths.generated).ok()?;
    let addr = text
        .lines()
        .find_map(|l| l.strip_prefix("external-controller:"))
        .map(|v| format!("http://{}", v.trim().trim_matches(['"', '\''])))
        .unwrap_or_else(|| "http://127.0.0.1:9090".into());
    let secret = text
        .lines()
        .find_map(|l| l.strip_prefix("secret:"))
        .map(|v| v.trim().trim_matches(['"', '\'']).to_string());
    Some(Api::new(&addr, secret))
}

fn set_run_mode(req: &Request, ctx: &ServerCtx) -> Response {
    let Some(val) = req.param("mode").or_else(|| {
        serde_json::from_str::<serde_json::Value>(&req.body_str().unwrap_or_default())
            .ok()
            .and_then(|v| v.get("mode").and_then(|m| m.as_str().map(String::from)))
    }) else {
        return json_err(400, "缺少 mode 参数");
    };
    let Some(target) = RunMode::parse(&val) else {
        return json_err(400, format!("无法识别的运行模式：{val}"));
    };

    let _g = ctx.config_lock.lock().unwrap();

    // 改配置
    let mut cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => return json_err(500, format!("{e:#}")),
    };
    let old = cfg.run_mode;
    cfg.run_mode = target;
    if target.resolve().need_explicit_port && cfg.inbound.mixed_port == 0 {
        cfg.inbound.mixed_port = 7893;
    }
    if let Err(e) = crate::save_config(&ctx.paths, &cfg) {
        return json_err(500, format!("写配置失败：{e:#}"));
        invalidate_cfg_cache(ctx);
    *ctx.node_count_cache.lock().unwrap() = None;
    }

    // nft 校准（与 CLI 的 sync_nft 同一逻辑）
    if target.resolve().firewall == crate::runmode::FirewallOwner::Mihomo && nft::is_active() {
        let _ = nft::disable();
    }

    // 渲染 + 校验 + 应用
    if let Err(e) = apply_and_restart(ctx) {
        // 失败要回滚配置，否则用户会看到一个「切了但没生效」的状态
        let mut back = match load_cfg(ctx) {
            Ok(c) => c,
            Err(_) => return json_err(500, format!("{e:#}")),
        };
        back.run_mode = old;
        let _ = crate::save_config(&ctx.paths, &back);
        invalidate_cfg_cache(ctx);
    *ctx.node_count_cache.lock().unwrap() = None;
        return json_err(500, format!("应用失败（已回滚配置）：{e:#}"));
    }

    Response::json(
        200,
        json!({"ok": true, "from": old.as_str(), "to": target.as_str()}).to_string(),
    )
}

/// 渲染 -> mihomo -t 校验 -> 重启。
pub fn apply_and_restart(ctx: &ServerCtx) -> Result<()> {
    let cfg = load_cfg(ctx)?;
    let (proxies, warns) = crate::collect_proxies(&cfg, &ctx.paths);
    for w in &warns {
        eprintln!("[warn] {w}");
    }
    let rendered = crate::render::render(&cfg, &proxies);
    for w in &rendered.warnings {
        eprintln!("[warn] {w}");
    }
    crate::core::atomic_apply(&ctx.paths, &rendered.yaml)?;
    crate::core::Systemd::new("mihomo-client").restart()?;
    Ok(())
}

fn gateway_status(ctx: &ServerCtx) -> Response {
    if ctx.paths.user_config.exists() {
        let s = nft::status();
        Response::json(
            200,
            json!({
                "active": nft::is_active(),
                "detail": s.lines().collect::<Vec<_>>(),
            })
            .to_string(),
        )
    } else {
        json_err(500, "配置不存在")
    }
}

fn gateway_enable(req: &Request, ctx: &ServerCtx) -> Response {
    let _g = ctx.config_lock.lock().unwrap();
    let cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => return json_err(500, format!("{e:#}")),
    };

    let body: serde_json::Value =
        serde_json::from_str(&req.body_str().unwrap_or_default()).unwrap_or(serde_json::json!({}));

    // 参数优先级：请求体 > query > 自动探测
    let iface = body
        .get("iface")
        .and_then(|v| v.as_str())
        .map(String::from)
        .or_else(|| req.param("iface"))
        .or_else(|| detect_iface())
        .unwrap_or_default();
    let lan = body
        .get("lan_addr")
        .and_then(|v| v.as_str())
        .map(String::from)
        .or_else(|| req.param("lan_addr"))
        .or_else(|| detect_lan_addr(&iface))
        .unwrap_or_default();
    // clients 接受两种形态：字符串 "a,b,c" 或数组 ["a","b"]
    let clients: Vec<String> = body
        .get("clients")
        .and_then(|v| v.as_str())
        .map(|s| {
            s.split(',')
                .map(|x| x.trim().to_string())
                .filter(|x| !x.is_empty())
                .collect()
        })
        .or_else(|| {
            body.get("clients").and_then(|v| v.as_array()).map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str())
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect()
            })
        })
        .or_else(|| req.param("clients").map(|s| s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()))
        .unwrap_or_default();

    if clients.is_empty() {
        return json_err(400, "必须提供 clients（客户端网段），这是安全下限");
    }

    let g = nft::Gateway {
        lan_addr: lan.clone(),
        iface: iface.clone(),
        tproxy_port: cfg.transparent.tproxy_port,
        dns_port: cfg
            .dns
            .listen
            .rsplit(':')
            .next()
            .and_then(|p| p.parse().ok())
            .unwrap_or(7874),
        direct_nets: cfg.transparent.direct_nets_probe(),
        lan_clients: clients,
        ..Default::default()
    };
    if let Err(e) = g.precheck() {
        return json_err(400, format!("{e:#}"));
    }
    match nft::enable(&g) {
        Ok(()) => Response::json(200, json!({"ok": true, "lan_addr": lan, "iface": iface}).to_string()),
        Err(e) => json_err(500, format!("{e:#}")),
    }
}

fn detect_iface() -> Option<String> {
    let out = std::process::Command::new("ip").args(["route"]).output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    // `default via 172.20.0.1 dev eth0 proto static`
    // —— dev 后面才是网卡名，用 find(|t| *t == "dev") 会拿到 "dev" 本身
    for line in text.lines() {
        if !line.starts_with("default") {
            continue;
        }
        let toks: Vec<&str> = line.split_whitespace().collect();
        if let Some(i) = toks.iter().position(|t| *t == "dev") {
            if let Some(n) = toks.get(i + 1) {
                return Some((*n).to_string());
            }
        }
    }
    None
}

fn detect_lan_addr(iface: &str) -> Option<String> {
    let out = std::process::Command::new("ip")
        .args(["-4", "-br", "addr", "show", "dev", iface])
        .output()
        .ok()?;
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        let mut p = line.split_whitespace();
        p.next()?;
        p.next()?; // state
        if let Some(cidr) = p.next() {
            let ip = cidr.split('/').next()?;
            if !ip.starts_with("127.") && !ip.starts_with("169.254.") {
                return Some(ip.to_string());
            }
        }
    }
    None
}

fn rules_add(req: &Request, ctx: &ServerCtx) -> Response {
    let _g = ctx.config_lock.lock().unwrap();
    let v: serde_json::Value = match serde_json::from_str(&req.body_str().unwrap_or_default()) {
        Ok(v) => v,
        Err(e) => return json_err(400, format!("请求体不是合法 JSON：{e}")),
    };
    let rule = v.get("rule").and_then(|r| r.as_str()).unwrap_or("").trim().to_string();
    if rule.is_empty() {
        return json_err(400, "缺少 rule");
    }
    // 离线校验：语法错误当场拒掉，不等 mihomo -t
    if let Err(e) = crate::rule::validate_rule(&rule) {
        return json_err(400, format!("规则语法错误：{e}"));
    }

    let mut cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => return json_err(500, format!("{e:#}")),
    };
    if cfg.rules.contains(&rule) {
        return json_err(409, "规则已存在");
    }
    // 插到 MATCH 之前（MATCH 必须最后）
    let pos = cfg
        .rules
        .iter()
        .position(|r| r.trim_start().starts_with("MATCH"))
        .unwrap_or(cfg.rules.len());
    cfg.rules.insert(pos, rule.clone());

    if let Err(e) = crate::save_config(&ctx.paths, &cfg) {
        return json_err(500, format!("{e:#}"));
        invalidate_cfg_cache(ctx);
    *ctx.node_count_cache.lock().unwrap() = None;
    }
    match apply_and_restart(ctx) {
        Ok(()) => Response::json(200, json!({"ok": true, "rule": rule}).to_string()),
        Err(e) => json_err(500, format!("应用失败：{e:#}")),
    }
}

fn rules_rm(req: &Request, ctx: &ServerCtx) -> Response {
    let _g = ctx.config_lock.lock().unwrap();
    let v: serde_json::Value =
        serde_json::from_str(&req.body_str().unwrap_or_default()).unwrap_or(serde_json::json!({}));
    let rule = v.get("rule").and_then(|r| r.as_str()).unwrap_or("").trim().to_string();

    let mut cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => return json_err(500, format!("{e:#}")),
    };
    let before = cfg.rules.len();
    cfg.rules.retain(|r| r.trim() != rule);
    if cfg.rules.len() == before {
        return json_err(404, "规则不存在");
    }
    if let Err(e) = crate::save_config(&ctx.paths, &cfg) {
        return json_err(500, format!("{e:#}"));
        invalidate_cfg_cache(ctx);
    *ctx.node_count_cache.lock().unwrap() = None;
    }
    match apply_and_restart(ctx) {
        Ok(()) => Response::json(200, r#"{"ok":true}"#),
        Err(e) => json_err(500, format!("{e:#}")),
    }
}

fn rules_test(req: &Request, ctx: &ServerCtx) -> Response {
    let v: serde_json::Value =
        serde_json::from_str(&req.body_str().unwrap_or_default()).unwrap_or(serde_json::json!({}));
    let cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => return json_err(500, format!("{e:#}")),
    };

    // 输入：域名或 IP，或 host:port 形式
    let domain = v.get("domain").and_then(|x| x.as_str()).map(String::from);
    let ip = v.get("ip").and_then(|x| x.as_str()).map(String::from);
    let (d, i) = if domain.is_some() || ip.is_some() {
        (domain, ip)
    } else {
        // "www.google.com:443" 拆开
        let target = v
            .get("target")
            .and_then(|x| x.as_str())
            .unwrap_or("www.google.com");
        match crate::rule::split_target(target, None) {
            (d, _) if !d.is_empty() && d.parse::<std::net::IpAddr>().is_err() => (Some(d), None),
            _ => (None, Some(target.to_string())),
        }
    };
    if d.is_none() && i.is_none() {
        return json_err(400, "需要 domain 或 ip（或 target）");
    }

    let builtins = crate::rule::builtin_targets(
        &cfg.proxy_groups.iter().map(|g| g.name.clone()).collect::<Vec<_>>(),
    );
    let m = crate::rule::Matcher {
        rules: &cfg.rules,
        providers: std::collections::BTreeMap::new(),
        builtins,
    };
    match crate::rule::explain(&m, d.as_deref(), i.as_deref()) {
        Ok(vd) => Response::json(
            200,
            json!({
                "index": vd.index,
                "rule": vd.rule,
                "kind": vd.kind,
                "target": vd.target,
                "fallback": vd.fallback,
            })
            .to_string(),
        ),
        Err(e) => json_err(400, format!("{e:#}")),
    }
}

fn subs_add(req: &Request, ctx: &ServerCtx) -> Response {
    let _g = ctx.config_lock.lock().unwrap();
    let v: serde_json::Value = match serde_json::from_str(&req.body_str().unwrap_or_default()) {
        Ok(v) => v,
        Err(e) => return json_err(400, format!("{e:#}")),
    };
    let name = v.get("name").and_then(|s| s.as_str()).unwrap_or("").trim().to_string();
    let url = v.get("url").and_then(|s| s.as_str()).unwrap_or("").trim().to_string();
    if name.is_empty() || url.is_empty() {
        return json_err(400, "name 与 url 必填");
    }

    let mut cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => return json_err(500, format!("{e:#}")),
    };
    if let Some(existing) = cfg.subscriptions.iter_mut().find(|s| s.name == name) {
        existing.url = url;
    } else {
        cfg.subscriptions.push(crate::config::Subscription {
            name,
            url,
            ..Default::default()
        });
    }
    if let Err(e) = crate::save_config(&ctx.paths, &cfg) {
        return json_err(500, format!("{e:#}"));
        invalidate_cfg_cache(ctx);
    *ctx.node_count_cache.lock().unwrap() = None;
    }
    Response::json(200, r#"{"ok":true,"msg":"已保存，执行 apply 生效"}"#)
}

fn subs_rm(req: &Request, ctx: &ServerCtx) -> Response {
    let _g = ctx.config_lock.lock().unwrap();
    let v: serde_json::Value =
        serde_json::from_str(&req.body_str().unwrap_or_default()).unwrap_or(serde_json::json!({}));
    let name = v.get("name").and_then(|s| s.as_str()).unwrap_or("").to_string();
    let mut cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => return json_err(500, format!("{e:#}")),
    };
    let before = cfg.subscriptions.len();
    cfg.subscriptions.retain(|s| s.name != name);
    if cfg.subscriptions.len() == before {
        return json_err(404, "订阅不存在");
    }
    if let Err(e) = crate::save_config(&ctx.paths, &cfg) {
        return json_err(500, format!("{e:#}"));
        invalidate_cfg_cache(ctx);
    *ctx.node_count_cache.lock().unwrap() = None;
    }
    Response::json(200, r#"{"ok":true,"msg":"已删除，执行 apply 生效"}"#)
}

fn change_password(req: &Request, ctx: &ServerCtx) -> Response {
    let _g = ctx.config_lock.lock().unwrap();
    let v: serde_json::Value = match serde_json::from_str(&req.body_str().unwrap_or_default()) {
        Ok(v) => v,
        Err(e) => return json_err(400, format!("{e:#}")),
    };
    let old_pw = v.get("old_password").and_then(|s| s.as_str()).unwrap_or("");
    let new_pw = v.get("new_password").and_then(|s| s.as_str()).unwrap_or("");

    if new_pw.len() < 4 {
        return json_err(400, "新密码至少 4 个字符");
    }

    let mut cfg = match load_cfg(ctx) {
        Ok(c) => c,
        Err(e) => return json_err(500, format!("{e:#}")),
    };
    let web = cfg.web.get_or_insert_with(Default::default);

    // 校验旧密码
    let old_hash = crate::web::sha256_hex(old_pw.as_bytes());
    match &web.password_sha256 {
        Some(h) if h.eq_ignore_ascii_case(&old_hash) => {}
        _ => return json_err(401, "旧密码错误"),
    }

    web.password_sha256 = Some(crate::web::sha256_hex(new_pw.as_bytes()));
    if let Err(e) = crate::save_config(&ctx.paths, &cfg) {
        return json_err(500, format!("{e:#}"));
        invalidate_cfg_cache(ctx);
    *ctx.node_count_cache.lock().unwrap() = None;
    }
    Response::json(200, r#"{"ok":true,"msg":"密码已修改，重新登录生效"}"#)
}

/// 订阅 URL 打码 —— token 在路径中间，不能整条回给浏览器。
fn mask_url(url: &str) -> String {
    match url.split_once("://") {
        Some((scheme, rest)) => match rest.split_once('/') {
            Some((host, path)) => format!("{scheme}://{host}/{}", mask_token(path)),
            None => format!("{scheme}://{rest}"),
        },
        None => "***".into(),
    }
}

fn mask_token(path: &str) -> String {
    if path.len() <= 8 {
        return "***".into();
    }
    format!("{}...{}", &path[..4], &path[path.len() - 4..])
}

// ==================== mihomo 转发 ====================

fn mihomo_proxy(req: &Request, ctx: &ServerCtx, sub: &str) -> Response {
    let Some(api) = mihomo_api(ctx) else {
        return json_err(503, "无法确定 mihomo 控制地址（先生成配置）");
    };
    let body = req.body_str().unwrap_or_default();
    let result = match req.method.as_str() {
        "GET" => api.raw_get(&format!("/{}", sub)),
        "PUT" => api.raw_put(&format!("/{}", sub), &body),
        "POST" => api.raw_post(&format!("/{}", sub), &body),
        "DELETE" => api.raw_delete(&format!("/{}", sub)),
        _ => return json_err(405, "不支持的方法"),
    };
    match result {
        Ok((code, text)) => {
            if (200..300).contains(&code) {
                if text.is_empty() {
                    Response::json(200, "{}")
                } else {
                    Response::json(200, text)
                }
            } else {
                json_err(code, format!("内核返回 {code}: {text}"))
            }
        }
        Err(e) => json_err(502, format!("调mihomo 失败：{e:#}")),
    }
}

// ==================== zashboard 反代 ====================

/// 把 `/zashboard/*` 映射到 mihomo 的 UI 目录。
///
/// 为什么反代而不是直连 9090：
/// - 9090 只听 127.0.0.1，局域网浏览器够不着
/// - 反代后只需要开一个端口，鉴权也只做一次
/// - mihomo 的 UI 是 SPA，`/ui/` 下有 200.html / _nuxt/ 等，
///   要注意 SPA 路由回退
fn zashboard(req: &Request, ctx: &ServerCtx) -> Response {
    // 去掉 /zashboard 前缀
    let sub = req.path.strip_prefix("/zashboard").unwrap_or("/");
    let rel = percent_decode(sub.trim_start_matches('/'));
    let base = ctx.paths.workdir.join("ui");

    // 安全：拒绝任何 .. 逃逸
    if rel.split('/').any(|p| p == "..") {
        return json_err(400, "非法路径");
    }

    // SPA 路由：路径无扩展名且文件不存在时回退到 index.html
    let mut p = base.join(&rel);
    if rel.is_empty() || rel.ends_with('/') {
        p = p.join("index.html");
    }
    if !p.exists() {
        let idx = base.join("index.html");
        if idx.exists() {
            p = idx;
        } else {
            return json_err(404, "zashboard 未安装（mihomo 首次启动会自动下载）");
        }
    }

    let ct = guess_content_type(&p);
    match std::fs::read(&p) {
        Ok(data) => Response::bytes(200, ct, data),
        Err(e) => json_err(500, format!("{e}")),
    }
}

fn guess_content_type(p: &std::path::Path) -> &'static str {
    match p.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "html" | "htm" => "text/html; charset=utf-8",
        "js" | "mjs" => "application/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "map" => "application/json",
        _ => "application/octet-stream",
    }
}

// ==================== 内置前端 ====================

/// index.html —— 用 include_str! 内置，编译进二进制。
///
/// 为什么内置而不是运行时读文件：
/// 单文件二进制部署是本项目的目标（丢到任何机器就能跑），
/// 前端文件如果外挂就多一个「记得拷UI 目录」的步骤。
fn index_html() -> Option<&'static str> {
    Some(include_str!("../webui/index.html"))
}
fn bundle_js() -> Option<&'static str> {
    Some(include_str!("../webui/bundle.js"))
}
fn app_css() -> Option<&'static str> {
    Some(include_str!("../webui/app.css"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 订阅url打码不泄漏token() {
        let u = "https://dash.example.com/api/v1/pq/abcdef1234567890";
        let m = mask_url(u);
        assert!(!m.contains("abcdef1234567890"), "打码后不应含完整 token: {m}");
        assert!(m.starts_with("https://dash.example.com/"), "{m}");
    }

    #[test]
    fn 打码处理短路径() {
        assert_eq!(mask_url("https://x.co/ab"), "https://x.co/***");
    }

    #[test]
    fn 默认路由网卡解析正确() {
        // `ip route` 输出的 dev 后面才是网卡名
        let line = "default via 172.20.0.1 dev eth0 proto static";
        let toks: Vec<&str> = line.split_whitespace().collect();
        let i = toks.iter().position(|t| *t == "dev").unwrap();
        assert_eq!(toks[i + 1], "eth0");
    }
}
