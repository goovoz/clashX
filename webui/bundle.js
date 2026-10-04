/* clashx Web UI bundle —— 由 build_bundle.sh 生成，请勿手改 */
/* 加载顺序：preact(UMD) -> preactHooks -> preactCompat -> shim -> htm -> app */
/* ★ preact 必须用 UMD 版（挂 self.preact）。CommonJS 版挂 module.exports，
     浏览器里 window.preact 是 undefined，compat/hooks 拿不到依赖直接崩。 */

/* ===== preact.umd.js ===== */
!function(n,l){"object"==typeof exports&&"undefined"!=typeof module?l(exports):"function"==typeof define&&define.amd?define(["exports"],l):l((n||self).preact={})}(this,function(n){var l,u,t,i,e,r,o,f,c,s,a,h,p,y="http://www.w3.org/2000/svg",v="http://www.w3.org/1999/xhtml",d=void 0,w={},_=[],g=/acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i,m=Array.isArray;function b(n,l){for(var u in l)n[u]=l[u];return n}function k(n){n&&n.parentNode&&n.parentNode.removeChild(n)}function x(n,u,t){var i,e,r,o={};for(r in u)"key"==r?i=u[r]:"ref"==r?e=u[r]:o[r]=u[r];if(arguments.length>2&&(o.children=arguments.length>3?l.call(arguments,2):t),"function"==typeof n&&null!=n.defaultProps)for(r in n.defaultProps)o[r]===d&&(o[r]=n.defaultProps[r]);return S(n,o,i,e,null)}function S(n,l,i,e,r){var o={type:n,props:l,key:i,ref:e,__k:null,__:null,__b:0,__e:null,__c:null,constructor:d,__v:null==r?++t:r,__i:-1,__u:0};return null==r&&null!=u.vnode&&u.vnode(o),o}function C(n){return n.children}function M(n,l){this.props=n,this.context=l}function P(n,l){if(null==l)return n.__?P(n.__,n.__i+1):null;for(var u;l<n.__k.length;l++)if(null!=(u=n.__k[l])&&null!=u.__e)return u.__e;return"function"==typeof n.type?P(n):null}function T(n){var l,u;if(null!=(n=n.__)&&null!=n.__c){for(n.__e=n.__c.base=null,l=0;l<n.__k.length;l++)if(null!=(u=n.__k[l])&&null!=u.__e){n.__e=n.__c.base=u.__e;break}return T(n)}}function $(n){(!n.__d&&(n.__d=!0)&&e.push(n)&&!I.__r++||r!==u.debounceRendering)&&((r=u.debounceRendering)||o)(I)}function I(){var n,l,t,i,r,o,c,s;for(e.sort(f);n=e.shift();)n.__d&&(l=e.length,i=void 0,o=(r=(t=n).__v).__e,c=[],s=[],t.__P&&((i=b({},r)).__v=r.__v+1,u.vnode&&u.vnode(i),N(t.__P,i,r,t.__n,t.__P.namespaceURI,32&r.__u?[o]:null,c,null==o?P(r):o,!!(32&r.__u),s),i.__v=r.__v,i.__.__k[i.__i]=i,V(c,i,s),i.__e!=o&&T(i)),e.length>l&&e.sort(f));I.__r=0}function A(n,l,u,t,i,e,r,o,f,c,s){var a,h,p,y,v,g,m=t&&t.__k||_,b=l.length;for(f=H(u,l,m,f,b),a=0;a<b;a++)null!=(p=u.__k[a])&&(h=-1===p.__i?w:m[p.__i]||w,p.__i=a,g=N(n,p,h,i,e,r,o,f,c,s),y=p.__e,p.ref&&h.ref!=p.ref&&(h.ref&&B(h.ref,null,p),s.push(p.ref,p.__c||y,p)),null==v&&null!=y&&(v=y),4&p.__u||h.__k===p.__k?f=L(p,f,n):"function"==typeof p.type&&g!==d?f=g:y&&(f=y.nextSibling),p.__u&=-7);return u.__e=v,f}function H(n,l,u,t,i){var e,r,o,f,c,s=u.length,a=s,h=0;for(n.__k=new Array(i),e=0;e<i;e++)null!=(r=l[e])&&"boolean"!=typeof r&&"function"!=typeof r?(f=e+h,(r=n.__k[e]="string"==typeof r||"number"==typeof r||"bigint"==typeof r||r.constructor==String?S(null,r,null,null,null):m(r)?S(C,{children:r},null,null,null):r.constructor===d&&r.__b>0?S(r.type,r.props,r.key,r.ref?r.ref:null,r.__v):r).__=n,r.__b=n.__b+1,o=null,-1!==(c=r.__i=j(r,u,f,a))&&(a--,(o=u[c])&&(o.__u|=2)),null==o||null===o.__v?(-1==c&&h--,"function"!=typeof r.type&&(r.__u|=4)):c!=f&&(c==f-1?h--:c==f+1?h++:(c>f?h--:h++,r.__u|=4))):n.__k[e]=null;if(a)for(e=0;e<s;e++)null!=(o=u[e])&&0==(2&o.__u)&&(o.__e==t&&(t=P(o)),D(o,o));return t}function L(n,l,u){var t,i;if("function"==typeof n.type){for(t=n.__k,i=0;t&&i<t.length;i++)t[i]&&(t[i].__=n,l=L(t[i],l,u));return l}n.__e!=l&&(l&&n.type&&!u.contains(l)&&(l=P(n)),u.insertBefore(n.__e,l||null),l=n.__e);do{l=l&&l.nextSibling}while(null!=l&&8==l.nodeType);return l}function j(n,l,u,t){var i,e,r=n.key,o=n.type,f=l[u];if(null===f||f&&r==f.key&&o===f.type&&0==(2&f.__u))return u;if(t>(null!=f&&0==(2&f.__u)?1:0))for(i=u-1,e=u+1;i>=0||e<l.length;){if(i>=0){if((f=l[i])&&0==(2&f.__u)&&r==f.key&&o===f.type)return i;i--}if(e<l.length){if((f=l[e])&&0==(2&f.__u)&&r==f.key&&o===f.type)return e;e++}}return-1}function F(n,l,u){"-"==l[0]?n.setProperty(l,null==u?"":u):n[l]=null==u?"":"number"!=typeof u||g.test(l)?u:u+"px"}function O(n,l,u,t,i){var e;n:if("style"==l)if("string"==typeof u)n.style.cssText=u;else{if("string"==typeof t&&(n.style.cssText=t=""),t)for(l in t)u&&l in u||F(n.style,l,"");if(u)for(l in u)t&&u[l]===t[l]||F(n.style,l,u[l])}else if("o"==l[0]&&"n"==l[1])e=l!=(l=l.replace(c,"$1")),l=l.toLowerCase()in n||"onFocusOut"==l||"onFocusIn"==l?l.toLowerCase().slice(2):l.slice(2),n.l||(n.l={}),n.l[l+e]=u,u?t?u.u=t.u:(u.u=s,n.addEventListener(l,e?h:a,e)):n.removeEventListener(l,e?h:a,e);else{if(i==y)l=l.replace(/xlink(H|:h)/,"h").replace(/sName$/,"s");else if("width"!=l&&"height"!=l&&"href"!=l&&"list"!=l&&"form"!=l&&"tabIndex"!=l&&"download"!=l&&"rowSpan"!=l&&"colSpan"!=l&&"role"!=l&&"popover"!=l&&l in n)try{n[l]=null==u?"":u;break n}catch(n){}"function"==typeof u||(null==u||!1===u&&"-"!=l[4]?n.removeAttribute(l):n.setAttribute(l,"popover"==l&&1==u?"":u))}}function z(n){return function(l){if(this.l){var t=this.l[l.type+n];if(null==l.t)l.t=s++;else if(l.t<t.u)return;return t(u.event?u.event(l):l)}}}function N(n,l,t,i,e,r,o,f,c,s){var a,h,p,y,v,w,_,g,x,S,P,T,$,I,H,L,j,F=l.type;if(l.constructor!==d)return null;128&t.__u&&(c=!!(32&t.__u),r=[f=l.__e=t.__e]),(a=u.__b)&&a(l);n:if("function"==typeof F)try{if(g=l.props,x="prototype"in F&&F.prototype.render,S=(a=F.contextType)&&i[a.__c],P=a?S?S.props.value:a.__:i,t.__c?_=(h=l.__c=t.__c).__=h.__E:(x?l.__c=h=new F(g,P):(l.__c=h=new M(g,P),h.constructor=F,h.render=E),S&&S.sub(h),h.props=g,h.state||(h.state={}),h.context=P,h.__n=i,p=h.__d=!0,h.__h=[],h._sb=[]),x&&null==h.__s&&(h.__s=h.state),x&&null!=F.getDerivedStateFromProps&&(h.__s==h.state&&(h.__s=b({},h.__s)),b(h.__s,F.getDerivedStateFromProps(g,h.__s))),y=h.props,v=h.state,h.__v=l,p)x&&null==F.getDerivedStateFromProps&&null!=h.componentWillMount&&h.componentWillMount(),x&&null!=h.componentDidMount&&h.__h.push(h.componentDidMount);else{if(x&&null==F.getDerivedStateFromProps&&g!==y&&null!=h.componentWillReceiveProps&&h.componentWillReceiveProps(g,P),!h.__e&&(null!=h.shouldComponentUpdate&&!1===h.shouldComponentUpdate(g,h.__s,P)||l.__v==t.__v)){for(l.__v!=t.__v&&(h.props=g,h.state=h.__s,h.__d=!1),l.__e=t.__e,l.__k=t.__k,l.__k.some(function(n){n&&(n.__=l)}),T=0;T<h._sb.length;T++)h.__h.push(h._sb[T]);h._sb=[],h.__h.length&&o.push(h);break n}null!=h.componentWillUpdate&&h.componentWillUpdate(g,h.__s,P),x&&null!=h.componentDidUpdate&&h.__h.push(function(){h.componentDidUpdate(y,v,w)})}if(h.context=P,h.props=g,h.__P=n,h.__e=!1,$=u.__r,I=0,x){for(h.state=h.__s,h.__d=!1,$&&$(l),a=h.render(h.props,h.state,h.context),H=0;H<h._sb.length;H++)h.__h.push(h._sb[H]);h._sb=[]}else do{h.__d=!1,$&&$(l),a=h.render(h.props,h.state,h.context),h.state=h.__s}while(h.__d&&++I<25);h.state=h.__s,null!=h.getChildContext&&(i=b(b({},i),h.getChildContext())),x&&!p&&null!=h.getSnapshotBeforeUpdate&&(w=h.getSnapshotBeforeUpdate(y,v)),f=A(n,m(L=null!=a&&a.type===C&&null==a.key?a.props.children:a)?L:[L],l,t,i,e,r,o,f,c,s),h.base=l.__e,l.__u&=-161,h.__h.length&&o.push(h),_&&(h.__E=h.__=null)}catch(n){if(l.__v=null,c||null!=r)if(n.then){for(l.__u|=c?160:128;f&&8==f.nodeType&&f.nextSibling;)f=f.nextSibling;r[r.indexOf(f)]=null,l.__e=f}else for(j=r.length;j--;)k(r[j]);else l.__e=t.__e,l.__k=t.__k;u.__e(n,l,t)}else null==r&&l.__v==t.__v?(l.__k=t.__k,l.__e=t.__e):f=l.__e=q(t.__e,l,t,i,e,r,o,c,s);return(a=u.diffed)&&a(l),128&l.__u?void 0:f}function V(n,l,t){for(var i=0;i<t.length;i++)B(t[i],t[++i],t[++i]);u.__c&&u.__c(l,n),n.some(function(l){try{n=l.__h,l.__h=[],n.some(function(n){n.call(l)})}catch(n){u.__e(n,l.__v)}})}function q(n,t,i,e,r,o,f,c,s){var a,h,p,_,g,b,x,S=i.props,C=t.props,M=t.type;if("svg"==M?r=y:"math"==M?r="http://www.w3.org/1998/Math/MathML":r||(r=v),null!=o)for(a=0;a<o.length;a++)if((g=o[a])&&"setAttribute"in g==!!M&&(M?g.localName==M:3==g.nodeType)){n=g,o[a]=null;break}if(null==n){if(null==M)return document.createTextNode(C);n=document.createElementNS(r,M,C.is&&C),c&&(u.__m&&u.__m(t,o),c=!1),o=null}if(null===M)S===C||c&&n.data===C||(n.data=C);else{if(o=o&&l.call(n.childNodes),S=i.props||w,!c&&null!=o)for(S={},a=0;a<n.attributes.length;a++)S[(g=n.attributes[a]).name]=g.value;for(a in S)if(g=S[a],"children"==a);else if("dangerouslySetInnerHTML"==a)p=g;else if(!(a in C)){if("value"==a&&"defaultValue"in C||"checked"==a&&"defaultChecked"in C)continue;O(n,a,null,g,r)}for(a in C)g=C[a],"children"==a?_=g:"dangerouslySetInnerHTML"==a?h=g:"value"==a?b=g:"checked"==a?x=g:c&&"function"!=typeof g||S[a]===g||O(n,a,g,S[a],r);if(h)c||p&&(h.__html===p.__html||h.__html===n.innerHTML)||(n.innerHTML=h.__html),t.__k=[];else if(p&&(n.innerHTML=""),A(n,m(_)?_:[_],t,i,e,"foreignObject"==M?v:r,o,f,o?o[0]:i.__k&&P(i,0),c,s),null!=o)for(a=o.length;a--;)k(o[a]);c||(a="value","progress"==M&&null==b?n.removeAttribute("value"):b!==d&&(b!==n[a]||"progress"==M&&!b||"option"==M&&b!==S[a])&&O(n,a,b,S[a],r),a="checked",x!==d&&x!==n[a]&&O(n,a,x,S[a],r))}return n}function B(n,l,t){try{if("function"==typeof n){var i="function"==typeof n.__u;i&&n.__u(),i&&null==l||(n.__u=n(l))}else n.current=l}catch(n){u.__e(n,t)}}function D(n,l,t){var i,e;if(u.unmount&&u.unmount(n),(i=n.ref)&&(i.current&&i.current!==n.__e||B(i,null,l)),null!=(i=n.__c)){if(i.componentWillUnmount)try{i.componentWillUnmount()}catch(n){u.__e(n,l)}i.base=i.__P=null}if(i=n.__k)for(e=0;e<i.length;e++)i[e]&&D(i[e],l,t||"function"!=typeof n.type);t||k(n.__e),n.__c=n.__=n.__e=d}function E(n,l,u){return this.constructor(n,u)}function G(n,t,i){var e,r,o,f;t==document&&(t=document.documentElement),u.__&&u.__(n,t),r=(e="function"==typeof i)?null:i&&i.__k||t.__k,o=[],f=[],N(t,n=(!e&&i||t).__k=x(C,null,[n]),r||w,w,t.namespaceURI,!e&&i?[i]:r?null:t.firstChild?l.call(t.childNodes):null,o,!e&&i?i:r?r.__e:t.firstChild,e,f),V(o,n,f)}l=_.slice,u={__e:function(n,l,u,t){for(var i,e,r;l=l.__;)if((i=l.__c)&&!i.__)try{if((e=i.constructor)&&null!=e.getDerivedStateFromError&&(i.setState(e.getDerivedStateFromError(n)),r=i.__d),null!=i.componentDidCatch&&(i.componentDidCatch(n,t||{}),r=i.__d),r)return i.__E=i}catch(l){n=l}throw n}},t=0,i=function(n){return null!=n&&n.constructor==d},M.prototype.setState=function(n,l){var u;u=null!=this.__s&&this.__s!==this.state?this.__s:this.__s=b({},this.state),"function"==typeof n&&(n=n(b({},u),this.props)),n&&b(u,n),null!=n&&this.__v&&(l&&this._sb.push(l),$(this))},M.prototype.forceUpdate=function(n){this.__v&&(this.__e=!0,n&&this.__h.push(n),$(this))},M.prototype.render=C,e=[],o="function"==typeof Promise?Promise.prototype.then.bind(Promise.resolve()):setTimeout,f=function(n,l){return n.__v.__b-l.__v.__b},I.__r=0,c=/(PointerCapture)$|Capture$/i,s=0,a=z(!1),h=z(!0),p=0,n.Component=M,n.Fragment=C,n.cloneElement=function(n,u,t){var i,e,r,o,f=b({},n.props);for(r in n.type&&n.type.defaultProps&&(o=n.type.defaultProps),u)"key"==r?i=u[r]:"ref"==r?e=u[r]:f[r]=u[r]===d&&o!==d?o[r]:u[r];return arguments.length>2&&(f.children=arguments.length>3?l.call(arguments,2):t),S(n.type,f,i||n.key,e||n.ref,null)},n.createContext=function(n,l){var u={__c:l="__cC"+p++,__:n,Consumer:function(n,l){return n.children(l)},Provider:function(n){var u,t;return this.getChildContext||(u=new Set,(t={})[l]=this,this.getChildContext=function(){return t},this.componentWillUnmount=function(){u=null},this.shouldComponentUpdate=function(n){this.props.value!==n.value&&u.forEach(function(n){n.__e=!0,$(n)})},this.sub=function(n){u.add(n);var l=n.componentWillUnmount;n.componentWillUnmount=function(){u&&u.delete(n),l&&l.call(n)}}),n.children}};return u.Provider.__=u.Consumer.contextType=u},n.createElement=x,n.createRef=function(){return{current:null}},n.h=x,n.hydrate=function n(l,u){G(l,u,n)},n.isValidElement=i,n.options=u,n.render=G,n.toChildArray=function n(l,u){return u=u||[],null==l||"boolean"==typeof l||(m(l)?l.some(function(l){n(l,u)}):u.push(l)),u}});
//# sourceMappingURL=preact.umd.js.map

