//! 运行模式：对齐 OpenClash 的二维设计。
//!
//! # 为什么抄OpenClash 的做法
//!
//! OpenClash 的 UI 上是一个 6 值枚举（`en_mode`），但脚本里**立刻拆成两个
//! 独立变量**（`root/etc/init.d/openclash:485-510`）：
//! ```sh
//! if [ "$en_mode" = "fake-ip-tun" ]; then
//!    en_mode_tun="1"; en_mode="fake-ip"
//! fi
//! if [ "$en_mode" = "redir-host-mix" ]; then
//!    en_mode_tun="2"; en_mode="redir-host"
//! fi
//! ```
//! 抄这个设计的三个理由：
//!
//! 1. **正交性**：DNS 怎么解析（fake-ip / redir-host）与流量怎么接管
//!    （传统 / tun / tun+传统）是**两个独立维度**。合成一个枚举后，
//!    6 个值里有 2 个是「组合出来」的，但用两个字段表达更自然，
//!    且能表达 UI 里没有的合法组合。
//! 2. **单一事实来源**：渲染器、CLI、预检逻辑都调`RunMode::resolve()`，
//!    不用各自写 if 判断。加新模式只改一处。
//! 3. **可读**：配置里写`run-mode: fake-ip-tun` 直观，但内部立刻变成
//!    `dns_mode = fake-ip, firewall = Tun`，渲染器不用再猜。
//!
//! # 六种模式
//!
//! | run-mode | dns.enhanced-mode | tun.enable | 防火墙谁写 | 适用场景 |
//! |---|---|---|---|---|
//! | `fake-ip` | fake-ip | 否 | clashx | 默认；规则按域名匹配最准 |
//! | `redir-host` | redir-host | 否 | clashx | 兼容性优先；部分 App 认不得 fake-ip |
//! | `fake-ip-tun` | fake-ip | 是 | mihomo | 旁路由首选；内核接管，不用手写 tproxy |
//! | `redir-host-tun` | redir-host | 是 | mihomo | 同上，但要兼容的站点多|
//! | `fake-ip-mix` | fake-ip | 是 | 两者 | tun 打底 + 传统兜底（OpenWrt 老配置迁移） |
//! | `redir-host-mix` | redir-host | 是 | 两者 | 同上 |
//!
//! # 从 OpenClash 抄来的两个关键细节
//!
//! - **redir-host 必须开 `sniffer.force-dns-mapping`**
//!   （OpenClash `yml_change.sh:509`）：没有 fake-ip 映射表，域名只能靠
//!   嗅探还原。忘了这行，redir-host 模式下所有域名规则都失效。
//! - **TUN 模式要关掉 `auto-route` / `auto-redirect`**
//!   （`yml_change.sh:526-528`）：让 clashx 自己管路由，mihomo 只负责接管。
//!   同时开 `endpoint-independent-nat` 改善 UDP 兼容性。
//!
//! # 还有一个「看起来无关但必须做」的事
//!
//! OpenClash 在渲染时主动删除内核自动生成的字段（`yml_change.sh:531-541`）：
//! ```ruby
//! Value.delete('iptables'); Value.delete('ebpf'); Value.delete('auto-redir')
//! ```
//! 原因是 mihomo 的 `tun.auto-redirect` 会自己写 nftables 规则，如果
//! clashx 也写，两边抢chain 会导致内核启动失败。
//! —— 这是 openclash-rt 那轮实测踩过的坑（`inet fw4` 的 `nat_output`
//! 冲突），本项目只要开TUN 就一定会遇到，所以提前规避。

use serde::{Deserialize, Serialize};

/// 防火墙由谁负责写。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum FirewallOwner {
    /// clashx 自己写 nftables（redir / tproxy 入站）。
    #[default]
    Clashx,
    /// mihomo 自己写（`tun.auto-redirect` / `auto-route`）。
    Mihomo,
    /// 两边都写：tproxy 处理 TCP/UDP，tun 兜底。
    /// 对应 OpenClash 的 `-mix` 后缀。
    Both,
}

/// 运行模式（对外一个枚举，内部拆成两个维度）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum RunMode {
    /// fake-ip + clashx 写防火墙。默认。
    #[default]
    #[serde(rename = "fake-ip")]
    FakeIp,
    /// redir-host + clashx 写防火墙。
    #[serde(rename = "redir-host")]
    RedirHost,
    /// fake-ip + mihomo 写防火墙（TUN）。
    #[serde(rename = "fake-ip-tun")]
    FakeIpTun,
    /// redir-host + mihomo 写防火墙（TUN）。
    #[serde(rename = "redir-host-tun")]
    RedirHostTun,
    /// fake-ip + 两者都写。
    #[serde(rename = "fake-ip-mix")]
    FakeIpMix,
    /// redir-host + 两者都写。
    #[serde(rename = "redir-host-mix")]
    RedirHostMix,
}

