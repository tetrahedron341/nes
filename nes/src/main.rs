use std::env;

use color_eyre::eyre::Result;

mod emulator;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = env::args().collect::<Vec<_>>();
    let rom_path = args.get(1).cloned();

    let flags = emulator::Flags { rom_path };

    emulator::run(flags)
}