/* ===== preact-hooks.js ===== */
!function(n,t){"object"==typeof exports&&"undefined"!=typeof module?t(exports,require("preact")):"function"==typeof define&&define.amd?define(["exports","preact"],t):t((n||self).preactHooks={},n.preact)}(this,function(n,t){var u,r,i,o,f=0,c=[],e=t.options,a=e.__b,v=e.__r,l=e.diffed,d=e.__c,s=e.unmount,p=e.__;function h(n,t){e.__h&&e.__h(r,n,f||t),f=0;var u=r.__H||(r.__H={__:[],__h:[]});return n>=u.__.length&&u.__.push({}),u.__[n]}function y(n){return f=1,m(j,n)}function m(n,t,i){var o=h(u++,2);if(o.t=n,!o.__c&&(o.__=[i?i(t):j(void 0,t),function(n){var t=o.__N?o.__N[0]:o.__[0],u=o.t(t,n);t!==u&&(o.__N=[u,o.__[1]],o.__c.setState({}))}],o.__c=r,!r.u)){var f=function(n,t,u){if(!o.__c.__H)return!0;var r=o.__c.__H.__.filter(function(n){return!!n.__c});if(r.every(function(n){return!n.__N}))return!c||c.call(this,n,t,u);var i=o.__c.props!==n;return r.forEach(function(n){if(n.__N){var t=n.__[0];n.__=n.__N,n.__N=void 0,t!==n.__[0]&&(i=!0)}}),c&&c.call(this,n,t,u)||i};r.u=!0;var c=r.shouldComponentUpdate,e=r.componentWillUpdate;r.componentWillUpdate=function(n,t,u){if(this.__e){var r=c;c=void 0,f(n,t,u),c=r}e&&e.call(this,n,t,u)},r.shouldComponentUpdate=f}return o.__N||o.__}function T(n,t){var i=h(u++,4);!e.__s&&g(i.__H,t)&&(i.__=n,i.i=t,r.__h.push(i))}function _(n,t){var r=h(u++,7);return g(r.__H,t)&&(r.__=n(),r.__H=t,r.__h=n),r.__}function b(){for(var n;n=c.shift();)if(n.__P&&n.__H)try{n.__H.__h.forEach(A),n.__H.__h.forEach(F),n.__H.__h=[]}catch(t){n.__H.__h=[],e.__e(t,n.__v)}}e.__b=function(n){r=null,a&&a(n)},e.__=function(n,t){n&&t.__k&&t.__k.__m&&(n.__m=t.__k.__m),p&&p(n,t)},e.__r=function(n){v&&v(n),u=0;var t=(r=n.__c).__H;t&&(i===r?(t.__h=[],r.__h=[],t.__.forEach(function(n){n.__N&&(n.__=n.__N),n.i=n.__N=void 0})):(t.__h.forEach(A),t.__h.forEach(F),t.__h=[],u=0)),i=r},e.diffed=function(n){l&&l(n);var t=n.__c;t&&t.__H&&(t.__H.__h.length&&(1!==c.push(t)&&o===e.requestAnimationFrame||((o=e.requestAnimationFrame)||x)(b)),t.__H.__.forEach(function(n){n.i&&(n.__H=n.i),n.i=void 0})),i=r=null},e.__c=function(n,t){t.some(function(n){try{n.__h.forEach(A),n.__h=n.__h.filter(function(n){return!n.__||F(n)})}catch(u){t.some(function(n){n.__h&&(n.__h=[])}),t=[],e.__e(u,n.__v)}}),d&&d(n,t)},e.unmount=function(n){s&&s(n);var t,u=n.__c;u&&u.__H&&(u.__H.__.forEach(function(n){try{A(n)}catch(n){t=n}}),u.__H=void 0,t&&e.__e(t,u.__v))};var q="function"==typeof requestAnimationFrame;function x(n){var t,u=function(){clearTimeout(r),q&&cancelAnimationFrame(t),setTimeout(n)},r=setTimeout(u,100);q&&(t=requestAnimationFrame(u))}function A(n){var t=r,u=n.__c;"function"==typeof u&&(n.__c=void 0,u()),r=t}function F(n){var t=r;n.__c=n.__(),r=t}function g(n,t){return!n||n.length!==t.length||t.some(function(t,u){return t!==n[u]})}function j(n,t){return"function"==typeof t?t(n):t}n.useCallback=function(n,t){return f=8,_(function(){return n},t)},n.useContext=function(n){var t=r.context[n.__c],i=h(u++,9);return i.c=n,t?(null==i.__&&(i.__=!0,t.sub(r)),t.props.value):n.__},n.useDebugValue=function(n,t){e.useDebugValue&&e.useDebugValue(t?t(n):n)},n.useEffect=function(n,t){var i=h(u++,3);!e.__s&&g(i.__H,t)&&(i.__=n,i.i=t,r.__H.__h.push(i))},n.useErrorBoundary=function(n){var t=h(u++,10),i=y();return t.__=n,r.componentDidCatch||(r.componentDidCatch=function(n,u){t.__&&t.__(n,u),i[1](n)}),[i[0],function(){i[1](void 0)}]},n.useId=function(){var n=h(u++,11);if(!n.__){for(var t=r.__v;null!==t&&!t.__m&&null!==t.__;)t=t.__;var i=t.__m||(t.__m=[0,0]);n.__="P"+i[0]+"-"+i[1]++}return n.__},n.useImperativeHandle=function(n,t,u){f=6,T(function(){return"function"==typeof n?(n(t()),function(){return n(null)}):n?(n.current=t(),function(){return n.current=null}):void 0},null==u?u:u.concat(n))},n.useLayoutEffect=T,n.useMemo=_,n.useReducer=m,n.useRef=function(n){return f=5,_(function(){return{current:n}},[])},n.useState=y});
//# sourceMappingURL=hooks.umd.js.map

