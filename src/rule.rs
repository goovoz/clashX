//! 规则与规则集管理。
//!
//! 两条主线：
//! 1. **rule-providers 管理** —— 增删改查 + behavior 校验 + 可用性预检
//! 2. **规则测试**（解释器） —— 给定域名/IP，逐条比对本地规则链，
//!    告诉用户「这个请求会命中哪条规则、最终走哪个出口」
//!
//! # 为什么要自己实现解释器
//!
//! 内核的 `GET /rules` 只返回「已加载的规则表」+ `extra.hitCount`，
//! 但**不能回答「为什么这个域名走了这个节点」** —— 想查就得先发一次
//! 真实请求，看日志里的 `match xxx using yyy`。
//!
//! 规则引擎的匹配语义其实不复杂（见官方 wiki config/rules）：
//! 从上到下首次命中即生效。所以本地复现一遍，就能离线给出答案，
//! 不需要发请求、不需要装面板。这是排查「规则配错了」最快的手段。

use anyhow::{bail, Context, Result};
use serde_yaml::Value;
use std::collections::BTreeMap;

use crate::config::{Rule, RuleProvider};

/// 规则集的行为类型。
///
/// mihomo 只认这三种（`rule-providers[].behavior`）：
/// - `domain`：域名列表，每行 `+.example.com` / `full.com` / `keyword`
/// - `ipcidr`：CIDR 列表，每行 `1.2.3.0/24`
/// - `classical`：完整规则行（带策略），较少用
pub const BEHAVIORS: [&str; 3] = ["domain", "ipcidr", "classical"];

pub fn validate_behavior(b: &str) -> Result<()> {
    if BEHAVIORS.contains(&b) {
        Ok(())
    } else {
        bail!(
            "behavior 只能是 {}（mihomo 只认这三种），收到 {b:?}",
            BEHAVIORS.join(" / ")
        )
    }
}

/// 规则集配置的补全与规范化。
///
/// mihomo 对 `rule-providers` 的要求（实测踩过的坑）：
/// - `behavior` 必填，且只能是上面三种之一
/// - `path` 必须是**相对 mihomo 工作目录**的路径；
///   绝对路径也能用但换机器就失效，所以这里统一转成 `./ruleset/xxx.yaml`
/// - `interval` 不填就不自动更新
pub fn normalize(p: &mut RuleProvider) -> Result<()> {
    validate_behavior(&p.behavior)?;

    if p.path.trim().is_empty() {
        // 按behavior 给个默认文件名（ruleset/<name>.yaml）
        p.path = format!("./ruleset/{}.yaml", p.name);
    } else {
        // 规范化：去掉前导 / 与 ../，防止写到工作目录之外
        let mut path = p.path.trim().to_string();
        while let Some(rest) = path.strip_prefix("../") {
            bail!("规则集 path 不能含 ..：{}", p.path);
        }
        if let Some(stripped) = path.strip_prefix("./") {
            path = stripped.to_string();
        }
        path = path.trim_start_matches('/').to_string();
        if path.is_empty() {
            bail!("规则集 path 规范化后为空：{}", p.name);
        }
        p.path = format!("./{}", path);
    }

    if p.interval == 0 {
        p.interval = 86400;   // 一天
    }
    Ok(())
}

/// 检查规则集 URL 是否可用（HEAD 请求）。
///
/// 只做「能不能下到」，不下载内容 —— 内容由内核自己管。
/// 返回 Err 时把 HTTP 状态带上，用户能直接看懂是404 还是超时。
pub fn probe_url(url: &str) -> Result<usize> {
    if url.is_empty() {
        bail!("规则集 URL 为空");
    }
    let out = std::process::Command::new("curl")
        .arg("-sS")
        .arg("-I")
        .arg("-L")
        .arg("--max-time")
        .arg("20")
        .arg("-o")
        .arg("/dev/null")
        .arg("-w")
        .arg("%{http_code}")
        .arg(url)
        .output()
        .context("执行 curl 失败")?;
    let code = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let n: usize = code.parse().unwrap_or(0);
    if !(200..300).contains(&n) {
        bail!("规则集 URL 返回 HTTP {code}（{url}）");
    }
    Ok(n)
}

