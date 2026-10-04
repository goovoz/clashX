/* clashx Web UI bundle —— 由 webui/build.py 合并生成，请勿手改 */
/*加载顺序要求：preact -> preactHooks -> preactCompat -> shim -> htm -> app */

/* ===== preact.js ===== */
!function(){var n,l,t,u,r,e,i,o,f,c,s,a,h,p,y="http://www.w3.org/2000/svg",v="http://www.w3.org/1999/xhtml",d=void 0,_={},w=[],m=/acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i,g=Array.isArray;function b(n,l){for(var t in l)n[t]=l[t];return n}function k(n){n&&n.parentNode&&n.parentNode.removeChild(n)}function C(l,t,u){var r,e,i,o={};for(i in t)"key"==i?r=t[i]:"ref"==i?e=t[i]:o[i]=t[i];if(arguments.length>2&&(o.children=arguments.length>3?n.call(arguments,2):u),"function"==typeof l&&null!=l.defaultProps)for(i in l.defaultProps)o[i]===d&&(o[i]=l.defaultProps[i]);return x(l,o,r,e,null)}function x(n,u,r,e,i){var o={type:n,props:u,key:r,ref:e,__k:null,__:null,__b:0,__e:null,__c:null,constructor:d,__v:null==i?++t:i,__i:-1,__u:0};return null==i&&null!=l.vnode&&l.vnode(o),o}function S(n){return n.children}function M(n,l){this.props=n,this.context=l}function P(n,l){if(null==l)return n.__?P(n.__,n.__i+1):null;for(var t;l<n.__k.length;l++)if(null!=(t=n.__k[l])&&null!=t.__e)return t.__e;return"function"==typeof n.type?P(n):null}function $(n){var l,t;if(null!=(n=n.__)&&null!=n.__c){for(n.__e=n.__c.base=null,l=0;l<n.__k.length;l++)if(null!=(t=n.__k[l])&&null!=t.__e){n.__e=n.__c.base=t.__e;break}return $(n)}}function A(n){(!n.__d&&(n.__d=!0)&&r.push(n)&&!I.__r++||e!==l.debounceRendering)&&((e=l.debounceRendering)||i)(I)}function I(){var n,t,u,e,i,f,c,s;for(r.sort(o);n=r.shift();)n.__d&&(t=r.length,e=void 0,f=(i=(u=n).__v).__e,c=[],s=[],u.__P&&((e=b({},i)).__v=i.__v+1,l.vnode&&l.vnode(e),j(u.__P,e,i,u.__n,u.__P.namespaceURI,32&i.__u?[f]:null,c,null==f?P(i):f,!!(32&i.__u),s),e.__v=i.__v,e.__.__k[e.__i]=e,z(c,e,s),e.__e!=f&&$(e)),r.length>t&&r.sort(o));I.__r=0}function E(n,l,t,u,r,e,i,o,f,c,s){var a,h,p,y,v,m,g=u&&u.__k||w,b=l.length;for(f=F(t,l,g,f,b),a=0;a<b;a++)null!=(p=t.__k[a])&&(h=-1===p.__i?_:g[p.__i]||_,p.__i=a,m=j(n,p,h,r,e,i,o,f,c,s),y=p.__e,p.ref&&h.ref!=p.ref&&(h.ref&&R(h.ref,null,p),s.push(p.ref,p.__c||y,p)),null==v&&null!=y&&(v=y),4&p.__u||h.__k===p.__k?f=H(p,f,n):"function"==typeof p.type&&m!==d?f=m:y&&(f=y.nextSibling),p.__u&=-7);return t.__e=v,f}function F(n,l,t,u,r){var e,i,o,f,c,s=t.length,a=s,h=0;for(n.__k=new Array(r),e=0;e<r;e++)null!=(i=l[e])&&"boolean"!=typeof i&&"function"!=typeof i?(f=e+h,(i=n.__k[e]="string"==typeof i||"number"==typeof i||"bigint"==typeof i||i.constructor==String?x(null,i,null,null,null):g(i)?x(S,{children:i},null,null,null):i.constructor===d&&i.__b>0?x(i.type,i.props,i.key,i.ref?i.ref:null,i.__v):i).__=n,i.__b=n.__b+1,o=null,-1!==(c=i.__i=L(i,t,f,a))&&(a--,(o=t[c])&&(o.__u|=2)),null==o||null===o.__v?(-1==c&&h--,"function"!=typeof i.type&&(i.__u|=4)):c!=f&&(c==f-1?h--:c==f+1?h++:(c>f?h--:h++,i.__u|=4))):n.__k[e]=null;if(a)for(e=0;e<s;e++)null!=(o=t[e])&&0==(2&o.__u)&&(o.__e==u&&(u=P(o)),q(o,o));return u}function H(n,l,t){var u,r;if("function"==typeof n.type){for(u=n.__k,r=0;u&&r<u.length;r++)u[r]&&(u[r].__=n,l=H(u[r],l,t));return l}n.__e!=l&&(l&&n.type&&!t.contains(l)&&(l=P(n)),t.insertBefore(n.__e,l||null),l=n.__e);do{l=l&&l.nextSibling}while(null!=l&&8==l.nodeType);return l}function L(n,l,t,u){var r,e,i=n.key,o=n.type,f=l[t];if(null===f||f&&i==f.key&&o===f.type&&0==(2&f.__u))return t;if(u>(null!=f&&0==(2&f.__u)?1:0))for(r=t-1,e=t+1;r>=0||e<l.length;){if(r>=0){if((f=l[r])&&0==(2&f.__u)&&i==f.key&&o===f.type)return r;r--}if(e<l.length){if((f=l[e])&&0==(2&f.__u)&&i==f.key&&o===f.type)return e;e++}}return-1}function T(n,l,t){"-"==l[0]?n.setProperty(l,null==t?"":t):n[l]=null==t?"":"number"!=typeof t||m.test(l)?t:t+"px"}function O(n,l,t,u,r){var e;n:if("style"==l)if("string"==typeof t)n.style.cssText=t;else{if("string"==typeof u&&(n.style.cssText=u=""),u)for(l in u)t&&l in t||T(n.style,l,"");if(t)for(l in t)u&&t[l]===u[l]||T(n.style,l,t[l])}else if("o"==l[0]&&"n"==l[1])e=l!=(l=l.replace(f,"$1")),l=l.toLowerCase()in n||"onFocusOut"==l||"onFocusIn"==l?l.toLowerCase().slice(2):l.slice(2),n.l||(n.l={}),n.l[l+e]=t,t?u?t.t=u.t:(t.t=c,n.addEventListener(l,e?a:s,e)):n.removeEventListener(l,e?a:s,e);else{if(r==y)l=l.replace(/xlink(H|:h)/,"h").replace(/sName$/,"s");else if("width"!=l&&"height"!=l&&"href"!=l&&"list"!=l&&"form"!=l&&"tabIndex"!=l&&"download"!=l&&"rowSpan"!=l&&"colSpan"!=l&&"role"!=l&&"popover"!=l&&l in n)try{n[l]=null==t?"":t;break n}catch(n){}"function"==typeof t||(null==t||!1===t&&"-"!=l[4]?n.removeAttribute(l):n.setAttribute(l,"popover"==l&&1==t?"":t))}}function V(n){return function(t){if(this.l){var u=this.l[t.type+n];if(null==t.u)t.u=c++;else if(t.u<u.t)return;return u(l.event?l.event(t):t)}}}function j(n,t,u,r,e,i,o,f,c,s){var a,h,p,y,v,_,w,m,C,x,P,$,A,I,F,H,L,T=t.type;if(t.constructor!==d)return null;128&u.__u&&(c=!!(32&u.__u),i=[f=t.__e=u.__e]),(a=l.__b)&&a(t);n:if("function"==typeof T)try{if(m=t.props,C="prototype"in T&&T.prototype.render,x=(a=T.contextType)&&r[a.__c],P=a?x?x.props.value:a.__:r,u.__c?w=(h=t.__c=u.__c).__=h.__E:(C?t.__c=h=new T(m,P):(t.__c=h=new M(m,P),h.constructor=T,h.render=B),x&&x.sub(h),h.props=m,h.state||(h.state={}),h.context=P,h.__n=r,p=h.__d=!0,h.__h=[],h._sb=[]),C&&null==h.__s&&(h.__s=h.state),C&&null!=T.getDerivedStateFromProps&&(h.__s==h.state&&(h.__s=b({},h.__s)),b(h.__s,T.getDerivedStateFromProps(m,h.__s))),y=h.props,v=h.state,h.__v=t,p)C&&null==T.getDerivedStateFromProps&&null!=h.componentWillMount&&h.componentWillMount(),C&&null!=h.componentDidMount&&h.__h.push(h.componentDidMount);else{if(C&&null==T.getDerivedStateFromProps&&m!==y&&null!=h.componentWillReceiveProps&&h.componentWillReceiveProps(m,P),!h.__e&&(null!=h.shouldComponentUpdate&&!1===h.shouldComponentUpdate(m,h.__s,P)||t.__v==u.__v)){for(t.__v!=u.__v&&(h.props=m,h.state=h.__s,h.__d=!1),t.__e=u.__e,t.__k=u.__k,t.__k.some(function(n){n&&(n.__=t)}),$=0;$<h._sb.length;$++)h.__h.push(h._sb[$]);h._sb=[],h.__h.length&&o.push(h);break n}null!=h.componentWillUpdate&&h.componentWillUpdate(m,h.__s,P),C&&null!=h.componentDidUpdate&&h.__h.push(function(){h.componentDidUpdate(y,v,_)})}if(h.context=P,h.props=m,h.__P=n,h.__e=!1,A=l.__r,I=0,C){for(h.state=h.__s,h.__d=!1,A&&A(t),a=h.render(h.props,h.state,h.context),F=0;F<h._sb.length;F++)h.__h.push(h._sb[F]);h._sb=[]}else do{h.__d=!1,A&&A(t),a=h.render(h.props,h.state,h.context),h.state=h.__s}while(h.__d&&++I<25);h.state=h.__s,null!=h.getChildContext&&(r=b(b({},r),h.getChildContext())),C&&!p&&null!=h.getSnapshotBeforeUpdate&&(_=h.getSnapshotBeforeUpdate(y,v)),f=E(n,g(H=null!=a&&a.type===S&&null==a.key?a.props.children:a)?H:[H],t,u,r,e,i,o,f,c,s),h.base=t.__e,t.__u&=-161,h.__h.length&&o.push(h),w&&(h.__E=h.__=null)}catch(n){if(t.__v=null,c||null!=i)if(n.then){for(t.__u|=c?160:128;f&&8==f.nodeType&&f.nextSibling;)f=f.nextSibling;i[i.indexOf(f)]=null,t.__e=f}else for(L=i.length;L--;)k(i[L]);else t.__e=u.__e,t.__k=u.__k;l.__e(n,t,u)}else null==i&&t.__v==u.__v?(t.__k=u.__k,t.__e=u.__e):f=t.__e=N(u.__e,t,u,r,e,i,o,c,s);return(a=l.diffed)&&a(t),128&t.__u?void 0:f}function z(n,t,u){for(var r=0;r<u.length;r++)R(u[r],u[++r],u[++r]);l.__c&&l.__c(t,n),n.some(function(t){try{n=t.__h,t.__h=[],n.some(function(n){n.call(t)})}catch(n){l.__e(n,t.__v)}})}function N(t,u,r,e,i,o,f,c,s){var a,h,p,w,m,b,C,x=r.props,S=u.props,M=u.type;if("svg"==M?i=y:"math"==M?i="http://www.w3.org/1998/Math/MathML":i||(i=v),null!=o)for(a=0;a<o.length;a++)if((m=o[a])&&"setAttribute"in m==!!M&&(M?m.localName==M:3==m.nodeType)){t=m,o[a]=null;break}if(null==t){if(null==M)return document.createTextNode(S);t=document.createElementNS(i,M,S.is&&S),c&&(l.__m&&l.__m(u,o),c=!1),o=null}if(null===M)x===S||c&&t.data===S||(t.data=S);else{if(o=o&&n.call(t.childNodes),x=r.props||_,!c&&null!=o)for(x={},a=0;a<t.attributes.length;a++)x[(m=t.attributes[a]).name]=m.value;for(a in x)if(m=x[a],"children"==a);else if("dangerouslySetInnerHTML"==a)p=m;else if(!(a in S)){if("value"==a&&"defaultValue"in S||"checked"==a&&"defaultChecked"in S)continue;O(t,a,null,m,i)}for(a in S)m=S[a],"children"==a?w=m:"dangerouslySetInnerHTML"==a?h=m:"value"==a?b=m:"checked"==a?C=m:c&&"function"!=typeof m||x[a]===m||O(t,a,m,x[a],i);if(h)c||p&&(h.__html===p.__html||h.__html===t.innerHTML)||(t.innerHTML=h.__html),u.__k=[];else if(p&&(t.innerHTML=""),E(t,g(w)?w:[w],u,r,e,"foreignObject"==M?v:i,o,f,o?o[0]:r.__k&&P(r,0),c,s),null!=o)for(a=o.length;a--;)k(o[a]);c||(a="value","progress"==M&&null==b?t.removeAttribute("value"):b!==d&&(b!==t[a]||"progress"==M&&!b||"option"==M&&b!==x[a])&&O(t,a,b,x[a],i),a="checked",C!==d&&C!==t[a]&&O(t,a,C,x[a],i))}return t}function R(n,t,u){try{if("function"==typeof n){var r="function"==typeof n.__u;r&&n.__u(),r&&null==t||(n.__u=n(t))}else n.current=t}catch(n){l.__e(n,u)}}function q(n,t,u){var r,e;if(l.unmount&&l.unmount(n),(r=n.ref)&&(r.current&&r.current!==n.__e||R(r,null,t)),null!=(r=n.__c)){if(r.componentWillUnmount)try{r.componentWillUnmount()}catch(n){l.__e(n,t)}r.base=r.__P=null}if(r=n.__k)for(e=0;e<r.length;e++)r[e]&&q(r[e],t,u||"function"!=typeof n.type);u||k(n.__e),n.__c=n.__=n.__e=d}function B(n,l,t){return this.constructor(n,t)}function D(t,u,r){var e,i,o,f;u==document&&(u=document.documentElement),l.__&&l.__(t,u),i=(e="function"==typeof r)?null:r&&r.__k||u.__k,o=[],f=[],j(u,t=(!e&&r||u).__k=C(S,null,[t]),i||_,_,u.namespaceURI,!e&&r?[r]:i?null:u.firstChild?n.call(u.childNodes):null,o,!e&&r?r:i?i.__e:u.firstChild,e,f),z(o,t,f)}n=w.slice,l={__e:function(n,l,t,u){for(var r,e,i;l=l.__;)if((r=l.__c)&&!r.__)try{if((e=r.constructor)&&null!=e.getDerivedStateFromError&&(r.setState(e.getDerivedStateFromError(n)),i=r.__d),null!=r.componentDidCatch&&(r.componentDidCatch(n,u||{}),i=r.__d),i)return r.__E=r}catch(l){n=l}throw n}},t=0,u=function(n){return null!=n&&n.constructor==d},M.prototype.setState=function(n,l){var t;t=null!=this.__s&&this.__s!==this.state?this.__s:this.__s=b({},this.state),"function"==typeof n&&(n=n(b({},t),this.props)),n&&b(t,n),null!=n&&this.__v&&(l&&this._sb.push(l),A(this))},M.prototype.forceUpdate=function(n){this.__v&&(this.__e=!0,n&&this.__h.push(n),A(this))},M.prototype.render=S,r=[],i="function"==typeof Promise?Promise.prototype.then.bind(Promise.resolve()):setTimeout,o=function(n,l){return n.__v.__b-l.__v.__b},I.__r=0,f=/(PointerCapture)$|Capture$/i,c=0,s=V(!1),a=V(!0),h=0,p={__proto__:null,render:D,hydrate:function n(l,t){D(l,t,n)},createElement:C,h:C,Fragment:S,createRef:function(){return{current:null}},isValidElement:u,Component:M,cloneElement:function(l,t,u){var r,e,i,o,f=b({},l.props);for(i in l.type&&l.type.defaultProps&&(o=l.type.defaultProps),t)"key"==i?r=t[i]:"ref"==i?e=t[i]:f[i]=t[i]===d&&o!==d?o[i]:t[i];return arguments.length>2&&(f.children=arguments.length>3?n.call(arguments,2):u),x(l.type,f,r||l.key,e||l.ref,null)},createContext:function(n,l){var t={__c:l="__cC"+h++,__:n,Consumer:function(n,l){return n.children(l)},Provider:function(n){var t,u;return this.getChildContext||(t=new Set,(u={})[l]=this,this.getChildContext=function(){return u},this.componentWillUnmount=function(){t=null},this.shouldComponentUpdate=function(n){this.props.value!==n.value&&t.forEach(function(n){n.__e=!0,A(n)})},this.sub=function(n){t.add(n);var l=n.componentWillUnmount;n.componentWillUnmount=function(){t&&t.delete(n),l&&l.call(n)}}),n.children}};return t.Provider.__=t.Consumer.contextType=t},toChildArray:function n(l,t){return t=t||[],null==l||"boolean"==typeof l||(g(l)?l.some(function(l){n(l,t)}):t.push(l)),t},options:l},typeof module<"u"?module.exports=p:self.preact=p}();
//# sourceMappingURL=preact.min.js.map

