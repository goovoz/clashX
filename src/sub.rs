//! 订阅抓取与格式识别。
//!
//! 机场按 **User-Agent 分流返回不同格式**，实测（192.168.10.1 上对
//! <机场域名> 的探测）：
//!
//! | UA | Content-Type | 大小 | 内容 |
//! |----|--------------|------|------|
//! | `clash.meta` | text/html | 567KB | **完整 clash YAML** |
//! | `ClashforWindows/0.19.23` | text/html | 197KB | 完整 clash YAML（字段少些）|
//! | `v2rayNG` | text/html | 36KB | **base64 编码的 vless:// 链接列表** |
//! | `sing-box` | application/json | 422KB | sing-box JSON |
//!
//! 所以默认 UA 必须是 `clash.meta`（或含 clash/meta 的串）—— 我们要
//! 的就是 YAML，其他格式的解析成本高很多且信息更少。
//!
//! 另：响应头带订阅信息，UI 应该显示流量：
//! ```
//! subscription-userinfo: upload=46456708; download=3488436957;
//!                        total=1073741824000; expire=1867235068
//! profile-update-interval: 24
//! ```

use anyhow::{anyhow, bail, Context, Result};
use base64::Engine;
use serde_yaml::Value;
use std::collections::BTreeMap;

use crate::config::{Proxy, Subscription};

/// 订阅响应里我们关心的信息。
#[derive(Debug, Clone, Default)]
pub struct SubInfo {
    /// 已用流量（字节）
    pub upload: Option<u64>,
    pub download: Option<u64>,
    pub total: Option<u64>,
    /// 到期时间（Unix 秒）
    pub expire: Option<i64>,
    /// 建议更新间隔（小时）
    pub update_interval: Option<u64>,
}

impl SubInfo {
    /// 已用流量百分比。
    pub fn used_percent(&self) -> Option<f64> {
        let (u, d, t) = (self.upload?, self.download?, self.total?);
        if t == 0 {
            return None;
        }
        Some((u + d) as f64 * 100.0 / t as f64)
    }

    /// 人类可读的剩余流量。
    pub fn remaining_human(&self) -> Option<String> {
        let t = self.total?;
        let used = self.upload.unwrap_or(0) + self.download.unwrap_or(0);
        let left = t.saturating_sub(used);
        Some(format!(
            "{:.2} GB / {:.2} GB",
            left as f64 / 1e9,
            t as f64 / 1e9
        ))
    }

    /// 到期时间的可读形式。
    pub fn expire_human(&self) -> Option<String> {
        let e = self.expire?;
        if e == 0 {
            return Some("不限".into());
        }
        // Unix 秒 -> 本地日期。避免引入 chrono（依赖已刻意保持最小），
        // 用 libc 的 localtime_r 也不方便，这里用 UTC 近似 + 手工格式化。
        let days = e / 86400;
        let (y, m, d) = civil_from_days(days);
        Some(format!("{y}-{m:02}-{d:02}"))
    }
}

/// 从「自1970-01-01 起的天数」算日历日期（Howard Hinnant 算法）。
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// 订阅内容格式。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// 完整 clash YAML（proxies + proxy-groups + rules）
    ClashYaml,
    /// base64 编码的节点链接列表（vless://、ss:// 等）
    Base64Links,
    /// SIP008（JSON with servers[]，Shadowsocks 订阅标准）
    Sip008,
    /// sing-box JSON
    SingBox,
    /// 看不懂
    Unknown,
}

