#!/usr/bin/env node
/*
 * clashx Web UI 渲染验证。
 *
 * 用途：改了 webui/ 之后跑一下，确认页面真能渲染出内容。
 * 之前用 curl 只能证明「接口通」，证明不了「前端不白屏」——
 * 而前端白屏的三个真实原因（preact 用错CommonJS 版、compat 没有
 * createRoot、UMD 挂 self 不挂 window）curl 一个都测不出来。
 *
 * 用法（在 .101 上）：
 *   cd /root/clashx
 *   bash webui/build_bundle.sh                # 先构建 bundle
 *   node webui/verify-render.js              # 再验证
 *
 * 需要：node（不含 jsdom也能跑，见下）
 *
 * ─────────────────────────────────────────────
 * 为什么不用 jsdom：装新版 jsdom 依赖链里有 ESM/CJS 混用
 * （@csstools/css-calc 是 .mjs，被 cjs 版css-color 用 require 加载）
 * 在 Node 22 的 CJS 下直接 ERR_REQUIRE_ESM。
 * 装 jsdom@22 又要几分钟（npm 走代理慢）。
 *
 * 所以用 node:vm + 自写 DOM 替身。关键是要模拟对：
 *   1. window === self === globalThis（浏览器语义）
 *   2. **不提供 module / exports / require** ——
 *      UMD 判断 `"object"==typeof exports && "undefined"!=typeof module`
 *      才走 CommonJS。Node 天然有 module，不去掉就会走错分支报
 *      "Cannot find module 'preact'"。这是环境问题不是 bundle 问题。
 *   3. 完整的 DOM 接口（insertBefore / parentNode / replaceChild ...），
 *      preact 渲染时会用。
 */

const fs = require('fs');
const vm = require('vm');
const path = require('path');

const ROOT = path.resolve(__dirname, '..');
const BUNDLE = process.env.BUNDLE || path.join(ROOT, 'webui', 'bundle.js');
const INDEX = path.join(ROOT, 'webui', 'index.html');
// 可选：从运行中的服务拉线上实际内容（更接近真实）
const LIVE = process.env.LIVE_BUNDLE;

function makeEl(tag) {
  const el = {
    tagName: String(tag || '').toUpperCase(),
    nodeType: 1,
    style: {},
    children: [],
    parentNode: null,
    className: '',
    _attrs: {},
    setAttribute(k, v) { this._attrs[k] = v; },
    getAttribute(k) { return this._attrs[k]; },
    removeAttribute(k) { delete this._attrs[k]; },
    addEventListener() {},
    removeEventListener() {},
    appendChild(c) { if (c) { c.parentNode = this; this.children.push(c); } return c; },
    insertBefore(c, ref) {
      if (c) c.parentNode = this;
      const i = ref ? this.children.indexOf(ref) : -1;
      if (i < 0) this.children.push(c); else this.children.splice(i, 0, c);
      return c;
    },
    removeChild(c) {
      const i = this.children.indexOf(c);
      if (i >= 0) this.children.splice(i, 1);
      return c;
    },
    replaceChild(n, o) {
      const i = this.children.indexOf(o);
      if (i >= 0) this.children[i] = n;
      return o;
    },
    contains() { return false; },
    querySelector() { return null; },
    querySelectorAll() { return []; },
    getElementsByTagName() { return []; },
    get firstChild() { return this.children[0] || null; },
    get lastChild() { return this.children[this.children.length - 1] || null; },
    set textContent(v) { this.children = []; this._text = v; },
    get textContent() { return this._text || ''; },
    set innerHTML(v) { this._html = v; this.children = []; },
    get innerHTML() { return this._html || ''; },
    focus() {}, blur() {},
    cloneNode() { return makeEl(tag); },
  };
  return el;
}

const rootEl = makeEl('div');
/** 排队延后的回调（setTimeout / requestAnimationFrame）。 */
const pending = [];
const logs = [];
const errors = [];

const sandbox = {
  console: {
    log: (...a) => logs.push(a.join(' ')),
    error: (...a) => errors.push('[console.error] ' + a.join(' ')),
    warn: (...a) => logs.push('[warn] ' + a.join(' ')),
  },
  document: {
    getElementById: (id) => (id === 'root' ? rootEl : null),
    createElement: (t) => makeEl(t),
    createElementNS: (ns, t) => makeEl(t),
    createTextNode: (t) => ({ nodeType: 3, textContent: String(t) }),
    createDocumentFragment: () => makeEl('#fragment'),
    addEventListener() {},
    removeEventListener() {},
    querySelector: () => null,
    querySelectorAll: () => [],
    body: makeEl('body'),
    head: makeEl('head'),
    documentElement: makeEl('html'),
  },
  // setTimeout 排队而非立即执行：preact 的渲染是异步的
  // （Promise.then + requestAnimationFrame），立即执行只能看到初始骨架。
  // 队列在 vm.runInContext 之后按序 drain。
  setTimeout: (fn) => { pending.push(fn); return 0; },
  clearTimeout() {},
  setInterval: () => 0,
  clearInterval() {},
  requestAnimationFrame: (fn) => { pending.push(() => fn(0)); return 0; },
  cancelAnimationFrame() {},
  Promise, Object, Array, String, Number, Boolean, Math, JSON, Date, RegExp,
  Error, TypeError, RangeError, Symbol, Map, Set, WeakMap, Reflect, Proxy,
  parseInt, parseFloat, isNaN, isFinite, encodeURIComponent, decodeURIComponent,
  // fetch 默认返回空 JSON；设 CLASHX_API_BASE 可打到真实服务
  fetch: process.env.CLASHX_API_BASE
    ? (url, opts) => realFetch(process.env.CLASHX_API_BASE, url, opts)
    : () => Promise.resolve({ ok: true, status: 200, text: () => Promise.resolve('{}') }),
  navigator: { userAgent: 'clashx-verify' },
  location: { href: 'http://verify/', protocol: 'http:', host: 'verify' },
  // app.js 用了 hashchange 监听，沙箱要提供（浏览器天然有）
  addEventListener() {},
  removeEventListener() {},
  // ★ 故意不给 module / exports / require —— 见文件头说明
};

