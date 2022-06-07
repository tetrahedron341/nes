import { Nes, NesSaveState } from "./pkg/index";
import * as nes from "./pkg/index";

const NES_WIDTH = 256;
const NES_HEIGHT = 240;

const DISPLAY_SCALE = 2;

const DISPLAY_WIDTH = NES_WIDTH * DISPLAY_SCALE;
const DISPLAY_HEIGHT = NES_HEIGHT * DISPLAY_SCALE;

const FPS = 60.0;

let audio_ctx = new AudioContext();
let next_frame = null;

let audio_nodes: Array<{ buf: AudioBuffer, starts: null | number }> = [
    { buf: audio_ctx.createBuffer(1, 4096, 44100), starts: null },
    { buf: audio_ctx.createBuffer(1, 4096, 44100), starts: null },
    { buf: audio_ctx.createBuffer(1, 4096, 44100), starts: null },
]
let audio_next_src: number = 0;
let audio_next_frame_start: number = 0;

function queueAudio(samples: Float32Array) {
    if (audio_nodes[audio_next_src].starts) {
        console.log("RECIEVING SAMPLES TOO FAST, AUDIO FRAME DROPPED");
        return;
    }

    audio_nodes[audio_next_src].buf.copyToChannel(samples, 0, 0);

    let s = new AudioBufferSourceNode(audio_ctx, { buffer: audio_nodes[audio_next_src].buf });
    s.onended = (function () { this.starts = null; }).bind(audio_nodes[audio_next_src]);
    s.connect(audio_ctx.destination);
    let duration = 4096 / 44100;

    if (audio_next_frame_start < audio_ctx.currentTime) {
        audio_next_frame_start = audio_ctx.currentTime + 0.05; // Short delay to avoid calling `start` in the past
    }
    audio_nodes[audio_next_src].starts = audio_next_frame_start;
    s.start(audio_next_frame_start, 0, duration);

    audio_next_frame_start += duration;
    audio_next_src = (audio_next_src + 1) % audio_nodes.length;
}
window["queueAudio"] = queueAudio;

function isAudioLagging(): boolean {
    let now_audio = audio_ctx.currentTime;
    for (let i = 0; i < audio_nodes.length; i++) {
        let s = audio_nodes[i].starts;
        if (s !== null && now_audio < s) {
            return false;
        }
    }
    return true;
}

function isAudioSaturated(): boolean {
    return audio_nodes[audio_next_src].starts !== null;
}

async function main() {
    nes.initialize();

    let paused = false;
    let anim_frame_id;
    let emulator_error = false;

    let emulator: Nes = nes.init_emulator(new nes.Audio());

    let saved_state: NesSaveState | null = null;

    function key_to_button(key) {
        switch (key) {
            case 'KeyZ':
                return 'a';
            case 'KeyX':
                return 'b';
            case 'KeyG':
                return 'start';
            case 'KeyH':
                return 'select';
            case 'ArrowUp':
                return 'up';
            case 'ArrowRight':
                return 'right';
            case 'ArrowDown':
                return 'down';
            case 'ArrowLeft':
                return 'left';

            default:
                return undefined;
        }
    }

    function toggle_info() {
        let info = document.getElementById("info")!;
        let vis = info.style.display;
        if (vis === "none") {
            info.style.display = "inherit";
        } else {
            info.style.display = "none";
        }
    }
    window["toggle_info"] = toggle_info;

    function set_pause(p) {
        if (p) {
            document.getElementById("pause_button")!.classList.remove("fa-pause");
            document.getElementById("pause_button")!.classList.add("fa-play");
        } else {
            document.getElementById("pause_button")!.classList.remove("fa-play");
            document.getElementById("pause_button")!.classList.add("fa-pause");
        }
        paused = p;
    }

    function toggle_pause() {
        if (!emulator_error) {
            set_pause(!paused);
        }
    }
    window["toggle_pause"] = toggle_pause;

    function set_error(e) {
        if (e) {
            set_pause(true);
            document.getElementById("pause_button")!.parentElement!.style.backgroundColor = "salmon";
        } else {
            document.getElementById("pause_button")!.parentElement!.style.backgroundColor = "";
        }
        emulator_error = e;
    }

    function play() {
        var fpsInterval = 1000.0 / FPS;
        var then = Date.now();
        var startTime = then;
        var now, elapsed;

        function play_frame() {
            if (isAudioSaturated()) {
                anim_frame_id = requestAnimationFrame(play_frame);
                return;
            }

            while (true && !isAudioLagging()) {
                now = Date.now();
                elapsed = now - then;
                if (elapsed > fpsInterval) {
                    then = now;
                    break;
                }
            }

            if (!paused) {
                try {
                    nes.advance_frame(emulator);
                } catch (e) {
                    cancelAnimationFrame(anim_frame_id);
                    set_pause(true);
                    set_error(true);
                    alert("An error has occured. Please reset the emulator or reload the ROM.")
                    throw e;
                }
            }

            anim_frame_id = requestAnimationFrame(play_frame);
        }

        anim_frame_id = requestAnimationFrame(play_frame);
    }

    function reset_emulator() {
        nes.reset(emulator);
        set_error(false);
        if (anim_frame_id != undefined) {
            cancelAnimationFrame(anim_frame_id);
        }
        play();
    }
    window["reset_emulator"] = reset_emulator;

    function save_state() {
        saved_state = nes.save_state(emulator);
    }
    window["save_state"] = save_state;

    function load_state() {
        if (saved_state != null) {
            nes.load_state(emulator, saved_state);
        }
    }
    window["load_state"] = load_state;

    let fileInput = document.getElementById("rom_input") as HTMLInputElement;

    let canvas = document.getElementById("nes_canvas") as HTMLCanvasElement;
    let ctx = canvas.getContext("2d")!;
    ctx.fillStyle = "#000000";
    ctx.fillRect(0, 0, DISPLAY_WIDTH, DISPLAY_HEIGHT);

    fileInput.addEventListener("change", e => {
        let reader = new FileReader();
        reader.onload = function () {
            var arrayBuffer = this.result as ArrayBuffer;
            var array = new Uint8Array(arrayBuffer);
            nes.insert_cartridge(emulator, array);
            console.log("Inserted cartridge");
            reset_emulator();
        }
        reader.readAsArrayBuffer(fileInput.files![0])
    });

    document.onkeydown = function (e) {
        switch (e.code) {
            case 'KeyP':
                toggle_pause();
                break;
            default:
                let button = key_to_button(e.code);
                if (button) {
                    nes.key_down(emulator, button);
                }
                break;
        }
    };

    document.onkeyup = function (e) {
        switch (e.code) {
            default:
                let button = key_to_button(e.code);
                if (button) {
                    nes.key_up(emulator, button);
                }
                break;
        }
    }
}

main();