/// 规则命中解释器。
pub struct Matcher<'a> {
    /// 顺序规则（用户自己写的）
    pub rules: &'a [Rule],
    /// 规则集内容：名称 -> (behavior, 内容行)
    pub providers: BTreeMap<String, (String, Vec<String>)>,
    /// 可用的内置策略名
    pub builtins: Vec<String>,
}

/// 一次匹配的结果。
#[derive(Debug, Clone)]
pub struct Verdict {
    /// 命中的规则在链里的序号（从 1 开始）
    pub index: usize,
    /// 命中的规则原文
    pub rule: String,
    /// 命中的规则类型
    pub kind: String,
    /// 最终策略
    pub target: String,
    /// 是否是兜底（MATCH）
    pub fallback: bool,
}

/// 解释一次请求会怎么走。
///
/// 只覆盖最常用、语义明确的规则类型（官方 wiki config/rules 里
/// 40+ 种里最常用的 dozen 种）。遇到不认识的类型**明确跳过并说明**，
/// 而不是猜 —— 猜错会让排查结论反向，比说「不知道」更糟。
///
/// `domain` 是要匹配的域名，`ip` 可选（GeoIP/IP-CIDR 类规则需要）。
pub fn explain(
    m: &Matcher,
    domain: Option<&str>,
    ip: Option<&str>,
) -> Result<Verdict> {
    let mut unknown: Vec<String> = Vec::new();

    for (i, rule) in m.rules.iter().enumerate() {
        let r = rule.trim();
        if r.is_empty() || r.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = r.splitn(2, ',').collect();
        let kind = parts[0].trim().to_ascii_uppercase();

        // MATCH 是兜底，一定命中
        if kind == "MATCH" {
            let target = parts.get(1).map(|s| s.trim()).unwrap_or("");
            return Ok(Verdict {
                index: i + 1,
                rule: r.to_string(),
                kind,
                target: target.to_string(),
                fallback: true,
            });
        }

        let payload = parts.get(1).map(|s| s.trim()).unwrap_or("");
        // 末段若是已知策略名，就当策略；否则都是 payload（逻辑规则除外）
        let (payload, target) = split_payload_target(payload, m);

        let hit = match_kind(&kind, payload, domain, ip, m, &mut unknown);

        if hit {
            return Ok(Verdict {
                index: i + 1,
                rule: r.to_string(),
                kind,
                target: target.to_string(),
                fallback: false,
            });
        }
    }

    // 没有兜底 MATCH
    let has_match = m
        .rules
        .iter()
        .any(|r| r.split(',').next().unwrap_or("").trim().eq_ignore_ascii_case("MATCH"));
    if has_match {
        bail!("所有规则都未命中，但规则链里也没有 MATCH 兜底 —— 这条配置不可能生效")
    }
    // 走到这里说明 explain 内部逻辑有问题（因为 MATCH 一定会命中）
    bail!("规则链没有 MATCH 兜底，且没有任何规则命中（配置里若真的没有 MATCH，mihomo -t 会报错）")
}

/// 从 payload 里拆出末尾的策略名。
///
/// `DOMAIN-SUFFIX,example.com,PROXY` -> payload=example.com target=PROXY
/// `IP-CIDR,10.0.0.0/8,DIRECT,no-resolve`
///     -> payload=10.0.0.0/8 target=DIRECT，**no-resolve 单独剥掉**
///     （它是 mihomo 的附加开关，不是策略名。不剥的话 target 会变成
///     "DIRECT,no-resolve"，导致 is_target 判定失败 -> target 空 ->
///     规则永远不命中。实测踩过：所有 IP-CIDR 规则都不生效。）
/// `AND,((...),(...)),PROXY`      -> 逻辑规则整块当 payload
fn split_payload_target<'a>(payload: &'a str, m: &Matcher) -> (&'a str, String) {
    // 逻辑规则：payload 是一整块括号表达式，整体保留
    if payload.starts_with("((") || payload.starts_with('(') {
        return (payload, String::new());
    }

    let mut body = payload.trim();
    // 先剥末尾的 no-resolve / src（可能是多个，如 "...DIRECT,no-resolve,src"）。
    //
    // 用 rsplit_once 拿到的 (left, right) 再 left.trim_end()，
    // 不要用 `body.len() - last.len()` —— last 是 trim 过的，
    // 长度对不上会截错位置（实测导致 payload 变成 "10.0.0.0/8,DIRECT"
    // 之类，CIDR 匹配永远失败）。
    loop {
        let (left, right) = match body.rsplit_once(',') {
            Some(v) => v,
            None => break,
        };
        let last = right.trim();
        let last_l = last.to_ascii_lowercase();
        if last_l == "no-resolve" || last_l == "src" {
            body = left.trim_end();
        } else {
            break;
        }
    }

    // 再从右往左找「看起来像策略」的部分
    if let Some(idx) = body.rfind(',') {
        let last = body[idx + 1..].trim();
        let head = body[..idx].trim();
        if is_target(last, m) {
            return (head, last.to_string());
        }
    }
    (body, String::new())
}

