extern crate nes_core;

static INSTRS_ROM: &'static [u8] = include_bytes!("../test_roms/official_only.nes");

fn run_blargg_test(rom: &'static [u8]) {
    let rom = Vec::from(rom);
    let cart = nes_core::cart::Cart::from_bytes(rom).unwrap();
    let mut nes = nes_core::nes::Nes::new(
        cart, 
        nes_core::ppu::DummyVideo(), 
        nes_core::controller::DummyController(), 
        nes_core::apu::DummyAudio(), 
        Some(nes_core::nes::NESConfig::DEBUG));

    let code = loop {
        nes.master_clock_tick().unwrap();
        let status = nes.mmu.blargg_debug_status();
        if status < 0x80 {
            break status
        } else if status == 0x81 {
            nes.cpu.reset();
        }
    };

    let output = nes.mmu.blargg_debug_text();
    for c in output {
        if c.is_ascii() {
            print!("{}", *c as char);
        }
    }
    println!();
    println!("Test finished with code: {}", code);

    assert_eq!(code, 0);
}

#[test]
fn official_instructions() {
    run_blargg_test(INSTRS_ROM)
}
