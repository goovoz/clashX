//! 命令行入口。
//!
//! 子命令按「配置 → 校验 → 应用 → 运行」的顺序排列，
//! 与 §5 的实施顺序对齐：本机代理模式先跑通，旁路由后置。

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::config::{Config, ProxyGroup, RuleProvider, Subscription};
use crate::core::{
    atomic_apply, core_running, find_core, geodata_ready, install_unit, validate_config,
    validate_current, Api, Paths, Systemd,
};
use crate::render;
use crate::rule;
use crate::{load_config, save_config};

#[derive(Parser)]
#[command(
    name = "mihomo-client",
    about = "Linux 无桌面 mihomo 客户端（本机代理 + 旁路由）",
    version
)]
pub struct Cli {
    /// 安装根目录（默认 /opt/mihomo-client，或环境变量 MIHOMO_CLIENT_HOME）
    #[arg(long, global = true)]
    pub home: Option<PathBuf>,

    /// mihomo 内核路径（默认在安装根下找，也尝试常见位置）
    #[arg(long, global = true)]
    pub core: Option<PathBuf>,

    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// 生成一份带默认值的配置并写入安装目录
    Init {
        /// 覆盖已有配置
        #[arg(long)]
        force: bool,
        /// 开启旁路由（tun + 自动管理防火墙）
        #[arg(long)]
        bypass_router: bool,
        /// 旁路由的出口网卡（防环路三手段之一）
        #[arg(long)]
        interface: Option<String>,
    },

    /// 校验配置（不改动任何东西）
    Validate,

    /// 渲染并原子替换配置：写临时文件 -> mihomo -t 校验 -> 改名
    Apply {
        /// 校验失败时也照样替换（危险，仅调试用）
        #[arg(long)]
        force: bool,
    },

    /// 启动 / 停止 / 重启内核服务
    #[command(subcommand)]
    Service(SvcCmd),

    /// 查看状态
    Status {
        /// 同时打印内核 RESTful API 的 /configs
        #[arg(long)]
        verbose: bool,
    },

    /// 安装 systemd unit 并 enable
    Install {
        /// mihomo 运行用户（解环路的关键：配合 tun.exclude-uid）
        #[arg(long, default_value = "nobody")]
        run_user: String,
        /// 运行组。留空则由 systemd 按用户名的默认组解析
        /// （Debian 12 的 nobody 属nogroup，硬写 nobody 会 216/GROUP）
        #[arg(long, default_value = "")]
        group: String,
        /// 不授予 CAP_NET_ADMIN（纯本机代理时不需要）
        #[arg(long)]
        no_net_admin: bool,
    },

    /// 订阅管理
    #[command(subcommand)]
    Sub(SubCmd),

    /// 列出当前所有节点（手动 + 各订阅合并后）
    Nodes {
        /// 只显示 名称/类型/服务器，不显示端口
        #[arg(long)]
        brief: bool,
    },

    /// 规则与规则集管理
    #[command(subcommand)]
    Rule(RuleCmd),

    /// 热切换运行模式（走内核 RESTful API，不重启）
    Mode {
        /// rule / global / direct
        value: String,
    },

    /// 打印渲染后的配置（调试用，不写文件）
    Show,

    /// 列出本机所有网络接口（配 bypass-router 时用）
    Ifaces,

    /// 旁路由（网关模式）：管理 nftables 转发规则
    #[command(subcommand)]
    Gateway(GwsCmd),

    /// 查看 / 切换运行模式（fake-ip / redir-host × 防火墙接管方式）
    #[command(subcommand)]
    RunMode(RunModeCmd),

    /// 启动 Web 管理界面（阻塞式，前台运行）
    Web {
        /// 监听地址。省略则读配置里的 web.listen（默认 0.0.0.0:9080）
        #[arg(long)]
        listen: Option<String>,
        /// 只监听本机（不开局域网访问）
        #[arg(long)]
        localhost: bool,
    },

    /// 安装 Web UI 的 systemd unit 并 enable（开机自启）
    WebInstall {
        /// 监听地址
        #[arg(long)]
        listen: Option<String>,
        /// 安装后立即启动
        #[arg(long, default_value_t = true)]
        start: bool,
    },

    /// 设置 Web 登录密码（不放在命令行参数里，避免进 shell 历史）
    WebPass {
        /// 从标准输入读密码（推荐：echo -n 'pw' | clashx web-pass）
        #[arg(long)]
        stdin: bool,
    },
}

/// 旁路由子命令。
/// 运行模式子命令。
#[derive(Subcommand)]
pub enum RunModeCmd {
    /// 显示当前运行模式与全部可选值
    Show,
    /// 切换运行模式（会重启内核）
    Set {
        /// fake-ip / redir-host / fake-ip-tun / redir-host-tun / fake-ip-mix / redir-host-mix
        value: String,
    },
}

/// 全部运行模式（顺序即 UI 展示顺序：对齐 OpenClash 的排列）。
pub const ALL_RUN_MODES: [crate::runmode::RunMode; 6] = [
    crate::runmode::RunMode::FakeIp,
    crate::runmode::RunMode::RedirHost,
    crate::runmode::RunMode::FakeIpTun,
    crate::runmode::RunMode::RedirHostTun,
    crate::runmode::RunMode::FakeIpMix,
    crate::runmode::RunMode::RedirHostMix,
];

#[derive(Subcommand)]
pub enum GwsCmd {
    /// 启用旁路由：nftables + policy routing
    Enable {
        /// 本机局域网地址（旁路由的网关地址）。
        /// 省略时自动探测（取默认路由所在网段的本机地址）
        #[arg(long)]
        lan_addr: Option<String>,
        /// 出口网卡。省略时自动探测默认路由出口
        #[arg(long)]
        iface: Option<String>,
        /// 客户端网段，逗号分隔（只这些网段的流量会被代理）
        #[arg(long, default_value = "")]
        clients: String,
        /// tproxy 端口（要与配置里的 tproxy-port 一致）
        #[arg(long, default_value_t = 7894)]
        tproxy_port: u16,
        /// DNS 劫持端口（0 = 不劫持）
        #[arg(long, default_value_t = 7874)]
        dns_port: u16,
    },
    /// 停用旁路由（删规则与 policy route，保留配置）
    Disable,
    /// 查看旁路由状态
    Status,
    /// 打印将要应用的 nft 规则（调试用，不实际应用）
    DryRun {
        /// 本机局域网地址（省略则自动探测）
        #[arg(long)]
        lan_addr: Option<String>,
        /// 出口网卡（省略则自动探测）
        #[arg(long)]
        iface: Option<String>,
        #[arg(long, default_value = "")]
        clients: String,
        #[arg(long, default_value_t = 7894)]
        tproxy_port: u16,
        #[arg(long, default_value_t = 7874)]
        dns_port: u16,
    },
}

#[derive(Subcommand)]
pub enum SubCmd {
    /// 添加或更新一个订阅
    Add {
        /// 订阅名（也是节点名前缀的来源）
        name: String,
        /// 订阅地址（含token，不要贴到公开仓库）
        url: String,
        /// 建议更新间隔（小时）
        #[arg(long, default_value_t = 24)]
        interval: u32,
        /// User-Agent。★必须是 clash.meta 才会返回 YAML 格式
        #[arg(long, default_value = "clash.meta")]
        user_agent: String,
        /// 只保留含这些关键词的节点（可重复）
        #[arg(long)]
        keyword: Vec<String>,
        /// 排除含这些关键词的节点（可重复）
        #[arg(long)]
        exclude: Vec<String>,
        /// 请求头，形如 `X-Custom: value`（可重复）
        #[arg(long)]
        header: Vec<String>,
        /// URL 额外参数，形如 `flag=clash`（可重复）
        #[arg(long)]
        param: Vec<String>,
    },
    /// 列出已配置的订阅
    List,
    /// 抓取每个订阅并报告结果（不改配置）
    Test,
    /// 删除一个订阅
    Rm {
        /// 订阅名
        name: String,
    },
}

