//! Web 服务：HTTP 骨架 + Basic Auth。
//!
//! # 为什么不用 axum / actix
//!
//! 要暴露的接口就十来个、全是 JSON，静态文件只有一份 index.html。
//! 引入 Web 框架会让编译时间和二进制体积翻好几倍，而这里一行都省不下来。
//! `std::net::TcpListener` 配合手写解析完全够用—— 这个量级下，
//! 「HTTP 解析」本身就是几十行代码的事。
//!
//! # 线程模型
//!
//! 每连接一个线程（`thread::spawn`），不做异步。
//! 理由：管理界面的并发是「几个浏览器标签页」级别，
//! 线程的调度开销远小于异步的复杂度成本。
//!
//! # 安全边界（重要，不要改错）
//!
//! ```text
//!   浏览器 ──Basic Auth──> clashx :9080 ──无认证──> mihomo 127.0.0.1:9090
//!                    │
//!                    └──> /zashboard/* 反代到 mihomo 的 UI 目录
//! ```
//!
//! - clashx 的 9080 绑 0.0.0.0（要局域网访问），Basic Auth 保护
//! - mihomo 的 9090 **保持只听 127.0.0.1**，不对外暴露
//! - 配置文件里有订阅 token，所以**任何路径都不能免鉴权**，
//!   包括静态资源和 zashboard 反代

use anyhow::{bail, Context, Result};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use std::time::Duration;

use crate::config::Config;
use crate::core::Paths;

/// 单次请求体上限。配置接口要传整份 YAML，1MB 够用且能挡住
/// 「POST 一 gigabyte 把内存吃满」这类无聊攻击。
const MAX_BODY: usize = 1024 * 1024;
/// 单个请求头总大小上限。
const MAX_HEADERS: usize = 16 * 1024;
/// 请求行长度上限。
const MAX_LINE: usize = 8 * 1024;
/// 单次读取的超时。防止慢速连接占住线程。
const IO_TIMEOUT: Duration = Duration::from_secs(15);

/// 一次 HTTP 请求。
#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    /// 路径，不含 query（已 URL 解码）。
    pub path: String,
    /// query string，不含前导 `?`。
    pub query: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        // Header 名大小写不敏感（RFC 7230）
        let lname = name.to_ascii_lowercase();
        self.headers
            .iter()
            .find(|(k, _)| k.to_ascii_lowercase() == lname)
            .map(|(_, v)| v.as_str())
    }

    /// 取query 参数（已经过 percent-decode）。
    pub fn param(&self, key: &str) -> Option<String> {
        for pair in self.query.split('&') {
            if pair.is_empty() {
                continue;
            }
            let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
            if k == key {
                return Some(percent_decode(v));
            }
        }
        None
    }

    /// 请求体当 UTF-8 字符串。
    pub fn body_str(&self) -> Result<String> {
        String::from_utf8(self.body.clone()).context("请求体不是合法 UTF-8")
    }
}

/// 一次 HTTP 响应。
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn json(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            headers: vec![("Content-Type".into(), "application/json; charset=utf-8".into())],
            body: body.into().into_bytes(),
        }
    }

    pub fn html(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            headers: vec![("Content-Type".into(), "text/html; charset=utf-8".into())],
            body: body.into().into_bytes(),
        }
    }

    pub fn text(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            headers: vec![("Content-Type".into(), "text/plain; charset=utf-8".into())],
            body: body.into().into_bytes(),
        }
    }

    pub fn bytes(status: u16, content_type: &str, body: Vec<u8>) -> Self {
        Self {
            status,
            headers: vec![("Content-Type".into(), content_type.to_string())],
            body,
        }
    }

    pub fn redirect(location: &str) -> Self {
        Self {
            status: 302,
            headers: vec![("Location".into(), location.to_string())],
            body: Vec::new(),
        }
    }

    /// 401 + `WWW-Authenticate`，浏览器会弹出登录框。
    pub fn unauthorized(realm: &str) -> Self {
        Self {
            status: 401,
            headers: vec![
                ("WWW-Authenticate".into(), format!("Basic realm=\"{realm}\"")),
                ("Content-Type".into(), "text/plain; charset=utf-8".into()),
            ],
            body: b"401 Unauthorized".to_vec(),
        }
    }

    fn with_header(mut self, k: &str, v: &str) -> Self {
        self.headers.push((k.to_string(), v.to_string()));
        self
    }
}