/// 识别格式。
///
/// 顺序有讲究：先看有没有 mihomo 能吃的 `proxies:`，再看 JSON 特征，
/// 最后才尝试 base64 —— 因为**很多 clash YAML 也能被 base64 解出乱码**，
/// 先判 YAML 更可靠。
pub fn detect_format(body: &str) -> Format {
    let trimmed = body.trim_start();
    if trimmed.is_empty() {
        return Format::Unknown;
    }

    // 1) clash YAML：顶层有 proxies: 或 proxy-groups:
    if trimmed.starts_with('{') {
        // JSON —— sing-box 或 SIP008
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(body) {
            if v.get("servers").is_some() {
                return Format::Sip008;
            }
            if v.get("outbounds").is_some() {
                return Format::SingBox;
            }
        }
        return Format::Unknown;
    }

    // 2) YAML：找顶层 `proxies:` / `proxy-groups:` / `rules:`
    //
    // ★ 曾经在这里加过一个「遇到其它顶层键就 break」的提前退出，
    // 结果真实订阅的第一行是 `mixed-port: 7890` —— 顶层键且含冒号，
    // 直接 break，后面真正的 `proxies:` 永远扫不到，567KB 的完整 YAML
    // 被误判成 Base64Links（真机实测踩过）。
    //
    // 正确做法：**扫全文**找顶层 proxies/proxy-groups，
    // 找不到再回退到其它格式判定。YAML 文件几百 KB，
    // 线性扫描的成本可以忽略。
    for line in trimmed.lines() {
        let l = line.trim_end();
        if l.starts_with("proxies:") || l.starts_with("proxy-groups:") {
            return Format::ClashYaml;
        }
    }

    // 3) 尝试 base64 解码，解出来像链接列表就算
    if let Some(dec) = try_base64(body) {
        let d = String::from_utf8_lossy(&dec);
        let t = d.trim();
        if t.contains("://") && (t.contains("vless://") || t.contains("ss://")
            || t.contains("vmess://") || t.contains("trojan://"))
        {
            return Format::Base64Links;
        }
    }

    // 4) 本身就��链接列表（未 base64）
    if trimmed.contains("://") {
        return Format::Base64Links;
    }

    Format::Unknown
}

fn try_base64(s: &str) -> Option<Vec<u8>> {
    // 机场返回的 base64 常带换行；先去掉所有空白
    let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    base64::engine::general_purpose::STANDARD
        .decode(cleaned.as_bytes())
        .ok()
        .or_else(|| {
            base64::engine::general_purpose::URL_SAFE
                .decode(cleaned.as_bytes())
                .ok()
        })
        .or_else(|| {
            base64::engine::general_purpose::STANDARD_NO_PAD
                .decode(cleaned.as_bytes())
                .ok()
        })
}

/// 从响应头解析订阅信息。
pub fn parse_sub_headers(h: &BTreeMap<String, String>) -> SubInfo {
    let mut info = SubInfo::default();
    let get = |k: &str| -> Option<&String> { h.get(k) };
    if let Some(v) = get("subscription-userinfo") {
        for part in v.split(';') {
            let part = part.trim();
            if let Some((k, val)) = part.split_once('=') {
                let n: u64 = val.trim().parse().unwrap_or(0);
                match k.trim() {
                    "upload" => info.upload = Some(n),
                    "download" => info.download = Some(n),
                    "total" => info.total = Some(n),
                    "expire" => info.expire = val.trim().parse().ok(),
                    _ => {}
                }
            }
        }
    }
    if let Some(v) = get("profile-update-interval") {
        info.update_interval = v.trim().parse().ok();
    }
    info
}