/* ===== preact-hooks.js ===== */
!function(n,t){"object"==typeof exports&&"undefined"!=typeof module?t(exports,require("preact")):"function"==typeof define&&define.amd?define(["exports","preact"],t):t((n||self).preactHooks={},n.preact)}(this,function(n,t){var u,r,i,o,f=0,c=[],e=t.options,a=e.__b,v=e.__r,l=e.diffed,d=e.__c,s=e.unmount,p=e.__;function h(n,t){e.__h&&e.__h(r,n,f||t),f=0;var u=r.__H||(r.__H={__:[],__h:[]});return n>=u.__.length&&u.__.push({}),u.__[n]}function y(n){return f=1,m(j,n)}function m(n,t,i){var o=h(u++,2);if(o.t=n,!o.__c&&(o.__=[i?i(t):j(void 0,t),function(n){var t=o.__N?o.__N[0]:o.__[0],u=o.t(t,n);t!==u&&(o.__N=[u,o.__[1]],o.__c.setState({}))}],o.__c=r,!r.u)){var f=function(n,t,u){if(!o.__c.__H)return!0;var r=o.__c.__H.__.filter(function(n){return!!n.__c});if(r.every(function(n){return!n.__N}))return!c||c.call(this,n,t,u);var i=o.__c.props!==n;return r.forEach(function(n){if(n.__N){var t=n.__[0];n.__=n.__N,n.__N=void 0,t!==n.__[0]&&(i=!0)}}),c&&c.call(this,n,t,u)||i};r.u=!0;var c=r.shouldComponentUpdate,e=r.componentWillUpdate;r.componentWillUpdate=function(n,t,u){if(this.__e){var r=c;c=void 0,f(n,t,u),c=r}e&&e.call(this,n,t,u)},r.shouldComponentUpdate=f}return o.__N||o.__}function T(n,t){var i=h(u++,4);!e.__s&&g(i.__H,t)&&(i.__=n,i.i=t,r.__h.push(i))}function _(n,t){var r=h(u++,7);return g(r.__H,t)&&(r.__=n(),r.__H=t,r.__h=n),r.__}function b(){for(var n;n=c.shift();)if(n.__P&&n.__H)try{n.__H.__h.forEach(A),n.__H.__h.forEach(F),n.__H.__h=[]}catch(t){n.__H.__h=[],e.__e(t,n.__v)}}e.__b=function(n){r=null,a&&a(n)},e.__=function(n,t){n&&t.__k&&t.__k.__m&&(n.__m=t.__k.__m),p&&p(n,t)},e.__r=function(n){v&&v(n),u=0;var t=(r=n.__c).__H;t&&(i===r?(t.__h=[],r.__h=[],t.__.forEach(function(n){n.__N&&(n.__=n.__N),n.i=n.__N=void 0})):(t.__h.forEach(A),t.__h.forEach(F),t.__h=[],u=0)),i=r},e.diffed=function(n){l&&l(n);var t=n.__c;t&&t.__H&&(t.__H.__h.length&&(1!==c.push(t)&&o===e.requestAnimationFrame||((o=e.requestAnimationFrame)||x)(b)),t.__H.__.forEach(function(n){n.i&&(n.__H=n.i),n.i=void 0})),i=r=null},e.__c=function(n,t){t.some(function(n){try{n.__h.forEach(A),n.__h=n.__h.filter(function(n){return!n.__||F(n)})}catch(u){t.some(function(n){n.__h&&(n.__h=[])}),t=[],e.__e(u,n.__v)}}),d&&d(n,t)},e.unmount=function(n){s&&s(n);var t,u=n.__c;u&&u.__H&&(u.__H.__.forEach(function(n){try{A(n)}catch(n){t=n}}),u.__H=void 0,t&&e.__e(t,u.__v))};var q="function"==typeof requestAnimationFrame;function x(n){var t,u=function(){clearTimeout(r),q&&cancelAnimationFrame(t),setTimeout(n)},r=setTimeout(u,100);q&&(t=requestAnimationFrame(u))}function A(n){var t=r,u=n.__c;"function"==typeof u&&(n.__c=void 0,u()),r=t}function F(n){var t=r;n.__c=n.__(),r=t}function g(n,t){return!n||n.length!==t.length||t.some(function(t,u){return t!==n[u]})}function j(n,t){return"function"==typeof t?t(n):t}n.useCallback=function(n,t){return f=8,_(function(){return n},t)},n.useContext=function(n){var t=r.context[n.__c],i=h(u++,9);return i.c=n,t?(null==i.__&&(i.__=!0,t.sub(r)),t.props.value):n.__},n.useDebugValue=function(n,t){e.useDebugValue&&e.useDebugValue(t?t(n):n)},n.useEffect=function(n,t){var i=h(u++,3);!e.__s&&g(i.__H,t)&&(i.__=n,i.i=t,r.__H.__h.push(i))},n.useErrorBoundary=function(n){var t=h(u++,10),i=y();return t.__=n,r.componentDidCatch||(r.componentDidCatch=function(n,u){t.__&&t.__(n,u),i[1](n)}),[i[0],function(){i[1](void 0)}]},n.useId=function(){var n=h(u++,11);if(!n.__){for(var t=r.__v;null!==t&&!t.__m&&null!==t.__;)t=t.__;var i=t.__m||(t.__m=[0,0]);n.__="P"+i[0]+"-"+i[1]++}return n.__},n.useImperativeHandle=function(n,t,u){f=6,T(function(){return"function"==typeof n?(n(t()),function(){return n(null)}):n?(n.current=t(),function(){return n.current=null}):void 0},null==u?u:u.concat(n))},n.useLayoutEffect=T,n.useMemo=_,n.useReducer=m,n.useRef=function(n){return f=5,_(function(){return{current:n}},[])},n.useState=y});
//# sourceMappingURL=hooks.umd.js.map