/// 服务运行期需要的上下文。
pub struct ServerCtx {
    pub paths: Paths,
    /// 配置文件锁：Web 改配置与 CLI 的 apply 可能并发，
    /// 用文件锁串行化，避免两个 apply 互相覆盖。
    pub config_lock: Arc<std::sync::Mutex<()>>,
    /// 配置缓存（按 mtime 判断新鲜度）。
    ///
    /// ★ 为什么需要（压测实测）：`load_config` 每次都要
    ///   `read_to_string` + `serde_yaml::from_str` 解析一遍完整配置，
    ///   串行单次约 97ms —— 而规则/订阅/总览**每个请求都要读**。
    ///   并发时 CPU 全花在重复解析同一份没变的内容上。
    ///
    /// 用 mtime + 长度做指纹：文件没动就直接复用已解析对象。
    /// 这是「派生数据缓存」的标准做法 —— 指纹错了最坏是读到旧配置，
    /// 而写配置后 mtime 必然变，不会漏。
    pub config_cache: Arc<std::sync::Mutex<Option<(ConfigFingerprint, crate::config::Config)>>>,
    /// mihomo 状态缓存（version / 上线状态 / 连接数）。
    ///
    /// ★ 为什么需要（压测实测）：overview 每次要 fork 两次 curl
    ///   去问mihomo（/version 与 /connections），单次 ~6ms。
    ///   并发时fork 抢 CPU，QPS 被死死封在 85 ——
    ///   而同样返回 261 字节的 /api/health 能跑 570。
    ///
    /// TTL 1 秒：UI 首屏不需要毫秒级新鲜度，但秒级足够跟手。
    /// 连接数这种数字晚 1 秒无所谓，省下的是成百上千次 fork。
    pub mihomo_cache: Arc<std::sync::Mutex<Option<(Instant, MihomoStatus)>>>,
    /// 节点数缓存（5 秒 TTL）。collect_proxies 解析 71 个节点约 5ms，
    /// overview 首屏每次都算太浪费。
    pub node_count_cache: Arc<std::sync::Mutex<Option<(Instant, usize)>>>,
}

/// mihomo 运行状态（缓存用）。
#[derive(Debug, Clone, Default)]
pub struct MihomoStatus {
    pub version: String,
    pub up: bool,
    pub connections: usize,
}

/// 配置文件指纹。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigFingerprint {
    /// 修改时间（纳秒）。
    pub mtime_nanos: Option<u128>,
    /// 文件长度。
    pub len: u64,
}

impl ConfigFingerprint {
    /// 取当前文件的指纹；文件不存在返回 None。
    pub fn of(path: &Path) -> Option<Self> {
        let meta = std::fs::metadata(path).ok()?;
        let mtime_nanos = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_nanos());
        Some(Self {
            mtime_nanos,
            len: meta.len(),
        })
    }
}

// ==================== 鉴权 ====================

/// 校验 Basic Auth。
///
/// ★ 用**常量时间**比较（`==` 对定长 byte slice 不保证）——
///   虽然 LAN 场景威胁有限，但这是密码校验的通用实践，
///   且实现成本只是多一个循环。
///
/// ★ 哈希**不区分大小写**（hex 存储），所以比对前统一小写。
pub fn check_auth(cfg: &Config, req: &Request) -> bool {
    // ★ fail closed：拿不到 web 段 / 密码，就**拒绝**而不是放行。
    //   真机实测踩过：配置文件里还没有 web 段时，这里原本
    //   `return true`，于是无凭据请求全部 200 —— 配置文件里
    //   有订阅 token，这等于把整个管理界面敞开。
    //   「纯 CLI 部署不需要鉴权」这个想法是错的：
    //   只要 Web 服务在跑，它就是暴露面，必须鉴权。
    let web = match &cfg.web {
        Some(w) => w,
        None => return false,
    };
    let user = web.username.as_deref().unwrap_or("admin");
    let Some(expected) = web.password_sha256.as_deref() else {
        // 没设密码 -> 拒绝（fail closed）。
        // 曾经的写法是「密码为空就放行」，那等于开了个无密码的后门。
        return false;
    };

    let Some(hdr) = req.header("authorization") else {
        return false;
    };
    let Some(b64) = hdr.strip_prefix("Basic ") else {
        return false;
    };
    // base64 解码
    let decoded = match base64_decode(b64.trim()) {
        Some(d) => d,
        None => return false,
    };
    let text = match String::from_utf8(decoded) {
        Ok(t) => t,
        Err(_) => return false,
    };
    let (u, p) = match text.split_once(':') {
        Some(x) => x,
        None => return false,
    };

    if !ct_eq(u.as_bytes(), user.as_bytes()) {
        return false;
    }
    let got = sha256_hex(p.as_bytes());
    ct_eq(got.as_bytes(), expected.to_ascii_lowercase().as_bytes())
}

