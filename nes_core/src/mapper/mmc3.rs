use crate::cart::Mirroring;

#[derive(Clone)]
pub struct MMC3 {
    bank_select: BankSelectRegister,

    bank_data: [u8; 8],

    mirroring: Mirroring,

    irq_latch: u8,
    irq_counter: u8,
    // TODO: implement scanline IRQs
    irq_enable: bool,

    prg_ram: [u8; 0x2000],
}

impl MMC3 {
    pub fn new() -> Self {
        MMC3 {
            bank_select: BankSelectRegister::empty(),
            bank_data: [0; 8],
            mirroring: Mirroring::Vertical,
            irq_latch: 0,
            irq_counter: 0,
            irq_enable: false,
            prg_ram: [0; 0x2000],
        }
    }

    fn chr_bank_number(&self, addr: u16) -> u8 {
        if self.bank_select.contains(BankSelectRegister::CHR_A12_INV) {
            match addr {
                (0x0000..=0x03FF) => self.bank_data[2],
                (0x0400..=0x07FF) => self.bank_data[3],
                (0x0800..=0x0BFF) => self.bank_data[4],
                (0x0C00..=0x0FFF) => self.bank_data[5],
                (0x1000..=0x13FF) => self.bank_data[0],
                (0x1400..=0x17FF) => self.bank_data[0] + 1,
                (0x1800..=0x1BFF) => self.bank_data[1],
                (0x1C00..=0x1FFF) => self.bank_data[1] + 1,
                _ => unreachable!(),
            }
        } else {
            match addr {
                (0x0000..=0x03FF) => self.bank_data[0],
                (0x0400..=0x07FF) => self.bank_data[0] + 1,
                (0x0800..=0x0BFF) => self.bank_data[1],
                (0x0C00..=0x0FFF) => self.bank_data[1] + 1,
                (0x1000..=0x13FF) => self.bank_data[2],
                (0x1400..=0x17FF) => self.bank_data[3],
                (0x1800..=0x1BFF) => self.bank_data[4],
                (0x1C00..=0x1FFF) => self.bank_data[5],
                _ => unreachable!(),
            }
        }
    }

    fn prg_bank_number(&self, addr: u16) -> u8 {
        if self.bank_select.contains(BankSelectRegister::PRG_ROM_MODE) {
            match addr {
                (0x8000..=0x9FFF) => 62,
                (0xA000..=0xBFFF) => self.bank_data[7],
                (0xC000..=0xDFFF) => self.bank_data[6],
                (0xE000..=0xFFFF) => 63,
                _ => unreachable!(),
            }
        } else {
            match addr {
                (0x8000..=0x9FFF) => self.bank_data[6],
                (0xA000..=0xBFFF) => self.bank_data[7],
                (0xC000..=0xDFFF) => 62,
                (0xE000..=0xFFFF) => 63,
                _ => unreachable!(),
            }
        }
    }
}

impl super::Mapper for MMC3 {
    fn name(&self) -> &'static str {
        "MMC3"
    }

    fn read(&self, ines: &crate::cart::Ines, addr: u16) -> u8 {
        match addr {
            // CHR
            (0x0000..=0x1FFF) => {
                let chr_data = ines.chr_rom_slice().expect("TODO: MMC3 with CHR RAM");
                let bank_number = self.chr_bank_number(addr) as usize;
                let offset = bank_number * 1024 + (addr % 0x400) as usize;
                chr_data[offset]
            }
            // PRG RAM
            (0x6000..=0x7FFF) => self.prg_ram[addr as usize - 0x6000],
            // PRG ROM
            (0x8000..=0xFFFF) => {
                let prg_data = ines.prg_rom_slice();
                let prg_banks = prg_data.len() / 8192;
                let bank_number = match self.prg_bank_number(addr) {
                    62 => prg_banks - 2,
                    63 => prg_banks - 1,
                    n => n as usize,
                };
                let offset = bank_number * 8192 + (addr % 0x2000) as usize;
                *prg_data.get(offset).unwrap_or_else(|| {
                    panic!(
                        "MMC3 PRG OOB offset={offset} len={} prg_banks={prg_banks} bank_number={bank_number} addr={addr}",
                        prg_data.len()
                    )
                })
            }
            _ => 0,
        }
    }

    fn write(&mut self, _ines: &crate::cart::Ines, addr: u16, v: u8) {
        match addr {
            // PRG RAM
            (0x6000..=0x7FFF) => self.prg_ram[addr as usize - 0x6000] = v,

            // Registers
            (0x8000..=0x9FFF) if addr % 2 == 0 => {
                self.bank_select = BankSelectRegister::from_bits_truncate(v)
            }
            (0x8000..=0x9FFF) if addr % 2 == 1 => match self.bank_select.get_select() {
                i @ 0..=1 => self.bank_data[i as usize] = v & 0xFE,
                i @ 2..=5 => self.bank_data[i as usize] = v,
                i @ 6..=7 => self.bank_data[i as usize] = v & 0x3F,
                _ => unreachable!(),
            },
            (0xA000..=0xBFFF) if addr % 2 == 0 => {
                self.mirroring = if v % 2 == 0 {
                    Mirroring::Vertical
                } else {
                    Mirroring::Horizontal
                }
            }
            (0xC000..=0xDFFF) if addr % 2 == 0 => self.irq_latch = v,
            (0xC000..=0xDFFF) if addr % 2 == 1 => self.irq_counter = self.irq_latch,
            (0xE000..=0xFFFF) if addr % 2 == 0 => self.irq_enable = false,
            (0xE000..=0xFFFF) if addr % 2 == 1 => self.irq_enable = true,
            _ => {}
        }
    }

    fn reset(&mut self) {}

    fn clone(&self) -> Box<dyn super::Mapper + Send + Sync> {
        Box::new(Clone::clone(self))
    }

    fn mirroring(&self) -> Option<Mirroring> {
        Some(self.mirroring)
    }
}

bitflags::bitflags! {
    struct BankSelectRegister: u8 {
        const CHR_A12_INV = 0b1000_0000;
        const PRG_ROM_MODE = 0b0100_0000;
        const SELECT_0 = 0b0000_0001;
        const SELECT_1 = 0b0000_0010;
        const SELECT_2 = 0b0000_0100;
    }
}

impl BankSelectRegister {
    fn get_select(&self) -> u8 {
        self.bits & 0b0000_0111
    }
}
