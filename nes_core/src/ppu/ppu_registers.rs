#[derive(Debug)]
pub struct PPURegisters {
    pub ppu_ctrl: u8,
    pub ppu_mask: u8,
    pub ppu_status: u8,
    pub oam_addr: u8,
    pub oam_data: u8,
    pub ppu_scroll: u8,
    pub ppu_addr: u8,
    pub ppu_data: u8,

    /// Contains the index of the last access, and if it was a write.
    pub(super) last_access_from: std::cell::Cell<Option<(u16, bool)>>,
}

impl Default for PPURegisters {
    fn default() -> Self {
        PPURegisters {
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0,
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll: 0,
            ppu_addr: 0,
            ppu_data: 0,

            last_access_from: std::cell::Cell::new(None),
        }
    }
}

impl PPURegisters {
    /// `0 -> PPUCTRL`
    /// `1 -> PPUMASK`
    /// `2 -> PPUSTATUS`
    /// `3 -> OAMADDR`
    /// `4 -> OAMDATA`
    /// `5 -> PPUSCROLL`
    /// `6 -> PPUADDR`
    /// `7 -> PPUDATA`
    /// `_ -> 255`
    pub fn read_by_index(&self, index: u16) -> u8 {
        self.last_access_from.set(Some((index, false)));
        match index {
            0 => self.ppu_ctrl,
            1 => self.ppu_mask,
            2 => self.ppu_status,
            3 => self.oam_addr,
            4 => self.oam_data,
            5 => self.ppu_scroll,
            6 => self.ppu_addr,
            7 => self.ppu_data,
            _ => {
                debug_assert!(false, "Illegal PPU register access!");
                255
            }
        }
    }
    /// Same indexing as `read_by_index`
    pub fn write_by_index(&mut self, index: u16, v: u8) {
        self.last_access_from.set(Some((index, true)));
        match index {
            0 => self.ppu_ctrl = v,
            1 => self.ppu_mask = v,
            2 => self.ppu_status = v,
            3 => self.oam_addr = v,
            4 => self.oam_data = v,
            5 => self.ppu_scroll = v,
            6 => self.ppu_addr = v,
            7 => self.ppu_data = v,
            _ => {
                debug_assert!(false, "Illegal PPU register access!")
            }
        }
    }
}
