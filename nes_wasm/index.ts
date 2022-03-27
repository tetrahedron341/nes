// Type declarations have to be pasted here because of webpack/webpack#6615
/**
*/
declare class Nes {
    free(): void;
}
/**
*/
declare class NesSaveState {
    free(): void;
}
const rust = import('./pkg/index');

const NES_WIDTH = 256;
const NES_HEIGHT = 240;

const DISPLAY_SCALE = 2;

const DISPLAY_WIDTH = NES_WIDTH * DISPLAY_SCALE;
const DISPLAY_HEIGHT = NES_HEIGHT * DISPLAY_SCALE;

const FPS = 60.0;

let audio_ctx = new AudioContext();
let audio_buf = audio_ctx.createBuffer(1, 4096, 44100);
let audio_source: AudioBufferSourceNode | null = null;

function queueAudio(samples: Float32Array) {
    if (audio_source != null) {
        audio_source.stop();
    }
    let source = audio_ctx.createBufferSource();
    audio_buf.copyToChannel(samples, 0, 0);
    source.buffer = audio_buf;
    source.connect(audio_ctx.destination);
    source.start();
    audio_source = source;
}
window["queueAudio"] = queueAudio;

async function main() {
    let nes = await rust;

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
            anim_frame_id = requestAnimationFrame(play_frame);

            now = Date.now();
            elapsed = now - then;

            if (!paused && elapsed > fpsInterval) {
                then = now - (elapsed % fpsInterval);
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

main().then(() => {

}).catch(console.error);