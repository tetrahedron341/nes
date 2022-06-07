/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./index.ts":
/*!******************!*\
  !*** ./index.ts ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg_index__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./pkg/index */ \"./pkg/index_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_pkg_index__WEBPACK_IMPORTED_MODULE_0__]);\n_pkg_index__WEBPACK_IMPORTED_MODULE_0__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\nconst NES_WIDTH = 256;\nconst NES_HEIGHT = 240;\nconst DISPLAY_SCALE = 2;\nconst DISPLAY_WIDTH = NES_WIDTH * DISPLAY_SCALE;\nconst DISPLAY_HEIGHT = NES_HEIGHT * DISPLAY_SCALE;\nconst FPS = 60.0;\nlet audio_ctx = new AudioContext();\nlet next_frame = null;\nlet audio_nodes = [\n    { buf: audio_ctx.createBuffer(1, 4096, 44100), starts: null },\n    { buf: audio_ctx.createBuffer(1, 4096, 44100), starts: null },\n    { buf: audio_ctx.createBuffer(1, 4096, 44100), starts: null },\n];\nlet audio_next_src = 0;\nlet audio_next_frame_start = 0;\nfunction queueAudio(samples) {\n    if (audio_nodes[audio_next_src].starts) {\n        console.log(\"RECIEVING SAMPLES TOO FAST, AUDIO FRAME DROPPED\");\n        return;\n    }\n    audio_nodes[audio_next_src].buf.copyToChannel(samples, 0, 0);\n    let s = new AudioBufferSourceNode(audio_ctx, { buffer: audio_nodes[audio_next_src].buf });\n    s.onended = (function () { this.starts = null; }).bind(audio_nodes[audio_next_src]);\n    s.connect(audio_ctx.destination);\n    let duration = 4096 / 44100;\n    if (audio_next_frame_start < audio_ctx.currentTime) {\n        audio_next_frame_start = audio_ctx.currentTime + 0.05; // Short delay to avoid calling `start` in the past\n    }\n    audio_nodes[audio_next_src].starts = audio_next_frame_start;\n    s.start(audio_next_frame_start, 0, duration);\n    audio_next_frame_start += duration;\n    audio_next_src = (audio_next_src + 1) % audio_nodes.length;\n}\nwindow[\"queueAudio\"] = queueAudio;\nfunction isAudioLagging() {\n    let now_audio = audio_ctx.currentTime;\n    for (let i = 0; i < audio_nodes.length; i++) {\n        let s = audio_nodes[i].starts;\n        if (s !== null && now_audio < s) {\n            return false;\n        }\n    }\n    return true;\n}\nfunction isAudioSaturated() {\n    return audio_nodes[audio_next_src].starts !== null;\n}\nasync function main() {\n    _pkg_index__WEBPACK_IMPORTED_MODULE_0__.initialize();\n    let paused = false;\n    let anim_frame_id;\n    let emulator_error = false;\n    let emulator = _pkg_index__WEBPACK_IMPORTED_MODULE_0__.init_emulator(new _pkg_index__WEBPACK_IMPORTED_MODULE_0__.Audio());\n    let saved_state = null;\n    function key_to_button(key) {\n        switch (key) {\n            case 'KeyZ':\n                return 'a';\n            case 'KeyX':\n                return 'b';\n            case 'KeyG':\n                return 'start';\n            case 'KeyH':\n                return 'select';\n            case 'ArrowUp':\n                return 'up';\n            case 'ArrowRight':\n                return 'right';\n            case 'ArrowDown':\n                return 'down';\n            case 'ArrowLeft':\n                return 'left';\n            default:\n                return undefined;\n        }\n    }\n    function toggle_info() {\n        let info = document.getElementById(\"info\");\n        let vis = info.style.display;\n        if (vis === \"none\") {\n            info.style.display = \"inherit\";\n        }\n        else {\n            info.style.display = \"none\";\n        }\n    }\n    window[\"toggle_info\"] = toggle_info;\n    function set_pause(p) {\n        if (p) {\n            document.getElementById(\"pause_button\").classList.remove(\"fa-pause\");\n            document.getElementById(\"pause_button\").classList.add(\"fa-play\");\n        }\n        else {\n            document.getElementById(\"pause_button\").classList.remove(\"fa-play\");\n            document.getElementById(\"pause_button\").classList.add(\"fa-pause\");\n        }\n        paused = p;\n    }\n    function toggle_pause() {\n        if (!emulator_error) {\n            set_pause(!paused);\n        }\n    }\n    window[\"toggle_pause\"] = toggle_pause;\n    function set_error(e) {\n        if (e) {\n            set_pause(true);\n            document.getElementById(\"pause_button\").parentElement.style.backgroundColor = \"salmon\";\n        }\n        else {\n            document.getElementById(\"pause_button\").parentElement.style.backgroundColor = \"\";\n        }\n        emulator_error = e;\n    }\n    function play() {\n        var fpsInterval = 1000.0 / FPS;\n        var then = Date.now();\n        var startTime = then;\n        var now, elapsed;\n        function play_frame() {\n            if (isAudioSaturated()) {\n                anim_frame_id = requestAnimationFrame(play_frame);\n                return;\n            }\n            while ( true && !isAudioLagging()) {\n                now = Date.now();\n                elapsed = now - then;\n                if (elapsed > fpsInterval) {\n                    then = now;\n                    break;\n                }\n            }\n            if (!paused) {\n                try {\n                    _pkg_index__WEBPACK_IMPORTED_MODULE_0__.advance_frame(emulator);\n                }\n                catch (e) {\n                    cancelAnimationFrame(anim_frame_id);\n                    set_pause(true);\n                    set_error(true);\n                    alert(\"An error has occured. Please reset the emulator or reload the ROM.\");\n                    throw e;\n                }\n            }\n            anim_frame_id = requestAnimationFrame(play_frame);\n        }\n        anim_frame_id = requestAnimationFrame(play_frame);\n    }\n    function reset_emulator() {\n        _pkg_index__WEBPACK_IMPORTED_MODULE_0__.reset(emulator);\n        set_error(false);\n        if (anim_frame_id != undefined) {\n            cancelAnimationFrame(anim_frame_id);\n        }\n        play();\n    }\n    window[\"reset_emulator\"] = reset_emulator;\n    function save_state() {\n        saved_state = _pkg_index__WEBPACK_IMPORTED_MODULE_0__.save_state(emulator);\n    }\n    window[\"save_state\"] = save_state;\n    function load_state() {\n        if (saved_state != null) {\n            _pkg_index__WEBPACK_IMPORTED_MODULE_0__.load_state(emulator, saved_state);\n        }\n    }\n    window[\"load_state\"] = load_state;\n    let fileInput = document.getElementById(\"rom_input\");\n    let canvas = document.getElementById(\"nes_canvas\");\n    let ctx = canvas.getContext(\"2d\");\n    ctx.fillStyle = \"#000000\";\n    ctx.fillRect(0, 0, DISPLAY_WIDTH, DISPLAY_HEIGHT);\n    fileInput.addEventListener(\"change\", e => {\n        let reader = new FileReader();\n        reader.onload = function () {\n            var arrayBuffer = this.result;\n            var array = new Uint8Array(arrayBuffer);\n            _pkg_index__WEBPACK_IMPORTED_MODULE_0__.insert_cartridge(emulator, array);\n            console.log(\"Inserted cartridge\");\n            reset_emulator();\n        };\n        reader.readAsArrayBuffer(fileInput.files[0]);\n    });\n    document.onkeydown = function (e) {\n        switch (e.code) {\n            case 'KeyP':\n                toggle_pause();\n                break;\n            default:\n                let button = key_to_button(e.code);\n                if (button) {\n                    _pkg_index__WEBPACK_IMPORTED_MODULE_0__.key_down(emulator, button);\n                }\n                break;\n        }\n    };\n    document.onkeyup = function (e) {\n        switch (e.code) {\n            default:\n                let button = key_to_button(e.code);\n                if (button) {\n                    _pkg_index__WEBPACK_IMPORTED_MODULE_0__.key_up(emulator, button);\n                }\n                break;\n        }\n    };\n}\nmain();\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://nes_wasm/./index.ts?");

