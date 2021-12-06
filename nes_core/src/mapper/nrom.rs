use super::Ines;
use super::Mapper;

#[allow(clippy::upper_case_acronyms)]
pub struct NROM {
    chr_ram: [u8; 0x2000],
}

impl NROM {
    pub fn new() -> Self {
        NROM {
            chr_ram: [0; 0x2000],
        }
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
            0x0000..=0x1fff => {
                if let Some(chr_rom) = chr_rom {
                    chr_rom[addr as usize]
                } else {
                    self.chr_ram[addr as usize]
                }
            }
            0x8000..=0xffff => prg_rom[(addr - 0x8000) as usize % prg_rom.len()],
            _ => 0xff,
        }
    }
    fn write(&mut self, _: &Ines, _: u16, _: u8) {}
    fn reset(&mut self) {}
    fn clone(&self) -> Box<dyn Mapper + Send + Sync> {
        Box::new(NROM { ..*self })
    }
}
