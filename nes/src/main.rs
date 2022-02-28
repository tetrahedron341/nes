use std::env;

mod emulator;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let rom_path = args.get(1).cloned();

    let flags = emulator::Flags { rom_path };

    if let Err(e) = emulator::run(flags) {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