#[derive(Subcommand)]
pub enum RuleCmd {
    /// 列出规则链（按顺序）
    List {
        /// 显示行号
        #[arg(long, short)]
        numbered: bool,
    },
    /// 追加一条规则到链尾（MATCH 之前）
    Add {
        /// 规则原文，如 "DOMAIN-SUFFIX,example.com,DIRECT"
        rule: String,
        /// 插到第几条之后（默认插到 MATCH 前，即末尾）
        #[arg(long)]
        after: Option<usize>,
    },
    /// 删除规则（按行号，从 1 开始）
    Rm {
        /// 行号
        index: usize,
    },
    /// 测试某个域名会走哪条规则
    Test {
        /// 要测试的域名
        domain: String,
        /// 域名解析到的 IP（可选，用于 IP-CIDR / GEOIP 类规则）
        #[arg(long)]
        ip: Option<String>,
    },

    /// 规则集管理
    #[command(subcommand)]
    Provider(ProviderCmd),

    /// 载入一套实用的规则预设（国内直连 + 广告拦截 + 境外代理）
    Preset {
        /// 预设名
        name: Option<String>,
        /// 列出可用预设
        #[arg(long)]
        list: bool,
    },
}

#[derive(Subcommand)]
pub enum ProviderCmd {
    /// 列出规则集
    List,
    /// 添加/更新规则集
    Add {
        name: String,
        url: String,
        /// domain / ipcidr / classical
        #[arg(long, default_value = "domain")]
        behavior: String,
        /// 更新间隔（小时）
        #[arg(long, default_value_t = 86400 / 3600)]
        interval: u32,
        /// 本地缓存路径（默认 ./ruleset/<name>.yaml）
        #[arg(long)]
        path: Option<String>,
    },
    /// 删除规则集
    Rm { name: String },
    /// 检查规则集 URL 是否可用
    Check { name: String },
}

#[derive(Subcommand)]
pub enum SvcCmd {
    Start,
    Stop,
    Restart,
    Log {
        #[arg(long, default_value_t = 50)]
        lines: u32,
    },
}

impl Cli {
    pub fn run(&self, paths: &Paths) -> Result<()> {
        match &self.cmd {
            Cmd::Init {
                force,
                bypass_router,
                interface,
            } => self.init(paths, *force, *bypass_router, interface.as_deref()),
            Cmd::Validate => self.validate(paths),
            Cmd::Apply { force } => self.apply(paths, *force),
            Cmd::Service(c) => self.service(paths, c),
            Cmd::Status { verbose } => self.status(paths, *verbose),
            Cmd::Install {
                run_user,
                group,
                no_net_admin,
            } => self.install(paths, run_user, group, !*no_net_admin),
            Cmd::Sub(c) => self.sub(paths, c),
            Cmd::Nodes { brief } => self.nodes(paths, *brief),
            Cmd::Rule(c) => self.rule(paths, c),
            Cmd::Mode { value } => self.set_mode(paths, value),
            Cmd::Show => self.show(paths),
            Cmd::Ifaces => self.ifaces(),
            Cmd::Gateway(c) => self.gateway(paths, c),
            Cmd::RunMode(c) => match c {
                RunModeCmd::Show => self.run_mode_show(paths),
                RunModeCmd::Set { value } => self.run_mode_set(paths, value),
            },
            Cmd::Web { listen, localhost } => self.web(paths, listen.as_deref(), *localhost),
            Cmd::WebInstall { listen, start } => self.web_install(paths, listen.as_deref(), *start),
            Cmd::WebPass { stdin } => self.web_pass(paths, *stdin),
        }
    }

    // ---- init ----
    fn init(
        &self,
        paths: &Paths,
        force: bool,
        bypass: bool,
        iface: Option<&str>,
    ) -> Result<()> {
        if paths.user_config.exists() && !force {
            bail!(
                "配置已存在：{}（要覆盖加 --force）",
                paths.user_config.display()
            );
        }
        let mut cfg = Config::default();

        // interface-name：默认填检测到的出口网卡。
        // 不设它在本机实测会导致「规则命中但连接超时」
        // （本机 IPv4 默认路由不明确），见 render::precheck 的说明。
        cfg.interface_name = iface
            .map(|s| s.to_string())
            .or_else(default_route_iface);

        // ★ IPv6：实测有些环境 IPv4 出站不通、只有 IPv6 能出网
        // （本机192.168.10.1 就是：curl -4 超时、curl -6 正常）。
        // 而 mihomo 的 `ipv6: false` 会**拒答AAAA 记录**，
        // 于是内核只拿到 A 记录 -> 连不通 -> 表现是
        //     [TCP] dial PROXY ... dns resolve failed: context deadline exceeded
        // 或直接连接超时。所以这里探测一下：IPv4 不通而 IPv6 通时
        // 自动开IPv6，避免用户一上手就撞墙。
        if iface.is_none() {
            cfg.dns.ipv6 = detect_ipv6_only();
        }

        if bypass {
            use crate::config::TransparentMode;
            cfg.transparent.mode = TransparentMode::Tun;
            cfg.inbound.allow_lan = true;
            cfg.transparent.exclude_interface = vec!["lo".into()];
            // 解环路：mihomo 以专用用户跑，靠 exclude-uid 让它自己绕过
            cfg.transparent.exclude_uid = vec![65534];
            cfg.run_user = Some("nobody".into());
        }

        // 给一个能跑的默认组，规则才有目标可指
        cfg.proxy_groups = vec![
            ProxyGroup {
                name: "PROXY".into(),
                proxies: vec![],
                ..Default::default()
            },
            ProxyGroup {
                name: "AUTO".into(),
                ..Default::default()
            },
        ];
        cfg.proxy_groups[1].group_type = crate::config::GroupType::UrlTest;
        cfg.proxy_groups[1].proxies = vec!["PROXY".into(), "DIRECT".into()];

        save_config(paths, &cfg)?;
        println!("已生成默认配置：{}", paths.user_config.display());
        if let Some(n) = cfg.interface_name.as_deref() {
            println!("  出口网卡：{n}（自动检测，可用 --interface 覆盖）");
        } else {
            println!("  出口网卡：未检测到，建议用 --interface 显式指定");
        }
        if cfg.dns.ipv6 {
            println!("  已检测到本机 IPv4 出站不通、仅 IPv6 可用 —— 自动开启 IPv6");
        }
        if bypass {
            println!(
                "  旁路由已开启（tun + auto-redirect）\n\
                 \x20 出口网卡：{}\n\
                 \x20 下一步：mihomo-client install --run-user nobody",
                cfg.interface_name.as_deref().unwrap_or("(自动)")
            );
        }
        println!("  下一步：mihomo-client apply");
        Ok(())
    }

