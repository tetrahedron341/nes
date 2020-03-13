extern crate nes_core;

static BASICS_ROM: &'static [u8] = include_bytes!("../test_roms/01-basics.nes");
static IMPLIED_ROM: &'static [u8] = include_bytes!("../test_roms/02-implied_official_only.nes");
static IMMEDIATE_ROM: &'static [u8] = include_bytes!("../test_roms/03-immediate_official_only.nes");
static ZERO_PAGE_ROM: &'static [u8] = include_bytes!("../test_roms/04-zero_page_official_only.nes");
static ZP_XY_ROM: &'static [u8] = include_bytes!("../test_roms/05-zp_xy_official_only.nes");
static ABSOLUTE_ROM: &'static [u8] = include_bytes!("../test_roms/06-absolute_official_only.nes");
static ABS_XY_ROM: &'static [u8] = include_bytes!("../test_roms/07-abs_xy_official_only.nes");
static IND_X_ROM: &'static [u8] = include_bytes!("../test_roms/08-ind_x_official_only.nes");
static IND_Y_ROM: &'static [u8] = include_bytes!("../test_roms/09-ind_y_official_only.nes");
static BRANCHES_ROM: &'static [u8] = include_bytes!("../test_roms/10-branches.nes");
static STACK_ROM: &'static [u8] = include_bytes!("../test_roms/11-stack.nes");
static JMP_JSR_ROM: &'static [u8] = include_bytes!("../test_roms/12-jmp_jsr.nes");
static RTS_ROM: &'static [u8] = include_bytes!("../test_roms/13-rts.nes");
static RTI_ROM: &'static [u8] = include_bytes!("../test_roms/14-rti.nes");
static BRK_ROM: &'static [u8] = include_bytes!("../test_roms/15-brk.nes");
static SPECIAL_ROM: &'static [u8] = include_bytes!("../test_roms/16-special.nes");

fn run_blargg_test(rom: &'static [u8]) {
    let rom = Vec::from(rom);
    let cart = nes_core::cart::Cart::from_bytes(rom).unwrap();
    let mut nes = nes_core::nes::Nes::new(cart, nes_core::ppu::DummyVideo(), nes_core::controller::DummyController(), nes_core::apu::DummyAudio(), Some(nes_core::nes::NESConfig::DEBUG));

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
fn basics() {
    run_blargg_test(BASICS_ROM)
}
#[test]
fn implied() {
    run_blargg_test(IMPLIED_ROM)
}
#[test]
fn immediate() {
    run_blargg_test(IMMEDIATE_ROM)
}
#[test]
fn zero_page() {
    run_blargg_test(ZERO_PAGE_ROM)
}
#[test]
fn zp_xy() {
    run_blargg_test(ZP_XY_ROM)
}
#[test]
fn absolute() {
    run_blargg_test(ABSOLUTE_ROM)
}
#[test]
fn abs_xy() {
    run_blargg_test(ABS_XY_ROM)
}
#[test]
fn ind_x() {
    run_blargg_test(IND_X_ROM)
}
#[test]
fn ind_y() {
    run_blargg_test(IND_Y_ROM)
}
#[test]
fn branches() {
    run_blargg_test(BRANCHES_ROM)
}
#[test]
fn stack() {
    run_blargg_test(STACK_ROM)
}
#[test]
fn jmp_jsr() {
    run_blargg_test(JMP_JSR_ROM)
}
#[test]
fn rts() {
    run_blargg_test(RTS_ROM)
}
#[test]
fn rti() {
    run_blargg_test(RTI_ROM)
}
#[test]
fn brk() {
    run_blargg_test(BRK_ROM)
}
#[test]
fn special() {
    run_blargg_test(SPECIAL_ROM)
}