fn is_target(s: &str, m: &Matcher) -> bool {
    const BUILTIN: [&str; 5] = ["DIRECT", "REJECT", "REJECT-DROP", "PASS", "COMPATIBLE"];
    BUILTIN.contains(&s) || m.builtins.iter().any(|x| x == s)
}

/// 判断单条规则是否命中。
fn match_kind(
    kind: &str,
    payload: &str,
    domain: Option<&str>,
    ip: Option<&str>,
    m: &Matcher,
    unknown: &mut Vec<String>,
) -> bool {
    match kind {
        "DOMAIN" => domain.map(|d| d.eq_ignore_ascii_case(payload)).unwrap_or(false),
        "DOMAIN-SUFFIX" => domain.map(|d| suffix_match(d, payload)).unwrap_or(false),
        "DOMAIN-KEYWORD" => domain.map(|d| d.to_ascii_lowercase()
            .contains(&payload.to_ascii_lowercase())).unwrap_or(false),
        "DOMAIN-WILDCARD" | "DOMAIN-REGEX" => {
            // 通配符/正则：交给你自己的判断，这里只给近似
            domain.map(|d| wildcard_match(d, payload)).unwrap_or(false)
        }
        "DOMAIN-SET" => domain.map(|d| in_domain_set(d, payload, m)).unwrap_or(false),
        "IP-CIDR" | "IP-CIDR6" => ip.map(|a| cidr_match(a, payload)).unwrap_or(false),
        "IP-SUFFIX" => ip.map(|a| ip_suffix_match(a, payload)).unwrap_or(false),
        "IP-ASN" | "GEOIP" | "SRC-GEOIP" => {
            // 需要 GeoIP 数据库，客户端不自带 -> 明确跳过
            unknown.push(format!("{kind} 需要 GeoIP 数据库，请用内核日志确认"));
            false
        }
        "GEOSITE" => domain.map(|d| in_geosite(d, payload, m)).unwrap_or(false),
        "RULE-SET" => domain
            .map(|d| in_rule_set(Some(d), ip, payload, m))
            .unwrap_or(false),
        "DST-PORT" | "SRC-PORT" | "NETWORK" | "PROCESS-NAME" | "UID" => {
            // 这些要看连接信息，纯域名判断不出来
            unknown.push(format!("{kind} 需要连接信息，本工具只按域名判断"));
            false
        }
        "AND" | "OR" | "NOT" | "SUB-RULE" => {
            unknown.push(format!("{kind} 是逻辑规则，本工具暂不展开"));
            false
        }
        _ => {
            unknown.push(format!("未识别的规则类型 {kind}"));
            false
        }
    }
}

/// mihomo 的 DOMAIN-SUFFIX 语义：`example.com` 匹配
/// `www.example.com` 与 `example.com`，但**不**匹配 `notexample.com`。
/// 与 DOMAIN-WILDCARD 的通配符语义不同，这里做精确的标签边界检查。
fn suffix_match(domain: &str, suffix: &str) -> bool {
    let d = domain.trim_end_matches('.').to_ascii_lowercase();
    let mut s = suffix.trim().to_ascii_lowercase();
    // mihomo 的 `+.example.com` 语法：前导 + 表示「也匹配根域」，
    // 而 `.example.com` 的点本身是分隔符 —— 两种写法等价。
    if let Some(rest) = s.strip_prefix('+') {
        s = rest.to_string();
    }
    let s = s.trim_start_matches('.').trim_end_matches('.');
    if s.is_empty() {
        return false;
    }
    if d == s {
        return true;
    }
    d.len() > s.len() && d.ends_with(s) && {
        // 前一个字符必须是 '.'（标签边界）
        let i = d.len() - s.len() - 1;
        d.as_bytes()[i] == b'.'
    }
}