/***/ }),

/***/ "./pkg/index_bg.js":
/*!*************************!*\
  !*** ./pkg/index_bg.js ***!
  \*************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   \"Audio\": () => (/* binding */ Audio),\n/* harmony export */   \"Nes\": () => (/* binding */ Nes),\n/* harmony export */   \"NesSaveState\": () => (/* binding */ NesSaveState),\n/* harmony export */   \"__wbg_call_ae78342adc33730a\": () => (/* binding */ __wbg_call_ae78342adc33730a),\n/* harmony export */   \"__wbg_document_99eddbbc11ec831e\": () => (/* binding */ __wbg_document_99eddbbc11ec831e),\n/* harmony export */   \"__wbg_error_09919627ac0992f5\": () => (/* binding */ __wbg_error_09919627ac0992f5),\n/* harmony export */   \"__wbg_getContext_0c19ba5c037e057f\": () => (/* binding */ __wbg_getContext_0c19ba5c037e057f),\n/* harmony export */   \"__wbg_getElementById_f83c5de20dc455d6\": () => (/* binding */ __wbg_getElementById_f83c5de20dc455d6),\n/* harmony export */   \"__wbg_globalThis_8e275ef40caea3a3\": () => (/* binding */ __wbg_globalThis_8e275ef40caea3a3),\n/* harmony export */   \"__wbg_global_5de1e0f82bddcd27\": () => (/* binding */ __wbg_global_5de1e0f82bddcd27),\n/* harmony export */   \"__wbg_instanceof_CanvasRenderingContext2d_405495bb0ea92c4f\": () => (/* binding */ __wbg_instanceof_CanvasRenderingContext2d_405495bb0ea92c4f),\n/* harmony export */   \"__wbg_instanceof_HtmlCanvasElement_b94545433bb4d2ef\": () => (/* binding */ __wbg_instanceof_HtmlCanvasElement_b94545433bb4d2ef),\n/* harmony export */   \"__wbg_instanceof_Window_0e6c0f1096d66c3c\": () => (/* binding */ __wbg_instanceof_Window_0e6c0f1096d66c3c),\n/* harmony export */   \"__wbg_new_693216e109162396\": () => (/* binding */ __wbg_new_693216e109162396),\n/* harmony export */   \"__wbg_newnoargs_e23b458e372830de\": () => (/* binding */ __wbg_newnoargs_e23b458e372830de),\n/* harmony export */   \"__wbg_newwithu8clampedarray_decce474908c8867\": () => (/* binding */ __wbg_newwithu8clampedarray_decce474908c8867),\n/* harmony export */   \"__wbg_putImageData_fad983ad6d58ee62\": () => (/* binding */ __wbg_putImageData_fad983ad6d58ee62),\n/* harmony export */   \"__wbg_queueAudio_fed10246b96c0add\": () => (/* binding */ __wbg_queueAudio_fed10246b96c0add),\n/* harmony export */   \"__wbg_self_99737b4dcdf6f0d8\": () => (/* binding */ __wbg_self_99737b4dcdf6f0d8),\n/* harmony export */   \"__wbg_stack_0ddaca5d1abfb52f\": () => (/* binding */ __wbg_stack_0ddaca5d1abfb52f),\n/* harmony export */   \"__wbg_window_9b61fbbf3564c4fb\": () => (/* binding */ __wbg_window_9b61fbbf3564c4fb),\n/* harmony export */   \"__wbindgen_debug_string\": () => (/* binding */ __wbindgen_debug_string),\n/* harmony export */   \"__wbindgen_is_undefined\": () => (/* binding */ __wbindgen_is_undefined),\n/* harmony export */   \"__wbindgen_object_clone_ref\": () => (/* binding */ __wbindgen_object_clone_ref),\n/* harmony export */   \"__wbindgen_object_drop_ref\": () => (/* binding */ __wbindgen_object_drop_ref),\n/* harmony export */   \"__wbindgen_string_get\": () => (/* binding */ __wbindgen_string_get),\n/* harmony export */   \"__wbindgen_string_new\": () => (/* binding */ __wbindgen_string_new),\n/* harmony export */   \"__wbindgen_throw\": () => (/* binding */ __wbindgen_throw),\n/* harmony export */   \"advance_frame\": () => (/* binding */ advance_frame),\n/* harmony export */   \"init_emulator\": () => (/* binding */ init_emulator),\n/* harmony export */   \"initialize\": () => (/* binding */ initialize),\n/* harmony export */   \"insert_cartridge\": () => (/* binding */ insert_cartridge),\n/* harmony export */   \"key_down\": () => (/* binding */ key_down),\n/* harmony export */   \"key_up\": () => (/* binding */ key_up),\n/* harmony export */   \"load_state\": () => (/* binding */ load_state),\n/* harmony export */   \"reset\": () => (/* binding */ reset),\n/* harmony export */   \"save_state\": () => (/* binding */ save_state)\n/* harmony export */ });\n/* harmony import */ var _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./index_bg.wasm */ \"./pkg/index_bg.wasm\");\n/* module decorator */ module = __webpack_require__.hmd(module);\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__]);\n_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetInt32Memory0 = new Int32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction debugString(val) {\n    // primitive types\n    const type = typeof val;\n    if (type == 'number' || type == 'boolean' || val == null) {\n        return  `${val}`;\n    }\n    if (type == 'string') {\n        return `\"${val}\"`;\n    }\n    if (type == 'symbol') {\n        const description = val.description;\n        if (description == null) {\n            return 'Symbol';\n        } else {\n            return `Symbol(${description})`;\n        }\n    }\n    if (type == 'function') {\n        const name = val.name;\n        if (typeof name == 'string' && name.length > 0) {\n            return `Function(${name})`;\n        } else {\n            return 'Function';\n        }\n    }\n    // objects\n    if (Array.isArray(val)) {\n        const length = val.length;\n        let debug = '[';\n        if (length > 0) {\n            debug += debugString(val[0]);\n        }\n        for(let i = 1; i < length; i++) {\n            debug += ', ' + debugString(val[i]);\n        }\n        debug += ']';\n        return debug;\n    }\n    // Test for built-in\n    const builtInMatches = /\\[object ([^\\]]+)\\]/.exec(toString.call(val));\n    let className;\n    if (builtInMatches.length > 1) {\n        className = builtInMatches[1];\n    } else {\n        // Failed to match the standard '[object ClassName]'\n        return toString.call(val);\n    }\n    if (className == 'Object') {\n        // we're a user defined class or Object\n        // JSON.stringify avoids problems with cycles, and is generally much\n        // easier than looping through ownProperties of `val`.\n        try {\n            return 'Object(' + JSON.stringify(val) + ')';\n        } catch (_) {\n            return 'Object';\n        }\n    }\n    // errors\n    if (val instanceof Error) {\n        return `${val.name}: ${val.message}\\n${val.stack}`;\n    }\n    // TODO we could test for more things here, like `Set`s and `Map`s.\n    return className;\n}\n\nlet cachegetFloat32Memory0 = null;\nfunction getFloat32Memory0() {\n    if (cachegetFloat32Memory0 === null || cachegetFloat32Memory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetFloat32Memory0 = new Float32Array(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetFloat32Memory0;\n}\n\nfunction getArrayF32FromWasm0(ptr, len) {\n    return getFloat32Memory0().subarray(ptr / 4, ptr / 4 + len);\n}\n/**\n*/\nfunction initialize() {\n    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.initialize();\n}\n\nfunction _assertClass(instance, klass) {\n    if (!(instance instanceof klass)) {\n        throw new Error(`expected instance of ${klass.name}`);\n    }\n    return instance.ptr;\n}\n/**\n* @param {Audio} audio\n* @returns {Nes}\n*/\nfunction init_emulator(audio) {\n    try {\n        const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);\n        _assertClass(audio, Audio);\n        var ptr0 = audio.ptr;\n        audio.ptr = 0;\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.init_emulator(retptr, ptr0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        var r2 = getInt32Memory0()[retptr / 4 + 2];\n        if (r2) {\n            throw takeObject(r1);\n        }\n        return Nes.__wrap(r0);\n    } finally {\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);\n    }\n}\n\n/**\n* @param {Nes} nes\n*/\nfunction advance_frame(nes) {\n    try {\n        const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);\n        _assertClass(nes, Nes);\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.advance_frame(retptr, nes.ptr);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        if (r1) {\n            throw takeObject(r0);\n        }\n    } finally {\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);\n    }\n}\n\nfunction passArray8ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 1);\n    getUint8Memory0().set(arg, ptr / 1);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n/**\n* @param {Nes} nes\n* @param {Uint8Array} rom\n*/\nfunction insert_cartridge(nes, rom) {\n    try {\n        const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);\n        _assertClass(nes, Nes);\n        const ptr0 = passArray8ToWasm0(rom, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc);\n        const len0 = WASM_VECTOR_LEN;\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.insert_cartridge(retptr, nes.ptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        if (r1) {\n            throw takeObject(r0);\n        }\n    } finally {\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);\n    }\n}\n\n/**\n* @param {Nes} nes\n*/\nfunction reset(nes) {\n    _assertClass(nes, Nes);\n    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.reset(nes.ptr);\n}\n\n/**\n* @param {Nes} nes\n* @param {any} button\n*/\nfunction key_down(nes, button) {\n    try {\n        const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);\n        _assertClass(nes, Nes);\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.key_down(retptr, nes.ptr, addHeapObject(button));\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        if (r1) {\n            throw takeObject(r0);\n        }\n    } finally {\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);\n    }\n}\n\n/**\n* @param {Nes} nes\n* @param {any} button\n*/\nfunction key_up(nes, button) {\n    try {\n        const retptr = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);\n        _assertClass(nes, Nes);\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.key_up(retptr, nes.ptr, addHeapObject(button));\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        if (r1) {\n            throw takeObject(r0);\n        }\n    } finally {\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);\n    }\n}\n\n/**\n* @param {Nes} nes\n* @returns {NesSaveState}\n*/\nfunction save_state(nes) {\n    _assertClass(nes, Nes);\n    const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.save_state(nes.ptr);\n    return NesSaveState.__wrap(ret);\n}\n\n/**\n* @param {Nes} nes\n* @param {NesSaveState} s\n*/\nfunction load_state(nes, s) {\n    _assertClass(nes, Nes);\n    _assertClass(s, NesSaveState);\n    _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.load_state(nes.ptr, s.ptr);\n}\n\nlet cachegetUint8ClampedMemory0 = null;\nfunction getUint8ClampedMemory0() {\n    if (cachegetUint8ClampedMemory0 === null || cachegetUint8ClampedMemory0.buffer !== _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {\n        cachegetUint8ClampedMemory0 = new Uint8ClampedArray(_index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);\n    }\n    return cachegetUint8ClampedMemory0;\n}\n\nfunction getClampedArrayU8FromWasm0(ptr, len) {\n    return getUint8ClampedMemory0().subarray(ptr / 1, ptr / 1 + len);\n}\n\nfunction handleError(f, args) {\n    try {\n        return f.apply(this, args);\n    } catch (e) {\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_exn_store(addHeapObject(e));\n    }\n}\n/**\n*/\nclass Audio {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Audio.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_audio_free(ptr);\n    }\n    /**\n    */\n    constructor() {\n        const ret = _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.audio_new();\n        return Audio.__wrap(ret);\n    }\n}\n/**\n*/\nclass Nes {\n\n    static __wrap(ptr) {\n        const obj = Object.create(Nes.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_nes_free(ptr);\n    }\n}\n/**\n*/\nclass NesSaveState {\n\n    static __wrap(ptr) {\n        const obj = Object.create(NesSaveState.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_nessavestate_free(ptr);\n    }\n}\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\nfunction __wbindgen_string_get(arg0, arg1) {\n    const obj = getObject(arg1);\n    const ret = typeof(obj) === 'string' ? obj : undefined;\n    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbindgen_string_new(arg0, arg1) {\n    const ret = getStringFromWasm0(arg0, arg1);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_queueAudio_fed10246b96c0add(arg0, arg1) {\n    queueAudio(getArrayF32FromWasm0(arg0, arg1));\n};\n\nfunction __wbg_instanceof_Window_0e6c0f1096d66c3c(arg0) {\n    const ret = getObject(arg0) instanceof Window;\n    return ret;\n};\n\nfunction __wbg_document_99eddbbc11ec831e(arg0) {\n    const ret = getObject(arg0).document;\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_getElementById_f83c5de20dc455d6(arg0, arg1, arg2) {\n    const ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_newwithu8clampedarray_decce474908c8867() { return handleError(function (arg0, arg1, arg2) {\n    const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0);\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_instanceof_HtmlCanvasElement_b94545433bb4d2ef(arg0) {\n    const ret = getObject(arg0) instanceof HTMLCanvasElement;\n    return ret;\n};\n\nfunction __wbg_getContext_0c19ba5c037e057f() { return handleError(function (arg0, arg1, arg2) {\n    const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_instanceof_CanvasRenderingContext2d_405495bb0ea92c4f(arg0) {\n    const ret = getObject(arg0) instanceof CanvasRenderingContext2D;\n    return ret;\n};\n\nfunction __wbg_putImageData_fad983ad6d58ee62() { return handleError(function (arg0, arg1, arg2, arg3) {\n    getObject(arg0).putImageData(getObject(arg1), arg2, arg3);\n}, arguments) };\n\nfunction __wbg_newnoargs_e23b458e372830de(arg0, arg1) {\n    const ret = new Function(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nfunction __wbg_call_ae78342adc33730a() { return handleError(function (arg0, arg1) {\n    const ret = getObject(arg0).call(getObject(arg1));\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbindgen_object_clone_ref(arg0) {\n    const ret = getObject(arg0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_self_99737b4dcdf6f0d8() { return handleError(function () {\n    const ret = self.self;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_window_9b61fbbf3564c4fb() { return handleError(function () {\n    const ret = window.window;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_globalThis_8e275ef40caea3a3() { return handleError(function () {\n    const ret = globalThis.globalThis;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_global_5de1e0f82bddcd27() { return handleError(function () {\n    const ret = __webpack_require__.g.global;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbindgen_is_undefined(arg0) {\n    const ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nfunction __wbg_new_693216e109162396() {\n    const ret = new Error();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {\n    const ret = getObject(arg1).stack;\n    const ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n    const len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_error_09919627ac0992f5(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(arg0, arg1);\n    }\n};\n\nfunction __wbindgen_debug_string(arg0, arg1) {\n    const ret = debugString(getObject(arg1));\n    const ptr0 = passStringToWasm0(ret, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _index_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);\n    const len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://nes_wasm/./pkg/index_bg.js?");