/* ===== preact-compat.js ===== */
!function(n,t){"object"==typeof exports&&"undefined"!=typeof module?t(exports,require("preact"),require("preact/hooks")):"function"==typeof define&&define.amd?define(["exports","preact","preact/hooks"],t):t((n||self).preactCompat={},n.preact,n.preactHooks)}(this,function(n,t,e){function r(n,t){for(var e in t)n[e]=t[e];return n}function u(n,t){for(var e in n)if("__source"!==e&&!(e in t))return!0;for(var r in t)if("__source"!==r&&n[r]!==t[r])return!0;return!1}function o(n,t){var r=t(),u=e.useState({t:{__:r,u:t}}),o=u[0].t,c=u[1];return e.useLayoutEffect(function(){o.__=r,o.u=t,i(o)&&c({t:o})},[n,r,t]),e.useEffect(function(){return i(o)&&c({t:o}),n(function(){i(o)&&c({t:o})})},[n]),r}function i(n){var t,e,r=n.u,u=n.__;try{var o=r();return!((t=u)===(e=o)&&(0!==t||1/t==1/e)||t!=t&&e!=e)}catch(n){return!0}}function c(n){n()}function f(n){return n}function l(){return[!1,c]}var a=e.useLayoutEffect;function s(n,t){this.props=n,this.context=t}function h(n,e){function r(n){var t=this.props.ref,r=t==n.ref;return!r&&t&&(t.call?t(null):t.current=null),e?!e(this.props,n)||!r:u(this.props,n)}function o(e){return this.shouldComponentUpdate=r,t.createElement(n,e)}return o.displayName="Memo("+(n.displayName||n.name)+")",o.prototype.isReactComponent=!0,o.__f=!0,o}(s.prototype=new t.Component).isPureReactComponent=!0,s.prototype.shouldComponentUpdate=function(n,t){return u(this.props,n)||u(this.state,t)};var d=t.options.__b;t.options.__b=function(n){n.type&&n.type.__f&&n.ref&&(n.props.ref=n.ref,n.ref=null),d&&d(n)};var v="undefined"!=typeof Symbol&&Symbol.for&&Symbol.for("react.forward_ref")||3911;function p(n){function t(t){var e=r({},t);return delete e.ref,n(e,t.ref||null)}return t.$$typeof=v,t.render=t,t.prototype.isReactComponent=t.__f=!0,t.displayName="ForwardRef("+(n.displayName||n.name)+")",t}var m=function(n,e){return null==n?null:t.toChildArray(t.toChildArray(n).map(e))},b={map:m,forEach:m,count:function(n){return n?t.toChildArray(n).length:0},only:function(n){var e=t.toChildArray(n);if(1!==e.length)throw"Children.only";return e[0]},toArray:t.toChildArray},y=t.options.__e;t.options.__e=function(n,t,e,r){if(n.then)for(var u,o=t;o=o.__;)if((u=o.__c)&&u.__c)return null==t.__e&&(t.__e=e.__e,t.__k=e.__k),u.__c(n,t);y(n,t,e,r)};var _=t.options.unmount;function g(n,t,e){return n&&(n.__c&&n.__c.__H&&(n.__c.__H.__.forEach(function(n){"function"==typeof n.__c&&n.__c()}),n.__c.__H=null),null!=(n=r({},n)).__c&&(n.__c.__P===e&&(n.__c.__P=t),n.__c=null),n.__k=n.__k&&n.__k.map(function(n){return g(n,t,e)})),n}function S(n,t,e){return n&&e&&(n.__v=null,n.__k=n.__k&&n.__k.map(function(n){return S(n,t,e)}),n.__c&&n.__c.__P===t&&(n.__e&&e.appendChild(n.__e),n.__c.__e=!0,n.__c.__P=e)),n}function E(){this.__u=0,this.o=null,this.__b=null}function C(n){var t=n.__.__c;return t&&t.__a&&t.__a(n)}function x(n){var e,r,u;function o(o){if(e||(e=n()).then(function(n){r=n.default||n},function(n){u=n}),u)throw u;if(!r)throw e;return t.createElement(r,o)}return o.displayName="Lazy",o.__f=!0,o}function O(){this.i=null,this.l=null}t.options.unmount=function(n){var t=n.__c;t&&t.__R&&t.__R(),t&&32&n.__u&&(n.type=null),_&&_(n)},(E.prototype=new t.Component).__c=function(n,t){var e=t.__c,r=this;null==r.o&&(r.o=[]),r.o.push(e);var u=C(r.__v),o=!1,i=function(){o||(o=!0,e.__R=null,u?u(c):c())};e.__R=i;var c=function(){if(!--r.__u){if(r.state.__a){var n=r.state.__a;r.__v.__k[0]=S(n,n.__c.__P,n.__c.__O)}var t;for(r.setState({__a:r.__b=null});t=r.o.pop();)t.forceUpdate()}};r.__u++||32&t.__u||r.setState({__a:r.__b=r.__v.__k[0]}),n.then(i,i)},E.prototype.componentWillUnmount=function(){this.o=[]},E.prototype.render=function(n,e){if(this.__b){if(this.__v.__k){var r=document.createElement("div"),u=this.__v.__k[0].__c;this.__v.__k[0]=g(this.__b,r,u.__O=u.__P)}this.__b=null}var o=e.__a&&t.createElement(t.Fragment,null,n.fallback);return o&&(o.__u&=-33),[t.createElement(t.Fragment,null,e.__a?null:n.children),o]};var R=function(n,t,e){if(++e[1]===e[0]&&n.l.delete(t),n.props.revealOrder&&("t"!==n.props.revealOrder[0]||!n.l.size))for(e=n.i;e;){for(;e.length>3;)e.pop()();if(e[1]<e[0])break;n.i=e=e[2]}};function w(n){return this.getChildContext=function(){return n.context},n.children}function j(n){var e=this,r=n.h;e.componentWillUnmount=function(){t.render(null,e.v),e.v=null,e.h=null},e.h&&e.h!==r&&e.componentWillUnmount(),e.v||(e.h=r,e.v={nodeType:1,parentNode:r,childNodes:[],contains:function(){return!0},appendChild:function(n){this.childNodes.push(n),e.h.appendChild(n)},insertBefore:function(n,t){this.childNodes.push(n),e.h.insertBefore(n,t)},removeChild:function(n){this.childNodes.splice(this.childNodes.indexOf(n)>>>1,1),e.h.removeChild(n)}}),t.render(t.createElement(w,{context:e.context},n.__v),e.v)}function T(n,e){var r=t.createElement(j,{__v:n,h:e});return r.containerInfo=e,r}(O.prototype=new t.Component).__a=function(n){var t=this,e=C(t.__v),r=t.l.get(n);return r[0]++,function(u){var o=function(){t.props.revealOrder?(r.push(u),R(t,n,r)):u()};e?e(o):o()}},O.prototype.render=function(n){this.i=null,this.l=new Map;var e=t.toChildArray(n.children);n.revealOrder&&"b"===n.revealOrder[0]&&e.reverse();for(var r=e.length;r--;)this.l.set(e[r],this.i=[1,0,this.i]);return n.children},O.prototype.componentDidUpdate=O.prototype.componentDidMount=function(){var n=this;this.l.forEach(function(t,e){R(n,e,t)})};var k="undefined"!=typeof Symbol&&Symbol.for&&Symbol.for("react.element")||60103,I=/^(?:accent|alignment|arabic|baseline|cap|clip(?!PathU)|color|dominant|fill|flood|font|glyph(?!R)|horiz|image(!S)|letter|lighting|marker(?!H|W|U)|overline|paint|pointer|shape|stop|strikethrough|stroke|text(?!L)|transform|underline|unicode|units|v|vector|vert|word|writing|x(?!C))[A-Z]/,N=/^on(Ani|Tra|Tou|BeforeInp|Compo)/,M=/[A-Z0-9]/g,A="undefined"!=typeof document,D=function(n){return("undefined"!=typeof Symbol&&"symbol"==typeof Symbol()?/fil|che|rad/:/fil|che|ra/).test(n)};function L(n,e,r){return null==e.__k&&(e.textContent=""),t.render(n,e),"function"==typeof r&&r(),n?n.__c:null}function F(n,e,r){return t.hydrate(n,e),"function"==typeof r&&r(),n?n.__c:null}t.Component.prototype.isReactComponent={},["componentWillMount","componentWillReceiveProps","componentWillUpdate"].forEach(function(n){Object.defineProperty(t.Component.prototype,n,{configurable:!0,get:function(){return this["UNSAFE_"+n]},set:function(t){Object.defineProperty(this,n,{configurable:!0,writable:!0,value:t})}})});var U=t.options.event;function V(){}function W(){return this.cancelBubble}function P(){return this.defaultPrevented}t.options.event=function(n){return U&&(n=U(n)),n.persist=V,n.isPropagationStopped=W,n.isDefaultPrevented=P,n.nativeEvent=n};var z,B={enumerable:!1,configurable:!0,get:function(){return this.class}},H=t.options.vnode;t.options.vnode=function(n){"string"==typeof n.type&&function(n){var e=n.props,r=n.type,u={},o=-1===r.indexOf("-");for(var i in e){var c=e[i];if(!("value"===i&&"defaultValue"in e&&null==c||A&&"children"===i&&"noscript"===r||"class"===i||"className"===i)){var f=i.toLowerCase();"defaultValue"===i&&"value"in e&&null==e.value?i="value":"download"===i&&!0===c?c="":"translate"===f&&"no"===c?c=!1:"o"===f[0]&&"n"===f[1]?"ondoubleclick"===f?i="ondblclick":"onchange"!==f||"input"!==r&&"textarea"!==r||D(e.type)?"onfocus"===f?i="onfocusin":"onblur"===f?i="onfocusout":N.test(i)&&(i=f):f=i="oninput":o&&I.test(i)?i=i.replace(M,"-$&").toLowerCase():null===c&&(c=void 0),"oninput"===f&&u[i=f]&&(i="oninputCapture"),u[i]=c}}"select"==r&&u.multiple&&Array.isArray(u.value)&&(u.value=t.toChildArray(e.children).forEach(function(n){n.props.selected=-1!=u.value.indexOf(n.props.value)})),"select"==r&&null!=u.defaultValue&&(u.value=t.toChildArray(e.children).forEach(function(n){n.props.selected=u.multiple?-1!=u.defaultValue.indexOf(n.props.value):u.defaultValue==n.props.value})),e.class&&!e.className?(u.class=e.class,Object.defineProperty(u,"className",B)):(e.className&&!e.class||e.class&&e.className)&&(u.class=u.className=e.className),n.props=u}(n),n.$$typeof=k,H&&H(n)};var q=t.options.__r;t.options.__r=function(n){q&&q(n),z=n.__c};var Z=t.options.diffed;t.options.diffed=function(n){Z&&Z(n);var t=n.props,e=n.__e;null!=e&&"textarea"===n.type&&"value"in t&&t.value!==e.value&&(e.value=null==t.value?"":t.value),z=null};var Y={ReactCurrentDispatcher:{current:{readContext:function(n){return z.__n[n.__c].props.value},useCallback:e.useCallback,useContext:e.useContext,useDebugValue:e.useDebugValue,useDeferredValue:f,useEffect:e.useEffect,useId:e.useId,useImperativeHandle:e.useImperativeHandle,useInsertionEffect:a,useLayoutEffect:e.useLayoutEffect,useMemo:e.useMemo,useReducer:e.useReducer,useRef:e.useRef,useState:e.useState,useSyncExternalStore:o,useTransition:l}}},$="18.3.1";function G(n){return t.createElement.bind(null,n)}function J(n){return!!n&&n.$$typeof===k}function K(n){return J(n)&&n.type===t.Fragment}function Q(n){return!!n&&!!n.displayName&&("string"==typeof n.displayName||n.displayName instanceof String)&&n.displayName.startsWith("Memo(")}function X(n){return J(n)?t.cloneElement.apply(null,arguments):n}function nn(n){return!!n.__k&&(t.render(null,n),!0)}function tn(n){return n&&(n.base||1===n.nodeType&&n)||null}var en=function(n,t){return n(t)},rn=function(n,t){return n(t)},un=t.Fragment,on=J,cn={useState:e.useState,useId:e.useId,useReducer:e.useReducer,useEffect:e.useEffect,useLayoutEffect:e.useLayoutEffect,useInsertionEffect:a,useTransition:l,useDeferredValue:f,useSyncExternalStore:o,startTransition:c,useRef:e.useRef,useImperativeHandle:e.useImperativeHandle,useMemo:e.useMemo,useCallback:e.useCallback,useContext:e.useContext,useDebugValue:e.useDebugValue,version:$,Children:b,render:L,hydrate:F,unmountComponentAtNode:nn,createPortal:T,createElement:t.createElement,createContext:t.createContext,createFactory:G,cloneElement:X,createRef:t.createRef,Fragment:t.Fragment,isValidElement:J,isElement:on,isFragment:K,isMemo:Q,findDOMNode:tn,Component:t.Component,PureComponent:s,memo:h,forwardRef:p,flushSync:rn,unstable_batchedUpdates:en,StrictMode:un,Suspense:E,SuspenseList:O,lazy:x,__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED:Y};Object.defineProperty(n,"Component",{enumerable:!0,get:function(){return t.Component}}),Object.defineProperty(n,"Fragment",{enumerable:!0,get:function(){return t.Fragment}}),Object.defineProperty(n,"createContext",{enumerable:!0,get:function(){return t.createContext}}),Object.defineProperty(n,"createElement",{enumerable:!0,get:function(){return t.createElement}}),Object.defineProperty(n,"createRef",{enumerable:!0,get:function(){return t.createRef}}),n.Children=b,n.PureComponent=s,n.StrictMode=un,n.Suspense=E,n.SuspenseList=O,n.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED=Y,n.cloneElement=X,n.createFactory=G,n.createPortal=T,n.default=cn,n.findDOMNode=tn,n.flushSync=rn,n.forwardRef=p,n.hydrate=F,n.isElement=on,n.isFragment=K,n.isMemo=Q,n.isValidElement=J,n.lazy=x,n.memo=h,n.render=L,n.startTransition=c,n.unmountComponentAtNode=nn,n.unstable_batchedUpdates=en,n.useDeferredValue=f,n.useInsertionEffect=a,n.useSyncExternalStore=o,n.useTransition=l,n.version=$,Object.keys(e).forEach(function(t){"default"===t||n.hasOwnProperty(t)||Object.defineProperty(n,t,{enumerable:!0,get:function(){return e[t]}})})});
//# sourceMappingURL=compat.umd.js.map

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