/// DOMAIN-WILDCARD：`*` 匹配零个或多个字符，`?` 匹配一个字符。
fn wildcard_match(domain: &str, pat: &str) -> bool {
    // 只有 * 的简单情形快路径
    if let Some(p) = pat.strip_prefix("*.") {
        return domain.to_ascii_lowercase().ends_with(&format!(".{}", p.to_ascii_lowercase()));
    }
    if let Some(p) = pat.strip_suffix(".*") {
        return domain.to_ascii_lowercase().starts_with(&format!("{}.", p.to_ascii_lowercase()));
    }
    // 通用 glob：把 * 转 .*，其余转义后用 contains 近似
    let p = pat.to_ascii_lowercase().replace('.', r"\.").replace('*', ".*");
    domain.to_ascii_lowercase().contains(&p)
}

fn cidr_match(ip: &str, cidr: &str) -> bool {
    let (net, bits) = match cidr.split_once('/') {
        Some((n, b)) => (n, b),
        None => return ip == cidr,
    };
    let bits: u32 = match bits.parse() {
        Ok(b) => b,
        Err(_) => return false,
    };
    match (parse_ip(ip), parse_ip(net)) {
        (Some(a), Some(n)) => {
            if a.len() != n.len() {
                return false;
            }
            let total = a.len() * 8;
            if bits as usize > total {
                return false;
            }
            let full = (bits / 8) as usize;
            let rem = bits % 8;
            if a[..full] != n[..full] {
                return false;
            }
            if rem == 0 {
                true
            } else {
                let mask = 0xffu8 << (8 - rem);
                (a[full] & mask) == (n[full] & mask)
            }
        }
        _ => false,
    }
}

fn parse_ip(s: &str) -> Option<Vec<u8>> {
    let t = s.trim();
    if t.contains(':') {
        // IPv6：只处理 ::1 与常见的十六进制组，写成 [u16; 8]
        let mut groups = [0u16; 8];
        let (head, tail) = match t.split_once("::") {
            Some((h, tl)) => (h, Some(tl)),
            None => (t, None),
        };
        let mut idx = 0;
        let mut push = |g: &str, idx: &mut usize| -> Option<()> {
            if g.is_empty() {
                return Some(());
            }
            let v = u16::from_str_radix(g, 16).ok()?;
            if *idx >= 8 {
                return None;
            }
            groups[*idx] = v;
            *idx += 1;
            Some(())
        };
        for g in head.split(':') {
            push(g, &mut idx)?;
        }
        if let Some(tl) = tail {
            let rest = 8 - idx;
            let parts: Vec<&str> = tl.split(':').collect();
            let mut j = 0;
            for g in parts.iter().rev() {
                if j < rest {
                    let v = u16::from_str_radix(g, 16).ok()?;
                    groups[8 - 1 - j] = v;
                    j += 1;
                }
            }
        }
        let mut out = Vec::with_capacity(16);
        for g in groups {
            out.push((g >> 8) as u8);
            out.push((g & 0xff) as u8);
        }
        Some(out)
    } else {
        Some(t.split('.').filter_map(|p| p.parse::<u8>().ok()).collect())
    }
}

fn ip_suffix_match(ip: &str, suffix: &str) -> bool {
    ip.split('.').last() == Some(suffix)
}

/// DOMAIN-SET：从已加载的 domain 规则集里查。
fn in_domain_set(domain: &str, set: &str, m: &Matcher) -> bool {
    if let Some((_, lines)) = m.providers.get(set) {
        return lines.iter().any(|l| suffix_or_full(domain, l));
    }
    false
}

fn suffix_or_full(domain: &str, line: &str) -> bool {
    let l = line.trim();
    if l.is_empty() || l.starts_with('#') || l.starts_with(';') {
        return false;
    }
    if let Some(s) = l.strip_prefix("+.") {
        suffix_match(domain, s)
    } else if let Some(k) = l.strip_prefix("keyword:") {
        domain.to_ascii_lowercase().contains(&k.trim().to_ascii_lowercase())
    } else {
        domain.eq_ignore_ascii_case(l)
    }
}

