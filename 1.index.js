(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(n,t,e){"use strict";e.r(t),function(n){e.d(t,"init_emulator",(function(){return m})),e.d(t,"advance_frame",(function(){return v})),e.d(t,"insert_cartridge",(function(){return O})),e.d(t,"reset",(function(){return j})),e.d(t,"key_down",(function(){return T})),e.d(t,"key_up",(function(){return x})),e.d(t,"save_state",(function(){return E})),e.d(t,"load_state",(function(){return S})),e.d(t,"__wbindgen_object_drop_ref",(function(){return A})),e.d(t,"__wbindgen_string_new",(function(){return C})),e.d(t,"__wbg_new_59cb74e423758ede",(function(){return P})),e.d(t,"__wbg_stack_558ba5917b466edd",(function(){return F})),e.d(t,"__wbg_error_4bb6c2a97407129a",(function(){return $})),e.d(t,"__widl_f_log_1_",(function(){return I})),e.d(t,"__widl_instanceof_Window",(function(){return N})),e.d(t,"__widl_instanceof_CanvasRenderingContext2D",(function(){return J})),e.d(t,"__widl_f_put_image_data_CanvasRenderingContext2D",(function(){return R})),e.d(t,"__widl_f_get_element_by_id_Document",(function(){return H})),e.d(t,"__widl_instanceof_HTMLCanvasElement",(function(){return L})),e.d(t,"__widl_f_get_context_HTMLCanvasElement",(function(){return M})),e.d(t,"__widl_f_new_with_u8_clamped_array_ImageData",(function(){return U})),e.d(t,"__widl_f_document_Window",(function(){return B})),e.d(t,"__wbg_call_12b949cfc461d154",(function(){return G})),e.d(t,"__wbindgen_object_clone_ref",(function(){return W})),e.d(t,"__wbg_newnoargs_c4b2cbbd30e2d057",(function(){return Z})),e.d(t,"__wbg_globalThis_22e06d4bea0084e3",(function(){return q})),e.d(t,"__wbg_self_00b0599bca667294",(function(){return K})),e.d(t,"__wbg_window_aa795c5aad79b8ac",(function(){return Q})),e.d(t,"__wbg_global_cc239dc2303f417c",(function(){return V})),e.d(t,"__wbindgen_is_undefined",(function(){return X})),e.d(t,"__wbindgen_string_get",(function(){return Y})),e.d(t,"__wbindgen_debug_string",(function(){return nn})),e.d(t,"__wbindgen_throw",(function(){return tn})),e.d(t,"__wbindgen_rethrow",(function(){return en}));var r=e(7);const o=new Array(32);function i(n){return o[n]}o.fill(void 0),o.push(void 0,null,!0,!1);let u=o.length;function c(n){const t=i(n);return function(n){n<36||(o[n]=u,u=n)}(n),t}let f=new("undefined"==typeof TextDecoder?e(2).TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});f.decode();let a=null;function l(){return null!==a&&a.buffer===r.k.buffer||(a=new Uint8Array(r.k.buffer)),a}function s(n,t){return f.decode(l().subarray(n,n+t))}function p(n){u===o.length&&o.push(o.length+1);const t=u;return u=o[t],o[t]=n,t}let d=0;let g=new("undefined"==typeof TextEncoder?e(2).TextEncoder:TextEncoder)("utf-8");const y="function"==typeof g.encodeInto?function(n,t){return g.encodeInto(n,t)}:function(n,t){const e=g.encode(n);return t.set(e),{read:n.length,written:e.length}};function b(n,t,e){if(void 0===e){const e=g.encode(n),r=t(e.length);return l().subarray(r,r+e.length).set(e),d=e.length,r}let r=n.length,o=t(r);const i=l();let u=0;for(;u<r;u++){const t=n.charCodeAt(u);if(t>127)break;i[o+u]=t}if(u!==r){0!==u&&(n=n.slice(u)),o=e(o,r,r=u+3*n.length);const t=l().subarray(o+u,o+r);u+=y(n,t).written}return d=u,o}function _(n){return null==n}let h=null;function w(){return null!==h&&h.buffer===r.k.buffer||(h=new Int32Array(r.k.buffer)),h}function m(){r.f()}function v(){r.e()}function O(n){var t=function(n,t){const e=t(1*n.length);return l().set(n,e/1),d=n.length,e}(n,r.c),e=d;r.g(t,e)}function j(){r.l()}function T(n){r.h(p(n))}function x(n){r.i(p(n))}function E(){r.m()}function S(){r.j()}function D(n){r.a(p(n))}let k=null;function z(n,t){return(null!==k&&k.buffer===r.k.buffer||(k=new Uint8ClampedArray(r.k.buffer)),k).subarray(n/1,n/1+t)}const A=function(n){c(n)},C=function(n,t){return p(s(n,t))},P=function(){return p(new Error)},F=function(n,t){var e=b(i(t).stack,r.c,r.d),o=d;w()[n/4+1]=o,w()[n/4+0]=e},$=function(n,t){try{console.error(s(n,t))}finally{r.b(n,t)}},I=function(n){console.log(i(n))},N=function(n){return i(n)instanceof Window},J=function(n){return i(n)instanceof CanvasRenderingContext2D},R=function(n,t,e,r){try{i(n).putImageData(i(t),e,r)}catch(n){D(n)}},H=function(n,t,e){var r=i(n).getElementById(s(t,e));return _(r)?0:p(r)},L=function(n){return i(n)instanceof HTMLCanvasElement},M=function(n,t,e){try{var r=i(n).getContext(s(t,e));return _(r)?0:p(r)}catch(n){D(n)}},U=function(n,t,e){try{return p(new ImageData(z(n,t),e>>>0))}catch(n){D(n)}},B=function(n){var t=i(n).document;return _(t)?0:p(t)},G=function(n,t){try{return p(i(n).call(i(t)))}catch(n){D(n)}},W=function(n){return p(i(n))},Z=function(n,t){return p(new Function(s(n,t)))},q=function(){try{return p(globalThis.globalThis)}catch(n){D(n)}},K=function(){try{return p(self.self)}catch(n){D(n)}},Q=function(){try{return p(window.window)}catch(n){D(n)}},V=function(){try{return p(n.global)}catch(n){D(n)}},X=function(n){return void 0===i(n)},Y=function(n,t){const e=i(t);var o="string"==typeof e?e:void 0,u=_(o)?0:b(o,r.c,r.d),c=d;w()[n/4+1]=c,w()[n/4+0]=u},nn=function(n,t){var e=b(function n(t){const e=typeof t;if("number"==e||"boolean"==e||null==t)return`${t}`;if("string"==e)return`"${t}"`;if("symbol"==e){const n=t.description;return null==n?"Symbol":`Symbol(${n})`}if("function"==e){const n=t.name;return"string"==typeof n&&n.length>0?`Function(${n})`:"Function"}if(Array.isArray(t)){const e=t.length;let r="[";e>0&&(r+=n(t[0]));for(let o=1;o<e;o++)r+=", "+n(t[o]);return r+="]",r}const r=/\[object ([^\]]+)\]/.exec(toString.call(t));let o;if(!(r.length>1))return toString.call(t);if(o=r[1],"Object"==o)try{return"Object("+JSON.stringify(t)+")"}catch(n){return"Object"}return t instanceof Error?`${t.name}: ${t.message}\n${t.stack}`:o}(i(t)),r.c,r.d),o=d;w()[n/4+1]=o,w()[n/4+0]=e},tn=function(n,t){throw new Error(s(n,t))},en=function(n){throw c(n)}}.call(this,e(3))},function(n,t,e){(function(n){var r=Object.getOwnPropertyDescriptors||function(n){for(var t=Object.keys(n),e={},r=0;r<t.length;r++)e[t[r]]=Object.getOwnPropertyDescriptor(n,t[r]);return e},o=/%[sdj%]/g;t.format=function(n){if(!_(n)){for(var t=[],e=0;e<arguments.length;e++)t.push(c(arguments[e]));return t.join(" ")}e=1;for(var r=arguments,i=r.length,u=String(n).replace(o,(function(n){if("%%"===n)return"%";if(e>=i)return n;switch(n){case"%s":return String(r[e++]);case"%d":return Number(r[e++]);case"%j":try{return JSON.stringify(r[e++])}catch(n){return"[Circular]"}default:return n}})),f=r[e];e<i;f=r[++e])y(f)||!m(f)?u+=" "+f:u+=" "+c(f);return u},t.deprecate=function(e,r){if(void 0!==n&&!0===n.noDeprecation)return e;if(void 0===n)return function(){return t.deprecate(e,r).apply(this,arguments)};var o=!1;return function(){if(!o){if(n.throwDeprecation)throw new Error(r);n.traceDeprecation?console.trace(r):console.error(r),o=!0}return e.apply(this,arguments)}};var i,u={};function c(n,e){var r={seen:[],stylize:a};return arguments.length>=3&&(r.depth=arguments[2]),arguments.length>=4&&(r.colors=arguments[3]),g(e)?r.showHidden=e:e&&t._extend(r,e),h(r.showHidden)&&(r.showHidden=!1),h(r.depth)&&(r.depth=2),h(r.colors)&&(r.colors=!1),h(r.customInspect)&&(r.customInspect=!0),r.colors&&(r.stylize=f),l(r,n,r.depth)}function f(n,t){var e=c.styles[t];return e?"["+c.colors[e][0]+"m"+n+"["+c.colors[e][1]+"m":n}function a(n,t){return n}function l(n,e,r){if(n.customInspect&&e&&j(e.inspect)&&e.inspect!==t.inspect&&(!e.constructor||e.constructor.prototype!==e)){var o=e.inspect(r,n);return _(o)||(o=l(n,o,r)),o}var i=function(n,t){if(h(t))return n.stylize("undefined","undefined");if(_(t)){var e="'"+JSON.stringify(t).replace(/^"|"$/g,"").replace(/'/g,"\\'").replace(/\\"/g,'"')+"'";return n.stylize(e,"string")}if(b(t))return n.stylize(""+t,"number");if(g(t))return n.stylize(""+t,"boolean");if(y(t))return n.stylize("null","null")}(n,e);if(i)return i;var u=Object.keys(e),c=function(n){var t={};return n.forEach((function(n,e){t[n]=!0})),t}(u);if(n.showHidden&&(u=Object.getOwnPropertyNames(e)),O(e)&&(u.indexOf("message")>=0||u.indexOf("description")>=0))return s(e);if(0===u.length){if(j(e)){var f=e.name?": "+e.name:"";return n.stylize("[Function"+f+"]","special")}if(w(e))return n.stylize(RegExp.prototype.toString.call(e),"regexp");if(v(e))return n.stylize(Date.prototype.toString.call(e),"date");if(O(e))return s(e)}var a,m="",T=!1,x=["{","}"];(d(e)&&(T=!0,x=["[","]"]),j(e))&&(m=" [Function"+(e.name?": "+e.name:"")+"]");return w(e)&&(m=" "+RegExp.prototype.toString.call(e)),v(e)&&(m=" "+Date.prototype.toUTCString.call(e)),O(e)&&(m=" "+s(e)),0!==u.length||T&&0!=e.length?r<0?w(e)?n.stylize(RegExp.prototype.toString.call(e),"regexp"):n.stylize("[Object]","special"):(n.seen.push(e),a=T?function(n,t,e,r,o){for(var i=[],u=0,c=t.length;u<c;++u)D(t,String(u))?i.push(p(n,t,e,r,String(u),!0)):i.push("");return o.forEach((function(o){o.match(/^\d+$/)||i.push(p(n,t,e,r,o,!0))})),i}(n,e,r,c,u):u.map((function(t){return p(n,e,r,c,t,T)})),n.seen.pop(),function(n,t,e){if(n.reduce((function(n,t){return t.indexOf("\n")>=0&&0,n+t.replace(/\u001b\[\d\d?m/g,"").length+1}),0)>60)return e[0]+(""===t?"":t+"\n ")+" "+n.join(",\n  ")+" "+e[1];return e[0]+t+" "+n.join(", ")+" "+e[1]}(a,m,x)):x[0]+m+x[1]}function s(n){return"["+Error.prototype.toString.call(n)+"]"}function p(n,t,e,r,o,i){var u,c,f;if((f=Object.getOwnPropertyDescriptor(t,o)||{value:t[o]}).get?c=f.set?n.stylize("[Getter/Setter]","special"):n.stylize("[Getter]","special"):f.set&&(c=n.stylize("[Setter]","special")),D(r,o)||(u="["+o+"]"),c||(n.seen.indexOf(f.value)<0?(c=y(e)?l(n,f.value,null):l(n,f.value,e-1)).indexOf("\n")>-1&&(c=i?c.split("\n").map((function(n){return"  "+n})).join("\n").substr(2):"\n"+c.split("\n").map((function(n){return"   "+n})).join("\n")):c=n.stylize("[Circular]","special")),h(u)){if(i&&o.match(/^\d+$/))return c;(u=JSON.stringify(""+o)).match(/^"([a-zA-Z_][a-zA-Z_0-9]*)"$/)?(u=u.substr(1,u.length-2),u=n.stylize(u,"name")):(u=u.replace(/'/g,"\\'").replace(/\\"/g,'"').replace(/(^"|"$)/g,"'"),u=n.stylize(u,"string"))}return u+": "+c}function d(n){return Array.isArray(n)}function g(n){return"boolean"==typeof n}function y(n){return null===n}function b(n){return"number"==typeof n}function _(n){return"string"==typeof n}function h(n){return void 0===n}function w(n){return m(n)&&"[object RegExp]"===T(n)}function m(n){return"object"==typeof n&&null!==n}function v(n){return m(n)&&"[object Date]"===T(n)}function O(n){return m(n)&&("[object Error]"===T(n)||n instanceof Error)}function j(n){return"function"==typeof n}function T(n){return Object.prototype.toString.call(n)}function x(n){return n<10?"0"+n.toString(10):n.toString(10)}t.debuglog=function(e){if(h(i)&&(i=n.env.NODE_DEBUG||""),e=e.toUpperCase(),!u[e])if(new RegExp("\\b"+e+"\\b","i").test(i)){var r=n.pid;u[e]=function(){var n=t.format.apply(t,arguments);console.error("%s %d: %s",e,r,n)}}else u[e]=function(){};return u[e]},t.inspect=c,c.colors={bold:[1,22],italic:[3,23],underline:[4,24],inverse:[7,27],white:[37,39],grey:[90,39],black:[30,39],blue:[34,39],cyan:[36,39],green:[32,39],magenta:[35,39],red:[31,39],yellow:[33,39]},c.styles={special:"cyan",number:"yellow",boolean:"yellow",undefined:"grey",null:"bold",string:"green",date:"magenta",regexp:"red"},t.isArray=d,t.isBoolean=g,t.isNull=y,t.isNullOrUndefined=function(n){return null==n},t.isNumber=b,t.isString=_,t.isSymbol=function(n){return"symbol"==typeof n},t.isUndefined=h,t.isRegExp=w,t.isObject=m,t.isDate=v,t.isError=O,t.isFunction=j,t.isPrimitive=function(n){return null===n||"boolean"==typeof n||"number"==typeof n||"string"==typeof n||"symbol"==typeof n||void 0===n},t.isBuffer=e(5);var E=["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];function S(){var n=new Date,t=[x(n.getHours()),x(n.getMinutes()),x(n.getSeconds())].join(":");return[n.getDate(),E[n.getMonth()],t].join(" ")}function D(n,t){return Object.prototype.hasOwnProperty.call(n,t)}t.log=function(){console.log("%s - %s",S(),t.format.apply(t,arguments))},t.inherits=e(6),t._extend=function(n,t){if(!t||!m(t))return n;for(var e=Object.keys(t),r=e.length;r--;)n[e[r]]=t[e[r]];return n};var k="undefined"!=typeof Symbol?Symbol("util.promisify.custom"):void 0;function z(n,t){if(!n){var e=new Error("Promise was rejected with a falsy value");e.reason=n,n=e}return t(n)}t.promisify=function(n){if("function"!=typeof n)throw new TypeError('The "original" argument must be of type Function');if(k&&n[k]){var t;if("function"!=typeof(t=n[k]))throw new TypeError('The "util.promisify.custom" argument must be of type Function');return Object.defineProperty(t,k,{value:t,enumerable:!1,writable:!1,configurable:!0}),t}function t(){for(var t,e,r=new Promise((function(n,r){t=n,e=r})),o=[],i=0;i<arguments.length;i++)o.push(arguments[i]);o.push((function(n,r){n?e(n):t(r)}));try{n.apply(this,o)}catch(n){e(n)}return r}return Object.setPrototypeOf(t,Object.getPrototypeOf(n)),k&&Object.defineProperty(t,k,{value:t,enumerable:!1,writable:!1,configurable:!0}),Object.defineProperties(t,r(n))},t.promisify.custom=k,t.callbackify=function(t){if("function"!=typeof t)throw new TypeError('The "original" argument must be of type Function');function e(){for(var e=[],r=0;r<arguments.length;r++)e.push(arguments[r]);var o=e.pop();if("function"!=typeof o)throw new TypeError("The last argument must be of type Function");var i=this,u=function(){return o.apply(i,arguments)};t.apply(this,e).then((function(t){n.nextTick(u,null,t)}),(function(t){n.nextTick(z,t,u)}))}return Object.setPrototypeOf(e,Object.getPrototypeOf(t)),Object.defineProperties(e,r(t)),e}}).call(this,e(4))},function(n,t){var e;e=function(){return this}();try{e=e||new Function("return this")()}catch(n){"object"==typeof window&&(e=window)}n.exports=e},function(n,t){var e,r,o=n.exports={};function i(){throw new Error("setTimeout has not been defined")}function u(){throw new Error("clearTimeout has not been defined")}function c(n){if(e===setTimeout)return setTimeout(n,0);if((e===i||!e)&&setTimeout)return e=setTimeout,setTimeout(n,0);try{return e(n,0)}catch(t){try{return e.call(null,n,0)}catch(t){return e.call(this,n,0)}}}!function(){try{e="function"==typeof setTimeout?setTimeout:i}catch(n){e=i}try{r="function"==typeof clearTimeout?clearTimeout:u}catch(n){r=u}}();var f,a=[],l=!1,s=-1;function p(){l&&f&&(l=!1,f.length?a=f.concat(a):s=-1,a.length&&d())}function d(){if(!l){var n=c(p);l=!0;for(var t=a.length;t;){for(f=a,a=[];++s<t;)f&&f[s].run();s=-1,t=a.length}f=null,l=!1,function(n){if(r===clearTimeout)return clearTimeout(n);if((r===u||!r)&&clearTimeout)return r=clearTimeout,clearTimeout(n);try{r(n)}catch(t){try{return r.call(null,n)}catch(t){return r.call(this,n)}}}(n)}}function g(n,t){this.fun=n,this.array=t}function y(){}o.nextTick=function(n){var t=new Array(arguments.length-1);if(arguments.length>1)for(var e=1;e<arguments.length;e++)t[e-1]=arguments[e];a.push(new g(n,t)),1!==a.length||l||c(d)},g.prototype.run=function(){this.fun.apply(null,this.array)},o.title="browser",o.browser=!0,o.env={},o.argv=[],o.version="",o.versions={},o.on=y,o.addListener=y,o.once=y,o.off=y,o.removeListener=y,o.removeAllListeners=y,o.emit=y,o.prependListener=y,o.prependOnceListener=y,o.listeners=function(n){return[]},o.binding=function(n){throw new Error("process.binding is not supported")},o.cwd=function(){return"/"},o.chdir=function(n){throw new Error("process.chdir is not supported")},o.umask=function(){return 0}},function(n,t){n.exports=function(n){return n&&"object"==typeof n&&"function"==typeof n.copy&&"function"==typeof n.fill&&"function"==typeof n.readUInt8}},function(n,t){"function"==typeof Object.create?n.exports=function(n,t){n.super_=t,n.prototype=Object.create(t.prototype,{constructor:{value:n,enumerable:!1,writable:!0,configurable:!0}})}:n.exports=function(n,t){n.super_=t;var e=function(){};e.prototype=t.prototype,n.prototype=new e,n.prototype.constructor=n}},function(n,t,e){"use strict";var r=e.w[n.i];n.exports=r;e(1);r.n()}]]);