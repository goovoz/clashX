//! nftables 旁路由规则管理。
//!
//! # 为什么自己写 nftables 而不用 mihomo 的 `tun.auto-redirect`
//!
//! `auto-redirect` 让 mihomo 自己写 iptables/nftables 规则，对**本机代理**
//! 足够。但旁路由场景要处理「转发流量」（来自其它设备、经本机路由过去的
//! 报文），这类规则的输入/输出钩子与本地套接字不同，内核自动生成的规则
//! 覆盖不到，需要显式写 `prerouting`/`forward` 链。
//!
//! # 为什么用独立的 `inet clashx` 表
//!
//! - **不碰别人的表**。上一轮 openclash-rt 的教训：它建了 `inet fw4` 表，
//!   mihomo 的 `auto-redirect` 看到 `nat_output` 已存在就失败并留下无hook
//!   的死链。独立表 + 独立名，从根上避免这类冲突。
//! - 卸载时只 `delete table inet clashx`，不会误删别人的链。
//!
//! # tproxy vs redir vs tun
//!
//! | 方式 | TCP | UDP | ICMP | 需要自己写 nft |
//! |---|---|---|---|---|
//! | redir | 是 | 否 | 否 | 是（只 TCP，规则简单） |
//! | **tproxy** | 是 | 是 | 否 | 是（需 policy route） |
//! | tun | 是 | 是 | 是 | 否（内核自己写） |
//!
//! 本模块实现 **tproxy**：TCP+UDP 覆盖够用，规则量远小于 TUN，
//! 且不与系统 nftables 全局状态耦合。真需要 ICMP 时再加 TUN 模式。

use anyhow::{bail, Context, Result};
use std::process::{Command, Stdio};

/// 路由表号（policy routing 用）。选个 unlikely 的号避免和别的项目撞。
pub const ROUTE_TABLE: u32 = 202;
/// 路由表的可读名。**仅用于日志与文档**——
/// 不能拿去喂 `ip` 命令（名字需要先在 rt_tables 注册，
/// 而注册格式不对会把系统网络工具搞坏，见 `render_policy_route` 的注释）。
pub const ROUTE_TABLE_NAME: &str = "clashx";
/// 规则优先级：必须排在 `main`（32766）之前，否则默认路由先把包送走了。
pub const RULE_PRIO_LOCAL: u32 = 100;
/// 内部 fwmark。区别于路由表号。
pub const FWMARK: u32 = 0x202;

/// 旁路由的网络配置。
#[derive(Debug, Clone)]
pub struct Gateway {
    /// 本机在局域网里的地址（旁路由的网关地址）。
    pub lan_addr: String,
    /// 出口网卡。
    pub iface: String,
    /// tproxy 入站端口（要跟 mihomo 的 `tproxy-port` 一致）。
    pub tproxy_port: u16,
    /// DNS 劫持端口（要跟 mihomo 的 `dns.listen` 端口一致，0 = 不劫持）。
    pub dns_port: u16,
    /// 直连网段（这些网段的流量不代理，只转发）。
    ///
    /// ★ 默认值刻意**只有组播与保留段**，不含任何私网段。
    /// 这不是偷懒，是实测逼出来的：本机所在网段是 172.20.0.0/24，
    /// 而 `172.16.0.0/12` 恰好包含 172.20 —— 把 RFC1918 全段写成
    /// 默认直连规则，会让「内网直连」匹配上所有 172.20.x.x 目标，
    /// 代理彻底失效，表现为「网关配对了但流量不走代理」。
    ///
    /// 所以私网段必须由用户按实际网段显式声明（`--direct-nets`）。
    pub direct_nets: Vec<String>,
    /// 客户端网段（只有这些源地址的转发流量才进代理）。
    /// 空 = 拒绝启用（安全下限，见 `precheck`）。
    pub lan_clients: Vec<String>,
    /// mihomo 进程的运行 UID。这些 uid 的流量必须绕过，
    /// 否则 mihomo 自己的出站连接会回到 tproxy 入口形成环路。
    pub exclude_uid: u32,
    /// 是否放行 ICMP（旁路由一般需要，让客户端能 ping 通网关）。
    pub allow_icmp: bool,
}