/* ===== htm.js ===== */
!function(n,e){"object"==typeof exports&&"undefined"!=typeof module?module.exports=e():"function"==typeof define&&define.amd?define(e):n.htm=e()}(this,function(){var n=function(e,t,u,s){var r;t[0]=0;for(var p=1;p<t.length;p++){var h=t[p++],o=t[p]?(t[0]|=h?1:2,u[t[p++]]):t[++p];3===h?s[0]=o:4===h?s[1]=Object.assign(s[1]||{},o):5===h?(s[1]=s[1]||{})[t[++p]]=o:6===h?s[1][t[++p]]+=o+"":h?(r=e.apply(o,n(e,o,u,["",null])),s.push(r),o[0]?t[0]|=2:(t[p-2]=0,t[p]=r)):s.push(o)}return s},e=new Map;return function(t){var u=e.get(this);return u||(u=new Map,e.set(this,u)),(u=n(this,u.get(t)||(u.set(t,u=function(n){for(var e,t,u=1,s="",r="",p=[0],h=function(n){1===u&&(n||(s=s.replace(/^\s*\n\s*|\s*\n\s*$/g,"")))?p.push(0,n,s):3===u&&(n||s)?(p.push(3,n,s),u=2):2===u&&"..."===s&&n?p.push(4,n,0):2===u&&s&&!n?p.push(5,0,!0,s):u>=5&&((s||!n&&5===u)&&(p.push(u,0,s,t),u=6),n&&(p.push(u,n,0,t),u=6)),s=""},o=0;o<n.length;o++){o&&(1===u&&h(),h(o));for(var f=0;f<n[o].length;f++)e=n[o][f],1===u?"<"===e?(h(),p=[p],u=3):s+=e:4===u?"--"===s&&">"===e?(u=1,s=""):s=e+s[0]:r?e===r?r="":s+=e:'"'===e||"'"===e?r=e:">"===e?(h(),u=1):u&&("="===e?(u=5,t=s,s=""):"/"===e&&(u<5||">"===n[o][f+1])?(h(),3===u&&(p=p[0]),u=p,(p=p[0]).push(2,0,u),u=0):" "===e||"\t"===e||"\n"===e||"\r"===e?(h(),u=2):s+=e),3===u&&"!--"===s&&(u=4,p=p[0])}return h(),p}(t)),u),arguments,[])).length>1?u:u[0]}});