    // ---- validate ----
    fn validate(&self, paths: &Paths) -> Result<()> {
        let cfg = load_config(paths)?;
        let warns = render::precheck(&cfg);
        if !warns.is_empty() {
            println!("预检查提示：");
            for w in &warns {
                println!("  - {w}");
            }
        }
        let core = resolve_core(self, paths)?;
        let (proxies, warns) = crate::collect_proxies(&cfg, paths);
        for w in &warns {
            eprintln!("提示: {w}");
        }
        let out = render::render(&cfg, &proxies);
        // 临时文件放在工作目录里（保证与真实运行同环境），用完删掉
        let tmp = paths.workdir.join(".validate.yaml");
        std::fs::create_dir_all(&paths.workdir)?;
        std::fs::write(&tmp, &out.yaml)?;
        let r = validate_config(&core, &paths.workdir, &tmp);
        let _ = std::fs::remove_file(&tmp);
        r?;
        println!("配置校验通过（内核 {}）", core.display());
        Ok(())
    }

    // ---- apply ----
    fn apply(&self, paths: &Paths, force: bool) -> Result<()> {
        let cfg = load_config(paths)?;
        let warns = render::precheck(&cfg);
        for w in &warns {
            eprintln!("提示: {w}");
        }

        let (proxies, warns) = crate::collect_proxies(&cfg, paths);
        for w in &warns {
            eprintln!("提示: {w}");
        }
        let out = render::render(&cfg, &proxies);
        for w in &out.warnings {
            eprintln!("提示: {w}");
        }

        if force {
            // --force：跳过内核校验直接写。用来测「配置写坏了会怎样」，
            // 但那样内核会起不来，旧配置也已被覆盖。
            std::fs::create_dir_all(paths.generated.parent().unwrap())?;
            std::fs::write(&paths.generated, &out.yaml)?;
            println!("已写入 {}（未校验）", paths.generated.display());
        } else {
            // 原子切换：写 .new -> 用找到的内核校验 -> 改名覆盖。
            // 若 --core 指向的路径与安装根下的不同，用那个路径校验。
            let core = resolve_core(self, paths)?;
            let eff = Paths {
                core,
                ..paths.clone()
            };
            atomic_apply(&eff, &out.yaml)?;
            println!("配置已生效：{}", eff.generated.display());
        }

        // 已在跑就热重载
        let sd = Systemd::new("mihomo-client");
        if sd.is_active() {
            let _ = sd.restart();
            println!("已重启 mihomo-client 服务");
        }
        Ok(())
    }

    // ---- service ----
    fn service(&self, paths: &Paths, c: &SvcCmd) -> Result<()> {
        let sd = Systemd::new("mihomo-client");
        match c {
            SvcCmd::Start => {
                // 启动前先校验，避免起不来还留着坏配置
                if paths.generated.exists() {
                    let core = resolve_core(self, paths)?;
                    if let Err(e) = validate_config(&core, &paths.workdir, &paths.generated) {
                        eprintln!("启动中止：当前配置未通过校验\n{e:#}");
                        bail!("如需强制启动，先修好配置或重新 apply");
                    }
                }
                sd.start()?;
                println!("已启动");
            }
            SvcCmd::Stop => {
                sd.stop()?;
                println!("已停止");
            }
            SvcCmd::Restart => {
                sd.restart()?;
                println!("已重启");
            }
            SvcCmd::Log { lines } => {
                let out = std::process::Command::new("journalctl")
                    .args(["-u", &sd.unit, "--no-pager", "-n", &lines.to_string()])
                    .output()?;
                print!("{}", String::from_utf8_lossy(&out.stdout));
            }
        }
        Ok(())
    }

    // ---- status ----
    fn status(&self, paths: &Paths, verbose: bool) -> Result<()> {
        let sd = Systemd::new("mihomo-client");
        println!("安装根: {}", paths.root.display());
        println!(
            "  用户配置: {}",
            paths
                .user_config
                .exists()
                .then(|| "有")
                .unwrap_or("无（先跑 init）")
        );
        println!(
            "  渲染配置: {}",
            paths
                .generated
                .exists()
                .then(|| "有")
                .unwrap_or("无（先跑 apply）")
        );
        println!(
            "  systemd: {}{}",
            if sd.is_active() { "active" } else { "inactive" },
            if sd.is_enabled() { "（开机自启）" } else { "" }
        );

        let core = resolve_core(self, paths).ok();
        if let Some(c) = core.as_ref() {
            println!("  内核: {}", c.display());
            println!("  内核进程: {}", if core_running(c) { "运行中" } else { "未运行" });
        }
        // 控制地址从**已生成的配置**读 —— 那是 mihomo 实际在用的
        // （用户可能改过端口，硬编码会打到别的服务上）
        if let Some(v) = read_external_controller(paths) {
            let api = Api::new(&v, read_secret(paths));
            match api.version() {
                Ok(ver) => println!("  内核版本: {}", ver.trim()),
                Err(e) => println!("  控制接口: {}（{e}）", v),
            }
        }

        // GeoData 未就绪会表现为「所有请求 502」，很容易被误判成配置错。
        // 首启要下载 GeoIP/GeoSite，所以这里显式报出来。
        let (ready, missing) = geodata_ready(&paths.workdir);
        if ready {
            println!("  GeoData: 就绪");
        } else {
            println!(
                "  GeoData: 缺失 {}（首启需下载，未就绪前规则请求会 502）",
                missing.join("、")
            );
        }

        if verbose {
            if validate_current(paths).is_ok() {
                println!("  当前配置校验: 通过");
            } else {
                println!("  当前配置校验: 未通过");
            }
            let sd2 = Systemd::new("mihomo-client");
            if sd2.is_active() {
                println!("\n{}", sd2.status().unwrap_or_default());
            }
        }
        Ok(())
    }

    // ---- install ----
    fn install(
        &self,
        paths: &Paths,
        run_user: &str,
        group: &str,
        net_admin: bool,
    ) -> Result<()> {
        let core = resolve_core(self, paths)?;
        let grp = (!group.is_empty()).then_some(group);
        install_unit(paths, Some(run_user), grp, net_admin)?;
        let sd = Systemd::new("mihomo-client");
        sd.enable()?;
        println!(
            "已安装 systemd unit（运行用户 {run_user}{}）",
            if net_admin {
                "，含 CAP_NET_ADMIN"
            } else {
                ""
            }
        );
        println!("  unit: {}", paths.systemd_unit().display());
        let _ = core;
        println!("  下一步：mihomo-client apply && mihomo-client service start");
        Ok(())
    }