impl Default for Gateway {
    fn default() -> Self {
        Self {
            lan_addr: String::new(),
            iface: String::new(),
            tproxy_port: 7894,
            dns_port: 7874,
            // 只放确定不会出现在公网的段：组播、保留、广播。
            // 私网段交给 --direct-nets按需追加（原因见字段注释）。
            direct_nets: vec![
                "224.0.0.0/4".into(),
                "240.0.0.0/4".into(),
                "255.255.255.255/32".into(),
            ],
            lan_clients: Vec::new(),
            exclude_uid: 65534,
            allow_icmp: true,
        }
    }
}

impl Gateway {
    /// 预检：配置不全就别改防火墙，避免把机器的网络搞瘫。
    pub fn precheck(&self) -> Result<()> {
        if self.lan_addr.is_empty() {
            bail!("lan_addr 未设置：不知道本机的局域网地址，无法区分「本地流量」和「转发流量」");
        }
        if self.iface.is_empty() {
            bail!("iface 未设置：旁路由必须显式指定出口网卡，否则内核可能把包从不该出去的网卡发出去");
        }
        if self.tproxy_port == 0 {
            bail!("tproxy_port 不能为 0");
        }
        if self.lan_clients.is_empty() {
            bail!(
                "lan_clients 为空：这意味着任何能路由到本机的设备流量都会被劫持。\n\
                 请显式列出客户端网段（如 172.20.0.0/24），这是安全下限。"
            );
        }
        Ok(())
    }
}

/// 生成 nftables 规则集（`nft -f -` 的输入）。
///
/// 分层结构：
/// ```text
/// table inet clashx
/// ├── chain prerouting   客户端来的包在这里被判定要不要代理
/// ├── chain forward     转发的包
/// └── chain output      本机自身出站：按 mark 消费 policy route
/// ```
///
/// 顺序至关重要，规则自上而下首次命中即生效：
///
/// 1. **mark 已置位→ return**。mihomo 转发出去的包会带着我们打的 mark，
///    这些包回来时不能再次进 tproxy，否则死循环。
/// 2. **目的地址是本机 → return**。客户端访问网关上的SSH/DNS 要能用。
/// 3. **源地址不是客户端网段 → return**。只代理明确列出的客户端。
/// 4. **直连网段 → return**。组播/保留段/user 声明的内网段。
/// 5. **DNS 劫持**。53 → mihomo 的 DNS 端口。
/// 6. **tproxy**。打 mark + 指向本地端口，路由由 output 的 policy route 决定。
pub fn render_ruleset(g: &Gateway) -> String {
    let mut s = String::new();
    let lan = &g.lan_addr;
    let fwmark = FWMARK;

    s.push_str("#!/usr/sbin/nft -f\n");
    s.push_str("# 由 clashx 生成，勿手改。改规则请改配置后重新 gateway enable。\n");
    s.push_str("table inet clashx {\n");

    // ---------- prerouting ----------
    s.push_str("  chain prerouting {\n");
    s.push_str("    type filter hook prerouting priority mangle; policy accept;\n");

    // 1) 已标记的包（mihomo 自己的出站流量）不再代理 —— 防环路第一道
    s.push_str(&format!("    meta mark {mark} return\n", mark = fwmark));

    // 2) 目的地址是本机自身 -> 不代理
    //    （客户端要能访问网关上的 SSH / mihomo DNS / WebUI）
    s.push_str(&format!(
        "    ip daddr {lan} return\n",
        lan = lan
    ));

    // 3) 源不在客户端网段 -> 不代理
    for c in &g.lan_clients {
        s.push_str(&format!("    ip saddr != {c} return\n", c = c));
    }

    // 4) 直连网段
    for n in &g.direct_nets {
        s.push_str(&format!("    ip daddr {n} return\n", n = n));
    }

    // 5) DNS 劫持。必须在 tproxy 之前，且用 dnat 而不是 tproxy ——
    //    DNS 是 UDP，dnat 到 mihomo 的 DNS 监听端口更直接、更好排查。
    //    ★ `dnat` 本身就是 terminal statement，后面不能再跟 `return`：
    //    nft -c 实测报 "Statement after terminal statement has no effect"。
    

    // 6) tproxy 核心 —— **必须按协议分成两条**。
    //    nft -c 实测报错：
    //        Transparent proxy support requires transport protocol match
    //    tproxy 表达式不能凭空工作，得先知道上层协议是 TCP 还是 UDP。
    //    两条都以 accept 收尾（terminal），后面的规则不再评估。
    s.push_str(&format!(
        "    meta l4proto tcp meta mark set {mark} tproxy to :{port} accept\n",
        mark = fwmark,
        port = g.tproxy_port
    ));
    s.push_str(&format!(
        "    meta l4proto udp meta mark set {mark} tproxy to :{port} accept\n",
        mark = fwmark,
        port = g.tproxy_port
    ));

    s.push_str("  }\n");

    // ---------- nat_prerouting: DNS 劫持 ----------
    // dnat 只能在 nat 类型链里做，所以单独开一条。
    if g.dns_port > 0 {
        s.push_str("  chain nat_prerouting {\n");
        s.push_str("    type nat hook prerouting priority dstnat; policy accept;\n");
        for c in &g.lan_clients {
            s.push_str(&format!(
                "    ip saddr {c} udp dport 53 dnat to :{dns}\n",
                c = c,
                dns = g.dns_port
            ));
            s.push_str(&format!(
                "    ip saddr {c} tcp dport 53 dnat to :{dns}\n",
                c = c,
                dns = g.dns_port
            ));
        }
        s.push_str("  }\n");
    }

    // ---------- forward ----------
    s.push_str("  chain forward {\n");
    s.push_str("    type filter hook forward priority filter; policy accept;\n");
    if g.allow_icmp {
        s.push_str("    ip protocol icmp accept\n");
    }
    s.push_str("  }\n");

    // ---------- output ----------
    // 本机出站：带 mark 的包（mihomo 透明代理的连接）按 policy route
    // 走 local。这里只放行，不做拦截。
    s.push_str("  chain output {\n");
    s.push_str("    type filter hook output priority mangle; policy accept;\n");
    s.push_str(&format!("    meta mark {mark} accept\n", mark = fwmark));
    s.push_str("  }\n");

    s.push_str("}\n");
    s
}