/***/ }),

/***/ "./pkg/index_bg.wasm":
/*!***************************!*\
  !*** ./pkg/index_bg.wasm ***!
  \***************************/
/***/ ((module, exports, __webpack_require__) => {

eval("var __webpack_instantiate__ = ([WEBPACK_IMPORTED_MODULE_0]) => {\n\treturn __webpack_require__.v(exports, module.id, \"70e45a0cf2020603b114\", {\n\t\t\"./index_bg.js\": {\n\t\t\t\"__wbindgen_object_drop_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref,\n\t\t\t\"__wbindgen_string_get\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_string_get,\n\t\t\t\"__wbindgen_string_new\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_string_new,\n\t\t\t\"__wbg_queueAudio_fed10246b96c0add\": WEBPACK_IMPORTED_MODULE_0.__wbg_queueAudio_fed10246b96c0add,\n\t\t\t\"__wbg_instanceof_Window_0e6c0f1096d66c3c\": WEBPACK_IMPORTED_MODULE_0.__wbg_instanceof_Window_0e6c0f1096d66c3c,\n\t\t\t\"__wbg_document_99eddbbc11ec831e\": WEBPACK_IMPORTED_MODULE_0.__wbg_document_99eddbbc11ec831e,\n\t\t\t\"__wbg_getElementById_f83c5de20dc455d6\": WEBPACK_IMPORTED_MODULE_0.__wbg_getElementById_f83c5de20dc455d6,\n\t\t\t\"__wbg_newwithu8clampedarray_decce474908c8867\": WEBPACK_IMPORTED_MODULE_0.__wbg_newwithu8clampedarray_decce474908c8867,\n\t\t\t\"__wbg_instanceof_HtmlCanvasElement_b94545433bb4d2ef\": WEBPACK_IMPORTED_MODULE_0.__wbg_instanceof_HtmlCanvasElement_b94545433bb4d2ef,\n\t\t\t\"__wbg_getContext_0c19ba5c037e057f\": WEBPACK_IMPORTED_MODULE_0.__wbg_getContext_0c19ba5c037e057f,\n\t\t\t\"__wbg_instanceof_CanvasRenderingContext2d_405495bb0ea92c4f\": WEBPACK_IMPORTED_MODULE_0.__wbg_instanceof_CanvasRenderingContext2d_405495bb0ea92c4f,\n\t\t\t\"__wbg_putImageData_fad983ad6d58ee62\": WEBPACK_IMPORTED_MODULE_0.__wbg_putImageData_fad983ad6d58ee62,\n\t\t\t\"__wbg_newnoargs_e23b458e372830de\": WEBPACK_IMPORTED_MODULE_0.__wbg_newnoargs_e23b458e372830de,\n\t\t\t\"__wbg_call_ae78342adc33730a\": WEBPACK_IMPORTED_MODULE_0.__wbg_call_ae78342adc33730a,\n\t\t\t\"__wbindgen_object_clone_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_clone_ref,\n\t\t\t\"__wbg_self_99737b4dcdf6f0d8\": WEBPACK_IMPORTED_MODULE_0.__wbg_self_99737b4dcdf6f0d8,\n\t\t\t\"__wbg_window_9b61fbbf3564c4fb\": WEBPACK_IMPORTED_MODULE_0.__wbg_window_9b61fbbf3564c4fb,\n\t\t\t\"__wbg_globalThis_8e275ef40caea3a3\": WEBPACK_IMPORTED_MODULE_0.__wbg_globalThis_8e275ef40caea3a3,\n\t\t\t\"__wbg_global_5de1e0f82bddcd27\": WEBPACK_IMPORTED_MODULE_0.__wbg_global_5de1e0f82bddcd27,\n\t\t\t\"__wbindgen_is_undefined\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_is_undefined,\n\t\t\t\"__wbg_new_693216e109162396\": WEBPACK_IMPORTED_MODULE_0.__wbg_new_693216e109162396,\n\t\t\t\"__wbg_stack_0ddaca5d1abfb52f\": WEBPACK_IMPORTED_MODULE_0.__wbg_stack_0ddaca5d1abfb52f,\n\t\t\t\"__wbg_error_09919627ac0992f5\": WEBPACK_IMPORTED_MODULE_0.__wbg_error_09919627ac0992f5,\n\t\t\t\"__wbindgen_debug_string\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_debug_string,\n\t\t\t\"__wbindgen_throw\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw\n\t\t}\n\t});\n}\n__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => {\n\ttry {\n\t/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./index_bg.js */ \"./pkg/index_bg.js\");\n\tvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([WEBPACK_IMPORTED_MODULE_0]);\n\tvar [WEBPACK_IMPORTED_MODULE_0] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__;\n\tawait __webpack_require__.v(exports, module.id, \"70e45a0cf2020603b114\", {\n\t\t\"./index_bg.js\": {\n\t\t\t\"__wbindgen_object_drop_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref,\n\t\t\t\"__wbindgen_string_get\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_string_get,\n\t\t\t\"__wbindgen_string_new\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_string_new,\n\t\t\t\"__wbg_queueAudio_fed10246b96c0add\": WEBPACK_IMPORTED_MODULE_0.__wbg_queueAudio_fed10246b96c0add,\n\t\t\t\"__wbg_instanceof_Window_0e6c0f1096d66c3c\": WEBPACK_IMPORTED_MODULE_0.__wbg_instanceof_Window_0e6c0f1096d66c3c,\n\t\t\t\"__wbg_document_99eddbbc11ec831e\": WEBPACK_IMPORTED_MODULE_0.__wbg_document_99eddbbc11ec831e,\n\t\t\t\"__wbg_getElementById_f83c5de20dc455d6\": WEBPACK_IMPORTED_MODULE_0.__wbg_getElementById_f83c5de20dc455d6,\n\t\t\t\"__wbg_newwithu8clampedarray_decce474908c8867\": WEBPACK_IMPORTED_MODULE_0.__wbg_newwithu8clampedarray_decce474908c8867,\n\t\t\t\"__wbg_instanceof_HtmlCanvasElement_b94545433bb4d2ef\": WEBPACK_IMPORTED_MODULE_0.__wbg_instanceof_HtmlCanvasElement_b94545433bb4d2ef,\n\t\t\t\"__wbg_getContext_0c19ba5c037e057f\": WEBPACK_IMPORTED_MODULE_0.__wbg_getContext_0c19ba5c037e057f,\n\t\t\t\"__wbg_instanceof_CanvasRenderingContext2d_405495bb0ea92c4f\": WEBPACK_IMPORTED_MODULE_0.__wbg_instanceof_CanvasRenderingContext2d_405495bb0ea92c4f,\n\t\t\t\"__wbg_putImageData_fad983ad6d58ee62\": WEBPACK_IMPORTED_MODULE_0.__wbg_putImageData_fad983ad6d58ee62,\n\t\t\t\"__wbg_newnoargs_e23b458e372830de\": WEBPACK_IMPORTED_MODULE_0.__wbg_newnoargs_e23b458e372830de,\n\t\t\t\"__wbg_call_ae78342adc33730a\": WEBPACK_IMPORTED_MODULE_0.__wbg_call_ae78342adc33730a,\n\t\t\t\"__wbindgen_object_clone_ref\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_clone_ref,\n\t\t\t\"__wbg_self_99737b4dcdf6f0d8\": WEBPACK_IMPORTED_MODULE_0.__wbg_self_99737b4dcdf6f0d8,\n\t\t\t\"__wbg_window_9b61fbbf3564c4fb\": WEBPACK_IMPORTED_MODULE_0.__wbg_window_9b61fbbf3564c4fb,\n\t\t\t\"__wbg_globalThis_8e275ef40caea3a3\": WEBPACK_IMPORTED_MODULE_0.__wbg_globalThis_8e275ef40caea3a3,\n\t\t\t\"__wbg_global_5de1e0f82bddcd27\": WEBPACK_IMPORTED_MODULE_0.__wbg_global_5de1e0f82bddcd27,\n\t\t\t\"__wbindgen_is_undefined\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_is_undefined,\n\t\t\t\"__wbg_new_693216e109162396\": WEBPACK_IMPORTED_MODULE_0.__wbg_new_693216e109162396,\n\t\t\t\"__wbg_stack_0ddaca5d1abfb52f\": WEBPACK_IMPORTED_MODULE_0.__wbg_stack_0ddaca5d1abfb52f,\n\t\t\t\"__wbg_error_09919627ac0992f5\": WEBPACK_IMPORTED_MODULE_0.__wbg_error_09919627ac0992f5,\n\t\t\t\"__wbindgen_debug_string\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_debug_string,\n\t\t\t\"__wbindgen_throw\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw\n\t\t}\n\t});\n\t__webpack_async_result__();\n\t} catch(e) { __webpack_async_result__(e); }\n}, 1);\n\n//# sourceURL=webpack://nes_wasm/./pkg/index_bg.wasm?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			loaded: false,
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Flag the module as loaded
/******/ 		module.loaded = true;
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/async module */
/******/ 	(() => {
/******/ 		var webpackThen = typeof Symbol === "function" ? Symbol("webpack then") : "__webpack_then__";
/******/ 		var webpackExports = typeof Symbol === "function" ? Symbol("webpack exports") : "__webpack_exports__";
/******/ 		var webpackError = typeof Symbol === "function" ? Symbol("webpack error") : "__webpack_error__";
/******/ 		var completeQueue = (queue) => {
/******/ 			if(queue) {
/******/ 				queue.forEach((fn) => (fn.r--));
/******/ 				queue.forEach((fn) => (fn.r-- ? fn.r++ : fn()));
/******/ 			}
/******/ 		}
/******/ 		var completeFunction = (fn) => (!--fn.r && fn());
/******/ 		var queueFunction = (queue, fn) => (queue ? queue.push(fn) : completeFunction(fn));
/******/ 		var wrapDeps = (deps) => (deps.map((dep) => {
/******/ 			if(dep !== null && typeof dep === "object") {
/******/ 				if(dep[webpackThen]) return dep;
/******/ 				if(dep.then) {
/******/ 					var queue = [];
/******/ 					dep.then((r) => {
/******/ 						obj[webpackExports] = r;
/******/ 						completeQueue(queue);
/******/ 						queue = 0;
/******/ 					}, (e) => {
/******/ 						obj[webpackError] = e;
/******/ 						completeQueue(queue);
/******/ 						queue = 0;
/******/ 					});
/******/ 					var obj = {};
/******/ 					obj[webpackThen] = (fn, reject) => (queueFunction(queue, fn), dep['catch'](reject));
/******/ 					return obj;
/******/ 				}
/******/ 			}
/******/ 			var ret = {};
/******/ 			ret[webpackThen] = (fn) => (completeFunction(fn));
/******/ 			ret[webpackExports] = dep;
/******/ 			return ret;
/******/ 		}));
/******/ 		__webpack_require__.a = (module, body, hasAwait) => {
/******/ 			var queue = hasAwait && [];
/******/ 			var exports = module.exports;
/******/ 			var currentDeps;
/******/ 			var outerResolve;
/******/ 			var reject;
/******/ 			var isEvaluating = true;
/******/ 			var nested = false;
/******/ 			var whenAll = (deps, onResolve, onReject) => {
/******/ 				if (nested) return;
/******/ 				nested = true;
/******/ 				onResolve.r += deps.length;
/******/ 				deps.map((dep, i) => (dep[webpackThen](onResolve, onReject)));
/******/ 				nested = false;
/******/ 			};
/******/ 			var promise = new Promise((resolve, rej) => {
/******/ 				reject = rej;
/******/ 				outerResolve = () => (resolve(exports), completeQueue(queue), queue = 0);
/******/ 			});
/******/ 			promise[webpackExports] = exports;
/******/ 			promise[webpackThen] = (fn, rejectFn) => {
/******/ 				if (isEvaluating) { return completeFunction(fn); }
/******/ 				if (currentDeps) whenAll(currentDeps, fn, rejectFn);
/******/ 				queueFunction(queue, fn);
/******/ 				promise['catch'](rejectFn);
/******/ 			};
/******/ 			module.exports = promise;
/******/ 			body((deps) => {
/******/ 				currentDeps = wrapDeps(deps);
/******/ 				var fn;
/******/ 				var getResult = () => (currentDeps.map((d) => {
/******/ 					if(d[webpackError]) throw d[webpackError];
/******/ 					return d[webpackExports];
/******/ 				}))
/******/ 				var promise = new Promise((resolve, reject) => {
/******/ 					fn = () => (resolve(getResult));
/******/ 					fn.r = 0;
/******/ 					whenAll(currentDeps, fn, reject);
/******/ 				});
/******/ 				return fn.r ? promise : getResult();
/******/ 			}, (err) => (err && reject(promise[webpackError] = err), outerResolve()));
/******/ 			isEvaluating = false;
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	(() => {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/harmony module decorator */
/******/ 	(() => {
/******/ 		__webpack_require__.hmd = (module) => {
/******/ 			module = Object.create(module);
/******/ 			if (!module.children) module.children = [];
/******/ 			Object.defineProperty(module, 'exports', {
/******/ 				enumerable: true,
/******/ 				set: () => {
/******/ 					throw new Error('ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: ' + module.id);
/******/ 				}
/******/ 			});
/******/ 			return module;
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/wasm loading */
/******/ 	(() => {
/******/ 		__webpack_require__.v = (exports, wasmModuleId, wasmModuleHash, importsObj) => {
/******/ 			var req = fetch(__webpack_require__.p + "" + wasmModuleHash + ".module.wasm");
/******/ 			if (typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 				return WebAssembly.instantiateStreaming(req, importsObj)
/******/ 					.then((res) => (Object.assign(exports, res.instance.exports)));
/******/ 			}
/******/ 			return req
/******/ 				.then((x) => (x.arrayBuffer()))
/******/ 				.then((bytes) => (WebAssembly.instantiate(bytes, importsObj)))
/******/ 				.then((res) => (Object.assign(exports, res.instance.exports)));
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		__webpack_require__.p = "";
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./index.ts");
/******/ 	
/******/ })()
;