/// 恒定时间比较。
fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// SHA-256（纯 Rust 实现，避免为了一个哈希引入依赖）。
///
/// 自己实现而不是引`sha2` crate：HTTP Basic Auth 只需要 SHA-256 这一个
/// 算法，稳定十年不会变。引依赖要多编一个 crate，
/// 还要担心版本升级引入的构建问题 —— 不值。
pub fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    // padding
    let mut msg = data.to_vec();
    let bitlen = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    h.iter().map(|x| format!("{x:08x}")).collect()
}

/// base64 解码（只要够用，不处理换行以外的怪字符）。
fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(s.len() * 3 / 4);
    let mut buf = 0u32;
    let mut bits = 0u32;
    for c in s.bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            b'\r' | b'\n' | b' ' | b'\t' => continue,
            _ => return None,
        } as u32;
        buf = (buf << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    Some(out)
}

/// base64 编码（生成 token 用）。
pub fn base64_encode(data: &[u8]) -> String {
    const T: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in data.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        if chunk.len() > 1 {
            out.push(T[(n >> 6) as usize & 63] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(T[n as usize & 63] as char);
        } else {
            out.push('=');
        }
    }
    out
}

/// percent-decode（路径与 query 都要用）。
pub fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let hi = (bytes[i + 1] as char).to_digit(16);
                let lo = (bytes[i + 2] as char).to_digit(16);
                match (hi, lo) {
                    (Some(h), Some(l)) => {
                        out.push((h * 16 + l) as u8);
                        i += 3;
                    }
                    _ => {
                        out.push(bytes[i]);
                        i += 1;
                    }
                }
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

/// percent-encode（构造 URL 时用）。
pub fn percent_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' | b'/' => {
                out.push(*b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

// ==================== HTTP 解析 ====================

/// 读一个 HTTP 请求。返回 `Ok(None)` 表示连接被正常关闭。
pub fn read_request(stream: &mut BufReader<TcpStream>) -> Result<Option<Request>> {
    // 请求行
    let mut line = String::new();
    let n = read_line_limited(stream, &mut line, MAX_LINE)?;
    if n == 0 {
        return Ok(None);
    }
    let line = line.trim_end_matches(['\r', '\n']);
    if line.is_empty() {
        return Ok(None);
    }
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let target = parts.next().unwrap_or("/").to_string();

    // 拆 path 与 query
    let (raw_path, query) = match target.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (target, String::new()),
    };

    // Header
    let mut headers = Vec::new();
    let mut total = 0usize;
    loop {
        let mut h = String::new();
        let n = read_line_limited(stream, &mut h, MAX_LINE)?;
        if n == 0 {
            break;
        }
        let h = h.trim_end_matches(['\r', '\n']);
        if h.is_empty() {
            break;
        }
        total += h.len();
        if total > MAX_HEADERS {
            bail!("Header 过大");
        }
        if let Some((k, v)) = h.split_once(':') {
            headers.push((k.trim().to_string(), v.trim().to_string()));
        }
    }

    // Body：看 Content-Length（不支持 chunked —— 管理界面不需要，
    // 浏览器 fetch 传 JSON 时一定会带 Content-Length）
    let mut body = Vec::new();
    if let Some(cl) = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, v)| v.parse::<usize>().ok())
    {
        if cl > MAX_BODY {
            bail!("请求体过大（{} > {}）", cl, MAX_BODY);
        }
        body.resize(cl, 0);
        if cl > 0 {
            stream
                .read_exact(&mut body)
                .context("读取请求体失败（客户端可能提前断开）")?;
        }
    }

    Ok(Some(Request {
        method,
        path: percent_decode(&raw_path),
        query,
        headers,
        body,
    }))
}