/// policy routing：把带 mark 的包导向本机（tproxy 入口在本地）。
///
/// 为什么需要：tproxy 的原理是「把包路由到 loopback 再交给本地 socket」。
/// 普通路由表没有这条规则，所以要专门加一条 `ip rule fwmark`。
///
/// ★ **必须用数字表号，且不去改 `/etc/iproute2/rt_tables`**。
///
/// 真机实测踩了两次：
/// ```text
/// # ip rule add priority 100 fwmark 0x202 table clashx
/// Error: argument "clashx" is wrong: invalid table ID
/// ```
/// 名字要先在 rt_tables 注册。而注册本身也有坑 —— Ubuntu 24.04 的解析器
/// 要求「数字在前、名字在后」（参照文件里的注释行 `#1  inr.ruhep`），
/// 写成 `clashx 202` 会让整个文件变成：
/// ```text
/// Database /etc/iproute2/rt_tables is corrupted at clashx 202
/// ```
/// 连 `ip route show table 202` 都开始报错 —— 一个可选的可读性增强，
/// 代价是把系统网络工具搞坏，完全不划算。所以直接用数字，零依赖。
pub fn render_policy_route() -> String {
    format!(
        "#!/bin/sh\n\
         # 由 clashx 生成。\n\
         # policy routing: 带 fwmark 0x{mark:x} 的包 -> local（tproxy 入口）\n\
         ip rule del priority {prio} fwmark 0x{mark:x} table {num} 2>/dev/null || true\n\
         ip rule add priority {prio} fwmark 0x{mark:x} table {num}\n\
         ip route replace local default dev lo table {num}\n",
        prio = RULE_PRIO_LOCAL,
        mark = FWMARK,
        num = ROUTE_TABLE
    )
}

/// 卸载 policy route。
pub fn render_policy_route_down() -> String {
    format!(
        "ip rule del priority {prio} fwmark 0x{mark:x} table {num} 2>/dev/null || true\n\
         ip route flush table {num} 2>/dev/null || true\n",
        prio = RULE_PRIO_LOCAL,
        mark = FWMARK,
        num = ROUTE_TABLE
    )
}

/// 把 sysctl 开关打开（转发 + 特殊路由）。
///
/// ★ `net.ipv4.conf.all.route_localnet=1` 很多人会漏：
/// tproxy 要让「来自 LAN 的包被路由到 loopback」，内核默认拒绝这种转发
/// （`route_localnet=0`），表现是「规则命中了但连接一直超时」。
pub fn render_sysctl() -> String {
    "net.ipv4.ip_forward=1\n\
     net.ipv4.conf.all.route_localnet=1\n\
     net.ipv4.conf.all.src_valid_mark=1\n\
     net.ipv6.conf.all.forwarding=1\n"
        .to_string()
}

