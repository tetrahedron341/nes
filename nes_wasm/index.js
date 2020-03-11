const rust = import('./pkg/index');

const NES_WIDTH = 256;
const NES_HEIGHT = 240;

const DISPLAY_SCALE = 2;

const DISPLAY_WIDTH = NES_WIDTH * DISPLAY_SCALE;
const DISPLAY_HEIGHT = NES_HEIGHT * DISPLAY_SCALE;

const FPS = 60.0;

var paused = false;
var anim_frame_id;
var emulator_error = false;

function key_to_button(key) {
    switch (key) {
        case 'KeyZ':
            return 'a';
        case 'KeyX':
            return 'b';
        case 'KeyG':
            return 'select';
        case 'KeyH':
            return 'start';
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
    let info = document.getElementById("info");
    let vis = info.style.display;
    if (vis === "none") {
        info.style.display = "inherit";
    } else {
        info.style.display = "none";
    }
}
window.toggle_info = toggle_info;

rust.then(
    m => {
        m.init_emulator();
        
        window.set_pause = function (p) {
            if (p) {
                document.getElementById("pause_button").classList.remove("fa-pause");
                document.getElementById("pause_button").classList.add("fa-play");
            } else {
                document.getElementById("pause_button").classList.remove("fa-play");
                document.getElementById("pause_button").classList.add("fa-pause");
            }
            paused = p;
        }
        
        window.toggle_pause = function () {
            if (!emulator_error) {
                set_pause(!paused);
            }
        }

        window.set_error = function (e) {
            if (e) {
                set_pause(true);
                document.getElementById("pause_button").parentElement.style.backgroundColor = "salmon";
            } else {
                document.getElementById("pause_button").parentElement.style.backgroundColor = null;
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
                        m.advance_frame();
                    } catch (e) {
                        cancelAnimationFrame(anim_frame_id);
                        set_pause(true);
                        set_error(true);
                        console.error(e);
                        alert("An error has occured. Please reset the emulator or reload the ROM.")
                    }
                }
            }

            anim_frame_id = requestAnimationFrame(play_frame);
        }

        window.reset_emulator = function () {
            m.reset();
            set_error(false);
            if (anim_frame_id != undefined) {
                cancelAnimationFrame(anim_frame_id);
            }
            play();
        }

        window.save_state = function () {
            m.save_state();
        }

        window.load_state = function () {
            m.load_state();
        }

        let fileInput = document.getElementById("rom_input");

        /** @type {HTMLCanvasElement} */
        let canvas = document.getElementById("nes_canvas");
        let ctx = canvas.getContext("2d");
        ctx.fillStyle = "#000000";
        ctx.fillRect(0,0,DISPLAY_WIDTH,DISPLAY_HEIGHT);

        fileInput.addEventListener("change", e => {
            let reader = new FileReader();
            reader.onload = function() {
                var arrayBuffer = this.result;
                var array = new Uint8Array(arrayBuffer);
                m.insert_cartridge(array);
                console.log("Inserted cartridge");
                reset_emulator();
            }
            reader.readAsArrayBuffer(fileInput.files[0])
        });

        document.onkeydown = function (e) {
            switch (e.code) {
                case 'KeyP':
                    toggle_pause();
                    break;
                default:
                    let button = key_to_button(e.code);
                    if (button) {
                        m.key_down(button);
                    }
                    break;
            }
        };

        document.onkeyup = function (e) {
            switch (e.code) {
                default:
                    let button = key_to_button(e.code);
                    if (button) {
                        m.key_up(button);
                    }
                    break;
            }
        }

        window.rust = m;
    }
).catch(console.error);