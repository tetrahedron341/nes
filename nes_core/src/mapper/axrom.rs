use super::Mapper;
use crate::cart::{Ines, Mirroring};

const BANK_SIZE: usize = 0x8000;

#[allow(non_snake_case)]
#[derive(Copy, Clone)]
pub struct AxROM {
    bank_select: u8,
    chr_ram: [u8; 0x2000],
    mirroring: Mirroring,
}

impl AxROM {
    pub fn new() -> Self {
        AxROM {
            bank_select: 0,
            chr_ram: [0; 0x2000],
            mirroring: Mirroring::OneScreenLowerBank,
        }
    }

    fn get_bank<'a>(&self, ines: &'a Ines) -> &'a [u8] {
        let prg_rom = ines.prg_rom_slice();
        let offset = self.bank_select as usize * BANK_SIZE;
        &prg_rom[offset .. offset+BANK_SIZE]
    }
}

impl Mapper for AxROM {
    fn name(&self) -> &'static str {
        "AxROM"
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
            },

            0x8000..=0xFFFF => {
                let bank = self.get_bank(ines);
                bank[addr as usize - 0x8000]
            },

            _ => 255
        }
    }
    fn write(&mut self, ines: &Ines, addr: u16, v: u8) {
        match addr {
            0x0000..=0x1FFF if ines.chr_rom_slice().is_none() => {
                self.chr_ram[addr as usize] = v;
            },
            0x8000..=0xFFFF => {
                self.bank_select = v & 0x07;
                self.mirroring = if v & 0x10 != 0 {
                    Mirroring::OneScreenUpperBank
                } else {
                    Mirroring::OneScreenLowerBank
                };
            },
            _ => {}
        }
    }
    fn reset(&mut self) {
        self.bank_select = 0;
    }
    fn clone(&self) -> Box<dyn Mapper + Send + Sync> {
        Box::new(std::clone::Clone::clone(self))
    }

    fn mirroring(&self) -> Option<Mirroring> {
        Some(self.mirroring)
    }
}