/// 拆解后的运行参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolved {
    /// `dns.enhanced-mode` 的值。
    pub fake_ip: bool,
    /// `tun.enable` 的值。
    pub tun_enable: bool,
    /// 防火墙由谁写。
    pub firewall: FirewallOwner,
    /// 客户端传统代理端口（0 = 不开）。fake-ip/tun 不需要；
    /// redir 系需要，因为要兼容不会用透明代理的客户端。
    pub need_explicit_port: bool,
}

impl RunMode {
    /// 拆解成两个正交维度。
    ///
    /// 这一处是**唯一**的事实来源 —— 渲染器、预检、CLI 全部调它。
    pub fn resolve(self) -> Resolved {
        match self {
            Self::FakeIp => Resolved {
                fake_ip: true,
                tun_enable: false,
                firewall: FirewallOwner::Clashx,
                need_explicit_port: false,
            },
            Self::RedirHost => Resolved {
                fake_ip: false,
                tun_enable: false,
                firewall: FirewallOwner::Clashx,
                need_explicit_port: true,
            },
            Self::FakeIpTun => Resolved {
                fake_ip: true,
                tun_enable: true,
                firewall: FirewallOwner::Mihomo,
                need_explicit_port: false,
            },
            Self::RedirHostTun => Resolved {
                fake_ip: false,
                tun_enable: true,
                firewall: FirewallOwner::Mihomo,
                need_explicit_port: true,
            },
            Self::FakeIpMix => Resolved {
                fake_ip: true,
                tun_enable: true,
                firewall: FirewallOwner::Both,
                need_explicit_port: true,
            },
            Self::RedirHostMix => Resolved {
                fake_ip: false,
                tun_enable: true,
                firewall: FirewallOwner::Both,
                need_explicit_port: true,
            },
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::FakeIp => "fake-ip",
            Self::RedirHost => "redir-host",
            Self::FakeIpTun => "fake-ip-tun",
            Self::RedirHostTun => "redir-host-tun",
            Self::FakeIpMix => "fake-ip-mix",
            Self::RedirHostMix => "redir-host-mix",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.trim() {
            "fake-ip" | "fakeip" => Self::FakeIp,
            "redir-host" | "redirhost" => Self::RedirHost,
            "fake-ip-tun" | "fakeip-tun" => Self::FakeIpTun,
            "redir-host-tun" | "redirhost-tun" => Self::RedirHostTun,
            "fake-ip-mix" | "fakeip-mix" => Self::FakeIpMix,
            "redir-host-mix" | "redirhost-mix" => Self::RedirHostMix,
            _ => return None,
        })
    }

    /// 该模式下DNS 模式名（给 CLI 展示用）。
    pub fn dns_mode_name(self) -> &'static str {
        if self.resolve().fake_ip {
            "fake-ip"
        } else {
            "redir-host"
        }
    }
}

impl std::fmt::Display for RunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 六个值都能解析回来() {
        for m in [
            RunMode::FakeIp,
            RunMode::RedirHost,
            RunMode::FakeIpTun,
            RunMode::RedirHostTun,
            RunMode::FakeIpMix,
            RunMode::RedirHostMix,
        ] {
            assert_eq!(RunMode::parse(m.as_str()), Some(m));
        }
    }

    #[test]
    fn 拆解表与openclash对齐() {
        // 这张表就是 docs/10 调研里 OpenClash yml_change.sh 的行为总结，
        // 一旦改动必须同步更新那边的文档。
        let cases = [
            (RunMode::FakeIp, true, false, FirewallOwner::Clashx),
            (RunMode::RedirHost, false, false, FirewallOwner::Clashx),
            (RunMode::FakeIpTun, true, true, FirewallOwner::Mihomo),
            (RunMode::RedirHostTun, false, true, FirewallOwner::Mihomo),
            (RunMode::FakeIpMix, true, true, FirewallOwner::Both),
            (RunMode::RedirHostMix, false, true, FirewallOwner::Both),
        ];
        for (m, fake_ip, tun, fw) in cases {
            let r = m.resolve();
            assert_eq!(r.fake_ip, fake_ip, "{m} fake_ip 不对");
            assert_eq!(r.tun_enable, tun, "{m} tun_enable 不对");
            assert_eq!(r.firewall, fw, "{m} firewall 不对");
        }
    }

    #[test]
    fn tun模式不需要传统代理端口() {
        // 纯 TUN 由内核接管，不该再开 mixed-port 让客户端手动配代理
        assert!(!RunMode::FakeIpTun.resolve().need_explicit_port);
        // 但 redir 系为了兼容还是要开
        assert!(RunMode::RedirHostTun.resolve().need_explicit_port);
        assert!(RunMode::RedirHost.resolve().need_explicit_port);
    }

    #[test]
    fn 序列化用连字符不被yaml1_1破坏() {
        // 裸 `fake-ip` 在 YAML 里是字符串，安全；
        // 但 `off`/`no` 会被读成 bool —— 所以枚举里绝不能有这类词。
        let s = serde_yaml::to_string(&RunMode::FakeIpTun).unwrap();
        assert!(s.contains("fake-ip-tun"));
    }
}