function realFetch(base, url, opts) {
  const http = require('http');
  const u = new URL(url, base);
  return new Promise((resolve, reject) => {
    const req = http.request({
      hostname: u.hostname,
      port: u.port || 80,
      path: u.pathname + u.search,
      method: (opts && opts.method) || 'GET',
      headers: Object.assign(
        { Authorization: 'Basic ' + Buffer.from('admin:admin').toString('base64') },
        (opts && opts.body) ? { 'Content-Type': 'application/json' } : {}
      ),
    }, (res) => {
      let data = '';
      res.on('data', (c) => (data += c));
      res.on('end', () => resolve({
        ok: res.statusCode >= 200 && res.statusCode < 300,
        status: res.statusCode,
        text: () => Promise.resolve(data),
      }));
    });
    req.on('error', reject);
    if (opts && opts.body) req.write(opts.body);
    req.end();
  });
}

vm.createContext(sandbox);
// 浏览器语义：window === self === globalThis
vm.runInContext('var __g = this; __g.window = __g; __g.self = __g;', sandbox);

const bundlePath = LIVE || BUNDLE;
if (!fs.existsSync(bundlePath)) {
  console.error('bundle 不存在：' + bundlePath);
  console.error('先跑 bash webui/build_bundle.sh');
  process.exit(2);
}
const code = fs.readFileSync(bundlePath, 'utf-8');

console.log('验证文件:', bundlePath, `(${code.length} bytes)`);
console.log('');

try {
  vm.runInContext(code, sandbox, { filename: 'bundle.js' });
} catch (e) {
  console.log('✗ 执行抛出:', e.message);
  console.log(e.stack.split('\n').slice(0, 6).join('\n'));
  if (logs.length) { console.log('--- console ---'); logs.forEach((l) => console.log(' ', l)); }
  process.exit(1);
}

// drain 延后回调。
// 注意：这里不能用顶层 await（会把文件变成 ESM，require 就没了），
// 也不能靠真实事件循环（微任务只在同步代码结束后才跑）。
// preact 的渲染链是 Promise.then -> rAF -> Promise.then，
// 所以做法是：多轮同步 drain 队列，每轮之间手动让微任务有机会跑。
// fetch 走真实 http 时数据回不来属正常 —— 骨架渲染对了即可。
let rounds = 0;
while (pending.length > 0 && rounds < 50) {
  const batch = pending.splice(0, pending.length);
  for (const fn of batch) {
    try { fn(); } catch (e) { errors.push('pending: ' + e.message); }
  }
  rounds++;
}
console.log('  (drain ' + rounds + ' 轮)');

const g = sandbox;
const problems = [];

// 1. 四个库/别名必须就位
for (const k of ['preact', 'preactHooks', 'preactCompat', 'React', 'ReactDOM', 'htm']) {
  const ok = !!g[k];
  console.log(`  ${ok ? '✓' : '✗'} ${k}`);
  if (!ok) problems.push(k + ' 未加载');
}

// 2. app.js 用到的 API 必须存在
const R = g.React;
const NEEDED = ['createElement', 'useState', 'useEffect', 'useCallback', 'createContext', 'useContext'];
if (R) {
  for (const k of NEEDED) {
    const ok = typeof R[k] === 'function';
    console.log(`  ${ok ? '✓' : '✗'} React.${k}`);
    if (!ok) problems.push('React.' + k + ' 缺失');
  }
  const cr = g.ReactDOM && typeof g.ReactDOM.createRoot === 'function';
  console.log(`  ${cr ? '✓' : '✗'} ReactDOM.createRoot`);
  if (!cr) problems.push('ReactDOM.createRoot 缺失');
}

// 3. 根节点必须被渲染
const text = rootEl.children.length ? collectText(rootEl) : '';
console.log('');
console.log('  #root 子节点数:', rootEl.children.length);
console.log('  #root 文本长度:', text.length);
if (text) console.log('  可见文本:', text.slice(0, 500));
if (rootEl.children.length === 0) problems.push('#root 为空 —— 页面白屏');
if (text.length < 10) problems.push('渲染内容过少');

// 4. console 里不该有 error
if (errors.length) {
  console.log('');
  console.log('--- console.error ---');
  errors.forEach((e) => console.log(' ', e));
  problems.push(errors.length + ' 个 console.error');
}

console.log('');
if (problems.length) {
  console.log('✗ 验证失败:');
  problems.forEach((p) => console.log('  -', p));
  process.exit(1);
}
console.log('✓ 验证通过：页面能渲染，未白屏');

function collectText(el) {
  let s = '';
  if (el._text) s += el._text;
  for (const c of el.children || []) {
    if (typeof c === 'string') s += c;
    else if (c && c.children) s += collectText(c);
    else if (c && c.textContent) s += c.textContent;
  }
  return s.replace(/\s+/g, ' ').trim();
}
