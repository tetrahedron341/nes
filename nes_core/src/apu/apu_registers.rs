use std::cell::Cell;

pub struct APURegisters {
    pub(super) registers: [u8; 0x20],
    pub(super) last_access: Cell<Option<(usize,bool)>>,
    pub(super) status_out: u8
}

impl APURegisters {
    pub fn new() -> Self {
        APURegisters {
            registers: [0; 0x20],
            last_access: Cell::new(None),
            status_out: 0
        }
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        // println!("Write to {:X}: {:08b}", addr, v);
        self.last_access.set(Some(((addr-0x4000) as usize, true)));
        self.registers[(addr - 0x4000) as usize] = v;
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.last_access.set(Some(((addr-0x4000) as usize, false)));
        if addr == 0x4015 {
            // println!("Read from 4015: {:08b}", self.status_out);
            self.status_out
        } else {
            0x00
        }
    }
}