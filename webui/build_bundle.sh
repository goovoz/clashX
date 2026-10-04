#!/bin/bash
# 合并前端库为单文件 bundle。
#
# 为什么用 shell 而不是 Python：这台机器上 Python 进程对某些文件
# 报 FileNotFoundError（而 ls / cat 能正常读），不折腾。
#
# 加载顺序（不能变）：
#   preact(UMD) -> preactHooks -> preactCompat -> shim -> htm -> app
# 因为 compat/hooks 的 UMD 浏览器分支从全局取依赖：
#   t((n||self).preactHooks={}, n.preact)
#   t((n||self).preactCompat={}, n.preact, n.preactHooks)
set -e
# 已在 webui/ 下时才 cd；否则（从 ocrt-rescue/ 调用）进 webui。
if [ ! -f "./preact-compat.js" ]; then
  cd webui
fi

for f in preact.umd.js preact-hooks.js preact-compat.js htm.js app.js; do
  if [ ! -s "$f" ]; then
    echo "缺少 $f（cwd=$(pwd)）" >&2
    exit 1
  fi
done

{
cat <<'HDR'
/* clashx Web UI bundle —— 由 build_bundle.sh 生成，请勿手改 */
/* 加载顺序：preact(UMD) -> preactHooks -> preactCompat -> shim -> htm -> app */
/* ★ preact 必须用 UMD 版（挂 self.preact）。CommonJS 版挂 module.exports，
     浏览器里 window.preact 是 undefined，compat/hooks 拿不到依赖直接崩。 */

HDR

echo "/* ===== preact.umd.js ===== */"
cat preact.umd.js

echo
echo "/* ===== preact-hooks.js ===== */"
cat preact-hooks.js

echo
echo "/* ===== preact-compat.js ===== */"
cat preact-compat.js

cat <<'SHIM'

/* ===== shim：React 兼容别名 =====
 *
 * preact-compat 的 UMD 挂到全局 preactCompat，不是 React。
 * app.js 一律用 window.React 显式取（不用裸标识符）——
 * 严格模式、模块作用域、测试沙箱里裸标识符都可能拿不到。
 * compat 只有 React 17 风格的 render()，没有 createRoot，补一个兼容层。
 */
(function () {
  // ★ 三个 UMD 挂的是 self.preactCompat，而 window === self 在浏览器里成立。
  //   但为了稳妥（模块作用域、测试沙箱），按 self -> window -> 全局 依次取。
  var G = (typeof self !== 'undefined' && self) || (typeof window !== 'undefined' && window) || this;
  var c = G.preactCompat || (typeof window !== 'undefined' && window.preactCompat);
  if (!c) {
    console.error('[clashx] preactCompat 未加载。self=', typeof self, 'window=', typeof window);
    return;
  }
  G.React = c;
  G.ReactDOM = c;
  if (typeof window !== 'undefined') { window.React = c; window.ReactDOM = c; }
  var h = G.preactHooks || {};
  ['useState', 'useEffect', 'useCallback', 'useRef', 'useMemo', 'useContext',
   'useReducer', 'useLayoutEffect'].forEach(function (k) {
    if (!window.React[k] && h[k]) window.React[k] = h[k];
  });
  if (typeof c.createRoot !== 'function') {
    c.createRoot = function (container) {
      return {
        render: function (el) { c.render(el, container); },
        unmount: function () { c.render(null, container); },
      };
    };
  }
})();

SHIM

echo "/* ===== htm.js ===== */"
cat htm.js

echo
echo "/* ===== app.js ===== */"
cat app.js
} > bundle.js.tmp

mv bundle.js.tmp bundle.js

# 校验：三个库必须挂到浏览器全局
for m in 'self).preact={}' 'self).preactHooks={}' 'self).preactCompat={}' 'window.React = c'; do
  n=$(grep -c "$m" bundle.js || true)
  if [ "$n" -lt 1 ]; then
    echo "校验失败：bundle 里缺少 $m" >&2
    exit 1
  fi
done

echo "bundle.js: $(stat -c%s bundle.js) bytes（校验通过）"