/// 执行命令，失败时把 stderr 拼进错误信息。
fn run(cmd: &str, args: &[&str], stdin: Option<&str>) -> Result<()> {
    let mut c = Command::new(cmd);
    c.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = c.spawn().with_context(|| format!("启动 {cmd}"))?;
    if let Some(data) = stdin {
        use std::io::Write;
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(data.as_bytes())
            .with_context(|| "写 stdin")?;
        drop(child.stdin.take());
    }
    let out = child.wait_with_output().context("等待子进程")?;
    if !out.status.success() {
        let mut msg = String::from_utf8_lossy(&out.stderr).trim().to_string();
        if msg.is_empty() {
            msg = String::from_utf8_lossy(&out.stdout).trim().to_string();
        }
        bail!("{cmd} {} 失败：{msg}", args.join(" "));
    }
    Ok(())
}

/// 当前是否处于启用状态（表存在 = 已启用）。
pub fn is_active() -> bool {
    Command::new("nft")
        .args(["list", "table", "inet", "clashx"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// 启用旁路由。
pub fn enable(g: &Gateway) -> Result<()> {
    g.precheck()?;

    // tproxy 表达式需要内核模块（多数发行版已内建，失败也不致命）
    for m in ["nft_tproxy", "xt_TPROXY"] {
        let _ = Command::new("modprobe")
            .arg(m)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    // 1) sysctl：转发 + 允许 route_localnet
    for line in render_sysctl().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (k, v) = line.split_once('=').unwrap();
        run("sysctl", &["-w", &format!("{k}={v}")], None)?;
    }

    // 2) policy routing
    run("sh", &["-c", &render_policy_route()], None)?;

    // 3) nftables 规则
    //    先删表（不存在会报错，忽略），保证幂等 —— 直接 `add table`
    //    在已存在时会失败，而我们要的是「应用最新配置」。
    let _ = Command::new("nft")
        .args(["delete", "table", "inet", "clashx"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    let rules = render_ruleset(g);

    // 先空跑校验：`nft -c` 是 check only，不实际应用。
    // 规则写错时这一步会报出行号，比直接 apply 好排查。
    {
        use std::io::Write;
        let mut child = Command::new("nft")
            .arg("-c")
            .arg("-f")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("启动 nft -c")?;
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(rules.as_bytes())
            .context("写规则到 nft -c")?;
        drop(child.stdin.take());
        let out = child.wait_with_output()?;
        if !out.status.success() {
            let msg = String::from_utf8_lossy(&out.stderr);
            bail!("nft 规则校验未通过：\n{msg}");
        }
    }

    run("nft", &["-f", "-"], Some(&rules))?;
    Ok(())
}

/// 停用旁路由。
pub fn disable() -> Result<()> {
    let _ = Command::new("nft")
        .args(["delete", "table", "inet", "clashx"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    run("sh", &["-c", &render_policy_route_down()], None)?;
    Ok(())
}

/// 打印当前状态。
pub fn status() -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "nft table inet clashx: {}\n",
        if is_active() { "已启用" } else { "未启用" }
    ));
    if let Ok(o) = Command::new("ip").args(["rule", "show"]).output() {
        let text = String::from_utf8_lossy(&o.stdout);
        // 规则里用的是数字表号，所以按数字 + fwmark 一起匹配
        let has = text.contains(&format!("fwmark 0x{:x}", FWMARK))
            || text.contains(&format!("fwmark 0x{:x} lookup {}", FWMARK, ROUTE_TABLE));
        s.push_str(&format!(
            "policy route (table {ROUTE_TABLE}, fwmark 0x{FWMARK:x}): {}\n",
            if has { "已配置" } else { "未配置" }
        ));
    }
    for k in [
        "net.ipv4.ip_forward",
        "net.ipv4.conf.all.route_localnet",
    ] {
        let v = std::fs::read_to_string(format!("/proc/sys/{}", k.replace('.', "/")))
            .unwrap_or_default()
            .trim()
            .to_string();
        s.push_str(&format!("{k} = {v}\n"));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gw() -> Gateway {
        Gateway {
            lan_addr: "172.20.0.101".into(),
            iface: "eth0".into(),
            tproxy_port: 7894,
            dns_port: 7874,
            lan_clients: vec!["172.20.0.0/24".into()],
            ..Default::default()
        }
    }

    #[test]
    fn 预检拦住空客户端网段() {
        let mut g = gw();
        g.lan_clients.clear();
        assert!(g.precheck().is_err());
    }

    #[test]
    fn 预检要求网卡() {
        let mut g = gw();
        g.iface.clear();
        assert!(g.precheck().is_err());
    }

    #[test]
    fn 预检要求本机地址() {
        let mut g = gw();
        g.lan_addr.clear();
        assert!(g.precheck().is_err());
    }

    #[test]
    fn 规则集含关键防环线条目() {
        let r = render_ruleset(&gw());
        assert!(r.contains("ip daddr 172.20.0.101 return"));
        assert!(r.contains("tproxy to :7894"));
        assert!(r.contains("dnat to :7874"));
        assert!(r.contains("table inet clashx"));
    }

    #[test]
    fn tproxy按协议分成两条() {
        // 回归测试：不带 meta l4proto 时nft -c 报
        // "Transparent proxy support requires transport protocol match"
        let r = render_ruleset(&gw());
        assert!(r.contains("meta l4proto tcp meta mark set 514 tproxy to :7894 accept"));
        assert!(r.contains("meta l4proto udp meta mark set 514 tproxy to :7894 accept"));
    }

    #[test]
    fn dnat后面不跟return() {
        // 回归测试：dnat 是 terminal statement，后面加 return 会被 nft 拒绝
        let r = render_ruleset(&gw());
        for line in r.lines() {
            if line.contains("dnat to") {
                assert!(
                    !line.trim_end().ends_with("return"),
                    "dnat 后面不能跟 return: {line}"
                );
            }
        }
    }

    #[test]
    fn 默认直连网段不含私网() {
        // 回归测试：曾经把 172.16/12 写进默认，导致 172.20 网段
        // 的目标全被当内网放行，代理彻底失效。
        let d = Gateway::default();
        for n in &d.direct_nets {
            assert!(
                !n.starts_with("172.16.") && !n.starts_with("10.") && !n.starts_with("192.168."),
                "默认直连网段不该含私网段：{n}"
            );
        }
    }

    #[test]
    fn 标记一致性() {
        // nft 里的 mark 与 policy route 里的 mark 必须一致，
        // 不一致的表现是「规则命中但连接超时」。
        let r = render_ruleset(&gw());
        let p = render_policy_route();
        let mark = format!("0x{:x}", FWMARK);
        assert!(p.contains(&mark), "policy route 缺少 mark {mark}");
        // 规则集里我们写的是十进制 FWMARK（nft 内部再规范化成
        // 0x00000202），所以按十进制比对。
        assert!(
            r.contains(&format!("meta mark set {FWMARK} tproxy")),
            "规则集缺少 mark {FWMARK}"
        );
        assert!(r.contains(&format!("meta mark {FWMARK} accept")));
    }

    #[test]
    fn 策略路由用数字表号() {
        // 回归测试：用表名会报 "invalid table ID"；
        // 而注册表名又会因格式不对让 rt_tables 整个损坏。
        let p = render_policy_route();
        assert!(
            p.contains(&format!("table {ROUTE_TABLE}")),
            "policy route 必须用数字表号 {ROUTE_TABLE}"
        );
        assert!(
            !p.contains("rt_tables"),
            "不该去改 /etc/iproute2/rt_tables：格式不对会把 ip 命令搞坏"
        );
    }


    #[test]
    fn prerouting必须是filter() {
        // 回归测试（本项目最关键的坑）：prerouting 写成 `type nat` 时，
        // 旁路由的转发流量整个绕过本链 —— 计数器实测 prerouting 0 包、
        // forward 1 包。表现为「baidu 能通但 google 全超时」。
        let r = render_ruleset(&gw());
        assert!(
            r.contains("type filter hook prerouting priority mangle"),
            "prerouting 必须是 filter+mangle，tproxy 才能拿到转发流量"
        );
        assert!(
            !r.contains("type nat hook prerouting priority dstnat; policy accept;\n    meta"),
            "prerouting 不该是 nat"
        );
        // dnat 只能在 nat 链，所以 DNS 劫持要单独一条 nat 链
        assert!(
            r.contains("type nat hook prerouting priority dstnat"),
            "DNS 劫持需要独立的 nat 链"
        );
    }

    #[test]
    fn 策略路由在main之前() {
        assert!(RULE_PRIO_LOCAL < 32766);
    }
}
