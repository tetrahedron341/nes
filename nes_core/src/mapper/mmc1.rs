use super::Mapper;
use super::Ines;
use super::Mirroring;

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct MMC1 {
    incoming_value: u8,
    bits_shifted: u8,

    prg_ram_enable: bool,
    prg_rom_bank_index: u8,

    chr_rom_0_index: u8,
    chr_rom_1_index: u8,

    chr_separate: bool,

    prg_mode: PrgMode,
    mirroring: Mirroring
}

#[derive(Clone)]
enum PrgMode {
    ThirtyTwoKilobyte,
    FixFirst,
    FixLast
}

impl MMC1 {
    pub fn new() -> Self {
        MMC1 {
            incoming_value: 0,
            bits_shifted: 0,

            prg_ram_enable: false,
            prg_rom_bank_index: 0,

            chr_rom_0_index: 0,
            chr_rom_1_index: 1,

            chr_separate: false,

            prg_mode: PrgMode::FixLast,
            mirroring: Mirroring::OneScreenLowerBank
        }
    }

    fn prg_bank_0(&self, ines: &Ines, addr: u16) -> u8 {
        let prg_rom = ines.prg_rom_slice();
        let slice = match self.prg_mode {
            PrgMode::ThirtyTwoKilobyte => {
                let index = (self.prg_rom_bank_index & 0b11111110) as u16;
                &prg_rom[(index*0x4000) as usize..(index*0x4000 + 0x4000) as usize]
            },
            PrgMode::FixFirst => {
                &prg_rom[0..0x4000]
            },
            PrgMode::FixLast => {
                &prg_rom[(self.prg_rom_bank_index as usize*0x4000) .. (self.prg_rom_bank_index as usize*0x4000 + 0x4000)]
            }
        };
        slice[addr as usize]
    }
    fn prg_bank_1(&self, ines: &Ines, addr: u16) -> u8 {
        let prg_rom = ines.prg_rom_slice();
        let slice = match self.prg_mode {
            PrgMode::ThirtyTwoKilobyte => {
                let index = (self.prg_rom_bank_index & 0b11111110) as u16 + 1;
                &prg_rom[(index*0x4000) as usize..(index*0x4000 + 0x4000) as usize]
            },
            PrgMode::FixFirst => {
                &prg_rom[(self.prg_rom_bank_index as usize*0x4000) .. (self.prg_rom_bank_index as usize*0x4000 + 0x4000)]
            },
            PrgMode::FixLast => {
                let length = ines.prg_rom_range.end - ines.prg_rom_range.start;
                &prg_rom[length-0x4000 ..]
            }
        };
        slice[addr as usize]
    }
    fn chr_bank_0(&self, ines: &Ines, addr: u16) -> u8 {
        let chr_rom = ines.chr_rom_slice();
        let slice = if self.chr_separate {
            let offset = 0x1000 * self.chr_rom_0_index as usize;
            &chr_rom[offset .. offset + 0x1000]
        } else {
            let offset = 0x2000 * (self.chr_rom_0_index as usize >> 1);
            &chr_rom[offset .. offset + 0x1000]
        };
        slice[addr as usize]
    }
    fn chr_bank_1(&self, ines: &Ines, addr: u16) -> u8 {
        let chr_rom = ines.chr_rom_slice();
        let slice = if self.chr_separate {
            let offset = 0x1000 * self.chr_rom_1_index as usize;
            &chr_rom[offset .. offset + 0x1000]
        } else {
            let offset = 0x2000 * (self.chr_rom_0_index as usize >> 1);
            &chr_rom[offset + 0x1000 .. offset + 0x2000]
        };
        slice[addr as usize]
    }
}

impl Mapper for MMC1 {
    fn name(&self) -> &'static str {
        "MMC1/SxROM"
    }
    fn read(&self, ines: &Ines, addr: u16) -> u8 {
        match addr {
            0x0000..=0x0FFF => {self.chr_bank_0(ines, addr)}, // 4KB switchable CHR bank
            0x1000..=0x1FFF => {self.chr_bank_1(ines, addr-0x1000)}, // 4KB switchable CHR bank
            0x6000..=0x7FFF => { 0 }, // 8KB PRG RAM (optional) (RAM isn't implemented)
            0x8000..=0xBFFF => {self.prg_bank_0(ines, addr-0x8000)}, // 16KB PRG ROM (first bank or switchable)
            0xC000..=0xFFFF => {self.prg_bank_1(ines, addr-0xC000)}, // 16KB PRG ROM (last bank or switchable)

            _ => 0 // Not mapped anywhere
        }
    }
    fn write(&mut self, _ines: &Ines, addr: u16, v: u8) {
        let low_bit = v & 0x01;
        let high_bit = v & 0x80;
        if high_bit == 0 {
            self.incoming_value >>= 1;
            self.incoming_value += low_bit << 4;
            self.bits_shifted += 1;
            if self.bits_shifted >= 5 {
                match addr {
                    0x8000..=0x9FFF => {
                        // println!("CONTROL {:b}", self.incoming_value);
                        let mirror_v = self.incoming_value & 0b00011;
                        let prg_v = (self.incoming_value & 0b01100) >> 2;
                        let mirror = match mirror_v { 0 => Mirroring::OneScreenLowerBank, 1 => Mirroring::OneScreenUpperBank, 2 => Mirroring::Vertical, 3 => Mirroring::Horizontal, _ => unreachable!()};
                        let prg_mode = match prg_v { 0 | 1 => PrgMode::ThirtyTwoKilobyte, 2 => PrgMode::FixFirst, 3 => PrgMode::FixLast, _ => unreachable!()};
                        self.mirroring = mirror;
                        self.prg_mode = prg_mode;
                        self.chr_separate = self.incoming_value & 0b10000 != 0;
                    },
                    0xA000..=0xBFFF => {
                        // println!("CHR0 {:b}", self.incoming_value);
                        self.chr_rom_0_index = self.incoming_value;
                    },
                    0xC000..=0xDFFF => {
                        // println!("CHR1 {:b}", self.incoming_value);
                        self.chr_rom_1_index = self.incoming_value;
                    },
                    0xE000..=0xFFFF => {
                        // println!("PRG {:b}", self.incoming_value);
                        self.prg_rom_bank_index = self.incoming_value & 0b01111;
                        self.prg_ram_enable = self.incoming_value & 0b10000 != 0;
                    },
                    _ => {}
                }
                self.incoming_value = 0b10000;
                self.bits_shifted = 0;
            } else {
                // println!("MMC1 WRITE {:b}", v);
            }
        } else {
            self.incoming_value = 0b10000;
            self.bits_shifted = 0;
            self.prg_mode = PrgMode::FixLast;
        }
    }
    fn reset(&mut self) {
        std::mem::replace(self, MMC1 {
            incoming_value: 0,
            bits_shifted: 0,

            prg_ram_enable: false,
            prg_rom_bank_index: 0,

            chr_rom_0_index: 0,
            chr_rom_1_index: 1,

            chr_separate: false,

            prg_mode: PrgMode::FixLast,
            mirroring: Mirroring::OneScreenLowerBank
        });
    }
    fn clone(&self) -> Box<dyn Mapper + Send + Sync> {
        Box::new(Clone::clone(self))
    }
    fn mirroring(&self) -> Option<Mirroring> {
        Some(self.mirroring)
    }
}