    // ---- sub ----
    fn sub(&self, paths: &Paths, c: &SubCmd) -> Result<()> {
        match c {
            SubCmd::Add {
                name,
                url,
                interval,
                user_agent,
                keyword,
                exclude,
                header,
                param,
            } => {
                let mut cfg = load_config(paths)?;
                let sub = Subscription {
                    name: name.clone(),
                    url: url.clone(),
                    user_agent: Some(user_agent.clone()),
                    update_interval: *interval,
                    keyword: keyword.clone(),
                    exclude_keyword: exclude.clone(),
                    headers: header.clone(),
                    extra_params: param.clone(),
                    ..Default::default()
                };
                // 已有同名则更新（保留未提供的字段）
                if let Some(old) = cfg.subscriptions.iter_mut().find(|s| &s.name == name) {
                    if !keyword.is_empty() {
                        old.keyword = keyword.clone();
                    }
                    if !exclude.is_empty() {
                        old.exclude_keyword = exclude.clone();
                    }
                    if !header.is_empty() {
                        old.headers = header.clone();
                    }
                    if !param.is_empty() {
                        old.extra_params = param.clone();
                    }
                    old.url = url.clone();
                    old.update_interval = *interval;
                    if user_agent != "clash.meta" {
                        old.user_agent = Some(user_agent.clone());
                    }
                    println!("已更新订阅 {name}");
                } else {
                    cfg.subscriptions.push(sub);
                    println!("已添加订阅 {name}");
                }
                save_config(paths, &cfg)?;

                // 立刻抓一次验证，让用户马上知道格式对不对
                match crate::sub::fetch(&cfg.subscriptions
                    .iter()
                    .find(|s| &s.name == name)
                    .unwrap(), "")
                {
                    Ok((body, headers)) => {
                        let fmt = crate::sub::detect_format(&body);
                        let info = crate::sub::parse_sub_headers(&headers);
                        println!("  格式: {fmt:?}");
                        if let Ok(list) = crate::sub::extract_proxies(&body, fmt) {
                            println!("  节点数: {}", list.len());
                            let filtered =
                                crate::sub::filter_proxies(list, &cfg.subscriptions[0]);
                            println!("  过滤后: {}", filtered.len());
                        }
                        if let Some(r) = info.remaining_human() {
                            println!("  流量: {r}");
                        }
                        if let Some(e) = info.expire_human() {
                            println!("  到期: {e}");
                        }
                    }
                    Err(e) => println!("  抓取失败: {e}"),
                }
                println!("  下一步: mihomo-client apply");
            }
            SubCmd::List => {
                let cfg = load_config(paths)?;
                if cfg.subscriptions.is_empty() {
                    println!("没有配置任何订阅");
                    return Ok(());
                }
                println!("{:<14} {:<28} {:>6} {:>6}  {}", "名称", "UA", "间隔", "过滤", "URL");
                for s in &cfg.subscriptions {
                    // URL 含 token，只显示前 24 字符
                    let u = if s.url.len() > 24 {
                        format!("{}…", &s.url[..24])
                    } else {
                        s.url.clone()
                    };
                    println!(
                        "{:<14} {:<28} {:>5}h {:>6}  {}",
                        s.name,
                        s.user_agent.clone().unwrap_or_default(),
                        s.update_interval,
                        s.keyword.len() + s.exclude_keyword.len(),
                        u
                    );
                }
            }
            SubCmd::Test => {
                let cfg = load_config(paths)?;
                if cfg.subscriptions.is_empty() {
                    println!("没有配置任何订阅");
                    return Ok(());
                }
                for s in &cfg.subscriptions {
                    print!("{}: ", s.name);
                    match crate::sub::fetch(s, "") {
                        Ok((body, headers)) => {
                            let fmt = crate::sub::detect_format(&body);
                            let info = crate::sub::parse_sub_headers(&headers);
                            match crate::sub::extract_proxies(&body, fmt) {
                                Ok(list) => {
                                    let f = crate::sub::filter_proxies(list, s);
                                    let mut types: Vec<String> = Vec::new();
                                    for p in &f {
                                        if !types.contains(&p.proxy_type) {
                                            types.push(p.proxy_type.clone());
                                        }
                                    }
                                    types.sort();
                                    print!(
                                        "OK  {} 个节点（过滤后 {}）协议[{}]",
                                        f.len(),
                                        f.len(),
                                        types.join(",")
                                    );
                                    if let Some(p) = info.used_percent() {
                                        print!(" 流量已用 {p:.1}%");
                                    }
                                    if let Some(r) = info.remaining_human() {
                                        print!(" 剩余 {r}");
                                    }
                                    println!();
                                }
                                Err(e) => println!("格式不可用（{fmt:?}）: {e}"),
                            }
                        }
                        Err(e) => println!("FAIL {e}"),
                    }
                }
            }
            SubCmd::Rm { name } => {
                let mut cfg = load_config(paths)?;
                let before = cfg.subscriptions.len();
                cfg.subscriptions.retain(|s| &s.name != name);
                if cfg.subscriptions.len() == before {
                    println!("没有名为 {name} 的订阅");
                } else {
                    save_config(paths, &cfg)?;
                    println!("已删除订阅 {name}，剩余 {} 个", cfg.subscriptions.len());
                }
            }
        }
        Ok(())
    }

    // ---- nodes ----
    fn nodes(&self, paths: &Paths, brief: bool) -> Result<()> {
        let cfg = load_config(paths)?;
        let (proxies, warns) = crate::collect_proxies(&cfg, paths);
        for w in &warns {
            eprintln!("提示: {w}");
        }
        if proxies.is_empty() {
            println!("没有可用节点（也没配订阅）。");
            println!("添加订阅：mihomo-client sub add <名字> <url>");
            return Ok(());
        }
        // 按类型统计
        let mut types: std::collections::BTreeMap<String, usize> = Default::default();
        for p in &proxies {
            *types.entry(p.proxy_type.clone()).or_default() += 1;
        }
        println!("共 {} 个节点：", proxies.len());
        for (t, n) in &types {
            println!("  {:<14} {n}", t);
        }
        println!();
        for p in &proxies {
            if brief {
                println!("  {:<32} {:<12} {}", p.name, p.proxy_type, p.server);
            } else {
                println!(
                    "  {:<32} {:<12} {}:{}",
                    p.name, p.proxy_type, p.server, p.port
                );
            }
        }
        Ok(())
    }

