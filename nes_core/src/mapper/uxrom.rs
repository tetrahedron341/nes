use super::Mapper;
use crate::cart::Ines;

const BANK_SIZE: usize = 0x4000;

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct UxROM {
    bank_select: u8,
    chr_ram: [u8; 0x2000],
}

impl UxROM {
    pub fn new() -> Self {
        UxROM {
            bank_select: 0,
            chr_ram: [0; 0x2000],
        }
    }

    fn get_last_bank<'a>(&self, ines: &'a Ines) -> &'a [u8] {
        let prg_rom = ines.prg_rom_slice();
        &prg_rom[prg_rom.len() - BANK_SIZE..]
    }

    fn get_first_bank<'a>(&self, ines: &'a Ines) -> &'a [u8] {
        let prg_rom = ines.prg_rom_slice();
        let offset = self.bank_select as usize * BANK_SIZE;
        &prg_rom[offset..offset + BANK_SIZE]
    }
}

impl Mapper for UxROM {
    fn name(&self) -> &'static str {
        "UxROM"
    }
    fn read(&self, ines: &Ines, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => {
                let chr = ines.chr_rom_slice();
                if let Some(chr) = chr {
                    chr[addr as usize]
                } else {
                    self.chr_ram[addr as usize]
                }
            }

            0x8000..=0xBFFF => {
                let bank = self.get_first_bank(ines);
                bank[addr as usize - 0x8000]
            }
            0xC000..=0xFFFF => {
                let bank = self.get_last_bank(ines);
                bank[addr as usize - 0xC000]
            }

            _ => 255,
        }
    }
    fn write(&mut self, ines: &Ines, addr: u16, v: u8) {
        match addr {
            0x0000..=0x1FFF if ines.chr_rom_slice().is_none() => {
                self.chr_ram[addr as usize] = v;
            }
            0x8000..=0xFFFF => {
                // The real boards usually used only the low 3-4 bits, im just going to use the full 8 bits
                self.bank_select = v;
            }
            _ => {}
        }
    }
    fn reset(&mut self) {
        self.bank_select = 0;
    }
    fn clone(&self) -> Box<dyn Mapper + Send + Sync> {
        Box::new(std::clone::Clone::clone(self))
    }
}
