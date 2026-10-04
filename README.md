# mihomo-client

Linux 无桌面mihomo 客户端（本机代理 + 旁路由）。

## 快速开始

    # 1. 装mihomo 内核到 bin/mihomo
    #    开发期可直接借用 OpenClash 带来的：
    ln -sf /etc/openclash/core/clash_meta /opt/mihomo-client/bin/mihomo

    # 2. 生成配置（自动探测出口网卡与IPv6 可用性）
    mihomo-client init

    # 3. 校验 -> 应用 -> 装服务 -> 启动
    mihomo-client validate
    mihomo-client apply
    mihomo-client install --run-user nobody --group nogroup
    mihomo-client service start

    # 4. 用
    export http_proxy=http://127.0.0.1:7893
    mihomo-client status

## 命令

| 命令 | 说明 |
|------|------|
| `init [--force] [--bypass-router] [--interface IF]` | 生成默认配置 |
| `validate` | 渲染后交给 `mihomo -t` 校验，不改动任何东西 |
| `apply [--force]` | 原子切换：写 .new -> 校验 -> 改名覆盖 |
| `service start\|stop\|restart\|log` | systemd 控制 |
| `status [--verbose]` | 含内核版本、GeoData 就绪状态 |
| `install --run-user U [--group G] [--no-net-admin]` | 装 systemd unit 并 enable |
| `sub add NAME URL [--interval H] [--user-agent UA]` | 添加/更新订阅 |
| `sub list` | 列出订阅（URL 只显示前 24 字符） |
| `sub test` | 抓取每个订阅并报告格式/节点数/流量 |
| `sub rm NAME` | 删除订阅 |
| `nodes [--brief]` | 列出所有节点与协议分布 |
| `mode rule\|global\|direct` | 走内核 RESTful API 热切换（不重启） |
| `rule list [-n]` | 列出规则链 |
| `rule add <RULE>` | 追加规则（自动插到 MATCH 之前） |
| `rule rm <N>` | 按行号删除 |
| `rule test <DOMAIN> [--ip IP]` | **离线判断**会命中哪条规则 |
| `rule provider list\|add\|rm\|check` | 规则集管理 |
| `rule preset --list` / `rule preset <名字>` | 规则预设 |
| `show` | 打印渲染后的 mihomo 配置 |
| `ifaces` | 列出网卡与默认路由出口 |

## 设计原则

**能委托内核的绝不自己实现**：

- 配置校验 -> `mihomo -t`（唯一权威校验）
- 模式/节点切换 -> RESTful API
- 进程管理 -> systemd
- 协议解析 -> mihomo 的显式入站

**不用 sed/awk 改 yaml**：全程 serde_yaml 操作数据结构。

## 目录布局

    /opt/mihomo-client/
    ├── bin/mihomo-client        本程序
    ├── bin/mihomo               内核二进制
    ├── etc/config.yaml          用户配置（改这个）
    ├── etc/subscriptions.yaml   订阅列表
    ├── var/config.gen.yaml      渲染产物（自动生成，勿手改）
    └── var/mihomo/              内核工作目录（geodata、ruleset）

## 订阅

```bash
# 添加（URL 含token，别贴进公开仓库）
mihomo-client sub add mysub 'https://example.com/api/v1/sub?token=xxx'

# 只保留香港/日本，排除「到期」
mihomo-client sub add mysub 'https://...' --keyword 香港 --keyword 日本 --exclude 到期

# 验证格式与节点数（不写配置）
mihomo-client sub test
```

**机场按 User-Agent 分流返回不同格式**（实测）：

| UA | 返回 |
|----|------|
| `clash.meta/1.19.20` | 完整 clash YAML |
| `ClashforWindows/*` | clash YAML |
| `v2rayNG` / 无 UA | base64 的 vless:// 链接列表 |

所以 UA 必须是 `clash.meta`（默认值），否则只拿到链接列表。
客户端会解析响应头里的流量与到期信息并在命令输出中显示。

**多订阅**：同名节点（含大小写差异）会自动加来源前缀
`原名@订阅名`，再冲突加 `#N`，保证渲染结果稳定不抖动。

## 规则与调试

```bash
# 最常用：为什么这个网站走了那个节点？
mihomo-client rule test www.google.com
#   命中: 第 9 条  MATCH  ->  PROXY
#   规则原文: MATCH,PROXY

# 带 IP（IP-CIDR / GeoIP 类规则需要）
mihomo-client rule test api.github.com --ip 140.82.121.4

# 规则集预设：国内直连 + 广告拦截 + 私有网段
mihomo-client rule preset private
mihomo-client rule preset cn
mihomo-client rule preset adblock
```

`rule test` 是**离线**判断，不用发请求、不用装面板 ——
内核的 `/rules` 只能告诉你「有哪些规则」，回答不了「这个域名命中了哪条」。

支持 DOMAIN / DOMAIN-SUFFIX / DOMAIN-KEYWORD / DOMAIN-WILDCARD /
IP-CIDR / IP-SUFFIX / GEOSITE（启发式）/ RULE-SET（读本地缓存）。
判不了的类型（需要 GeoIP 库或连接信息的）会**明确跳过并说明**，不猜。

## 排障

| 症状 | 原因 |
|------|------|
| `216/GROUP` 服务起不来 | `--group` 没给对。Debian 的 nobody 属 `nogroup` |
| 规则命中但连接超时 | 没设 `interface-name`。`mihomo-client ifaces` 看出口网卡 |
| `dns resolve failed: context deadline exceeded` | nameserver 里的 DoH 不通。改用明文 IP DNS |
| 所有请求 502 | GeoData 未就绪（首启要下载）。`status` 会显示 |
| 端口被占 | 用 `--core` 指定别的内核，或改配置里的端口 |
| 订阅「格式不可用 Base64Links」 | 该订阅的 UA 没生效，用 `--user-agent clash.meta` 重加 |
| 订阅返回空内容 | URL 失效或被限流。`sub test` 看具体错误 |
| 规则不生效 | 先 `rule test <域名>` 看命中哪条，再 `apply` 确认已生效 |
| 规则全落到 MATCH | 检查是否有前置规则把请求 `REJECT` 了，或规则集没下载（`rule provider list` 看「已缓存」列） |

## 开发

    # 真机构建（Rust 装在 /opt/cargo）
    python3 scripts/_deploy/build-rust.py

依赖刻意保持最小：serde / serde_yaml / clap / anyhow / dirs，
**无 tokio**（CLI 与 systemd 场景下同步阻塞不是瓶颈）。

设计文档：`docs/10-新项目设计/01-上游调研与架构设计.md`