    // ---- rule ----
    fn rule(&self, paths: &Paths, c: &RuleCmd) -> Result<()> {
        match c {
            RuleCmd::List { numbered } => {
                let numbered = *numbered;
                let cfg = load_config(paths)?;
                if cfg.rules.is_empty() {
                    println!("规则链为空。至少要有一条 MATCH 兜底，否则 mihomo 拒绝启动。");
                    return Ok(());
                }
                println!("规则链（自上而下，首次命中即生效）：");
                for (i, r) in cfg.rules.iter().enumerate() {
                    if numbered {
                        println!("  {:>3}. {}", i + 1, r);
                    } else {
                        println!("  {}", r);
                    }
                }
                println!("\n共 {} 条", cfg.rules.len());
                if cfg.rules.iter().all(|r| !r.trim().to_ascii_uppercase().starts_with("MATCH"))
                {
                    println!("! 注意：没有 MATCH 兜底规则，mihomo -t 会报错");
                }
            }
            RuleCmd::Add { rule, after } => {
                let rule = rule.clone();
                let after = *after;
                rule::validate_rule(&rule)?;
                let mut cfg = load_config(paths)?;
                // 目标必须是已定义的组或内置策略
                let groups: Vec<String> =
                    cfg.proxy_groups.iter().map(|g| g.name.clone()).collect();
                let targets = rule::builtin_targets(&groups);
                let parts: Vec<&str> = rule.split(',').collect();
                if let Some(t) = parts.last() {
                    let t = t.trim();
                    if !t.is_empty()
                        && !t.eq_ignore_ascii_case("no-resolve")
                        && !targets.iter().any(|x| x.eq_ignore_ascii_case(t))
                    {
                        bail!(
                            "规则目标 \"{t}\" 既不是内置策略，也不是已定义的代理组。\n\
                             已定义：{}",
                            groups.join(" / ")
                        );
                    }
                }
                let pos = match after {
                    Some(n) => n.min(cfg.rules.len()),
                    None => {
                        // 默认插到 MATCH 之前（保持兜底永远在最后）
                        cfg.rules
                            .iter()
                            .position(|r| r.trim().to_ascii_uppercase().starts_with("MATCH"))
                            .unwrap_or(cfg.rules.len())
                    }
                };
                cfg.rules.insert(pos, rule.clone());
                save_config(paths, &cfg)?;
                println!("已在第 {} 条插入：{}", pos + 1, rule);
                println!("  下一步: mihomo-client apply");
            }
            RuleCmd::Rm { index } => {
                let index = *index;
                let mut cfg = load_config(paths)?;
                if index == 0 || index > cfg.rules.len() {
                    bail!("行号超出范围（1..={}）", cfg.rules.len());
                }
                let removed = cfg.rules.remove(index - 1);
                save_config(paths, &cfg)?;
                println!("已删除第 {} 条：{}", index, removed);
                println!("  下一步: mihomo-client apply");
            }
            RuleCmd::Test { domain, ip } => {
                let cfg = load_config(paths)?;
                let (d, i) = rule::split_target(domain, ip.as_deref());
                if d.is_empty() {
                    bail!("请给出要测试的域名");
                }
                let groups: Vec<String> =
                    cfg.proxy_groups.iter().map(|g| g.name.clone()).collect();
                let providers = rule::load_provider_contents(&cfg.rule_providers, &paths.workdir);
                let m = rule::Matcher {
                    rules: &cfg.rules,
                    providers: providers.clone(),
                    builtins: groups,
                };
                match rule::explain(&m, Some(&d), i.as_deref()) {
                    Ok(v) => {
                        println!("域名: {d}{}", i.as_ref().map(|x| format!("  IP: {x}")).unwrap_or_default());
                        println!("命中: 第 {} 条  {}  ->  {}", v.index, v.kind, v.target);
                        println!("规则原文: {}", v.rule);
                        if v.fallback {
                            println!("（这是 MATCH 兜底规则，说明没有任何更具体的规则命中）");
                        }
                        let missing: Vec<&String> = cfg
                            .rule_providers
                            .iter()
                            .filter(|p| !providers.contains_key(&p.name))
                            .map(|p| &p.name)
                            .collect();
                        if !missing.is_empty() {
                            println!(
                                "\n注意：规则集 {:?} 尚未下载到 {}，\
                                 相关规则本次按「不命中」处理。\n      \
                                 先 mihomo-client apply 让内核拉取，再测。",
                                missing,
                                paths.workdir.display()
                            );
                        }
                    }
                    Err(e) => println!("无法判定: {e}"),
                }
            }
            RuleCmd::Provider(c) => self.provider(paths, c)?,
            RuleCmd::Preset { name, list } => {
                let list = *list;
                if list || name.is_none() {
                    println!("可用规则预设：");
                    for p in rule::PRESETS {
                        println!("  {:<10} {}（{} 条规则）", p.name, p.desc, p.rules.len());
                    }
                    println!("\n用法: mihomo-client rule preset <名字>");
                    println!("  可叠加多个，如: preset private && preset cn");
                    return Ok(());
                }
                let p = rule::find_preset(name.as_deref().unwrap())
                    .ok_or_else(|| anyhow::anyhow!(
                        "没有名为 {:?} 的预设。可用：{}",
                        name,
                        rule::PRESETS.iter().map(|x| x.name).collect::<Vec<_>>().join(" / ")
                    ))?;
                let mut cfg = load_config(paths)?;
                let mut added_p = 0;
                for (n, b, u) in p.providers {
                    if cfg.rule_providers.iter().any(|x| x.name == *n) {
                        println!("  规则集 {n} 已存在，跳过");
                        continue;
                    }
                    let mut np = RuleProvider {
                        name: n.to_string(),
                        behavior: b.to_string(),
                        url: u.to_string(),
                        path: String::new(),
                        interval: 0,
                    };
                    rule::normalize(&mut np)?;
                    cfg.rule_providers.push(np);
                    added_p += 1;
                    println!("  加规则集 {n} ({b})");
                }
                // 规则插到 MATCH 前
                let mut added_r = 0;
                for r in p.rules {
                    if cfg.rules.iter().any(|x| x == r) {
                        continue;
                    }
                    let pos = cfg
                        .rules
                        .iter()
                        .position(|x| x.trim().to_ascii_uppercase().starts_with("MATCH"))
                        .unwrap_or(cfg.rules.len());
                    cfg.rules.insert(pos, r.to_string());
                    added_r += 1;
                }
                save_config(paths, &cfg)?;
                println!(
                    "预设 {} 已套用：{} 个规则集 + {} 条规则",
                    p.name, added_p, added_r
                );
                println!("  下一步: mihomo-client apply");
            }
        }
        Ok(())
    }

    // ---- provider ----
    fn provider(&self, paths: &Paths, c: &ProviderCmd) -> Result<()> {
        match c {
            ProviderCmd::List => {
                let cfg = load_config(paths)?;
                if cfg.rule_providers.is_empty() {
                    println!("没有配置任何规则集");
                    println!("添加：mihomo-client rule provider add <名> <url> --behavior domain");
                    return Ok(());
                }
                println!("{:<16} {:<10} {:>8} {:>10}  {}", "名称", "behavior", "间隔", "已缓存", "URL");
                for p in &cfg.rule_providers {
                    let rel = p.path.trim_start_matches("./");
                    let cached = paths.workdir.join(rel);
                    let (sz, mt) = if cached.exists() {
                        (
                            std::fs::metadata(&cached).map(|m| m.len()).unwrap_or(0),
                            std::fs::metadata(&cached)
                                .and_then(|m| m.modified())
                                .ok()
                                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                                .map(|d| d.as_secs())
                                .unwrap_or(0),
                        )
                    } else {
                        (0, 0)
                    };
                    let age = if mt > 0 {
                        let h = (std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(0)
                            - mt)
                            / 3600;
                        format!("{}B/{}h", sz, h)
                    } else {
                        "未下载".into()
                    };
                    println!(
                        "{:<16} {:<10} {:>7}h {:>10}  {}",
                        p.name,
                        p.behavior,
                        p.interval / 3600,
                        age,
                        p.url
                    );
                }
            }
            ProviderCmd::Add {
                name,
                url,
                behavior,
                interval,
                path,
            } => {
                let mut p = RuleProvider {
                    name: name.clone(),
                    behavior: behavior.clone(),
                    url: url.clone(),
                    path: path.clone().unwrap_or_default(),
                    interval: *interval,
                };
                rule::normalize(&mut p)?;
                let mut cfg = load_config(paths)?;
                if let Some(old) = cfg.rule_providers.iter_mut().find(|x| x.name == *name) {
                    *old = p;
                    println!("已更新规则集 {name}");
                } else {
                    cfg.rule_providers.push(p);
                    println!("已添加规则集 {name}（{}）", behavior);
                }
                save_config(paths, &cfg)?;
                println!("  下一步: mihomo-client apply（内核会首次下载）");
            }
            ProviderCmd::Rm { name } => {
                let mut cfg = load_config(paths)?;
                let before = cfg.rule_providers.len();
                cfg.rule_providers.retain(|x| &x.name != name);
                if cfg.rule_providers.len() == before {
                    println!("没有名为 {name} 的规则集");
                } else {
                    save_config(paths, &cfg)?;
                    println!("已删除规则集 {name}");
                }
            }
            ProviderCmd::Check { name } => {
                let cfg = load_config(paths)?;
                let p = cfg
                    .rule_providers
                    .iter()
                    .find(|x| &x.name == name)
                    .ok_or_else(|| anyhow::anyhow!("没有名为 {name} 的规则集"))?;
                print!("检查 {name} 的 URL 可达性 ... ");
                match rule::probe_url(&p.url) {
                    Ok(code) => println!("OK（HTTP {code}）"),
                    Err(e) => println!("失败：{e}"),
                }
                let rel = p.path.trim_start_matches("./");
                let cached = paths.workdir.join(rel);
                println!(
                    "  本地缓存：{}",
                    if cached.exists() { "已存在" } else { "尚未下载（先 apply）" }
                );
            }
        }
        Ok(())
    }