/* ===== app.js ===== */
/* clashx Web UI
 *
 * 零构建：htm 的 tagged template + Preact，源码用 include_str! 进二进制。
 * 布局对齐 OpenClash：侧边栏 + 主内容区。
 *
 * 依赖都在 bundle 里（preact UMD + hooks + compat + shim + htm），
 * 加载顺序必须是 preact -> preactHooks -> preactCompat -> shim -> htm -> app。
 * 三个前端坑都记在 build_bundle.sh 的注释里。
 */
(function () {
  'use strict';

  // ★ 一律显式从 window 取，不用裸标识符 —— 严格模式/模块作用域/
  //   测试沙箱里裸标识符都可能拿不到。
  var R = window.React;
  if (!R) { console.error('[clashx] React 未就绪，bundle 顺序或 shim 有问题'); return; }
  var RD = window.ReactDOM;

  var html = window.htm.bind(R.createElement);
  var useState = R.useState;
  var useEffect = R.useEffect;
  var useCallback = R.useCallback;

  // ---- API ----
  function api(path, opts) {
    opts = opts || {};
    return window.fetch(path, {
      method: opts.method || 'GET',
      headers: opts.body ? { 'Content-Type': 'application/json' } : {},
      body: opts.body ? JSON.stringify(opts.body) : undefined,
    }).then(function (r) {
      return r.text().then(function (t) {
        var d;
        try { d = JSON.parse(t); } catch (e) { d = { raw: t }; }
        if (!r.ok) throw new Error(d.error || ('HTTP ' + r.status));
        return d;
      });
    });
  }

  // ---- 全局提示 ----
  var MsgCtx = R.createContext(function () {});
  function useMsg() { return R.useContext(MsgCtx); }

  function MsgHost(props) {
    var s = useState(null);
    var show = useCallback(function (kind, text) {
      s[1](text ? { kind: kind, text: text } : null);
    }, []);
    return html`<${MsgCtx.Provider} value=${show}>
      ${s[0] ? html`<div class="msg ${s[0].kind}">${s[0].text}</div>` : ''}
      ${props.children}
    <//>`;
  }

  function Badge(p) {
    return html`<span class="badge ${p.kind || 'dim'}">${p.children}</span>`;
  }

  function fmtBytes(n) {
    n = n || 0;
    if (n < 1024) return n + ' B';
    if (n < 1048576) return (n / 1024).toFixed(1) + ' KB';
    if (n < 1073741824) return (n / 1048576).toFixed(1) + ' MB';
    return (n / 1073741824).toFixed(2) + ' GB';
  }
  function fmtRate(bps) { return fmtBytes(bps) + '/s'; }

  // ============ 侧边栏导航（分组对齐 OpenClash）============
  var NAV = [
    { group: '概览', items: [{ id: 'overview', name: '总览' }] },
    { group: '代理', items: [
      { id: 'mode', name: '运行模式' },
      { id: 'gateway', name: '旁路由' },
    ]},
    { group: '配置管理', items: [
      { id: 'rules', name: '规则' },
      { id: 'subs', name: '订阅' },
      { id: 'config', name: '配置文件' },
    ]},
    { group: '系统', items: [
      { id: 'service', name: '服务与诊断' },
      { id: 'password', name: '密码' },
    ]},
  ];

  // ============ 总览（对齐 OpenClash status.htm 五张卡）============
  function Overview() {
    var ov = useState(null);
    var traffic = useState({ up: 0, down: 0, upTotal: 0, downTotal: 0 });
    var busy = useState('');
    var diag = useState(null);
    var msg = useMsg();
    var d = ov[0];

    var load = useCallback(function () {
      api('/api/clashx/overview').then(ov[1]).catch(function (e) {
        msg('err', String(e.message || e));
      });
    }, []);
    useEffect(load, []);

    // 流量轮询（mihomo 的 /traffic 每 1.5 秒取一次足够）
    useEffect(function () {
      var stop = false;
      var tick = null;
      function poll() {
        if (stop) return;
        api('/api/mihomo/traffic').then(function (r) {
          if (stop) return;
          // 后端把流式响应当多行 JSON 转回来了，取第一行完整的
          if (r && r.up !== undefined) {
            traffic[1]({ up: r.up, down: r.down,
                         upTotal: r.upTotal, downTotal: r.downTotal });
          }
          tick = setTimeout(poll, 1500);
        }).catch(function () {
          if (!stop) tick = setTimeout(poll, 3000);
        });
      }
      poll();
      return function () { stop = true; if (tick) clearTimeout(tick); };
    }, []);

    function act(name, fn, okMsg) {
      busy[1](name);
      fn().then(function () { msg('ok', okMsg); load(); })
        .catch(function (e) { msg('err', String(e.message || e)); })
        .then(function () { busy[1](''); });
    }

    var b = busy[0];

    return html`
      <div class="card status-card ${d && !d.mihomo_up ? 'down' : ''}">
        <div class="status-strip"></div>
        <div class="status-main">
          <div class="status-title">
            <strong>内核</strong>
            ${d && d.mihomo_up
              ? html`<${Badge} kind="ok">运行中<//>`
              : html`<${Badge} kind="err">已停止<//>`}
            ${d && d.mihomo_version
              ? html`<span class="mono" style="color:var(--text-dim)">${(d.mihomo_version.match(/"version":"([^"]+)"/) || [, ''])[1]}</span>`
              : ''}
          </div>
          <div class="status-sub">
            ${d ? d.node_count + ' 个节点 · ' + d.rule_count + ' 条规则 · ' + d.sub_count + ' 个订阅' : '加载中…'}
          </div>
        </div>
        <div class="status-actions">
          <button class="btn danger" disabled=${!!b}
            onClick=${function () {
              if (!confirm('停止内核？期间所有代理流量会中断。')) return;
              act('stop', function () { return api('/api/clashx/service/stop', { method: 'POST' }); }, '内核已停止');
            }}>停止</button>
          <button class="btn" disabled=${!!b}
            onClick=${function () {
              act('start', function () { return api('/api/clashx/service/start', { method: 'POST' }); }, '内核已启动');
            }}>启动</button>
          <button class="btn" disabled=${!!b}
            onClick=${function () {
              act('restart', function () { return api('/api/clashx/service/restart', { method: 'POST' }); }, '内核已重启');
            }}>重启</button>
        </div>
      </div>

      <div class="card">
        <div class="card-head">
          <h2>运行模式</h2>
          <span class="spacer"></span>
          ${d ? html`<${Badge} kind="accent">${d.run_mode}<//>` : ''}
        </div>
        <div class="card-body tight">
          <div class="grid">
            <div class="stat"><div class="k">DNS</div><div class="v" style="font-size:15px">${d ? d.dns_mode : '—'}</div></div>
            <div class="stat"><div class="k">TUN</div><div class="v" style="font-size:15px">${d ? (d.tun ? '启用' : '关闭') : '—'}</div></div>
            <div class="stat"><div class="k">防火墙</div><div class="v" style="font-size:15px">${d ? d.firewall : '—'}</div></div>
            <div class="stat"><div class="k">旁路由</div><div class="v" style="font-size:15px">${d ? (d.gateway ? '已启用' : '未启用') : '—'}</div></div>
          </div>
          <div class="btnrow" style="margin-top:12px">
            <a class="btn sm" href="#/mode">切换模式</a>
            <a class="btn sm" href="#/gateway">旁路由设置</a>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="card-head">
          <h2>面板</h2>
          <span class="spacer"></span>
          <span class="hint tight">节点选择与连接查看在这里完成</span>
        </div>
        <div class="card-body tight">
          <div class="btnrow">
            <a class="btn primary" href="/zashboard/" target="_blank" rel="noopener">zashboard</a>
            <span class="hint tight">独立面板，独立鉴权</span>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="card-head">
          <h2>快捷操作</h2>
          <span class="spacer"></span>
          ${diag[0] ? html`<${Badge} kind="dim">${diag[0]}<//>` : ''}
        </div>
        <div class="card-body tight">
          <div class="btnrow">
            <button class="btn sm" disabled=${!!b}
              onClick=${function () {
                act('dns', function () { return api('/api/clashx/dns/flush', { method: 'POST' }); }, 'DNS 缓存已刷新');
              }}>刷新 DNS</button>
            <button class="btn sm" disabled=${!!b}
              onClick=${function () {
                if (!confirm('关闭全部连接？正在传输的连接会中断。')) return;
                act('conn', function () { return api('/api/mihomo/connections', { method: 'DELETE' }); }, '已关闭全部连接');
              }}>关闭全部连接</button>
            <button class="btn sm" disabled=${!!b}
              onClick=${function () {
                if (!confirm('立即抓取所有订阅并重新生成配置？')) return;
                act('sub', function () { return api('/api/clashx/sub/update', { method: 'POST' }); }, '订阅已更新');
              }}>更新订阅</button>
            <button class="btn sm" disabled=${!!b}
              onClick=${function () {
                busy[1]('d');
                api('/api/clashx/diag/dns').then(function (r) {
                  var items = r.items || [];
                  diag[1](items.filter(function (i) { return i.ok; }).length + '/' +
                           items.length + ' 解析正常');
                }).catch(function (e) { msg('err', String(e.message || e)); })
                  .then(function () { busy[1](''); });
              }}>DNS 诊断</button>
            <button class="btn sm" disabled=${!!b}
              onClick=${function () {
                busy[1]('c');
                api('/api/clashx/diag/conn').then(function (r) {
                  diag[1](r.connections + ' 连接 · 内核 ' +
                           Math.round(r.core_rss_kb / 1024) + ' MB');
                }).catch(function (e) { msg('err', String(e.message || e)); })
                  .then(function () { busy[1](''); });
              }}>连接诊断</button>
            <button class="btn sm" onClick=${function () { load(); }}>刷新</button>
          </div>
        </div>
      </div>

      <div class="card">
        <div class="card-head">
          <h2>流量统计</h2>
          <span class="spacer"></span>
          <span class="hint tight">每 1.5 秒刷新</span>
        </div>
        <div class="card-body">
          <div class="traffic-bars">
            <div class="tbar">
              <span class="lbl">下载</span>
              <span class="track"><span class="fill down"
                style=${'width:' + Math.min(100, (traffic[0].down / 1048576) * 100) + '%'}></span></span>
              <span class="val">${fmtRate(traffic[0].down)}</span>
            </div>
            <div class="tbar">
              <span class="lbl">上传</span>
              <span class="track"><span class="fill up"
                style=${'width:' + Math.min(100, (traffic[0].up / 1048576) * 100) + '%'}></span></span>
              <span class="val">${fmtRate(traffic[0].up)}</span>
            </div>
          </div>
          <div class="grid" style="margin-top:14px">
            <div class="stat"><div class="k">累计下载</div><div class="v" style="font-size:16px">${fmtBytes(traffic[0].downTotal)}</div></div>
            <div class="stat"><div class="k">累计上传</div><div class="v" style="font-size:16px">${fmtBytes(traffic[0].upTotal)}</div></div>
            <div class="stat"><div class="k">活动连接</div><div class="v" style="font-size:16px">${d ? d.mihomo_connections : '—'}</div></div>
          </div>
        </div>
      </div>`;
  }

  // ============ 运行模式 ============
  function RunMode() {
    var s = useState(null);
    var d = s[0];
    var msg = useMsg();
    useEffect(function () {
      api('/api/clashx/run-mode').then(s[1]).catch(function (e) { msg('err', String(e.message || e)); });
    }, []);

    function apply(v) {
      msg('info', '正在切换到 ' + v + ' …');
      api('/api/clashx/run-mode/set?mode=' + encodeURIComponent(v))
        .then(function (r) { msg('ok', '已切换：' + r.from + ' → ' + r.to); s[1](null); return api('/api/clashx/run-mode'); })
        .then(s[1])
        .catch(function (e) { msg('err', String(e.message || e)); });
    }

    return html`
      <div class="card">
        <div class="card-head"><h2>运行模式</h2></div>
        <div class="card-body">
          <p class="hint tight">
            模式 = DNS 解析方式（fake-ip / redir-host）× 防火墙接管方。
            切换时会自动校准 nftables 规则并重启内核，约 1 秒。
          </p>
          <div class="modes">
            ${(d ? d.all : []).map(function (m) {
              return html`<button class="mode" aria-pressed=${d.current === m.value}
                onClick=${function () { apply(m.value); }}>
                <div class="n">${m.value}</div>
                <div class="d">DNS ${m.dns} · ${m.tun ? 'TUN' : '传统'} · ${m.firewall}</div>
              </button>`;
            })}
          </div>
          ${!d ? html`<div class="spin">加载中…</div>` : ''}
        </div>
      </div>`;
  }

  // ============ 旁路由 ============
  function Gateway() {
    var s = useState(null);
    var ifaces = useState([]);
    // 网口绑定 + 地址都允许用户改，探测只作为默认值。
    // 多网卡机器（双 WAN、管理口分离）上猜错网卡 = 规则打错网段 =
    // 流量不回环但也不工作 —— 探测只是省事，不能当唯一来源。
    var iface = useState('');
    var lanAddr = useState('');
    var clients = useState('');
    var d = s[0];
    var busy = useState('');
    var msg = useMsg();

    var refresh = useCallback(function () {
      api('/api/clashx/gateway').then(s[1]).catch(function (e) { msg('err', String(e.message || e)); });
    }, []);
    useEffect(refresh, []);

    // 拉网卡列表并预填
    useEffect(function () {
      api('/api/clashx/gateway/ifaces').then(function (r) {
        var list = r.ifaces || [];
        ifaces[1](list);
        // 预填：优先选默认路由所在网卡
        if (list.length) {
          var pick = list[0];
          api('/api/clashx/overview').then(function () {}).catch(function () {});
          iface[1](pick.name);
          lanAddr[1](pick.addr);
          // 客户端网段默认取该网卡所在 /24
          if (pick.cidr && pick.cidr.includes('/')) {
            var bits = pick.cidr.split('/')[1];
            if (bits === '24') {
              var p3 = pick.addr.split('.').slice(0, 3).join('.');
              clients[1](p3 + '.0/24');
            }
          }
        }
      }).catch(function (e) { msg('err', String(e.message || e)); });
    }, []);

    // 换网口时同步更新地址与客户端网段
    function onIfaceChange(name) {
      iface[1](name);
      var f = ifaces[0].filter(function (x) { return x.name === name; })[0];
      if (f) {
        lanAddr[1](f.addr);
        if (f.cidr && f.cidr.endsWith('/24')) {
          clients[1](f.addr.split('.').slice(0, 3).join('.') + '.0/24');
        }
      }
    }

    function call(path, okMsg) {
      busy[1]('1');
      var body = { clients: clients[0] };
      // 用户显式填了才传 —— 留空则由后端探测
      if (iface[0]) body.iface = iface[0];
      if (lanAddr[0]) body.lan_addr = lanAddr[0];
      api(path, { method: 'POST', body: body })
        .then(function (r) {
          msg('ok', r.msg || okMsg);
          refresh();
        })
        .catch(function (e) {
          // 后端预检失败时带回探测信息，直接展示给用户
          var msgText = String(e.message || e);
          api('/api/clashx/gateway/ifaces').then(function (r) {
            ifaces[1](r.ifaces || []);
          }).catch(function () {});
          msg('err', msgText);
        })
        .then(function () { busy[1](''); });
    }

    return html`
      <div class="card">
        <div class="card-head">
          <h2>旁路由（网关模式）</h2>
          <span class="spacer"></span>
          ${d ? html`<${Badge} kind=${d.active ? 'ok' : 'dim'}>${d.active ? '已启用' : '未启用'}<//>` : ''}
        </div>
        <div class="card-body">
          <p class="hint tight">
            启用后把客户端设备的网关指向下面的「网关地址」即可。
            仅对<strong>传统模式</strong>有效（TUN 模式由 mihomo 自己接管）。
          </p>
          <label class="field">
            <span>网口绑定</span>
            <select value=${iface[0]} onChange=${function (e) { onIfaceChange(e.target.value); }}>
              <option value="">自动探测（默认路由网卡）</option>
              ${ifaces[0].map(function (f) {
                return html`<option key=${f.name} value=${f.name}>
                  ${f.name} — ${f.addr}${f.state && f.state !== 'UP' ? '（' + f.state + '）' : ''}
                </option>`;
              })}
            </select>
          </label>
          <label class="field">
            <span>网关地址（本机的局域网地址，客户端要指向它）</span>
            <input class="mono" type="text" value=${lanAddr[0]}
              onInput=${function (e) { lanAddr[1](e.target.value); }}
              placeholder="留空则自动探测" />
          </label>
          <label class="field">
            <span>客户端网段（逗号分隔，必填）</span>
            <input class="mono" type="text" value=${clients[0]}
              onInput=${function (e) { clients[1](e.target.value); }}
              placeholder="192.168.10.0/24" />
            <div class="hint">
              只有这些网段的设备会被代理。填错会导致「网关指向了但流量不走代理」。
            </div>
          </label>
          <div class="btnrow">
            <button class="btn primary" disabled=${!!busy[0]}
              onClick=${function () { call('/api/clashx/gateway/enable', '旁路由已启用'); }}>启用</button>
            <button class="btn danger" disabled=${!!busy[0]}
              onClick=${function () { call('/api/clashx/gateway/disable', '旁路由已停用'); }}>停用</button>
            <button class="btn sm" disabled=${!!busy[0]}
              onClick=${function () {
                busy[1]('1');
                api('/api/mihomo/connections', { method: 'DELETE' })
                  .then(function () { msg('ok', '已关闭全部连接'); })
                  .catch(function (e) { msg('err', String(e.message || e)); })
                  .then(function () { busy[1](''); });
              }}>关闭全部连接</button>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="card-head"><h2>防火墙状态</h2></div>
        <div class="card-body"><pre class="out">${d ? (d.detail || []).join('\n') : '加载中…'}</pre></div>
      </div>`;
  }

  // ============ 规则 ============
  function Rules() {
    var s = useState(null);
    var add = useState('');
    var test = useState('www.google.com');
    var verdict = useState(null);
    var d = s[0];
    var msg = useMsg();

    var refresh = useCallback(function () {
      api('/api/clashx/rules').then(s[1]).catch(function (e) { msg('err', String(e.message || e)); });
    }, []);
    useEffect(refresh, []);

    function doAdd() {
      var r = add[0].trim();
      if (!r) return msg('err', '请输入规则');
      api('/api/clashx/rules/add', { method: 'POST', body: { rule: r } })
        .then(function () { msg('ok', '已添加并生效'); add[1](''); refresh(); })
        .catch(function (e) { msg('err', String(e.message || e)); });
    }
    function doRm(r) {
      if (!confirm('删除这条规则？\n\n' + r)) return;
      api('/api/clashx/rules/rm', { method: 'POST', body: { rule: r } })
        .then(function () { msg('ok', '已删除并生效'); refresh(); })
        .catch(function (e) { msg('err', String(e.message || e)); });
    }
    function doTest() {
      api('/api/clashx/rules/test', { method: 'POST', body: { target: test[0] } })
        .then(verdict[1])
        .catch(function (e) { msg('err', String(e.message || e)); });
    }

    return html`
      <div class="card">
        <div class="card-head">
          <h2>规则链</h2>
          <span class="spacer"></span>
          <span class="mono hint tight">${d ? d.rules.length : 0} 条</span>
        </div>
        <div class="card-body">
          <ul class="list">
            ${(d ? d.rules : []).map(function (r, i) {
              return html`<li key=${i}>
                <span class="mono grow" title=${r}>${i + 1}. ${r}</span>
                <button class="btn sm danger" onClick=${function () { doRm(r); }}>删</button>
              </li>`;
            })}
          </ul>
          ${d && !d.rules.length ? html`<div class="empty">还没有规则</div>` : ''}
        </div>
      </div>
      <div class="card">
        <div class="card-head"><h2>添加规则</h2></div>
        <div class="card-body">
          <p class="hint tight">
            内核原生语法，单行标量。例如 <code>DOMAIN-SUFFIX,example.com,PROXY</code>。
            添加时会插到 MATCH 之前并立即生效。
          </p>
          <label class="field">
            <span>规则</span>
            <input class="mono" type="text" value=${add[0]}
              onInput=${function (e) { add[1](e.target.value); }}
              placeholder="GEOIP,CN,DIRECT" />
          </label>
          <div class="btnrow"><button class="btn primary" onClick=${doAdd}>添加</button></div>
        </div>
      </div>
      <div class="card">
        <div class="card-head"><h2>规则测试器（离线）</h2></div>
        <div class="card-body">
          <p class="hint tight">不发包，按规则链顺序判断某个目标会命中哪条。调试分流问题用这个。</p>
          <label class="field">
            <span>目标（域名或 IP，可带端口）</span>
            <input class="mono" type="text" value=${test[0]}
              onInput=${function (e) { test[1](e.target.value); }} />
          </label>
          <div class="btnrow"><button class="btn" onClick=${doTest}>测试</button></div>
          ${verdict[0] ? html`
            <div class="msg ok" style="margin-top:12px">
              命中第 <strong>${verdict[0].index}</strong> 条：<code>${verdict[0].rule}</code><br/>
              类型 <code>${verdict[0].kind}</code> → 策略 <code>${verdict[0].target}</code>
              ${verdict[0].fallback ? html`<br/><strong>（MATCH 兜底，说明前面都没命中）</strong>` : ''}
            </div>` : ''}
        </div>
      </div>`;
  }

  // ============ 订阅 ============
  function Subs() {
    var s = useState(null);
    var name = useState('');
    var url = useState('');
    var d = s[0];
    var busy = useState('');
    var msg = useMsg();

    var refresh = useCallback(function () {
      api('/api/clashx/subs').then(s[1]).catch(function (e) { msg('err', String(e.message || e)); });
    }, []);
    useEffect(refresh, []);

    function doAdd() {
      if (!name[0].trim() || !url[0].trim()) return msg('err', '名称与 URL 必填');
      api('/api/clashx/subs/add', { method: 'POST', body: { name: name[0], url: url[0] } })
        .then(function () { msg('ok', '已保存'); name[1](''); url[1](''); refresh(); })
        .catch(function (e) { msg('err', String(e.message || e)); });
    }
    function doRm(n) {
      if (!confirm('删除订阅 ' + n + '？')) return;
      api('/api/clashx/subs/rm', { method: 'POST', body: { name: n } })
        .then(function () { msg('ok', '已删除'); refresh(); })
        .catch(function (e) { msg('err', String(e.message || e)); });
    }
    function doUpdate() {
      if (!confirm('立即抓取所有订阅并重新生成配置？')) return;
      busy[1]('u');
      api('/api/clashx/sub/update', { method: 'POST' })
        .then(function (r) { msg('ok', r.msg || '已更新'); refresh(); })
        .catch(function (e) { msg('err', String(e.message || e)); })
        .then(function () { busy[1](''); });
    }

    return html`
      <div class="card">
        <div class="card-head">
          <h2>订阅</h2>
          <span class="spacer"></span>
          <button class="btn sm" disabled=${!!busy[0]} onClick=${doUpdate}>全部更新</button>
        </div>
        <div class="card-body">
          <ul class="list">
            ${(d ? d.subs : []).map(function (sb) {
              return html`<li key=${sb.name}>
                <div class="grow">
                  <div>${sb.name}</div>
                  <div class="sub mono" title=${sb.url}>${sb.url}</div>
                </div>
                <${Badge} kind="dim">${sb.user_agent || 'clash.meta'}<//>
                <span class="sub">${sb.update_interval}h</span>
                <button class="btn sm danger" onClick=${function () { doRm(sb.name); }}>删</button>
              </li>`;
            })}
          </ul>
          ${d && !d.subs.length ? html`<div class="empty">还没有订阅</div>` : ''}
        </div>
      </div>
      <div class="card">
        <div class="card-head"><h2>添加订阅</h2></div>
        <div class="card-body">
          <p class="hint tight">
            URL 会在界面上打码显示，但完整地址存在服务器配置文件里（已受 Basic Auth 保护）。
            抓取失败时自动回退本地缓存，不会阻塞配置生成。
          </p>
          <label class="field"><span>名称</span>
            <input type="text" value=${name[0]} onInput=${function (e) { name[1](e.target.value); }}
              placeholder="my-provider" /></label>
          <label class="field"><span>订阅地址</span>
            <input class="mono" type="text" value=${url[0]} onInput=${function (e) { url[1](e.target.value); }}
              placeholder="https://…" /></label>
          <div class="btnrow"><button class="btn primary" onClick=${doAdd}>添加</button></div>
        </div>
      </div>`;
  }

  // ============ 服务与诊断 ============
  function ServicePage() {
    var diag = useState(null);
    var busy = useState('');
    var msg = useMsg();

    function call(path, okMsg) {
      busy[1]('1');
      api(path, { method: 'POST' })
        .then(function () { msg('ok', okMsg); })
        .catch(function (e) { msg('err', String(e.message || e)); })
        .then(function () { busy[1](''); });
    }
    function doDnsDiag() {
      busy[1]('d');
      api('/api/clashx/diag/dns').then(function (r) { diag[1]({ kind: 'dns', data: r.items || [] }); })
        .catch(function (e) { msg('err', String(e.message || e)); })
        .then(function () { busy[1](''); });
    }
    function doConnDiag() {
      busy[1]('c');
      api('/api/clashx/diag/conn').then(function (r) { diag[1]({ kind: 'conn', data: r }); })
        .catch(function (e) { msg('err', String(e.message || e)); })
        .then(function () { busy[1](''); });
    }

    var d = diag[0];

    return html`
      <div class="card">
        <div class="card-head"><h2>服务控制</h2></div>
        <div class="card-body">
          <div class="btnrow">
            <button class="btn" disabled=${!!busy[0]}
              onClick=${function () { call('/api/clashx/service/start', '已启动'); }}>启动内核</button>
            <button class="btn danger" disabled=${!!busy[0]}
              onClick=${function () { if (confirm('停止内核？')) call('/api/clashx/service/stop', '已停止'); }}>停止内核</button>
            <button class="btn" disabled=${!!busy[0]}
              onClick=${function () { call('/api/clashx/service/restart', '已重启'); }}>重启内核</button>
            <span style="flex:1"></span>
            <button class="btn sm" disabled=${!!busy[0]}
              onClick=${function () { call('/api/clashx/dns/flush', 'DNS 缓存已刷新'); }}>刷新 DNS</button>
            <button class="btn sm" disabled=${!!busy[0]}
              onClick=${function () {
                if (!confirm('关闭全部连接？')) return;
                api('/api/mihomo/connections', { method: 'DELETE' })
                  .then(function () { msg('ok', '已关闭全部连接'); })
                  .catch(function (e) { msg('err', String(e.message || e)); });
              }}>关闭全部连接</button>
          </div>
        </div>
      </div>
      <div class="card">
        <div class="card-head">
          <h2>诊断</h2>
          <span class="spacer"></span>
          <button class="btn sm" disabled=${!!busy[0]} onClick=${doConnDiag}>连接诊断</button>
          <button class="btn sm" disabled=${!!busy[0]} onClick=${doDnsDiag}>DNS 诊断</button>
        </div>
        <div class="card-body">
          ${!d ? html`<div class="empty">点上方按钮运行诊断</div>` : ''}
          ${d && d.kind === 'conn' ? html`
            <div class="grid">
              <div class="stat"><div class="k">活动连接</div><div class="v">${d.data.connections}</div></div>
              <div class="stat"><div class="k">累计下载</div><div class="v" style="font-size:16px">${fmtBytes(d.data.download_total)}</div></div>
              <div class="stat"><div class="k">累计上传</div><div class="v" style="font-size:16px">${fmtBytes(d.data.upload_total)}</div></div>
              <div class="stat"><div class="k">内核内存</div><div class="v" style="font-size:16px">${Math.round(d.data.core_rss_kb / 1024)} <small>MB</small></div></div>
              <div class="stat"><div class="k">内核线程</div><div class="v" style="font-size:16px">${d.data.core_threads}</div></div>
            </div>` : ''}
          ${d && d.kind === 'dns' ? html`
            ${d.data.map(function (it) {
              var ans = '';
              if (it.ok) {
                try {
                  var j = JSON.parse(it.result);
                  var a = (j.Answer || []).filter(function (x) { return x.type === 1; });
                  ans = a.length ? a.map(function (x) { return x.data; }).join(', ')
                                  : ((j.Answer || [])[0] || {}).data || '';
                } catch (e) { ans = it.result; }
              }
              return html`<div class="diag-item" key=${it.name}>
                <span class="name">${it.name}</span>
                <${Badge} kind=${it.ok ? 'ok' : 'err'}>${it.ok ? '正常' : '失败'}<//>
                <span class="grow mono">${ans}</span>
              </div>`;
            })}` : ''}
        </div>
      </div>`;
  }

  // ============ 配置文件 ============
  function ConfigPage() {
    var uc = useState(null);
    var rc = useState(null);
    useEffect(function () {
      api('/api/clashx/config').then(function (t) { uc[1](t.raw || ''); })
        .catch(function (e) { uc[1]('（读取失败：' + e.message + '）'); });
      api('/api/clashx/rendered').then(function (t) { rc[1](t.raw || ''); })
        .catch(function (e) { rc[1]('（读取失败：' + e.message + '）'); });
    }, []);
    return html`
      <div class="card">
        <div class="card-head"><h2>渲染产物（config.gen.yaml）</h2></div>
        <div class="card-body">
          <p class="hint tight">clashx 生成、交给 mihomo 的实际配置，含订阅展开后的节点。</p>
          <pre class="out">${rc[0] || '加载中…'}</pre>
        </div>
      </div>
      <div class="card">
        <div class="card-head"><h2>用户配置（config.yaml）</h2></div>
        <div class="card-body">
          <p class="hint tight">只读。修改请用命令行编辑后执行 apply。</p>
          <pre class="out">${uc[0] || '加载中…'}</pre>
        </div>
      </div>`;
  }

  // ============ 密码 ============
  function PasswordPage() {
    var old = useState('');
    var nw = useState('');
    var nw2 = useState('');
    var msg = useMsg();

    function change() {
      if (nw[0] !== nw2[0]) return msg('err', '两次输入的新密码不一致');
      api('/api/clashx/password', {
        method: 'POST',
        body: { old_password: old[0], new_password: nw[0] },
      })
        .then(function (r) {
          msg('ok', r.msg || '已修改，重新登录后生效');
          old[1](''); nw[1](''); nw2[1]('');
        })
        .catch(function (e) { msg('err', String(e.message || e)); });
    }

    return html`
      <div class="card">
        <div class="card-head"><h2>修改登录密码</h2></div>
        <div class="card-body">
          <p class="hint tight">
            密码以 SHA-256 哈希存储，配置文件里没有明文。默认 admin/admin，
            首次登录后请立即修改。
          </p>
          <label class="field"><span>当前密码</span>
            <input type="password" value=${old[0]} onInput=${function (e) { old[1](e.target.value); }} /></label>
          <label class="field"><span>新密码（至少 4 个字符）</span>
            <input type="password" value=${nw[0]} onInput=${function (e) { nw[1](e.target.value); }} /></label>
          <label class="field"><span>确认新密码</span>
            <input type="password" value=${nw2[0]} onInput=${function (e) { nw2[1](e.target.value); }} /></label>
          <div class="btnrow"><button class="btn primary" onClick=${change}>修改</button></div>
        </div>
      </div>`;
  }

  // ============ 根组件 ============
  var PAGES = {
    overview: Overview,
    mode: RunMode,
    gateway: Gateway,
    rules: Rules,
    subs: Subs,
    service: ServicePage,
    config: ConfigPage,
    password: PasswordPage,
  };
  var TITLES = {
    overview: '总览', mode: '运行模式', gateway: '旁路由',
    rules: '规则', subs: '订阅', service: '服务与诊断',
    config: '配置文件', password: '密码',
  };

  function currentPage() {
    var h = (window.location.hash || '').replace(/^#\/?/, '');
    return PAGES[h] ? h : 'overview';
  }

  function Sidebar(props) {
    var ov = useState(null);
    // 侧边栏状态点：定期刷新内核是否在跑
    useEffect(function () {
      var t = null;
      function load() {
        api('/api/clashx/overview').then(function (r) {
          ov[1]({ up: r.mihomo_up, gw: r.gateway });
        }).catch(function () {});
      }
      load();
      t = setInterval(load, 5000);
      return function () { if (t) clearInterval(t); };
    }, []);

    var s = ov[0] || {};
    return html`
      <aside class="sidebar">
        <div class="side-brand">
          <div class="name">clashx</div>
          <div class="ver">Linux 无桌面 mihomo 客户端</div>
        </div>
        <nav class="side-nav">
          ${NAV.map(function (g) {
            return html`<div key=${g.group}>
              <div class="side-group">${g.group}</div>
              ${g.items.map(function (it) {
                var dot = '';
                if (it.id === 'overview' && s.up !== undefined) {
                  dot = html`<span class=${'badge-dot' + (s.up ? '' : ' off')}></span>`;
                } else if (it.id === 'gateway' && s.gw !== undefined) {
                  dot = html`<span class=${'badge-dot' + (s.gw ? '' : ' off')}></span>`;
                }
                return html`<button key=${it.id} class="side-item"
                  aria-current=${props.page === it.id ? 'page' : null}
                  onClick=${function () { window.location.hash = '#/' + it.id; }}>
                  ${it.name}${dot}
                </button>`;
              })}
            </div>`;
          })}
        </nav>
        <div class="side-foot">
          <a class="btn sm" href="/zashboard/" target="_blank" rel="noopener">zashboard ↗</a>
        </div>
      </aside>`;
  }

  function App() {
    var pg = useState(currentPage());

    // hash 变化时切页（浏览器前进后退也生效）
    useEffect(function () {
      function onHash() { pg[1](currentPage()); }
      window.addEventListener('hashchange', onHash);
      return function () { window.removeEventListener('hashchange', onHash); };
    }, []);

    var Body = PAGES[pg[0]] || Overview;

    return html`
      <div class="shell">
        <${Sidebar} page=${pg[0]}/>
        <div class="main">
          <div class="topbar">
            <h1>${TITLES[pg[0]] || '总览'}</h1>
            <span class="spacer"></span>
            <a class="btn sm" href="/zashboard/" target="_blank" rel="noopener">面板 ↗</a>
          </div>
          <div class="content">
            <${MsgHost}><${Body}/><//>
          </div>
        </div>
      </div>`;
  }

  RD.createRoot(document.getElementById('root')).render(html`<${App}/>`);
  window.__clashx = { api: api };
})();