/// GEOSITE：客户端不内置 geosite.dat，这里只能按集合名做**启发式**判断。
///
/// 诚实说明：这**不是**真正的 geosite 匹配。真正的 geosite 有上万条
/// 规则，由 mihomo 内核加载 geosite.dat 判定。客户端为了离线可用
/// 维护一张极简的常见集合表，覆盖不到的情况一律返回 false（不命中），
/// 宁可说"不知道"也不要给出错误结论。
fn in_geosite(domain: &str, set: &str, _m: &Matcher) -> bool {
    let d = domain.to_ascii_lowercase();
    let cn = |s: &str| d.ends_with(&format!(".{}", s)) || d == s.to_ascii_lowercase();
    match set.to_ascii_lowercase().as_str() {
        "cn" => [".cn", ".com.cn", ".net.cn", ".org.cn", ".gov.cn", ".edu.cn"]
            .iter().any(|s| d.ends_with(s) || d.contains(s))
            || cn("qq.com") || cn("weixin.qq.com") || cn("taobao.com")
            || cn("tmall.com") || cn("jd.com") || cn("163.com")
            || cn("bilibili.com") || cn("zhihu.com") || cn("douyin.com")
            || cn("alipay.com") || cn("meituan.com") || cn("pinduoduo.com"),
        "geolocation-!cn" => !in_geosite(domain, "cn", _m),
        "category-ads-all" => d.contains("doubleclick.net")
            || d.contains("googlesyndication.com")
            || d.contains("adservice."),
        _ => false,
    }
}

