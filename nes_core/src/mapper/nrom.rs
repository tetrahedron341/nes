use super::Mapper;
use super::Ines;

#[allow(non_snake_case)]
pub struct NROM {}

impl NROM {
    pub fn new() -> Self {
        NROM {}
    }
}

impl Mapper for NROM {
    fn name(&self) -> &'static str {
        "NROM"
    }
    fn read(&self, ines: &Ines, addr: u16) -> u8 {
        let prg_rom = ines.prg_rom_slice();
        let chr_rom = ines.chr_rom_slice();
        match addr {
            0x0000..=0x1fff => chr_rom[addr as usize],
            0x8000..=0xffff => prg_rom[(addr - 0x8000) as usize % prg_rom.len()],
            _ => 0xff
        }
    }
    fn write(&mut self, _: &Ines, _: u16, _: u8) { }
}