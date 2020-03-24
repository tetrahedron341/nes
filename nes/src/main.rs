use std::env;

mod emulator;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let rom_name = args.get(1).expect("Provide a path to a ROM as an argument.");

    emulator::run(rom_name.to_owned());
}