/// 抓取订阅。
///
/// 走 curl 而不用 reqwest —— 保持依赖最小（见 Cargo.toml 注释）。
/// mihomo 内核已经在系统里，curl 也必然有。
pub fn fetch(sub: &Subscription, base_url: &str) -> Result<(String, BTreeMap<String, String>)> {
    if sub.url.is_empty() {
        bail!("订阅 {} 没有 URL", sub.name);
    }
    let url = with_params(&sub.url, &sub.extra_params);

    let ua = sub
        .user_agent
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("clash.meta");

    // 写到临时文件再读：响应可能几百 KB，且要同时拿 body 与 header。
    // 文件名带 pid +纳秒，避免并发运行（apply 与 sub test 同时跑）时互相覆盖。
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp = std::env::temp_dir()
        .join(format!("mihomo-client-sub-{}-{stamp}", std::process::id()));
    let hdr = tmp.with_extension("hdr");

    let mut cmd = std::process::Command::new("curl");
    cmd.arg("-sS")
        .arg("--max-time")
        .arg("60")
        .arg("--compressed")
        .arg("-A")
        .arg(ua)
        .arg("-D")
        .arg(&hdr)
        .arg("-o")
        .arg(&tmp);
    for h in &sub.headers {
        // 每行一个自定义请求头
        if let Some((k, v)) = h.split_once(':') {
            cmd.arg("-H").arg(format!("{}: {}", k.trim(), v.trim()));
        }
    }
    cmd.arg(&url);

    // 临时诊断：把实际执行的 curl 命令打出来（URL 里的 token 打码）
    if std::env::var_os("MIHOMO_CLIENT_DEBUG").is_some() {
        let shown = url.split('/').enumerate().map(|(i, seg)| {
            if i == 2 {
                seg.chars().take(12).collect::<String>() + "..."
            } else {
                seg.to_string()
            }
        }).collect::<Vec<_>>().join("/");
        eprintln!("[DEBUG] curl -sS --max-time 60 --compressed -A {ua:?} -D <hdr> -o <tmp> {shown}");
        eprintln!("[DEBUG]   tmp={} hdr={}", tmp.display(), hdr.display());
    }

    let out = cmd.output().context("执行 curl 失败（需要 curl）")?;
    let code = out.status.code().unwrap_or(-1);
    if code != 0 {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        let _ = std::fs::remove_file(&tmp);
        bail!("抓取订阅失败（curl {code}）：{err}");
    }
    // ★ 不要 unwrap_or_default() —— 读不到body 时会静默变成空串，
    // 然后被 detect_format 判成 Unknown，用户看到的是「格式不对」，
    // 真实原因（临时目录不可写）被完全掩盖。
    let body = std::fs::read_to_string(&tmp).map_err(|e| {
        anyhow!("读取订阅响应失败（临时文件 {}）：{e}", tmp.display())
    })?;
    if std::env::var_os("MIHOMO_CLIENT_DEBUG").is_some() {
        let _ = std::fs::write("/tmp/mihomo-client-sub-debug.txt", &body);
        eprintln!("[DEBUG] body已存 /tmp/mihomo-client-sub-debug.txt ({}B)", body.len());
    }
    let _ = std::fs::remove_file(&tmp);
    if body.is_empty() {
        bail!("订阅返回空内容（curl 退出码 {code}），可能是 URL 失效或被限流");
    }

    let mut headers = BTreeMap::new();
    if let Ok(text) = std::fs::read_to_string(&hdr) {
        for line in text.lines() {
            if let Some((k, v)) = line.split_once(':') {
                headers.insert(k.trim().to_ascii_lowercase(), v.trim().to_string());
            }
        }
    }
    let _ = std::fs::remove_file(&hdr);

    // HTTP 错误状态码检查
    for (k, v) in &headers {
        if k == "status" {
            bail!("订阅返回 HTTP {v}");
        }
    }
    let _ = base_url;

    Ok((body, headers))
}

/// 给订阅 URL 追加额外查询参数。
///
/// 分隔符判断只看 **第一个** 参数之前 URL 里有没有 `?`；
/// 后续参数一律用 `&`（否则会拼成 `?a=1&b=2?c=3` —— 被单测抓到过）。
fn with_params(url: &str, params: &[String]) -> String {
    if params.is_empty() {
        return url.to_string();
    }
    let sep = if url.contains('?') { '&' } else { '?' };
    let tail: Vec<String> = params
        .iter()
        .map(|p| p.trim().trim_start_matches(['?', '&']).to_string())
        .filter(|p| !p.is_empty())
        .collect();
    if tail.is_empty() {
        return url.to_string();
    }
    format!("{url}{sep}{}", tail.join("&"))
}

/// 从订阅内容里提取节点列表。
pub fn extract_proxies(body: &str, format: Format) -> Result<Vec<Proxy>> {
    match format {
        Format::ClashYaml => proxies_from_yaml(body),
        Format::Base64Links => bail!(
            "订阅返回 base64 节点链接列表（vless://、ss:// 等）。\
             请把订阅的 User-Agent 改成 clash.meta 以获取 YAML 格式，\
             或在 config.yaml 的该订阅里设 user-agent: clash.meta"
        ),
        Format::Sip008 => bail!(
            "订阅返回 SIP008 格式（Shadowsocks 专用）。\
             请把 User-Agent 改成 clash.meta"
        ),
        Format::SingBox => bail!(
            "订阅返回 sing-box JSON。请把 User-Agent 改成 clash.meta"
        ),
        Format::Unknown => bail!("无法识别订阅格式（前 80 字符：{:?}）", truncate(body, 80)),
    }
}

