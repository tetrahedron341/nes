const rust = import('./pkg/index');

let canvas = document.createElement("canvas");
canvas.id = "nes_canvas";
canvas.width = 256;
canvas.height = 240;

document.body.appendChild(canvas);

let fileInput = document.createElement("input");
fileInput.id = "rom_input";
fileInput.type = "file";
fileInput.accept = ".nes";

document.body.appendChild(fileInput);

rust.then(
    m => {
        m.init_emulator();

        fileInput.addEventListener("change", e => {
            let reader = new FileReader();
            reader.onload = function() {
                var arrayBuffer = this.result;
                var array = new Uint8Array(arrayBuffer);
                m.insert_cartridge(array);
            }
            reader.readAsArrayBuffer(fileInput.files[0])
        });

        window.rust = m;
    }
).catch(console.error);