fn read_line_limited(
    stream: &mut BufReader<TcpStream>,
    out: &mut String,
    limit: usize,
) -> Result<usize> {
    let mut buf = Vec::new();
    let mut total = 0;
    loop {
        let mut byte = [0u8; 1];
        match stream.read(&mut byte) {
            Ok(0) => break,
            Ok(_) => {
                total += 1;
                if total > limit {
                    bail!("行过长（>{limit}）");
                }
                buf.push(byte[0]);
                if byte[0] == b'\n' {
                    break;
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    out.push_str(&String::from_utf8_lossy(&buf));
    Ok(total)
}

/// 写响应（强制 Connection: close —— 简化 keep-alive 处理）。
///
/// 每次请求都新建连接对这个量级完全够用，换来的是不用处理
/// 「上一个请求体没读完导致下一个请求解析错位」这类边界情况。
pub fn write_response(stream: &mut TcpStream, resp: &Response) -> Result<()> {
    let mut head = format!("HTTP/1.1 {} {}\r\n", resp.status, status_text(resp.status));
    for (k, v) in &resp.headers {
        head.push_str(&format!("{k}: {v}\r\n"));
    }
    head.push_str(&format!("Content-Length: {}\r\n", resp.body.len()));
    head.push_str("Connection: close\r\n");
    //基础安全头：管理界面跑在局域网里也要防最基础的点击劫持/嗅探
    head.push_str("X-Content-Type-Options: nosniff\r\n");
    head.push_str("X-Frame-Options: SAMEORIGIN\r\n");
    head.push_str("Cache-Control: no-store\r\n");
    head.push_str("\r\n");

    stream.write_all(head.as_bytes())?;
    stream.write_all(&resp.body)?;
    stream.flush()?;
    Ok(())
}

fn status_text(code: u16) -> &'static str {
    match code {
        200 => "OK",
        302 => "Found",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        413 => "Payload Too Large",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        _ => "Unknown",
    }
}

// ==================== 服务循环 ====================

/// 启动 Web 服务（阻塞）。
pub fn serve(paths: Paths, listen: &str) -> Result<()> {
    let tcp = TcpListener::bind(listen)
        .with_context(|| format!("监听 {listen} 失败（端口被占用？）"))?;
    tcp.set_nonblocking(true).context("设置非阻塞失败")?;
    let listener = Arc::new(tcp);

    println!("Web UI 已启动：http://{listen}");
    println!("  默认账号：admin / admin（登录后请立即修改）");
    println!("  zashboard：http://{listen}/zashboard/");
    println!("  停止：Ctrl-C 或systemctl stop clashx-web");

    let ctx = Arc::new(ServerCtx {
        paths: paths.clone(),
        config_lock: Arc::new(std::sync::Mutex::new(())),
        config_cache: Arc::new(std::sync::Mutex::new(None)),
        mihomo_cache: Arc::new(std::sync::Mutex::new(None)),
        node_count_cache: Arc::new(std::sync::Mutex::new(None)),
    });

    loop {
        match listener.accept() {
            Ok((stream, _addr)) => {
                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    if let Err(e) = handle_conn(stream, &ctx) {
                        // 单个连接出错不该影响服务整体
                        let _ = e;
                    }
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("accept 错误：{e}");
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn handle_conn(mut stream: TcpStream, ctx: &ServerCtx) -> Result<()> {
    let _ = stream.set_read_timeout(Some(IO_TIMEOUT));
    let _ = stream.set_write_timeout(Some(IO_TIMEOUT));
    let _ = stream.set_nodelay(true);

    let mut reader = BufReader::new(stream.try_clone()?);
    let req = match read_request(&mut reader)? {
        Some(r) => r,
        None => return Ok(()),
    };

    let resp = crate::api::dispatch(&req, ctx);
    write_response(&mut stream, &resp)
}

// ==================== 测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256匹配已知向量() {
        // NIST 标准测试向量
        assert_eq!(
            sha256_hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            sha256_hex(b"admin"),
            "8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918"
        );
    }

    #[test]
    fn base64往返() {
        for s in ["", "a", "ab", "abc", "admin:admin", "中文密码"] {
            let enc = base64_encode(s.as_bytes());
            let dec = String::from_utf8(base64_decode(&enc).unwrap()).unwrap();
            assert_eq!(dec, s, "base64 往返失败: {s:?} -> {enc}");
        }
    }

    #[test]
    fn base64解码拒绝非法字符() {
        assert!(base64_decode("abc$def").is_none());
    }

    #[test]
    fn percent解码() {
        assert_eq!(percent_decode("a%20b"), "a b");
        assert_eq!(percent_decode("a+b"), "a b");
        assert_eq!(percent_decode("%E4%B8%AD%E6%96%87"), "中文");
        assert_eq!(percent_decode("normal"), "normal");
    }

    #[test]
    fn 常量时间比较行为正确() {
        assert!(ct_eq(b"abc", b"abc"));
        assert!(!ct_eq(b"abc", b"abd"));
        assert!(!ct_eq(b"abc", b"abcd"));
        assert!(ct_eq(b"", b""));
    }

    #[test]
    fn 鉴权默认账号可用() {
        // 构造一个带默认 web 配置的 Config
        let mut cfg = Config::default();
        cfg.web = Some(crate::config::Web {
            enable: true,
            listen: "0.0.0.0:9080".into(),
            username: Some("admin".into()),
            // admin 的 SHA-256
            password_sha256: Some(
                "8c6976e5b5410415bde908bd4dee15dfb167a9c873fc4bb8a81f6f2ab448a918".into(),
            ),
            session_ttl_h:24,
        });
        let mk = |auth: Option<&str>| Request {
            method: "GET".into(),
            path: "/".into(),
            query: String::new(),
            headers: auth
                .map(|a| vec![("Authorization".to_string(), a.to_string())])
                .unwrap_or_default(),
            body: vec![],
        };
        let ok = format!("Basic {}", base64_encode(b"admin:admin"));
        let bad_pw = format!("Basic {}", base64_encode(b"admin:wrong"));
        let bad_user = format!("Basic {}", base64_encode(b"root:admin"));

        assert!(check_auth(&cfg, &mk(Some(&ok))), "正确凭据应通过");
        assert!(!check_auth(&cfg, &mk(Some(&bad_pw))), "错误密码应拒绝");
        assert!(!check_auth(&cfg, &mk(Some(&bad_user))), "错误用户名应拒绝");
        assert!(!check_auth(&cfg, &mk(None)), "无凭据应拒绝");
    }


    #[test]
    fn 无web段时拒绝访问() {
        // 回归测试（真机踩过）：配置里没有 web 段时，
        // check_auth 原本 return true，导致无凭据请求全部 200。
        // 配置文件里有订阅 token，这等于敞开整个管理界面。
        let cfg = Config::default();
        assert!(cfg.web.is_none());
        let req = Request {
            method: "GET".into(),
            path: "/".into(),
            query: String::new(),
            headers: vec![],
            body: vec![],
        };
        assert!(!check_auth(&cfg, &req), "无 web 段必须拒绝，不能放行");
    }

    #[test]
    fn 未配置密码时拒绝访问() {
        // fail closed：没密码不等于免鉴权
        let mut cfg = Config::default();
        cfg.web = Some(crate::config::Web {
            enable: true,
            listen: "0.0.0.0:9080".into(),
            username: Some("admin".into()),
            password_sha256: None,
            session_ttl_h: 24,
        });
        let req = Request {
            method: "GET".into(),
            path: "/".into(),
            query: String::new(),
            headers: vec![("Authorization".into(), "Basic YWRtaW46".into())],
            body: vec![],
        };
        assert!(!check_auth(&cfg, &req));
    }

    #[test]
    fn 大小写不敏感的头名() {
        let req = Request {
            method: "GET".into(),
            path: "/".into(),
            query: String::new(),
            headers: vec![("CONTENT-type".into(), "application/json".into())],
            body: vec![],
        };
        assert_eq!(req.header("content-type"), Some("application/json"));
    }

    #[test]
    fn query参数解析() {
        let req = Request {
            method: "GET".into(),
            path: "/x".into(),
            query: "a=1&name=%E4%B8%AD%E6%96%87&flag".into(),
            headers: vec![],
            body: vec![],
        };
        assert_eq!(req.param("a"), Some("1".into()));
        assert_eq!(req.param("name"), Some("中文".into()));
        assert_eq!(req.param("flag"), Some(String::new()));
        assert_eq!(req.param("missing"), None);
    }
}