/* ===== preact-compat.js ===== */
!function(n,t){"object"==typeof exports&&"undefined"!=typeof module?t(exports,require("preact"),require("preact/hooks")):"function"==typeof define&&define.amd?define(["exports","preact","preact/hooks"],t):t((n||self).preactCompat={},n.preact,n.preactHooks)}(this,function(n,t,e){function r(n,t){for(var e in t)n[e]=t[e];return n}function u(n,t){for(var e in n)if("__source"!==e&&!(e in t))return!0;for(var r in t)if("__source"!==r&&n[r]!==t[r])return!0;return!1}function o(n,t){var r=t(),u=e.useState({t:{__:r,u:t}}),o=u[0].t,c=u[1];return e.useLayoutEffect(function(){o.__=r,o.u=t,i(o)&&c({t:o})},[n,r,t]),e.useEffect(function(){return i(o)&&c({t:o}),n(function(){i(o)&&c({t:o})})},[n]),r}function i(n){var t,e,r=n.u,u=n.__;try{var o=r();return!((t=u)===(e=o)&&(0!==t||1/t==1/e)||t!=t&&e!=e)}catch(n){return!0}}function c(n){n()}function f(n){return n}function l(){return[!1,c]}var a=e.useLayoutEffect;function s(n,t){this.props=n,this.context=t}function h(n,e){function r(n){var t=this.props.ref,r=t==n.ref;return!r&&t&&(t.call?t(null):t.current=null),e?!e(this.props,n)||!r:u(this.props,n)}function o(e){return this.shouldComponentUpdate=r,t.createElement(n,e)}return o.displayName="Memo("+(n.displayName||n.name)+")",o.prototype.isReactComponent=!0,o.__f=!0,o}(s.prototype=new t.Component).isPureReactComponent=!0,s.prototype.shouldComponentUpdate=function(n,t){return u(this.props,n)||u(this.state,t)};var d=t.options.__b;t.options.__b=function(n){n.type&&n.type.__f&&n.ref&&(n.props.ref=n.ref,n.ref=null),d&&d(n)};var v="undefined"!=typeof Symbol&&Symbol.for&&Symbol.for("react.forward_ref")||3911;function p(n){function t(t){var e=r({},t);return delete e.ref,n(e,t.ref||null)}return t.$$typeof=v,t.render=t,t.prototype.isReactComponent=t.__f=!0,t.displayName="ForwardRef("+(n.displayName||n.name)+")",t}var m=function(n,e){return null==n?null:t.toChildArray(t.toChildArray(n).map(e))},b={map:m,forEach:m,count:function(n){return n?t.toChildArray(n).length:0},only:function(n){var e=t.toChildArray(n);if(1!==e.length)throw"Children.only";return e[0]},toArray:t.toChildArray},y=t.options.__e;t.options.__e=function(n,t,e,r){if(n.then)for(var u,o=t;o=o.__;)if((u=o.__c)&&u.__c)return null==t.__e&&(t.__e=e.__e,t.__k=e.__k),u.__c(n,t);y(n,t,e,r)};var _=t.options.unmount;function g(n,t,e){return n&&(n.__c&&n.__c.__H&&(n.__c.__H.__.forEach(function(n){"function"==typeof n.__c&&n.__c()}),n.__c.__H=null),null!=(n=r({},n)).__c&&(n.__c.__P===e&&(n.__c.__P=t),n.__c=null),n.__k=n.__k&&n.__k.map(function(n){return g(n,t,e)})),n}function S(n,t,e){return n&&e&&(n.__v=null,n.__k=n.__k&&n.__k.map(function(n){return S(n,t,e)}),n.__c&&n.__c.__P===t&&(n.__e&&e.appendChild(n.__e),n.__c.__e=!0,n.__c.__P=e)),n}function E(){this.__u=0,this.o=null,this.__b=null}function C(n){var t=n.__.__c;return t&&t.__a&&t.__a(n)}function x(n){var e,r,u;function o(o){if(e||(e=n()).then(function(n){r=n.default||n},function(n){u=n}),u)throw u;if(!r)throw e;return t.createElement(r,o)}return o.displayName="Lazy",o.__f=!0,o}function O(){this.i=null,this.l=null}t.options.unmount=function(n){var t=n.__c;t&&t.__R&&t.__R(),t&&32&n.__u&&(n.type=null),_&&_(n)},(E.prototype=new t.Component).__c=function(n,t){var e=t.__c,r=this;null==r.o&&(r.o=[]),r.o.push(e);var u=C(r.__v),o=!1,i=function(){o||(o=!0,e.__R=null,u?u(c):c())};e.__R=i;var c=function(){if(!--r.__u){if(r.state.__a){var n=r.state.__a;r.__v.__k[0]=S(n,n.__c.__P,n.__c.__O)}var t;for(r.setState({__a:r.__b=null});t=r.o.pop();)t.forceUpdate()}};r.__u++||32&t.__u||r.setState({__a:r.__b=r.__v.__k[0]}),n.then(i,i)},E.prototype.componentWillUnmount=function(){this.o=[]},E.prototype.render=function(n,e){if(this.__b){if(this.__v.__k){var r=document.createElement("div"),u=this.__v.__k[0].__c;this.__v.__k[0]=g(this.__b,r,u.__O=u.__P)}this.__b=null}var o=e.__a&&t.createElement(t.Fragment,null,n.fallback);return o&&(o.__u&=-33),[t.createElement(t.Fragment,null,e.__a?null:n.children),o]};var R=function(n,t,e){if(++e[1]===e[0]&&n.l.delete(t),n.props.revealOrder&&("t"!==n.props.revealOrder[0]||!n.l.size))for(e=n.i;e;){for(;e.length>3;)e.pop()();if(e[1]<e[0])break;n.i=e=e[2]}};function w(n){return this.getChildContext=function(){return n.context},n.children}function j(n){var e=this,r=n.h;e.componentWillUnmount=function(){t.render(null,e.v),e.v=null,e.h=null},e.h&&e.h!==r&&e.componentWillUnmount(),e.v||(e.h=r,e.v={nodeType:1,parentNode:r,childNodes:[],contains:function(){return!0},appendChild:function(n){this.childNodes.push(n),e.h.appendChild(n)},insertBefore:function(n,t){this.childNodes.push(n),e.h.insertBefore(n,t)},removeChild:function(n){this.childNodes.splice(this.childNodes.indexOf(n)>>>1,1),e.h.removeChild(n)}}),t.render(t.createElement(w,{context:e.context},n.__v),e.v)}function T(n,e){var r=t.createElement(j,{__v:n,h:e});return r.containerInfo=e,r}(O.prototype=new t.Component).__a=function(n){var t=this,e=C(t.__v),r=t.l.get(n);return r[0]++,function(u){var o=function(){t.props.revealOrder?(r.push(u),R(t,n,r)):u()};e?e(o):o()}},O.prototype.render=function(n){this.i=null,this.l=new Map;var e=t.toChildArray(n.children);n.revealOrder&&"b"===n.revealOrder[0]&&e.reverse();for(var r=e.length;r--;)this.l.set(e[r],this.i=[1,0,this.i]);return n.children},O.prototype.componentDidUpdate=O.prototype.componentDidMount=function(){var n=this;this.l.forEach(function(t,e){R(n,e,t)})};var k="undefined"!=typeof Symbol&&Symbol.for&&Symbol.for("react.element")||60103,I=/^(?:accent|alignment|arabic|baseline|cap|clip(?!PathU)|color|dominant|fill|flood|font|glyph(?!R)|horiz|image(!S)|letter|lighting|marker(?!H|W|U)|overline|paint|pointer|shape|stop|strikethrough|stroke|text(?!L)|transform|underline|unicode|units|v|vector|vert|word|writing|x(?!C))[A-Z]/,N=/^on(Ani|Tra|Tou|BeforeInp|Compo)/,M=/[A-Z0-9]/g,A="undefined"!=typeof document,D=function(n){return("undefined"!=typeof Symbol&&"symbol"==typeof Symbol()?/fil|che|rad/:/fil|che|ra/).test(n)};function L(n,e,r){return null==e.__k&&(e.textContent=""),t.render(n,e),"function"==typeof r&&r(),n?n.__c:null}function F(n,e,r){return t.hydrate(n,e),"function"==typeof r&&r(),n?n.__c:null}t.Component.prototype.isReactComponent={},["componentWillMount","componentWillReceiveProps","componentWillUpdate"].forEach(function(n){Object.defineProperty(t.Component.prototype,n,{configurable:!0,get:function(){return this["UNSAFE_"+n]},set:function(t){Object.defineProperty(this,n,{configurable:!0,writable:!0,value:t})}})});var U=t.options.event;function V(){}function W(){return this.cancelBubble}function P(){return this.defaultPrevented}t.options.event=function(n){return U&&(n=U(n)),n.persist=V,n.isPropagationStopped=W,n.isDefaultPrevented=P,n.nativeEvent=n};var z,B={enumerable:!1,configurable:!0,get:function(){return this.class}},H=t.options.vnode;t.options.vnode=function(n){"string"==typeof n.type&&function(n){var e=n.props,r=n.type,u={},o=-1===r.indexOf("-");for(var i in e){var c=e[i];if(!("value"===i&&"defaultValue"in e&&null==c||A&&"children"===i&&"noscript"===r||"class"===i||"className"===i)){var f=i.toLowerCase();"defaultValue"===i&&"value"in e&&null==e.value?i="value":"download"===i&&!0===c?c="":"translate"===f&&"no"===c?c=!1:"o"===f[0]&&"n"===f[1]?"ondoubleclick"===f?i="ondblclick":"onchange"!==f||"input"!==r&&"textarea"!==r||D(e.type)?"onfocus"===f?i="onfocusin":"onblur"===f?i="onfocusout":N.test(i)&&(i=f):f=i="oninput":o&&I.test(i)?i=i.replace(M,"-$&").toLowerCase():null===c&&(c=void 0),"oninput"===f&&u[i=f]&&(i="oninputCapture"),u[i]=c}}"select"==r&&u.multiple&&Array.isArray(u.value)&&(u.value=t.toChildArray(e.children).forEach(function(n){n.props.selected=-1!=u.value.indexOf(n.props.value)})),"select"==r&&null!=u.defaultValue&&(u.value=t.toChildArray(e.children).forEach(function(n){n.props.selected=u.multiple?-1!=u.defaultValue.indexOf(n.props.value):u.defaultValue==n.props.value})),e.class&&!e.className?(u.class=e.class,Object.defineProperty(u,"className",B)):(e.className&&!e.class||e.class&&e.className)&&(u.class=u.className=e.className),n.props=u}(n),n.$$typeof=k,H&&H(n)};var q=t.options.__r;t.options.__r=function(n){q&&q(n),z=n.__c};var Z=t.options.diffed;t.options.diffed=function(n){Z&&Z(n);var t=n.props,e=n.__e;null!=e&&"textarea"===n.type&&"value"in t&&t.value!==e.value&&(e.value=null==t.value?"":t.value),z=null};var Y={ReactCurrentDispatcher:{current:{readContext:function(n){return z.__n[n.__c].props.value},useCallback:e.useCallback,useContext:e.useContext,useDebugValue:e.useDebugValue,useDeferredValue:f,useEffect:e.useEffect,useId:e.useId,useImperativeHandle:e.useImperativeHandle,useInsertionEffect:a,useLayoutEffect:e.useLayoutEffect,useMemo:e.useMemo,useReducer:e.useReducer,useRef:e.useRef,useState:e.useState,useSyncExternalStore:o,useTransition:l}}},$="18.3.1";function G(n){return t.createElement.bind(null,n)}function J(n){return!!n&&n.$$typeof===k}function K(n){return J(n)&&n.type===t.Fragment}function Q(n){return!!n&&!!n.displayName&&("string"==typeof n.displayName||n.displayName instanceof String)&&n.displayName.startsWith("Memo(")}function X(n){return J(n)?t.cloneElement.apply(null,arguments):n}function nn(n){return!!n.__k&&(t.render(null,n),!0)}function tn(n){return n&&(n.base||1===n.nodeType&&n)||null}var en=function(n,t){return n(t)},rn=function(n,t){return n(t)},un=t.Fragment,on=J,cn={useState:e.useState,useId:e.useId,useReducer:e.useReducer,useEffect:e.useEffect,useLayoutEffect:e.useLayoutEffect,useInsertionEffect:a,useTransition:l,useDeferredValue:f,useSyncExternalStore:o,startTransition:c,useRef:e.useRef,useImperativeHandle:e.useImperativeHandle,useMemo:e.useMemo,useCallback:e.useCallback,useContext:e.useContext,useDebugValue:e.useDebugValue,version:$,Children:b,render:L,hydrate:F,unmountComponentAtNode:nn,createPortal:T,createElement:t.createElement,createContext:t.createContext,createFactory:G,cloneElement:X,createRef:t.createRef,Fragment:t.Fragment,isValidElement:J,isElement:on,isFragment:K,isMemo:Q,findDOMNode:tn,Component:t.Component,PureComponent:s,memo:h,forwardRef:p,flushSync:rn,unstable_batchedUpdates:en,StrictMode:un,Suspense:E,SuspenseList:O,lazy:x,__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED:Y};Object.defineProperty(n,"Component",{enumerable:!0,get:function(){return t.Component}}),Object.defineProperty(n,"Fragment",{enumerable:!0,get:function(){return t.Fragment}}),Object.defineProperty(n,"createContext",{enumerable:!0,get:function(){return t.createContext}}),Object.defineProperty(n,"createElement",{enumerable:!0,get:function(){return t.createElement}}),Object.defineProperty(n,"createRef",{enumerable:!0,get:function(){return t.createRef}}),n.Children=b,n.PureComponent=s,n.StrictMode=un,n.Suspense=E,n.SuspenseList=O,n.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED=Y,n.cloneElement=X,n.createFactory=G,n.createPortal=T,n.default=cn,n.findDOMNode=tn,n.flushSync=rn,n.forwardRef=p,n.hydrate=F,n.isElement=on,n.isFragment=K,n.isMemo=Q,n.isValidElement=J,n.lazy=x,n.memo=h,n.render=L,n.startTransition=c,n.unmountComponentAtNode=nn,n.unstable_batchedUpdates=en,n.useDeferredValue=f,n.useInsertionEffect=a,n.useSyncExternalStore=o,n.useTransition=l,n.version=$,Object.keys(e).forEach(function(t){"default"===t||n.hasOwnProperty(t)||Object.defineProperty(n,t,{enumerable:!0,get:function(){return e[t]}})})});
//# sourceMappingURL=compat.umd.js.map