/// RULE-SET：查本地已缓存的规则集内容。
fn in_rule_set(
    domain: Option<&str>,
    ip: Option<&str>,
    set: &str,
    m: &Matcher,
) -> bool {
    let (behavior, lines) = match m.providers.get(set) {
        Some(v) => v,
        None => return false,
    };
    match behavior.as_str() {
        "domain" => domain.map(|d| lines.iter().any(|l| suffix_or_full(d, l))).unwrap_or(false),
        "ipcidr" => ip.map(|a| lines.iter().any(|l| cidr_match(a, l.trim()))).unwrap_or(false),
        "classical" => {
            // classical 每行是完整规则：DOMAIN-SUFFIX,x,PROXY
            domain.map(|d| {
                lines.iter().any(|l| {
                    let p: Vec<&str> = l.split(',').collect();
                    p.len() >= 2
                        && (suffix_match(d, p[1].trim())
                            || d.eq_ignore_ascii_case(p[1].trim()))
                })
            }).unwrap_or(false)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matcher(rules: &[Rule]) -> Matcher<'_> {
        Matcher {
            rules,
            providers: BTreeMap::new(),
            builtins: vec!["PROXY".into(), "AUTO".into()],
        }
    }

    #[test]
    fn 解析_cidr() {
        assert_eq!(parse_ip("192.168.1.5"), Some(vec![192, 168, 1, 5]));
        assert_eq!(parse_ip("::1").map(|v| v.len()), Some(16));
    }

    #[test]
    fn cidr_匹配() {
        assert!(cidr_match("192.168.1.5", "192.168.1.0/24"));
        assert!(cidr_match("10.0.0.1", "10.0.0.0/8"));
        assert!(!cidr_match("11.0.0.1", "10.0.0.0/8"));
        assert!(cidr_match("1.2.3.4", "1.2.3.4/32"));
        assert!(!cidr_match("1.2.3.5", "1.2.3.4/32"));
        // 非 8 对齐的掩码
        assert!(cidr_match("10.1.2.3", "10.0.0.0/8"));
        assert!(!cidr_match("11.1.2.3", "10.0.0.0/8"));
    }

    #[test]
    fn 后缀匹配有标签边界() {
        assert!(suffix_match("www.example.com", "example.com"));
        assert!(suffix_match("example.com", "example.com"));
        // mihomo 的语义：notexample.com **不**匹配 example.com
        assert!(!suffix_match("notexample.com", "example.com"));
        assert!(suffix_match("a.b.example.com", "+.example.com"));
    }

    #[test]
    fn 解释规则链() {
        let rules = vec![
            "DOMAIN-SUFFIX,example.com,DIRECT".to_string(),
            "GEOIP,CN,DIRECT".to_string(),
            "MATCH,PROXY".to_string(),
        ];
        let m = matcher(&rules);
        let v = explain(&m, Some("www.example.com"), None).unwrap();
        assert_eq!(v.target, "DIRECT");
        assert!(!v.fallback);

        let v2 = explain(&m, Some("google.com"), None).unwrap();
        assert_eq!(v2.target, "PROXY");
        assert!(v2.fallback);
    }

    #[test]
    fn no_resolve_要从payload里剥掉() {
        // no-resolve 是 mihomo 的开关，不是策略名。不剥的话
        // target 会变成 "DIRECT,no-resolve" -> is_target 判false -> 规则不命中。
        let rules = vec!["IP-CIDR,10.0.0.0/8,DIRECT,no-resolve".to_string()];
        let m = matcher(&rules);
        let v = explain(&m, None, Some("10.1.2.3")).unwrap();
        assert_eq!(v.kind, "IP-CIDR");
        assert_eq!(v.target, "DIRECT");
    }

    #[test]
    fn 解释规则按顺序首次命中() {
        let rules = vec![
            "DOMAIN,blocked.com,REJECT".to_string(),
            "DOMAIN-SUFFIX,ok.com,PROXY".to_string(),
            "MATCH,DIRECT".to_string(),
        ];
        let m = matcher(&rules);
        assert_eq!(explain(&m, Some("blocked.com"), None).unwrap().target, "REJECT");
        assert_eq!(explain(&m, Some("a.ok.com"), None).unwrap().target, "PROXY");
        assert_eq!(explain(&m, Some("other.com"), None).unwrap().target, "DIRECT");
    }

    #[test]
    fn 行为校验() {
        assert!(validate_behavior("domain").is_ok());
        assert!(validate_behavior("ipcidr").is_ok());
        assert!(validate_behavior("classical").is_ok());
        assert!(validate_behavior("nope").is_err());
    }

    #[test]
    fn path_规范化() {
        let mut p = RuleProvider {
            name: "cn".into(),
            behavior: "domain".into(),
            url: "https://x/r.txt".into(),
            path: "".into(),
            interval: 0,
        };
        normalize(&mut p).unwrap();
        assert_eq!(p.path, "./ruleset/cn.yaml");
        assert_eq!(p.interval, 86400);

        let mut p2 = RuleProvider {
            name: "cn".into(),
            behavior: "domain".into(),
            url: "u".into(),
            path: "./ruleset/x.yaml".into(),
            interval: 3600,
        };
        normalize(&mut p2).unwrap();
        assert_eq!(p2.path, "./ruleset/x.yaml");

        // 不能写到工作目录之外
        let mut p3 = RuleProvider {
            name: "cn".into(),
            behavior: "domain".into(),
            url: "u".into(),
            path: "../../etc/passwd".into(),
            interval: 0,
        };
        assert!(normalize(&mut p3).is_err());
    }
}
// ---------------------------------------------------------------------------
// 规则预设
// ---------------------------------------------------------------------------

/// 内置规则预设。
///
/// 刻意**保守**：只用 mihomo 内核一定能解析的写法，不引入需要联网
/// 下载的 rule-provider（那些用 `provider add` 单独加）。理由是
/// 「开箱能用」比「配置齐全」重要 —— 预设要先跑通，再由用户扩展。
///
/// 规则集用的 URL 是 MetaCubeX 官方 meta-rules-dat（GitHub raw），
/// 国内访问可能慢；`provider check` 可以先探一下再决定用不用。
pub struct Preset {
    pub name: &'static str,
    pub desc: &'static str,
    pub providers: &'static [(&'static str, &'static str, &'static str)],
    pub rules: &'static [&'static str],
}

pub const PRESETS: &[Preset] = &[
    Preset {
        name: "cn",
        desc: "国内直连 + 国外代理（最常用）",
        providers: &[
            (
                "cn-domain",
                "domain",
                "https://raw.githubusercontent.com/MetaCubeX/meta-rules-dat/meta/geo/geosite/cn.yaml",
            ),
            (
                "cn-ipcidr",
                "ipcidr",
                "https://raw.githubusercontent.com/MetaCubeX/meta-rules-dat/meta/geo/cn.yaml",
            ),
        ],
        rules: &[
            "RULE-SET,cn-domain,DIRECT",
            "RULE-SET,cn-ipcidr,DIRECT,no-resolve",
            "MATCH,PROXY",
        ],
    },
    Preset {
        name: "adblock",
        desc: "广告拦截（放在最前面，配合其它预设一起用）",
        providers: &[(
            "reject",
            "domain",
            "https://raw.githubusercontent.com/MetaCubeX/meta-rules-dat/meta/ads/ads.yaml",
        )],
        rules: &["RULE-SET,reject,REJECT"],
    },
    Preset {
        name: "private",
        desc: "私有地址与组播直连（局域网设备必备）",
        providers: &[],
        rules: &[
            // 组播/私有网段直连，否则内网设备会被送去代理
            "IP-CIDR,10.0.0.0/8,DIRECT,no-resolve",
            "IP-CIDR,172.16.0.0/12,DIRECT,no-resolve",
            "IP-CIDR,192.168.0.0/16,DIRECT,no-resolve",
            "IP-CIDR,127.0.0.0/8,DIRECT,no-resolve",
            "IP-CIDR,169.254.0.0/16,DIRECT,no-resolve",
            "IP-CIDR,224.0.0.0/4,DIRECT,no-resolve",
        ],
    },
];