fn truncate(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

fn proxies_from_yaml(body: &str) -> Result<Vec<Proxy>> {
    let v: Value = serde_yaml::from_str(body).context("订阅 YAML 解析失败")?;
    let arr = v
        .get("proxies")
        .and_then(|p| p.as_sequence())
        .ok_or_else(|| anyhow!("订阅 YAML 里没有 proxies 段"))?;

    let mut out = Vec::with_capacity(arr.len());
    for item in arr {
        let m: serde_yaml::Mapping = match item {
            Value::Mapping(m) => m.clone(),
            _ => continue,
        };
        let get = |k: &str| -> Option<String> {
            m.get(Value::String(k.into()))
                .and_then(|v| v.as_str().map(|s| s.to_string()))
        };
        let name = match get("name") {
            Some(n) if !n.is_empty() => n,
            _ => continue,
        };
        let ptype = match get("type") {
            Some(t) if !t.is_empty() => t,
            _ => continue,
        };
        let server = get("server").unwrap_or_default();
        let port = m
            .get(Value::String("port".into()))
            .and_then(|v| v.as_u64())
            .unwrap_or(0) as u16;

        // 其余字段（含协议专有的 ws-opts / reality-opts / grpc-opts 等）
        // 原样保留 —— 机场订阅里的这些字段是节点能不能用的关键，
        // 丢了就等于白抓。
        let mut extra = serde_yaml::Mapping::new();
        for (k, val) in m.iter() {
            if let Value::String(key) = k {
                if key != "name" && key != "type" && key != "server" && key != "port" {
                    extra.insert(k.clone(), val.clone());
                }
            }
        }

        out.push(Proxy {
            name,
            proxy_type: ptype,
            server,
            port,
            extra,
        });
    }
    Ok(out)
}

/// 按订阅配置过滤节点。
///
/// keyword 命中任一即保留；exclude_keyword 命中任一即丢弃。
/// 都不配时全部保留。
pub fn filter_proxies(mut proxies: Vec<Proxy>, sub: &Subscription) -> Vec<Proxy> {
    let kw: Vec<&str> = sub.keyword.iter().map(|s| s.as_str()).collect();
    let ex: Vec<&str> = sub.exclude_keyword.iter().map(|s| s.as_str()).collect();
    if kw.is_empty() && ex.is_empty() {
        return proxies;
    }
    proxies.retain(|p| {
        let n = &p.name;
        if ex.iter().any(|k| n.contains(k)) {
            return false;
        }
        if kw.is_empty() {
            return true;
        }
        kw.iter().any(|k| n.contains(k))
    });
    proxies
}

/// 多订阅合并：同名节点加来源前缀去重。
///
/// 保留顺序稳定（按订阅在配置里的顺序），这样每次渲染结果一致，
/// 不会因为 HashMap 顺序变化导致配置抖动。
pub fn merge(subs: &[(String, Vec<Proxy>)]) -> Vec<Proxy> {
    let mut out: Vec<Proxy> = Vec::new();
    // 已出现的节点名（小写比较 —— 节点名大小写差异在机场里很常见，
    // 但对 mihomo 是两个不同节点，容易让用户困惑）
    let mut seen: Vec<String> = Vec::new();

    for (src, list) in subs {
        for p in list.iter().cloned() {
            let base = p.name.clone();
            // 同名（含大小写差异）加来源前缀
            let mut candidate = if seen.iter().any(|s| s.eq_ignore_ascii_case(&base)) {
                format!("{base}@{src}")
            } else {
                base.clone()
            };
            // 加前缀后仍可能撞车（两个订阅同名，且已有 xxx@src）
            let mut n = 2;
            while seen.iter().any(|s| s.eq_ignore_ascii_case(&candidate)) {
                candidate = format!("{base}@{src}#{n}");
                n += 1;
            }
            seen.push(candidate.to_ascii_lowercase());
            out.push(Proxy { name: candidate, ..p });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 识别_clash_yaml() {
        let y = "proxies:\n  - name: a\n    type: ss\nproxy-groups: []\n";
        assert_eq!(detect_format(y), Format::ClashYaml);
    }

    #[test]
    fn 识别_顶层键开头的真实订阅() {
        // 真实订阅（<机场域名>，2026-10-04）的开头就是这样：
        // proxies: 在第 17 行，前面有十几个顶层键。第一版实现里有个
        // 「遇到其它顶层键就 break」的提前退出，导致这里被误判成 Base64Links。
        let y = "mixed-port: 7890\n\
                 allow-lan: false\n\
                 bind-address: '*'\n\
                 mode: rule\n\
                 dns:\n    \
                 enable: true\n    \
                 nameserver: [223.5.5.5]\n\
                 proxies:\n    \
                 - { name: 'x', type: vless, server: a.com, port: 443 }\n\
                 proxy-groups: []\n\
                 rules:\n    \
                 - MATCH,DIRECT\n";
        assert_eq!(detect_format(y), Format::ClashYaml);
    }

    #[test]
    fn 识别_json() {
        assert_eq!(detect_format(r#"{"servers":[]}"#), Format::Sip008);
        assert_eq!(detect_format(r#"{"outbounds":[]}"#), Format::SingBox);
    }

    #[test]
    fn 识别_base64_链接() {
        let links = "vless://uuid@host:443?security=tls#node1\n";
        let b = base64::engine::general_purpose::STANDARD.encode(links.as_bytes());
        assert_eq!(detect_format(&b), Format::Base64Links);
    }

    #[test]
    fn 提取节点保留协议专有字段() {
        let y = r#"
proxies:
  - name: "HK-01"
    type: vless
    server: hk.example.com
    port: 443
    uuid: abc-123
    tls: true
    reality-opts:
      public-key: pk
      short-id: ab
"#;
        let ps = proxies_from_yaml(y).unwrap();
        assert_eq!(ps.len(), 1);
        assert_eq!(ps[0].name, "HK-01");
        assert_eq!(ps[0].proxy_type, "vless");
        assert_eq!(ps[0].server, "hk.example.com");
        assert_eq!(ps[0].port, 443);
        // 协议专有字段必须保留
        assert!(ps[0].extra.contains_key(Value::String("reality-opts".into())));
        assert!(ps[0].extra.contains_key(Value::String("uuid".into())));
    }

    #[test]
    fn 关键词过滤() {
        let mk = |n: &str| Proxy {
            name: n.into(),
            proxy_type: "ss".into(),
            server: "s".into(),
            port: 1,
            extra: serde_yaml::Mapping::new(),
        };
        let list = vec![mk("香港 01"), mk("日本 02"), mk("台湾 03")];
        let mut sub = Subscription::default();
        sub.keyword = vec!["港".into(), "日本".into()];
        let got = filter_proxies(list, &sub);
        assert_eq!(got.len(), 2);
        assert!(got.iter().any(|p| p.name.contains("香港")));
        assert!(got.iter().any(|p| p.name.contains("日本")));
    }

    #[test]
    fn 排除关键词优先() {
        let mk = |n: &str| Proxy {
            name: n.into(),
            proxy_type: "ss".into(),
            server: "s".into(),
            port: 1,
            extra: serde_yaml::Mapping::new(),
        };
        let list = vec![mk("香港 01"), mk("香港 过期")];
        let mut sub = Subscription::default();
        sub.keyword = vec!["香港".into()];
        sub.exclude_keyword = vec!["过期".into()];
        let got = filter_proxies(list, &sub);
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].name, "香港 01");
    }

    #[test]
    fn 多订阅同名加来源前缀() {
        let mk = |n: &str| Proxy {
            name: n.into(),
            proxy_type: "ss".into(),
            server: "s".into(),
            port: 1,
            extra: serde_yaml::Mapping::new(),
        };
        let merged = merge(&[
            ("机场A".into(), vec![mk("HK-01"), mk("JP-01")]),
            ("机场B".into(), vec![mk("HK-01")]),
        ]);
        let names: Vec<&str> = merged.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, vec!["HK-01", "JP-01", "HK-01@机场B"]);
    }

    #[test]
    fn 大小写不同也算重复() {
        let mk = |n: &str| Proxy {
            name: n.into(),
            proxy_type: "ss".into(),
            server: "s".into(),
            port: 1,
            extra: serde_yaml::Mapping::new(),
        };
        let merged = merge(&[
            ("A".into(), vec![mk("HK-01")]),
            ("B".into(), vec![mk("hk-01")]),
        ]);
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[1].name, "hk-01@B");
    }

    #[test]
    fn 解析订阅头() {
        let mut h = BTreeMap::new();
        h.insert(
            "subscription-userinfo".to_string(),
            "upload=100; download=200; total=1000; expire=1867235068".to_string(),
        );
        h.insert("profile-update-interval".to_string(), "24".to_string());
        let i = parse_sub_headers(&h);
        assert_eq!(i.upload, Some(100));
        assert_eq!(i.total, Some(1000));
        assert_eq!(i.update_interval, Some(24));
        assert!((i.used_percent().unwrap() - 30.0).abs() < 0.01);
    }

    #[test]
    fn 额外参数拼到_url() {
        assert_eq!(
            with_params("https://x.com/sub", &["flag=clash".into(), "a=1".into()]),
            "https://x.com/sub?flag=clash&a=1"
        );
        assert_eq!(
            with_params("https://x.com/sub?z=0", &["flag=clash".into()]),
            "https://x.com/sub?z=0&flag=clash"
        );
        assert_eq!(with_params("https://x.com/sub", &[]), "https://x.com/sub");
    }
}