(window.webpackJsonp=window.webpackJsonp||[]).push([[1],[,function(n,t,e){"use strict";e.r(t),function(n){e.d(t,"init_emulator",(function(){return v})),e.d(t,"advance_frame",(function(){return p})),e.d(t,"insert_cartridge",(function(){return k})),e.d(t,"reset",(function(){return C})),e.d(t,"key_down",(function(){return x})),e.d(t,"key_up",(function(){return j})),e.d(t,"save_state",(function(){return D})),e.d(t,"load_state",(function(){return E})),e.d(t,"__wbindgen_object_drop_ref",(function(){return $})),e.d(t,"__wbindgen_string_new",(function(){return O})),e.d(t,"__wbg_new_59cb74e423758ede",(function(){return S})),e.d(t,"__wbg_stack_558ba5917b466edd",(function(){return F})),e.d(t,"__wbg_error_4bb6c2a97407129a",(function(){return M})),e.d(t,"__widl_instanceof_Window",(function(){return H})),e.d(t,"__widl_instanceof_CanvasRenderingContext2D",(function(){return J})),e.d(t,"__widl_f_put_image_data_CanvasRenderingContext2D",(function(){return L})),e.d(t,"__widl_f_get_element_by_id_Document",(function(){return R})),e.d(t,"__widl_instanceof_HTMLCanvasElement",(function(){return W})),e.d(t,"__widl_f_get_context_HTMLCanvasElement",(function(){return B})),e.d(t,"__widl_f_new_with_u8_clamped_array_ImageData",(function(){return U})),e.d(t,"__widl_f_document_Window",(function(){return N})),e.d(t,"__widl_f_log_1_",(function(){return q})),e.d(t,"__wbg_call_12b949cfc461d154",(function(){return z})),e.d(t,"__wbindgen_object_clone_ref",(function(){return G})),e.d(t,"__wbg_newnoargs_c4b2cbbd30e2d057",(function(){return K})),e.d(t,"__wbg_globalThis_22e06d4bea0084e3",(function(){return P})),e.d(t,"__wbg_self_00b0599bca667294",(function(){return Q})),e.d(t,"__wbg_window_aa795c5aad79b8ac",(function(){return V})),e.d(t,"__wbg_global_cc239dc2303f417c",(function(){return X})),e.d(t,"__wbindgen_is_undefined",(function(){return Y})),e.d(t,"__wbindgen_string_get",(function(){return Z})),e.d(t,"__wbindgen_debug_string",(function(){return nn})),e.d(t,"__wbindgen_throw",(function(){return tn})),e.d(t,"__wbindgen_rethrow",(function(){return en}));var r=e(3);const u=new Array(32);function o(n){return u[n]}u.fill(void 0),u.push(void 0,null,!0,!1);let c=u.length;function i(n){const t=o(n);return function(n){n<36||(u[n]=c,c=n)}(n),t}let f=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});f.decode();let _=null;function a(){return null!==_&&_.buffer===r.k.buffer||(_=new Uint8Array(r.k.buffer)),_}function d(n,t){return f.decode(a().subarray(n,n+t))}function l(n){c===u.length&&u.push(u.length+1);const t=c;return c=u[t],u[t]=n,t}let s=0,w=new TextEncoder("utf-8");const b="function"==typeof w.encodeInto?function(n,t){return w.encodeInto(n,t)}:function(n,t){const e=w.encode(n);return t.set(e),{read:n.length,written:e.length}};function g(n,t,e){if(void 0===e){const e=w.encode(n),r=t(e.length);return a().subarray(r,r+e.length).set(e),s=e.length,r}let r=n.length,u=t(r);const o=a();let c=0;for(;c<r;c++){const t=n.charCodeAt(c);if(t>127)break;o[u+c]=t}if(c!==r){0!==c&&(n=n.slice(c)),u=e(u,r,r=c+3*n.length);const t=a().subarray(u+c,u+r);c+=b(n,t).written}return s=c,u}function h(n){return null==n}let y=null;function m(){return null!==y&&y.buffer===r.k.buffer||(y=new Int32Array(r.k.buffer)),y}function v(){r.f()}function p(){r.e()}function k(n){var t=function(n,t){const e=t(1*n.length);return a().set(n,e/1),s=n.length,e}(n,r.c),e=s;r.g(t,e)}function C(){r.l()}function x(n){r.h(l(n))}function j(n){r.i(l(n))}function D(){r.m()}function E(){r.j()}function T(n){r.a(l(n))}let A=null;function I(n,t){return(null!==A&&A.buffer===r.k.buffer||(A=new Uint8ClampedArray(r.k.buffer)),A).subarray(n/1,n/1+t)}const $=function(n){i(n)},O=function(n,t){return l(d(n,t))},S=function(){return l(new Error)},F=function(n,t){var e=g(o(t).stack,r.c,r.d),u=s;m()[n/4+1]=u,m()[n/4+0]=e},M=function(n,t){try{console.error(d(n,t))}finally{r.b(n,t)}},H=function(n){return o(n)instanceof Window},J=function(n){return o(n)instanceof CanvasRenderingContext2D},L=function(n,t,e,r){try{o(n).putImageData(o(t),e,r)}catch(n){T(n)}},R=function(n,t,e){var r=o(n).getElementById(d(t,e));return h(r)?0:l(r)},W=function(n){return o(n)instanceof HTMLCanvasElement},B=function(n,t,e){try{var r=o(n).getContext(d(t,e));return h(r)?0:l(r)}catch(n){T(n)}},U=function(n,t,e){try{return l(new ImageData(I(n,t),e>>>0))}catch(n){T(n)}},N=function(n){var t=o(n).document;return h(t)?0:l(t)},q=function(n){console.log(o(n))},z=function(n,t){try{return l(o(n).call(o(t)))}catch(n){T(n)}},G=function(n){return l(o(n))},K=function(n,t){return l(new Function(d(n,t)))},P=function(){try{return l(globalThis.globalThis)}catch(n){T(n)}},Q=function(){try{return l(self.self)}catch(n){T(n)}},V=function(){try{return l(window.window)}catch(n){T(n)}},X=function(){try{return l(n.global)}catch(n){T(n)}},Y=function(n){return void 0===o(n)},Z=function(n,t){const e=o(t);var u="string"==typeof e?e:void 0,c=h(u)?0:g(u,r.c,r.d),i=s;m()[n/4+1]=i,m()[n/4+0]=c},nn=function(n,t){var e=g(function n(t){const e=typeof t;if("number"==e||"boolean"==e||null==t)return`${t}`;if("string"==e)return`"${t}"`;if("symbol"==e){const n=t.description;return null==n?"Symbol":`Symbol(${n})`}if("function"==e){const n=t.name;return"string"==typeof n&&n.length>0?`Function(${n})`:"Function"}if(Array.isArray(t)){const e=t.length;let r="[";e>0&&(r+=n(t[0]));for(let u=1;u<e;u++)r+=", "+n(t[u]);return r+="]",r}const r=/\[object ([^\]]+)\]/.exec(toString.call(t));let u;if(!(r.length>1))return toString.call(t);if(u=r[1],"Object"==u)try{return"Object("+JSON.stringify(t)+")"}catch(n){return"Object"}return t instanceof Error?`${t.name}: ${t.message}\n${t.stack}`:u}(o(t)),r.c,r.d),u=s;m()[n/4+1]=u,m()[n/4+0]=e},tn=function(n,t){throw new Error(d(n,t))},en=function(n){throw i(n)}}.call(this,e(2))},function(n,t){var e;e=function(){return this}();try{e=e||new Function("return this")()}catch(n){"object"==typeof window&&(e=window)}n.exports=e},function(n,t,e){"use strict";var r=e.w[n.i];n.exports=r;e(1);r.n()}]]);