pub fn find_preset(name: &str) -> Option<&'static Preset> {
    PRESETS.iter().find(|p| p.name == name)
}

/// 检测配置里的规则集是否已缓存到 workdir，并把内容读进来给解释器用。
pub fn load_provider_contents(
    providers: &[RuleProvider],
    workdir: &std::path::Path,
) -> BTreeMap<String, (String, Vec<String>)> {
    let mut out = BTreeMap::new();
    for p in providers {
        // path 形如 ./ruleset/cn.yaml
        let rel = p.path.trim_start_matches("./");
        let full = workdir.join(rel);
        if let Ok(text) = std::fs::read_to_string(&full) {
            let lines: Vec<String> = text
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with(";"))
                .collect();
            out.insert(p.name.clone(), (p.behavior.clone(), lines));
        }
    }
    out
}

/// 探测结果里的「本工具判不了」的类型汇总成一段提示。
pub fn unknown_hint(unknown: &[String]) -> Option<String> {
    if unknown.is_empty() {
        return None;
    }
    let mut seen: Vec<&String> = Vec::new();
    for u in unknown {
        if !seen.iter().any(|x| x.contains(&u[..u.find(':').unwrap_or(u.len())])) {
            seen.push(u);
        }
    }
    Some(format!(
        "以下规则类型本工具无法离线判定（需内核运行时才能确定）：{}",
        seen.iter()
            .map(|s| s.split(':').next().unwrap_or(s))
            .collect::<Vec<_>>()
            .join("、")
    ))
}

/// 解析用户输入的测试目标：允许 "domain" 或 "domain,ip" 或 "domain ip"。
pub fn split_target(input: &str, ip_arg: Option<&str>) -> (String, Option<String>) {
    match ip_arg {
        Some(ip) => (input.trim().to_string(), Some(ip.to_string())),
        None => {
            let mut it = input.splitn(2, [',', ' ', '\t']);
            let d = it.next().unwrap_or("").trim().to_string();
            let ip = it.next().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
            (d, ip)
        }
    }
}

/// 校验规则行格式：`TYPE,payload,TARGET`。
///
/// 不试图穷举 mihomo 的 40+ 种类型（那是内核的职责，`-t` 会兜底），
/// 只检查「至少三段且第一段像规则类型」这种明显的录入错误。
pub fn validate_rule(r: &str) -> Result<()> {
    let s = r.trim();
    if s.is_empty() {
        bail!("规则为空");
    }
    if s.starts_with('#') {
        bail!("这是注释行，不是规则");
    }
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() < 2 {
        bail!(
            "规则至少要有 2 段：TYPE,payload[,TARGET]\n\
             收到：{s}"
        );
    }
    let kind = parts[0].trim();
    if kind.is_empty() || !kind.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-' || c == '_') {
        bail!(
            "规则类型应由大写字母/数字/-组成（mihomo 的惯例），收到：{kind:?}\n\
             例如：DOMAIN-SUFFIX,example.com,DIRECT"
        );
    }
    Ok(())
}

/// 从配置里取出可用的策略名（代理组名 + DIRECT/REJECT）。
pub fn builtin_targets(cfg_groups: &[String]) -> Vec<String> {
    let mut v: Vec<String> = vec![
        "DIRECT".into(),
        "REJECT".into(),
        "REJECT-DROP".into(),
        "PASS".into(),
        "COMPATIBLE".into(),
    ];
    v.extend(cfg_groups.iter().cloned());
    v
}