    // ---- mode ----
    fn set_mode(&self, paths: &Paths, value: &str) -> Result<()> {
        if !matches!(value, "rule" | "global" | "direct") {
            bail!("模式只能是 rule / global / direct");
        }
        // 控制地址与 secret 都从**已生成的配置**读 —— 那是内核实际在用的
        let addr = read_external_controller(paths).ok_or_else(|| {
            anyhow::anyhow!(
                "读不到控制接口地址（{} 里没有 external-controller），先跑 apply",
                paths.generated.display()
            )
        })?;
        let api = Api::new(&addr, read_secret(paths));
        api.set_mode(value)?;
        println!("已切换到 {value}（热切换，未重启）");
        Ok(())
    }

    // ---- show ----
    fn show(&self, paths: &Paths) -> Result<()> {
        let cfg = load_config(paths)?;
        let (proxies, warns) = crate::collect_proxies(&cfg, paths);
        for w in &warns {
            eprintln!("提示: {w}");
        }
        let out = render::render(&cfg, &proxies);
        print!("{}", out.yaml);
        for w in &out.warnings {
            eprintln!("// 提示: {w}");
        }
        Ok(())
    }

    // ---- ifaces ----
    fn ifaces(&self) -> Result<()> {
        let out = std::process::Command::new("ip")
            .args(["-o", "link", "show"])
            .output()
            .context("执行 ip link 失败（需要 iproute2）")?;
        let text = String::from_utf8_lossy(&out.stdout);
        // 形如 `2: eth0: <BROADCAST,MULTICAST,UP,...> mtu 1500 ...`
        // 也有 `1: lo@if2: <LOOPBACK,...>`
        // 接口名是**第一个** `:` 之后到下一个 `:`（或 `@`）之间的部分。
        // 第一版用 `find(": ")` 会误匹配到后面的 `link/loopback` 之类
        // （因为 flags 段里有空格+冒号），导致整行被当成一个接口。
        println!("{:<12} {}", "接口", "状态");
        for line in text.lines() {
            let rest = match line.find(':') {
                Some(i) => &line[i + 1..],
                None => continue,
            };
            let end = rest
                .find(|c| c == ':' || c == '@')
                .unwrap_or_else(|| rest.find(' ').unwrap_or(rest.len()));
            let name = rest[..end].trim();
            if name.is_empty() {
                continue;
            }
            let state = if line.contains("state UP") {
                "UP"
            } else if line.contains("state DOWN") {
                "DOWN"
            } else {
                "UNKNOWN"
            };
            println!("{name:<12} {state}");
        }
        if let Some(d) = default_route_iface() {
            println!("\n默认路由出口网卡: {d}");
        } else {
            println!("\n默认路由出口网卡: 未检测到（请用 ip route show default 手工确认）");
        }
        Ok(())
    }
}

// ---- 辅助 ----

