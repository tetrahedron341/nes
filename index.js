!function(e){function n(n){for(var t,o,a=n[0],_=n[1],u=0,i=[];u<a.length;u++)o=a[u],Object.prototype.hasOwnProperty.call(r,o)&&r[o]&&i.push(r[o][0]),r[o]=0;for(t in _)Object.prototype.hasOwnProperty.call(_,t)&&(e[t]=_[t]);for(s&&s(n);i.length;)i.shift()()}var t={},r={0:0};var o={};var a={7:function(){return{"./index.js":{__wbindgen_object_drop_ref:function(e){return t[1].exports.__wbindgen_object_drop_ref(e)},__wbindgen_string_new:function(e,n){return t[1].exports.__wbindgen_string_new(e,n)},__wbg_queueAudio_fed10246b96c0add:function(e,n){return t[1].exports.__wbg_queueAudio_fed10246b96c0add(e,n)},__wbg_instanceof_Window_04bba8b54ef81db0:function(e){return t[1].exports.__wbg_instanceof_Window_04bba8b54ef81db0(e)},__wbg_document_f023a2b0d5b3d060:function(e){return t[1].exports.__wbg_document_f023a2b0d5b3d060(e)},__wbg_getElementById_87fd6611f51eaa51:function(e,n,r){return t[1].exports.__wbg_getElementById_87fd6611f51eaa51(e,n,r)},__wbg_newwithu8clampedarray_21105846b882f367:function(e,n,r){return t[1].exports.__wbg_newwithu8clampedarray_21105846b882f367(e,n,r)},__wbg_instanceof_HtmlCanvasElement_69ef8df401e5d26d:function(e){return t[1].exports.__wbg_instanceof_HtmlCanvasElement_69ef8df401e5d26d(e)},__wbg_getContext_fc68e7f629e2b10a:function(e,n,r){return t[1].exports.__wbg_getContext_fc68e7f629e2b10a(e,n,r)},__wbg_instanceof_CanvasRenderingContext2d_a3cc87f343a7e4b9:function(e){return t[1].exports.__wbg_instanceof_CanvasRenderingContext2d_a3cc87f343a7e4b9(e)},__wbg_putImageData_35f77f1ba86bcdfc:function(e,n,r,o){return t[1].exports.__wbg_putImageData_35f77f1ba86bcdfc(e,n,r,o)},__wbg_call_183c0b733b35a027:function(e,n){return t[1].exports.__wbg_call_183c0b733b35a027(e,n)},__wbindgen_object_clone_ref:function(e){return t[1].exports.__wbindgen_object_clone_ref(e)},__wbg_newnoargs_4f6527054d7f1f1d:function(e,n){return t[1].exports.__wbg_newnoargs_4f6527054d7f1f1d(e,n)},__wbg_globalThis_eb9027a878db64ad:function(){return t[1].exports.__wbg_globalThis_eb9027a878db64ad()},__wbg_self_69a78003cf074413:function(){return t[1].exports.__wbg_self_69a78003cf074413()},__wbg_window_db757fdea9443777:function(){return t[1].exports.__wbg_window_db757fdea9443777()},__wbg_global_8efdae4f126ac8b4:function(){return t[1].exports.__wbg_global_8efdae4f126ac8b4()},__wbindgen_is_undefined:function(e){return t[1].exports.__wbindgen_is_undefined(e)},__wbg_new_59cb74e423758ede:function(){return t[1].exports.__wbg_new_59cb74e423758ede()},__wbg_stack_558ba5917b466edd:function(e,n){return t[1].exports.__wbg_stack_558ba5917b466edd(e,n)},__wbg_error_4bb6c2a97407129a:function(e,n){return t[1].exports.__wbg_error_4bb6c2a97407129a(e,n)},__wbindgen_string_get:function(e,n){return t[1].exports.__wbindgen_string_get(e,n)},__wbindgen_debug_string:function(e,n){return t[1].exports.__wbindgen_debug_string(e,n)},__wbindgen_throw:function(e,n){return t[1].exports.__wbindgen_throw(e,n)},__wbindgen_rethrow:function(e){return t[1].exports.__wbindgen_rethrow(e)}}}}};function _(n){if(t[n])return t[n].exports;var r=t[n]={i:n,l:!1,exports:{}};return e[n].call(r.exports,r,r.exports,_),r.l=!0,r.exports}_.e=function(e){var n=[],t=r[e];if(0!==t)if(t)n.push(t[2]);else{var u=new Promise((function(n,o){t=r[e]=[n,o]}));n.push(t[2]=u);var i,c=document.createElement("script");c.charset="utf-8",c.timeout=120,_.nc&&c.setAttribute("nonce",_.nc),c.src=function(e){return _.p+""+e+".index.js"}(e);var s=new Error;i=function(n){c.onerror=c.onload=null,clearTimeout(f);var t=r[e];if(0!==t){if(t){var o=n&&("load"===n.type?"missing":n.type),a=n&&n.target&&n.target.src;s.message="Loading chunk "+e+" failed.\n("+o+": "+a+")",s.name="ChunkLoadError",s.type=o,s.request=a,t[1](s)}r[e]=void 0}};var f=setTimeout((function(){i({type:"timeout",target:c})}),12e4);c.onerror=c.onload=i,document.head.appendChild(c)}return({1:[7]}[e]||[]).forEach((function(e){var t=o[e];if(t)n.push(t);else{var r,u=a[e](),i=fetch(_.p+""+{7:"1dfca3007e95989176e6"}[e]+".module.wasm");if(u instanceof Promise&&"function"==typeof WebAssembly.compileStreaming)r=Promise.all([WebAssembly.compileStreaming(i),u]).then((function(e){return WebAssembly.instantiate(e[0],e[1])}));else if("function"==typeof WebAssembly.instantiateStreaming)r=WebAssembly.instantiateStreaming(i,u);else{r=i.then((function(e){return e.arrayBuffer()})).then((function(e){return WebAssembly.instantiate(e,u)}))}n.push(o[e]=r.then((function(n){return _.w[e]=(n.instance||n).exports})))}})),Promise.all(n)},_.m=e,_.c=t,_.d=function(e,n,t){_.o(e,n)||Object.defineProperty(e,n,{enumerable:!0,get:t})},_.r=function(e){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},_.t=function(e,n){if(1&n&&(e=_(e)),8&n)return e;if(4&n&&"object"==typeof e&&e&&e.__esModule)return e;var t=Object.create(null);if(_.r(t),Object.defineProperty(t,"default",{enumerable:!0,value:e}),2&n&&"string"!=typeof e)for(var r in e)_.d(t,r,function(n){return e[n]}.bind(null,r));return t},_.n=function(e){var n=e&&e.__esModule?function(){return e.default}:function(){return e};return _.d(n,"a",n),n},_.o=function(e,n){return Object.prototype.hasOwnProperty.call(e,n)},_.p="",_.oe=function(e){throw console.error(e),e},_.w={};var u=window.webpackJsonp=window.webpackJsonp||[],i=u.push.bind(u);u.push=n,u=u.slice();for(var c=0;c<u.length;c++)n(u[c]);var s=i;_(_.s=0)}([function(e,n,t){const r=t.e(1).then(t.bind(null,1));let o=new AudioContext,a=o.createBuffer(1,4096,44100),_=null;window.queueAudio=function(e){null!=_&&_.stop();let n=o.createBufferSource();a.copyToChannel(e,0,0),n.buffer=a,n.connect(o.destination),n.start(),_=n},async function(){let e,n=await r,t=!1,o=!1,a=n.init_emulator(new n.Audio),_=null;function u(e){switch(e){case"KeyZ":return"a";case"KeyX":return"b";case"KeyG":return"select";case"KeyH":return"start";case"ArrowUp":return"up";case"ArrowRight":return"right";case"ArrowDown":return"down";case"ArrowLeft":return"left";default:return}}function i(e){e?(document.getElementById("pause_button").classList.remove("fa-pause"),document.getElementById("pause_button").classList.add("fa-play")):(document.getElementById("pause_button").classList.remove("fa-play"),document.getElementById("pause_button").classList.add("fa-pause")),t=e}function c(){o||i(!t)}function s(e){e?(i(!0),document.getElementById("pause_button").parentElement.style.backgroundColor="salmon"):document.getElementById("pause_button").parentElement.style.backgroundColor="",o=e}function f(){var r,o,_;n.reset(a),s(!1),null!=e&&cancelAnimationFrame(e),_=Date.now(),e=requestAnimationFrame((function u(){if(e=requestAnimationFrame(u),r=Date.now(),o=r-_,!t&&o>1e3/60){_=r-o%(1e3/60);try{n.advance_frame(a)}catch(n){throw cancelAnimationFrame(e),i(!0),s(!0),alert("An error has occured. Please reset the emulator or reload the ROM."),n}}}))}window.toggle_info=function(){let e=document.getElementById("info"),n=e.style.display;e.style.display="none"===n?"inherit":"none"},window.toggle_pause=c,window.reset_emulator=f,window.save_state=function(){_=n.save_state(a)},window.load_state=function(){null!=_&&n.load_state(a,_)};let d=document.getElementById("rom_input"),l=document.getElementById("nes_canvas").getContext("2d");l.fillStyle="#000000",l.fillRect(0,0,512,480),d.addEventListener("change",e=>{let t=new FileReader;t.onload=function(){var e=this.result,t=new Uint8Array(e);n.insert_cartridge(a,t),console.log("Inserted cartridge"),f()},t.readAsArrayBuffer(d.files[0])}),document.onkeydown=function(e){switch(e.code){case"KeyP":c();break;default:let t=u(e.code);t&&n.key_down(a,t)}},document.onkeyup=function(e){{e.code;let t=u(e.code);t&&n.key_up(a,t)}}}().then(()=>{}).catch(console.error)}]);