/* ===== shim ===== */

/* ===== React 兼容别名（preact-compat 挂的是 preactCompat，不是 React）=====
 *
 * 两个坑（curl 测不出来，只有真浏览器能发现）：
 * 1. preact-compat 的 UMD 挂到全局 `preactCompat`，不是 `React`。
 *    app.js 里写 `React.createElement` 会 ReferenceError，页面全白
 *    但接口全部正常 —— 极易误判为后端问题。
 * 2. compat 只有 React 17 风格的 `render`（无 `createRoot`）。
 *    app.js 按 React 18 写 `createRoot().render()` 会 TypeError。
 *
 * 所以这里既做别名，也补一个 createRoot 兼容层。
 */
(function () {
  var c = window.preactCompat;
  if (!c) { console.error('[clashx] preactCompat 未加载，检查 bundle 顺序'); return; }

  window.React = c;
  window.ReactDOM = c;

  // compat 已把 hooks 合并进自己的导出（useState/useEffect 等），
  // 下面只是显式补齐，防止个别版本没合并。
  var h = window.preactHooks || {};
  ['useState', 'useEffect', 'useCallback', 'useRef', 'useMemo', 'useContext',
   'useReducer', 'useLayoutEffect'].forEach(function (k) {
    if (!window.React[k] && h[k]) window.React[k] = h[k];
  });

  // createRoot 兼容层：包一层 render，返回 { render, unmount }
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
 * 零构建：用 htm 的 tagged template 写组件，浏览器直接跑。
 * 源码用 include_str! 编译进 clashx 二进制，单文件部署。
 *
 * 依赖只有两个，都已内联在 index.html 里（preact 11KB + htm 1.3KB）：
 *   - preact@10.25.4  组件模型 + hooks
 *   - htm@3.1.1       html`` 标签编译成 h() 调用
 * 引入它们是为了「不写 React 却有 React 的写法」。手写 html 解析器不划算 ——
 * 边界情况（自闭合标签、属性引号、文本转义）会耗掉的时间远超收益。
 *
 * 为什么不用 React + Vite：管理界面要部署到 OpenWrt / 小主机，
 * 单文件二进制是硬需求。为此引入 pnpm 工具链不划算。
 */
(function () {
  'use strict';

  var html = htm.bind(React.createElement);
  var useState = React.useState;
  var useEffect = React.useEffect;
  var useCallback = React.useCallback;

  // ---- API ----
  function api(path, opts) {
    opts = opts || {};
    return fetch(path, {
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
  var MsgCtx = React.createContext(function () {});
  function useMsg() { return React.useContext(MsgCtx); }

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

  function Badge(props) {
    return html`<span class="badge ${props.kind || 'dim'}">${props.children}</span>`;
  }

  // ==================== 总览 ====================
  function Overview() {
    var s = useState(null);
    var d = s[0];
    useEffect(function () {
      api('/api/clashx/overview').then(s[1]).catch(function (e) { s[1]({ error: String(e.message || e) }); });
    }, []);
    if (!d) return html`<div class="card"><div class="spin">加载中…</div></div>`;
    if (d.error) return html`<div class="card"><div class="msg err">${d.error}</div></div>`;

    return html`
      <div class="card">
        <h2>状态总览</h2>
        <div class="grid">
          <div class="stat"><div class="k">运行模式</div><div class="v" style=${{ fontSize: '15px' }}>${d.run_mode}</div></div>
          <div class="stat"><div class="k">DNS 模式</div><div class="v" style=${{ fontSize: '15px' }}>${d.dns_mode}</div></div>
          <div class="stat"><div class="k">节点数</div><div class="v">${d.node_count}</div></div>
          <div class="stat"><div class="k">规则数</div><div class="v">${d.rule_count}</div></div>
        </div>
      </div>
      <div class="card">
        <h2>服务状态</h2>
        <ul class="list">
          <li>
            <span class="grow">内核进程</span>
            <${Badge} kind=${d.mihomo_up ? 'ok' : 'err'}>${d.mihomo_up ? d.mihomo_version : '未运行'}<//>
            <a class="btn sm" href="/zashboard/" target="_blank">打开 zashboard</a>
          </li>
          <li><span class="grow">活动连接</span><span class="mono">${d.mihomo_connections}</span></li>
          <li><span class="grow">TUN 设备</span><${Badge} kind=${d.tun ? 'ok' : 'dim'}>${d.tun ? '已启用' : '未启用'}<//></li>
          <li><span class="grow">旁路由 nft 规则</span><${Badge} kind=${d.gateway ? 'ok' : 'dim'}>${d.gateway ? '已启用' : '未启用'}<//></li>
          <li><span class="grow">防火墙由谁写</span><span class="mono">${d.firewall}</span></li>
        </ul>
      </div>`;
  }

  // ==================== 运行模式 ====================
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
        <h2>运行模式</h2>
        <p class="hint" style=${{ marginTop: 0 }}>
          模式 = DNS 解析方式（fake-ip / redir-host）× 防火墙接管方。
          切换时会自动校准 nftables 规则并重启内核，约 1 秒。
        </p>
        <div class="modes">
          ${(d ? d.all : []).map(function (m) {
            return html`<button class="mode" aria-pressed=${d.current === m.value} onClick=${function () { apply(m.value); }}>
              <div class="n">${m.value}</div>
              <div class="d">DNS ${m.dns} · ${m.tun ? 'TUN' : '传统'} · ${m.firewall}</div>
            </button>`;
          })}
        </div>
        ${!d ? html`<div class="spin">加载中…</div>` : ''}
      </div>`;
  }

  // ==================== 规则 ====================
  function Rules() {
    var s = useState(null);
    var add = useState('');
    var test = useState('www.google.com');
    var verdict = useState(null);
    var d = s[0];
    var msg = useMsg();

    var refresh = useCallback(function () {
      api('/api/clashx/rules').then(s[1])
        .catch(function (e) { msg('err', String(e.message || e)); });
    }, []);

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
        <h2>规则链<span class="spacer"></span><span class="mono hint">${d ? d.rules.length : 0} 条</span></h2>
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
      <div class="card">
        <h2>添加规则</h2>
        <p class="hint" style=${{ marginTop: 0 }}>
          内核原生语法，单行标量。例如 <code>DOMAIN-SUFFIX,example.com,PROXY</code>。
          添加时会插到 MATCH 之前并立即生效。
        </p>
        <label class="field">
          <span>规则</span>
          <input class="mono" type="text" value=${add[0]} onInput=${function (e) { add[1](e.target.value); }}
                 placeholder="GEOIP,CN,DIRECT" />
        </label>
        <div class="btnrow"><button class="btn primary" onClick=${doAdd}>添加</button></div>
      </div>
      <div class="card">
        <h2>规则测试器（离线）</h2>
        <p class="hint" style=${{ marginTop: 0 }}>
          不发包，按规则链顺序判断某个目标会命中哪条。调试分流问题用这个。
        </p>
        <label class="field">
          <span>目标（域名或 IP，可带端口）</span>
          <input class="mono" type="text" value=${test[0]} onInput=${function (e) { test[1](e.target.value); }} />
        </label>
        <div class="btnrow"><button class="btn" onClick=${doTest}>测试</button></div>
        ${verdict[0] ? html`
          <div class="msg ok" style=${{ marginTop: '12px' }}>
            命中第 <strong>${verdict[0].index}</strong> 条：<code>${verdict[0].rule}</code><br/>
            类型 <code>${verdict[0].kind}</code> → 策略 <code>${verdict[0].target}</code>
            ${verdict[0].fallback ? html`<br/><strong>（MATCH 兜底，说明前面都没命中）</strong>` : ''}
          </div>` : ''}
      </div>`;
  }

  // ==================== 订阅 ====================
  function Subs() {
    var s = useState(null);
    var name = useState('');
    var url = useState('');
    var d = s[0];
    var msg = useMsg();

    var refresh = useCallback(function () {
      api('/api/clashx/subs').then(s[1])
        .catch(function (e) { msg('err', String(e.message || e)); });
    }, []);

    function doAdd() {
      if (!name[0].trim() || !url[0].trim()) return msg('err', '名称与 URL 必填');
      api('/api/clashx/subs/add', { method: 'POST', body: { name: name[0], url: url[0] } })
        .then(function () { msg('ok', '已保存，在命令行执行 apply 生效'); name[1](''); url[1](''); refresh(); })
        .catch(function (e) { msg('err', String(e.message || e)); });
    }
    function doRm(n) {
      if (!confirm('删除订阅 ' + n + '？')) return;
      api('/api/clashx/subs/rm', { method: 'POST', body: { name: n } })
        .then(function () { msg('ok', '已删除'); refresh(); })
        .catch(function (e) { msg('err', String(e.message || e)); });
    }

    return html`
      <div class="card">
        <h2>订阅</h2>
        <ul class="list">
          ${(d ? d.subs : []).map(function (sb) {
            return html`<li key=${sb.name}>
              <div class="grow">
                <div>${sb.name}</div>
                <div class="mono hint" title=${sb.url}>${sb.url}</div>
              </div>
              <${Badge} kind="dim">${sb.user_agent || 'clash.meta'}<//>
              <span class="hint">${sb.update_interval}h</span>
              <button class="btn sm danger" onClick=${function () { doRm(sb.name); }}>删</button>
            </li>`;
          })}
        </ul>
        ${d && !d.subs.length ? html`<div class="empty">还没有订阅</div>` : ''}
      </div>
      <div class="card">
        <h2>添加订阅</h2>
        <p class="hint" style=${{ marginTop: 0 }}>
          URL 会在界面上打码显示，但完整地址存在服务器配置文件里（已受Basic Auth 保护）。
          抓取失败时自动回退本地缓存，不会阻塞配置生成。
        </p>
        <label class="field"><span>名称</span>
          <input type="text" value=${name[0]} onInput=${function (e) { name[1](e.target.value); }} placeholder="my-provider" /></label>
        <label class="field"><span>订阅地址</span>
          <input class="mono" type="text" value=${url[0]} onInput=${function (e) { url[1](e.target.value); }} placeholder="https://…" /></label>
        <div class="btnrow"><button class="btn primary" onClick=${doAdd}>添加</button></div>
      </div>`;
  }

  // ==================== 旁路由 ====================
  function Gateway() {
    var s = useState(null);
    var clients = useState('192.168.10.0/24');
    var d = s[0];
    var msg = useMsg();

    var refresh = useCallback(function () {
      api('/api/clashx/gateway').then(s[1])
        .catch(function (e) { msg('err', String(e.message || e)); });
    }, []);

    function enable() {
      msg('info', '正在启用旁路由…');
      api('/api/clashx/gateway/enable', { method: 'POST', body: { clients: clients[0] } })
        .then(function (r) { msg('ok', '已启用（网关 ' + r.lan_addr + '，网卡 ' + r.iface + '）'); refresh(); })
        .catch(function (e) { msg('err', String(e.message || e)); });
    }
    function disable() {
      msg('info', '正在停用…');
      api('/api/clashx/gateway/disable', { method: 'POST' })
        .then(function () { msg('ok', '已停用'); refresh(); })
        .catch(function (e) { msg('err', String(e.message || e)); });
    }

    return html`
      <div class="card">
        <h2>旁路由（网关模式）
          <span class="spacer"></span>
          <${Badge} kind=${d && d.active ? 'ok' : 'dim'}>${d && d.active ? '已启用' : '未启用'}<//>
        </h2>
        <p class="hint" style=${{ marginTop: 0 }}>
          启用后把客户端设备的网关指向本机 IP 即可。仅对<strong>传统模式</strong>有效
          （TUN 模式由 mihomo 自己接管，不需要 nft 规则）。
        </p>
        <label class="field">
          <span>客户端网段（逗号分隔，必填）</span>
          <input class="mono" type="text" value=${clients[0]} onInput=${function (e) { clients[1](e.target.value); }} />
        </label>
        <div class="btnrow">
          <button class="btn primary" onClick=${enable}>启用</button>
          <button class="btn danger" onClick=${disable}>停用</button>
        </div>
      </div>
      <div class="card">
        <h2>防火墙状态</h2>
        <pre class="out">${d ? (d.detail || []).join('\n') : '加载中…'}</pre>
      </div>`;
  }

  // ==================== 配置 ====================
  function ConfigView() {
    var s = useState(null);
    useEffect(function () {
      api('/api/clashx/rendered').then(function (t) { s[1](t.raw || ''); })
        .catch(function (e) { s[1]('（读取失败：' + e.message + '）'); });
    }, []);
    return html`<pre class="out">${s[0] || '加载中…'}</pre>`;
  }

  function ConfigPage() {
    var s = useState(null);
    useEffect(function () {
      api('/api/clashx/config').then(function (t) { s[1](t.raw || ''); })
        .catch(function (e) { s[1]('（读取失败：' + e.message + '）'); });
    }, []);
    return html`
      <div class="card">
        <h2>渲染产物（config.gen.yaml）</h2>
        <p class="hint" style=${{ marginTop: 0 }}>clashx 生成、交给 mihomo 的实际配置，含订阅展开后的节点。</p>
        <${ConfigView}/>
      </div>
      <div class="card">
        <h2>用户配置（config.yaml）</h2>
        <p class="hint" style=${{ marginTop: 0 }}>只读。修改请用命令行编辑后执行 apply。</p>
        <pre class="out">${s[0] || '加载中…'}</pre>
      </div>`;
  }

  // ==================== 密码 ====================
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
        <h2>修改登录密码</h2>
        <p class="hint" style=${{ marginTop: 0 }}>
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
      </div>`;
  }

  // ==================== 根 ====================
  var TABS = [
    { id: 'overview', name: '总览', C: Overview },
    { id: 'mode', name: '运行模式', C: RunMode },
    { id: 'rules', name: '规则', C: Rules },
    { id: 'subs', name: '订阅', C: Subs },
    { id: 'gateway', name: '旁路由', C: Gateway },
    { id: 'config', name: '配置', C: ConfigPage },
    { id: 'password', name: '密码', C: PasswordPage },
  ];

  function App() {
    var tab = useState('overview');
    var cur = TABS.filter(function (t) { return t.id === tab[0]; })[0];
    var Body = cur.C;

    return html`
      <div class="wrap">
        <header class="top">
          <h1>clashx</h1>
          <${Badge} kind="dim">Linux 无桌面 mihomo 客户端<//>
          <span class="spacer"></span>
          <a class="btn sm" href="/zashboard/" target="_blank">zashboard ↗</a>
        </header>
        <nav class="tabs">
          ${TABS.map(function (t) {
            return html`<button key=${t.id} aria-selected=${tab[0] === t.id}
                    onClick=${function () { tab[1](t.id); }}>${t.name}</button>`;
          })}
        </nav>
        <${MsgHost}><${Body}/><//>
      </div>`;
  }

  ReactDOM.createRoot(document.getElementById('root')).render(html`<${App}/>`);
  window.__clashx = { api: api };
})();