/// 探测「只有 IPv6 能出网」的环境。
///
/// 真机 192.168.10.1实测：`curl -4 http://www.baidu.com` 8 秒超时，
/// 而 `curl -6` 0.08 秒返回 200。这种环境里 mihomo 的 `ipv6: false`
/// 会拒答 AAAA 记录，内核只拿到 A 记录然后连不通，
/// 表现为「DNS 解析超时 / 连接超时」，很容易被误判成配置错误。
///
/// 判据：IPv4 **不通** 且 IPv6 **通**。任一探测失败都保守返回 false
/// （维持默认的 ipv6: false，不擅自改变用户网络的解析行为）。
fn detect_ipv6_only() -> bool {
    let probe = |flag: &str| -> bool {
        std::process::Command::new("curl")
            .args([flag, "-s", "-o", "/dev/null", "--max-time", "5",
                   "http://www.baidu.com"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    };
    let v4 = probe("-4");
    let v6 = probe("-6");
    v6 && !v4
}

fn resolve_core(cli: &Cli, paths: &Paths) -> Result<PathBuf> {
    match &cli.core {
        Some(c) => Ok(c.clone()),
        None => {
            if paths.core.exists() {
                Ok(paths.core.clone())
            } else {
                find_core(None)
            }
        }
    }
}

/// 从已生成的配置里读 secret（mihomo 实际在用的那个）。
fn read_secret(paths: &Paths) -> Option<String> {
    read_gen_str(paths, "secret")
}

/// 从已生成的配置里读 external-controller。
fn read_external_controller(paths: &Paths) -> Option<String> {
    read_gen_str(paths, "external-controller")
        .map(|s| if s.starts_with("http") { s } else { format!("http://{s}") })
}

fn read_gen_str(paths: &Paths, key: &str) -> Option<String> {
    let text = std::fs::read_to_string(&paths.generated).ok()?;
    let v: serde_yaml::Value = serde_yaml::from_str(&text).ok()?;
    v.get(key)
        .and_then(|s| s.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
}

/// 取默认路由的出口网卡。
///
/// 旁路由与本机代理都必填：mihomo 靠 `interface-name` 确定「从哪张网卡
/// 出去」。不设它在某些网络环境下会表现为「规则命中但连接超时」
/// （真机实测：本机IPv4 默认路由不明确，`curl -4` 直连超时、`curl -6` 正常）。
///
/// 解析要小心：`ip route` 输出是
///     default via 192.168.10.254 dev eth0 proto static
/// `dev` 后面**才是**网卡名 —— 用 `find(|t| *t == "dev")` 会拿到
/// "dev" 这个单词本身（第一版就踩了这个坑，见docs/10）。
fn default_route_iface() -> Option<String> {
    let parse = |text: &str| -> Option<String> {
        let first = text.lines().next()?;
        // 找 "dev" 这个 token，取它**后面一个** token
        let toks: Vec<&str> = first.split_whitespace().collect();
        toks.iter()
            .position(|t| *t == "dev")
            .and_then(|i| toks.get(i + 1))
            .map(|s| s.to_string())
    };
    // 两组参数长度不同（IPv6 多了 -6），所以用切片而不是数组
    let arg_sets: [&[&str]; 2] = [
        &["route", "show", "default"],
        &["-6", "route", "show", "default"],
    ];
    for args in arg_sets {
        let out = std::process::Command::new("ip").args(args).output().ok()?;
        let text = String::from_utf8_lossy(&out.stdout);
        if let Some(n) = parse(&text) {
            return Some(n);
        }
    }
    None
}
// ---- 旁路由实现 ---------------------------------------------------
// 单独一个 impl 块：逻辑上跟其它子命令无关，且后面若要加 Web UI
// 的 API handler，代码位置是现成的。
impl Cli {
    fn parse_clients(s: &str) -> Vec<String> {
        s.split(',')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect()
    }

    fn build_gw(
        lan_addr: &str,
        iface: &str,
        clients: &str,
        tproxy_port: u16,
        dns_port: u16,
    ) -> crate::nft::Gateway {
        let mut g = crate::nft::Gateway {
            lan_addr: lan_addr.to_string(),
            iface: iface.to_string(),
            tproxy_port,
            dns_port,
            lan_clients: Self::parse_clients(clients),
            ..Default::default()
        };
        // tproxy 端口若为 0，尝试从用户配置里读，避免两边不一致
        if g.tproxy_port == 0 {
            g.tproxy_port = 7894;
        }
        g
    }

    fn gateway(&self, paths: &Paths, c: &GwsCmd) -> Result<()> {
        use crate::nft;
        match c {
            GwsCmd::Enable {
                lan_addr,
                iface,
                clients,
                tproxy_port,
                dns_port,
            } => {
                // 缺省时自动探测：网卡走默认路由检测，本机地址取
                // 该网卡上的第一个全局 IPv4。这样 `gateway enable`
                // 不必每次都填三个参数。
                let iface = iface
                    .clone()
                    .or_else(default_route_iface)
                    .ok_or_else(|| anyhow::anyhow!("探测不到出口网卡，请用 --iface 指定"))?;
                let lan_addr = match lan_addr {
                    Some(a) => a.clone(),
                    None => Self::detect_lan_addr(&iface).ok_or_else(|| {
                        anyhow::anyhow!(
                            "探测不到 {iface} 上的本机 IPv4 地址，请用 --lan-addr 指定"
                        )
                    })?,
                };
                let g = Self::build_gw(&lan_addr, &iface, clients, *tproxy_port, *dns_port);
                g.precheck()?;
                nft::enable(&g)?;
                println!("旁路由已启用");
                println!("  nft 表: inet clashx");
                println!("  出口网卡: {}", g.iface);
                println!("  客户端网段: {}", g.lan_clients.join(", "));
                println!("  tproxy 端口: {}", g.tproxy_port);
                if g.dns_port > 0 {
                    println!("  DNS 劫持: 53 -> {}", g.dns_port);
                }
                println!();
                println!("客户端设置网关为 {} 后，其流量会经本机代理。", g.lan_addr);
            }
            GwsCmd::Disable => {
                nft::disable()?;
                println!("旁路由已停用（配置与内核服务保持运行）");
            }
            GwsCmd::Status => {
                print!("{}", nft::status());
            }
            GwsCmd::DryRun {
                lan_addr,
                iface,
                clients,
                tproxy_port,
                dns_port,
            } => {
                let iface = iface.clone().or_else(default_route_iface).unwrap_or_default();
                let lan_addr = match lan_addr {
                    Some(a) => a.clone(),
                    None => Self::detect_lan_addr(&iface).unwrap_or_default(),
                };
                let g = Self::build_gw(&lan_addr, &iface, clients, *tproxy_port, *dns_port);
                print!("{}", nft::render_ruleset(&g));
                eprintln!("--- policy routing ---");
                eprint!("{}", nft::render_policy_route());
                eprintln!("--- sysctl ---");
                eprint!("{}", nft::render_sysctl());
            }
        }
        Ok(())
    }
}

// ---- 运行模式（OpenClash 式二维模型）--------------------------------
//
// 与 `mode` 命令的分工：
// - `mode` 改的是 mihomo 的 rule/global/direct（流量走哪条路）
// - `run-mode` 改的是 fake-ip/redir-host × 防火墙接管方式（流量怎么进来）
//
// 切换 run-mode 会重启内核 —— tun 开关是启动期参数，
// 内核的 RESTful API 改不了（只有 dns 模式能热改），所以走
// 「改配置 -> apply -> 重启」路径，与 OpenClash 的做法一致。
impl Cli {
    fn run_mode_show(&self, paths: &Paths) -> Result<()> {
        let cfg = load_config(paths)?;
        let rm = cfg.run_mode;
        let r = rm.resolve();
        println!("运行模式: {}", rm.as_str());
        println!("  DNS 模式      : {}", rm.dns_mode_name());
        println!("  TUN           : {}", if r.tun_enable { "启用" } else { "关闭" });
        println!("  防火墙由谁写  : {:?}", r.firewall);
        println!("  需要 mixed-port: {}", r.need_explicit_port);
        println!();
        println!("可选值:");
        for m in ALL_RUN_MODES {
            let tag = if m == rm { "  (当前)" } else { "" };
            println!("  {}{}", m.as_str(), tag);
        }
        Ok(())
    }

    fn run_mode_set(&self, paths: &Paths, value: &str) -> Result<()> {
        let target = crate::runmode::RunMode::parse(value).ok_or_else(|| {
            anyhow::anyhow!(
                "无法识别的运行模式: {value}\n可选: {}",
                ALL_RUN_MODES
                    .iter()
                    .map(|m| m.as_str())
                    .collect::<Vec<_>>()
                    .join(" / ")
            )
        })?;

        let mut cfg = load_config(paths)?;
        if cfg.run_mode == target {
            //★ 即使模式没变，也要校准 nft 状态。
            //   真机踩过：先前手动 gateway enable 装过规则，切TUN 模式时
            //   因为「已经是该模式」提前返回，nft 没卸载，
            //   clashx 与 mihomo 两套表同时 hook prerouting 导致出站不通。
            //   状态是「派生量」，必须每次都对齐，不能只在变化时更新。
            Self::sync_nft(&target);
            println!("运行模式已经是 {}，已校准防火墙状态。", target.as_str());
            return Ok(());
        }
        let old = cfg.run_mode;
        cfg.run_mode = target;

        // 副作用：redir 系需要开 mixed-port，供不会用透明代理的客户端连接。
        // 纯 fake-ip + tun 由内核全接管，不需要。
        if target.resolve().need_explicit_port && cfg.inbound.mixed_port == 0 {
            cfg.inbound.mixed_port = 7893;
        }

        save_config(paths, &cfg)?;
        println!("运行模式: {} -> {}", old.as_str(), target.as_str());

        // ★ 切模式前先调整 nft 规则 —— 顺序很关键。
        Self::sync_nft(&target);
        let _ = &cfg;
        //
        // 真机踩过的坑：TUN 模式下 mihomo 的 auto-redirect 会自己建
        // `table inet mihomo`（hook prerouting，priority dstnat+1），
        // 而 clashx 的 `table inet clashx`（priority mangle=-150）也在
        // 抓流量。两套表同时生效 = 流量被 tproxy 与 redirect 双重接管，
        // 表现为「mihomo 选中了节点、连接也建立了，但出站就是不通」。
        //
        // 所以：mihomo 接管防火墙时，clashx 必须先撤掉自己的规则。
        let fw = target.resolve().firewall;
        if fw == crate::runmode::FirewallOwner::Mihomo {
            if crate::nft::is_active() {
                crate::nft::disable()?;
                println!("已卸载 clashx 的 nft 规则（改由 mihomo 接管）");
            }
        } else if !crate::nft::is_active() {
            // 反向：从 TUN 切回传统模式时，把 clashx 的规则装回去
            println!("提示：当前是 {} 模式，clashx 的 nft 规则未启用。", target.as_str());
            println!("      旁路由需要的话执行：clashx gateway enable --lan-addr <本机IP> \\");
            println!("        --iface <网卡> --clients <客户端网段>");
        }

        // 落盘后立即 apply，让 mihomo -t 先校验一遍
        self.apply(paths, false)?;

        println!("配置已校验，重启内核使 tun / dns 模式生效...");
        crate::core::Systemd::new("mihomo-client").restart()?;

        // 重启后再确认一次状态，给出明确反馈
        let svc = if crate::core::Systemd::new("mihomo-client").is_active() {
            "运行中"
        } else {
            "启动失败（跑 clashx service log 看详情）"
        };
        println!("完成：内核 {svc}，nft 规则 {}。", if crate::nft::is_active() { "已启用" } else { "未启用" });
        println!("验证：clashx run-mode show && clashx gateway status");
        Ok(())
    }
}

impl Cli {
    /// 按运行模式校准防火墙规则状态。
    ///
    /// nft 规则是运行模式的**派生量**：模式决定谁写防火墙，
    /// 规则必须与之一致。nft 规则是「装上就一直在」的东西，
    /// 而模式可能因手动 gateway 命令而与规则脱节，
    /// 所以每次切模式（含「已经是该模式」的情况）都要对齐。
    fn sync_nft(target: &crate::runmode::RunMode) {
        let fw = target.resolve().firewall;
        // 只有 **Mihomo 独占** 时才卸载 clashx 的规则。
        // Both（mix 模式）不能卸 —— 它的语义就是「clashx 的 tproxy 兜底
        // + tun 补TCP/UDP」，两条腿都要留着。上一版按
        // `fw != Clashx` 判断，把mix 也误卸了，回归时发现。
        if fw == crate::runmode::FirewallOwner::Mihomo {
            if crate::nft::is_active() {
                match crate::nft::disable() {
                    Ok(()) => println!("已卸载 clashx 的 nft 规则（改由 mihomo 接管）"),
                    // 卸载失败不算致命：mihomo 的 TUN 不依赖它，
                    // 最坏情况是流量被双重接管，用户自己能看出来。
                    Err(e) => println!("警告：卸载 clashx nft 规则失败（{e:#}）"),
                }
            }
        } else if !crate::nft::is_active() {
            println!("提示：{} 模式下clashx 的 nft 规则未启用。", target.as_str());
            println!("      旁路由需要时执行：clashx gateway enable --lan-addr <本机IP> \\");
            println!("        --iface <网卡> --clients <客户端网段>");
        }
    }
}

impl Cli {
    /// 取指定网卡上的第一个全局 IPv4 地址。
    ///
    /// 为什么不用 `ip route get` 那套：它给的是「对外可达的源地址」，
    /// 在多网卡 / policy routing 环境下可能不是我们想暴露给局域网的那个。
    /// 直接读网卡地址更直观，也和用户配置 `interface-name` 的意图一致。
    fn detect_lan_addr(iface: &str) -> Option<String> {
        let out = std::process::Command::new("ip")
            .args(["-4", "-br", "addr", "show", "dev", iface])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&out.stdout);
        for line in text.lines() {
            // 输出形如：eth0  UP  192.168.10.1/24
            let mut parts = line.split_whitespace();
            let _name = parts.next()?;
            let _state = parts.next()?;
            if let Some(cidr) = parts.next() {
                let ip = cidr.split('/').next()?;
                // 跳过 127.x 与链路本地
                if !ip.starts_with("127.") && !ip.starts_with("169.254.") {
                    return Some(ip.to_string());
                }
            }
        }
        None
    }
}

impl Cli {
    /// 启动 Web 管理界面。
    ///
    /// 阻塞运行 —— 与 mihomo 服务分开：mihomo 由 systemd 管，
    /// Web UI 手动起或另写 unit。理由是 Web UI 不是必需组件，
    /// 不该因为它起不来而影响代理功能。
    fn web(&self, paths: &Paths, listen: Option<&str>, localhost: bool) -> Result<()> {
        let cfg = load_config(paths)?;
        let wcfg = cfg
            .web
            .clone()
            .unwrap_or_else(|| crate::config::Web::default());

        if !wcfg.enable {
            bail!("Web UI 已在配置里禁用（web.enable: false）");
        }

        // 密码缺失时明确告知，不要让人对着 401 猜
        match wcfg.password_sha256.as_deref() {
            None => {
                eprintln!("警告：未设置 web.password，所有请求都会被拒绝（fail closed）。");
                eprintln!("      设置方式：echo -n '你的密码' | clashx web-pass --stdin");
            }
            Some(h) => {
                // 默认密码未改的风险提示
                let default_hash = crate::web::sha256_hex(b"admin");
                if h.eq_ignore_ascii_case(&default_hash) {
                    eprintln!("提示：当前仍在用默认密码 admin，建议在界面「密码」页修改。");
                }
            }
        }

        let addr = match (listen, localhost) {
            (Some(a), _) => a.to_string(),
            (None, true) => wcfg
                .listen
                .split_once(':')
                .map(|(h, p)| format!("127.0.0.1:{p}"))
                .unwrap_or_else(|| "127.0.0.1:9080".into()),
            (None, false) => wcfg.listen.clone(),
        };

        crate::web::serve(paths.clone(), &addr)
    }

    /// 设置 Web 登录密码。
    ///
    /// ★ 密码走 stdin 而不是命令行参数 —— 命令行会进 shell 历史、
    /// 会出现在 `ps` 的 argv 里。`echo -n 'pw' | clashx web-pass --stdin`
    /// 仍然是明文过管道，但不会留痕。
    fn web_pass(&self, paths: &Paths, from_stdin: bool) -> Result<()> {
        if !from_stdin {
            bail!(
                "为避免密码进 shell 历史与 ps 输出，请用：\n  echo -n '你的密码' | clashx web-pass --stdin"
            );
        }
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        let pw = buf.trim();
        if pw.len() < 4 {
            bail!("密码太短（至少 4 个字符）");
        }
        if pw.chars().count() > 128 {
            bail!("密码过长（>128 字符）");
        }

        let mut cfg = load_config(paths)?;
        let mut w = cfg.web.clone().unwrap_or_else(|| crate::config::Web::default());
        w.password_sha256 = Some(crate::web::sha256_hex(pw.as_bytes()));
        cfg.web = Some(w);
        save_config(paths, &cfg)?;
        println!("密码已更新（配置文件里只存 SHA-256 哈希，无明文）");
        Ok(())
    }
}

impl Cli {
    /// 安装 Web UI 的 systemd unit。
    fn web_install(&self, paths: &Paths, listen: Option<&str>, start: bool) -> Result<()> {
        let cfg = load_config(paths)?;
        let w = cfg.web.clone().unwrap_or_else(|| crate::config::Web::default());
        let addr = listen.unwrap_or(&w.listen).to_string();

        crate::core::install_web_unit(
            &paths.root.to_string_lossy(),
            &addr,
        )?;
        println!("已安装 clashx-web.service（监听 {addr}）");

        let svc = crate::core::Systemd::new("clashx-web");
        svc.enable()?;
        println!("已设置开机自启");

        if start {
            svc.restart()?;
            std::thread::sleep(std::time::Duration::from_millis(800));
            let state = if svc.is_active() { "运行中" } else { "启动失败" };
            println!("服务状态: {state}");
            if state == "运行中" {
                println!();
                println!("Web UI: http://{addr}/");
                println!("  默认账号：admin / admin（登录后请立即修改）");
                if w.password_sha256.is_none() {
                    println!();
                    println!("⚠未设置密码，所有请求都会被拒绝（fail closed）。");
                    println!("  设置方式：echo -n '你的密码' | clashx web-pass --stdin");
                } else {
                    let default_hash = crate::web::sha256_hex(b"admin");
                    if w.password_sha256
                        .as_deref()
                        .map(|h| h.eq_ignore_ascii_case(&default_hash))
                        .unwrap_or(false)
                    {
                        println!("⚠ 仍在用默认密码 admin，建议在界面「密码」页修改。");
                    }
                }
            } else {
                println!("查看日志：journalctl -u clashx-web -n 30 --no-pager");
            }
        }
        Ok(())
    }
}
