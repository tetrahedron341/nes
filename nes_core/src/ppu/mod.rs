#![allow(clippy::unusual_byte_groupings)]

mod ppu_registers;
pub use ppu_registers::PPURegisters;
mod video_interface;
pub use video_interface::{Color, DummyVideo, VideoInterface};
mod memory_interface;
pub use memory_interface::PPUMemory;

pub struct PPU {
    // Scrolling registers
    pub vram_addr: u16,
    t_addr: u16,
    pub fine_x: u8,
    second_write: bool,

    // VRAM
    pub palette_ram: [u8; 0x20],
    pub oam: [u8; 64 * 4],

    // BG rendering registers
    next_bg_tile_id: u8,
    next_bg_palette: u8,
    next_bg_lsb: u8,
    next_bg_msb: u8,
    pattern_shift_high: u16,
    pattern_shift_low: u16,
    palette_shift_high: u16,
    palette_shift_low: u16,

    // FG rendering registers
    scanline_sprites: [u8; 8 * 4],
    sprite_count: u32,
    fg_pattern_shift_hi: [u8; 8],
    fg_pattern_shift_lo: [u8; 8],

    // Sprite zero hit
    sprite_zero_hit_possible: bool,
    sprite_zero_rendering: bool,

    dot: u16,
    scanline: u16,
    pub frame: u64,

    pub nmi: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct OAMEntry {
    pub y: u8,
    pub tile_id: u8,
    pub attr: u8,
    pub x: u8,
}
impl OAMEntry {
    pub fn to_bytes(self) -> [u8; 4] {
        [self.y, self.tile_id, self.attr, self.x]
    }
}

#[derive(Clone)]
pub struct PPUSaveState {
    palette_ram: [u8; 0x20],
    oam: [u8; 64 * 4],
}

impl Default for PPU {
    fn default() -> Self {
        PPU {
            vram_addr: 0,
            t_addr: 0,
            fine_x: 0,
            second_write: false,

            palette_ram: [0; 0x20],
            oam: [0; 64 * 4],

            next_bg_tile_id: 0,
            next_bg_palette: 0,
            next_bg_lsb: 0,
            next_bg_msb: 0,
            pattern_shift_high: 0,
            pattern_shift_low: 0,
            palette_shift_high: 0,
            palette_shift_low: 0,

            scanline_sprites: [0; 8 * 4],
            sprite_count: 0,
            fg_pattern_shift_hi: [0; 8],
            fg_pattern_shift_lo: [0; 8],

            sprite_zero_hit_possible: false,
            sprite_zero_rendering: false,

            dot: 0,
            scanline: 261,
            frame: 0,
            nmi: false,
        }
    }
}

impl PPU {
    pub fn save_state(&self) -> PPUSaveState {
        PPUSaveState {
            palette_ram: {
                let mut p_ram = [0; 0x20];
                p_ram.as_mut().copy_from_slice(&self.palette_ram);
                p_ram
            },
            oam: {
                let mut oam = [0; 64 * 4];
                oam.as_mut().copy_from_slice(&self.oam);
                oam
            },
        }
    }
    pub fn load_state(&mut self, s: PPUSaveState) {
        self.palette_ram.as_mut().copy_from_slice(&s.palette_ram);
        self.oam.as_mut().copy_from_slice(&s.oam);
    }

    /// Run one cycle of the PPU, and output a pixel to the video interface
    pub fn tick(&mut self, chr: &mut dyn PPUMemory, video_out: &mut dyn VideoInterface) {
        self.update_from_registers(chr);

        match self.scanline {
            0..=239 | 261 => {
                // Visible scanlines (And pre-render scanline)
                if self.scanline == 261 && self.dot == 1 {
                    chr.registers_mut().ppu_status = 0;
                    self.fg_pattern_shift_hi = [0; 8];
                    self.fg_pattern_shift_lo = [0; 8];
                }
                if self.scanline == 1 && self.dot == 1 {
                    self.nmi = false // Start of a new frame
                };
                if (2..258).contains(&self.dot) || (321..338).contains(&self.dot) {
                    // If rendering background is enabled, shift the shift registers
                    if chr.registers().ppu_mask & 0x08 != 0 {
                        self.palette_shift_high <<= 1;
                        self.palette_shift_low <<= 1;
                        self.pattern_shift_high <<= 1;
                        self.pattern_shift_low <<= 1;
                    }
                    if chr.registers().ppu_mask & 0x10 != 0 && self.dot >= 1 && self.dot < 258 {
                        for i in 0..self.sprite_count {
                            let sprite_offset = i as usize * 4;
                            let entry = OAMEntry {
                                y: self.scanline_sprites[sprite_offset],
                                tile_id: self.scanline_sprites[sprite_offset + 1],
                                attr: self.scanline_sprites[sprite_offset + 2],
                                x: self.scanline_sprites[sprite_offset + 3],
                            };
                            if entry.x > 0 {
                                self.scanline_sprites[sprite_offset + 3] -= 1;
                            } else {
                                self.fg_pattern_shift_hi[i as usize] <<= 1;
                                self.fg_pattern_shift_lo[i as usize] <<= 1;
                            }
                        }
                    }
                    match (self.dot - 1) % 8 {
                        7 => {
                            self.increment_scroll_x(chr.registers());
                        }
                        0 => {
                            // Shift in next tile to be drawn and load NT byte for next tile
                            self.pattern_shift_low =
                                (self.pattern_shift_low & 0xff00) | self.next_bg_lsb as u16;
                            self.pattern_shift_high =
                                (self.pattern_shift_high & 0xff00) | self.next_bg_msb as u16;
                            self.palette_shift_low = (self.palette_shift_low & 0xff00)
                                | (if self.next_bg_palette & 0b01 != 0 {
                                    0xff
                                } else {
                                    0x00
                                });
                            self.palette_shift_high = (self.palette_shift_high & 0xff00)
                                | (if self.next_bg_palette & 0b10 != 0 {
                                    0xff
                                } else {
                                    0x00
                                });

                            let addr = 0x2000 + (self.vram_addr & 0b000_11_11111_11111);
                            self.next_bg_tile_id = self.read_vram(addr, chr);
                        }
                        2 => {
                            // Load AT byte for next tile
                            let addr: u16 = 0x2000                                      // Base nametable address
                                        | self.vram_addr & 0b000_11_00000_00000         // Nametable select
                                        | 0x03C0                                        // Index
                                        | ((self.vram_addr & 0b000_00_11100_00000)>>4)  // Coarse y truncated to 3 bits and shifted into place
                                        | ((self.vram_addr & 0b000_00_00000_11100)>>2); // Coarse x truncated to 3 bits and shifted into place
                            self.next_bg_palette = self.read_vram(addr, chr);
                            if self.vram_addr & 0b000_00_00010_00000 != 0 {
                                self.next_bg_palette >>= 4
                            }
                            if self.vram_addr & 0b000_00_00000_00010 != 0 {
                                self.next_bg_palette >>= 2
                            }
                            self.next_bg_palette &= 0x03;
                        }
                        4 => {
                            // Load low BG Tile byte
                            let addr: u16 = if chr.registers().ppu_ctrl & 0x10 != 0 {
                                0x1000
                            } else {
                                0x0000
                            } + ((self.next_bg_tile_id as u16) << 4)
                                + ((self.vram_addr & 0b111_00_00000_00000) >> 12);
                            self.next_bg_lsb = self.read_vram(addr, chr);
                        }
                        6 => {
                            // Load high BG Tile byte
                            let addr: u16 = if chr.registers().ppu_ctrl & 0x10 != 0 {
                                0x1000
                            } else {
                                0x0000
                            } + ((self.next_bg_tile_id as u16) << 4)
                                + ((self.vram_addr & 0b111_00_00000_00000) >> 12)
                                + 0b1000; // Second bitplane
                            self.next_bg_msb = self.read_vram(addr, chr);
                        }
                        _ => {}
                    }
                }

                if self.dot == 256 {
                    self.increment_scroll_y(chr.registers_mut());
                }
                if self.dot == 257 && chr.registers().ppu_mask & 0x18 != 0 {
                    self.pattern_shift_low =
                        (self.pattern_shift_low & 0xff00) | self.next_bg_lsb as u16;
                    self.pattern_shift_high =
                        (self.pattern_shift_high & 0xff00) | self.next_bg_msb as u16;
                    self.palette_shift_low = (self.palette_shift_low & 0xff00)
                        | (if self.next_bg_palette & 0b01 != 0 {
                            0xff
                        } else {
                            0x00
                        });
                    self.palette_shift_high = (self.palette_shift_high & 0xff00)
                        | (if self.next_bg_palette & 0b10 != 0 {
                            0xff
                        } else {
                            0x00
                        });

                    self.vram_addr &= 0b111_10_11111_00000;
                    self.vram_addr |= self.t_addr & 0b000_01_00000_11111;
                }
                if self.scanline == 261
                    && (280..305).contains(&self.dot)
                    && chr.registers().ppu_mask & 0x18 != 0
                {
                    self.vram_addr &= 0b000_01_00000_11111;
                    self.vram_addr |= self.t_addr & 0b111_10_11111_00000;
                }

                // Sprite rendering
                if self.dot == 257 && self.scanline < 261 {
                    self.scanline_sprites = [0; 8 * 4];
                    self.sprite_count = 0;
                    let sprite_height = if chr.registers().ppu_ctrl & 0x20 != 0 {
                        16
                    } else {
                        8
                    };
                    self.sprite_zero_hit_possible = false;
                    for i in 0..64 {
                        if self.sprite_count >= 9 {
                            break;
                        }
                        let entry = self.get_oam_entry(i);
                        let diff = self.scanline as i32 - entry.y as i32;
                        if diff >= 0 && diff < sprite_height {
                            if self.sprite_count < 8 {
                                if i == 0 {
                                    self.sprite_zero_hit_possible = true;
                                }
                                let off = self.sprite_count as usize * 4;
                                self.scanline_sprites[off..off + 4]
                                    .clone_from_slice(&entry.to_bytes());
                            }
                            self.sprite_count += 1;
                        }
                    }
                    chr.registers_mut().ppu_status |= if self.sprite_count > 8 { 0x20 } else { 0 };
                    if self.sprite_count > 8 {
                        self.sprite_count = 8
                    }
                }

                if self.dot == 340 {
                    for i in 0..self.sprite_count {
                        let sprite_offset = i as usize * 4;
                        let entry = OAMEntry {
                            y: self.scanline_sprites[sprite_offset],
                            tile_id: self.scanline_sprites[sprite_offset + 1],
                            attr: self.scanline_sprites[sprite_offset + 2],
                            x: self.scanline_sprites[sprite_offset + 3],
                        };
                        let pattern_addr_lo: u16 = if chr.registers().ppu_ctrl & 0x20 == 0 {
                            // 8x8 sprite mode
                            let row_sel = if entry.attr & 0x80 == 0 {
                                self.scanline - entry.y as u16
                            } else {
                                7 - (self.scanline - entry.y as u16)
                            };
                            let table_addr = if chr.registers().ppu_ctrl & 0x08 != 0 {
                                0x1000
                            } else {
                                0
                            };

                            table_addr | ((entry.tile_id as u16) << 4) | row_sel
                        } else {
                            // 8x16 sprite mode
                            let row_sel = if entry.attr & 0x80 == 0 {
                                (self.scanline - entry.y as u16) & 0x07
                            } else {
                                (15 - (self.scanline - entry.y as u16)) & 0x07
                            };
                            let table_addr = ((entry.tile_id & 0x01) as u16) << 12;
                            let mut top_tile_id = entry.tile_id & 0xFE;
                            let mut bottom_tile_id = (entry.tile_id & 0xFE) + 1;
                            if entry.attr & 0x80 != 0 {
                                std::mem::swap(&mut top_tile_id, &mut bottom_tile_id)
                            };
                            let tile_id = if self.scanline - (entry.y as u16) < 8 {
                                top_tile_id
                            } else {
                                bottom_tile_id
                            };

                            table_addr | ((tile_id as u16) << 4) | row_sel
                        };
                        let pattern_addr_hi = pattern_addr_lo + 8;

                        let mut pattern_bytes_lo = chr.read_ppu(pattern_addr_lo);
                        let mut pattern_bytes_hi = chr.read_ppu(pattern_addr_hi);

                        if entry.attr & 0x40 != 0 {
                            // Flip horizontally
                            let flip = |mut b: u8| -> u8 {
                                b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
                                b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
                                b = (b & 0xAA) >> 1 | (b & 0x55) << 1;
                                b
                            };
                            pattern_bytes_lo = flip(pattern_bytes_lo);
                            pattern_bytes_hi = flip(pattern_bytes_hi);
                        }
                        self.fg_pattern_shift_hi[i as usize] = pattern_bytes_hi;
                        self.fg_pattern_shift_lo[i as usize] = pattern_bytes_lo;
                    }
                }
            }
            240 => { // Post-render scanline
            }
            241..=260 => {
                // VBlank
                if self.scanline == 241 && self.dot == 1 {
                    chr.registers_mut().ppu_status |= 0x80; // Set VBlank enable bit
                    video_out.end_of_frame();
                    if chr.registers().ppu_ctrl & 0x80 != 0 {
                        // Send an NMI if it is enabled
                        self.nmi = true;
                    }
                }
            }
            _ => unreachable!(),
        };

        // Background rendering
        let mut bg_palette: u8 = 0;
        let mut bg_pixel: u8 = 0;
        if chr.registers().ppu_mask & 0x08 != 0
            && (self.dot > 8 || chr.registers().ppu_mask & 0x02 != 0)
        {
            let bit_mux = 0x8000 >> self.fine_x;

            let p0 = if self.pattern_shift_low & bit_mux != 0 {
                1
            } else {
                0
            };
            let p1 = if self.pattern_shift_high & bit_mux != 0 {
                1
            } else {
                0
            };
            bg_pixel = (p1 << 1) | p0;

            let pl0 = if self.palette_shift_low & bit_mux != 0 {
                1
            } else {
                0
            };
            let pl1 = if self.palette_shift_high & bit_mux != 0 {
                1
            } else {
                0
            };
            bg_palette = (pl1 << 1) | pl0;
        }

        // Foreground rendering
        let mut fg_palette: u8 = 0;
        let mut fg_pixel: u8 = 0;
        let mut fg_priority = false;
        if chr.registers().ppu_mask & 0x10 != 0 {
            self.sprite_zero_rendering = false;
            for i in 0..self.sprite_count {
                let sprite_offset = i as usize * 4;
                let entry = OAMEntry {
                    y: self.scanline_sprites[sprite_offset],
                    tile_id: self.scanline_sprites[sprite_offset + 1],
                    attr: self.scanline_sprites[sprite_offset + 2],
                    x: self.scanline_sprites[sprite_offset + 3],
                };
                if entry.x == 0 {
                    let fg_pixel_lo = (self.fg_pattern_shift_lo[i as usize] & 0x80) >> 7;
                    let fg_pixel_hi = (self.fg_pattern_shift_hi[i as usize] & 0x80) >> 7;
                    fg_pixel = (fg_pixel_hi << 1) | fg_pixel_lo;
                    fg_palette = (entry.attr & 0x03) + 0b0100;
                    fg_priority = (entry.attr & 0x20) == 0;
                    if fg_pixel != 0 {
                        if i == 0 {
                            self.sprite_zero_rendering = true;
                        }
                        break;
                    }
                }
            }
        }

        let pixel: u8;
        let palette: u8;
        if bg_pixel == 0 && fg_pixel == 0 {
            pixel = 0;
            palette = 0;
        } else if bg_pixel == 0 && fg_pixel != 0 {
            pixel = fg_pixel;
            palette = fg_palette;
        } else if bg_pixel != 0 && fg_pixel == 0 {
            pixel = bg_pixel;
            palette = bg_palette;
        } else {
            // Both pixels are visible
            if fg_priority {
                pixel = fg_pixel;
                palette = fg_palette;
            } else {
                pixel = bg_pixel;
                palette = bg_palette;
            }

            // Perform sprite 0 hit detection
            if self.sprite_zero_hit_possible && self.sprite_zero_rendering {
                let mask = chr.registers().ppu_mask;
                // Check if both render background and render sprites are enabled
                if mask & 0x18 == 0x18 {
                    // Check if both FG and BG are disabled on the leftmost 8 pixels
                    if mask & 0x06 == 0x00 {
                        if self.dot >= 9 && self.dot < 258 {
                            // Set the sprite 0 hit bit
                            chr.registers_mut().ppu_status |= 0x40;
                        }
                    } else if self.dot >= 1 && self.dot < 258 {
                        // Set the sprite 0 hit bit
                        chr.registers_mut().ppu_status |= 0x40;
                    }
                }
            }
        }

        let color = self.read_palette_ram((palette << 2 | pixel) as usize);
        video_out.draw_pixel(
            self.dot.wrapping_sub(1),
            self.scanline,
            self.convert_color_to_rgb(color),
        );

        self.dot += 1;
        if self.dot > 340 {
            self.dot = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
                self.frame += 1;
            }
        }
    }

    fn read_palette_ram(&self, i: usize) -> u8 {
        if i >= 16 && i & 0b11 == 0 {
            // Mirror the background color of all palettes to $3F00
            self.palette_ram[0]
        } else {
            self.palette_ram[i]
        }
    }
    fn write_palette_ram(&mut self, i: usize, v: u8) {
        if i > 0 && i & 0b11 == 0 {
            // Writes to the first color of sprite palettes are mirrored to the
            // first color of their respective background palette
            self.palette_ram[i & 0b0111] = v;
        } else {
            self.palette_ram[i] = v;
        }
    }

    fn update_from_registers(&mut self, chr: &mut dyn PPUMemory) {
        if let Some((index, is_write)) = chr.registers_mut().last_access_from.take() {
            match index {
                0 if is_write => {
                    let name_table_select = (chr.registers().ppu_ctrl & 0b0000_0011) as u16;
                    self.t_addr &= 0b111_00_11111_11111;
                    self.t_addr |= name_table_select << 10;
                }
                2 if !is_write => {
                    self.second_write = false;
                }
                3 if is_write => {
                    let v = chr.registers().oam_addr;
                    chr.registers_mut().oam_data = self.oam[v as usize];
                }
                4 if is_write => {
                    let v = chr.registers().oam_data;
                    let a = chr.registers().oam_addr;
                    self.oam[a as usize] = v;
                }
                5 if is_write => {
                    let coarse = (chr.registers().ppu_scroll & 0b1111_1000) as u16 >> 3;
                    let fine = chr.registers().ppu_scroll & 0b0000_0111;
                    if !self.second_write {
                        self.t_addr &= 0b111_11_11111_00000;
                        self.t_addr |= coarse;
                        self.fine_x = fine;
                        self.second_write = true;
                    } else {
                        // Second write
                        self.t_addr &= 0b000_11_00000_11111;
                        self.t_addr |= coarse << 5;
                        self.t_addr |= (fine as u16) << 12;
                        self.second_write = false;
                    }
                }
                6 if is_write => {
                    if !self.second_write {
                        // First write
                        let msb = (chr.registers().ppu_addr & 0b0011_1111) as u16;
                        self.t_addr &= 0b0_000_00_00111_11111;
                        self.t_addr |= msb << 8;
                        self.second_write = true;
                    } else {
                        // Second write
                        let lsb = chr.registers().ppu_addr as u16;
                        self.t_addr &= 0b1_111_11_11000_00000;
                        self.t_addr |= lsb;
                        self.second_write = false;
                        self.vram_addr = self.t_addr;
                        chr.registers_mut().ppu_data = self.read_vram(self.vram_addr, chr);
                    }
                }
                7 => {
                    if is_write {
                        self.write_vram(self.vram_addr, chr.registers().ppu_data, chr);
                    }

                    chr.registers_mut().ppu_data = self.read_vram(self.vram_addr, chr);
                    if chr.registers().ppu_ctrl & 0b0000_0100 != 0 {
                        self.vram_addr += 32;
                    } else {
                        self.vram_addr += 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn write_vram(&mut self, addr: u16, v: u8, chr: &mut dyn PPUMemory) {
        match addr {
            0x0000..=0x1fff => chr.write_ppu(addr, v),
            0x2000..=0x2fff => chr.write_ppu(addr, v),
            0x3000..=0x3eff => chr.write_ppu(addr - 0x1000, v),
            0x3f00..=0x3fff => self.write_palette_ram((addr - 0x3f00) as usize % 0x20, v),
            _ => (),
        }
    }
    fn read_vram(&mut self, addr: u16, chr: &dyn PPUMemory) -> u8 {
        match addr {
            0x0000..=0x1fff => chr.read_ppu(addr),
            // Nametables
            0x2000..=0x2fff => chr.read_ppu(addr),
            0x3000..=0x3eff => chr.read_ppu(addr - 0x1000), // Mirror
            0x3f00..=0x3fff => self.read_palette_ram((addr - 0x3f00) as usize % 0x20),
            _ => 0xff,
        }
    }

    pub fn convert_color_to_rgb(&self, color: u8) -> Color {
        const PALETTE_TABLE: [Color; 0x40] = [
            Color(84, 84, 84),
            Color(0, 30, 116),
            Color(8, 16, 144),
            Color(48, 0, 136),
            Color(68, 0, 100),
            Color(92, 0, 48),
            Color(84, 4, 0),
            Color(60, 24, 0),
            Color(32, 42, 0),
            Color(8, 58, 0),
            Color(0, 64, 0),
            Color(0, 60, 0),
            Color(0, 50, 60),
            Color(0, 0, 0),
            Color(0, 0, 0),
            Color(0, 0, 0),
            Color(152, 150, 152),
            Color(8, 76, 196),
            Color(48, 50, 236),
            Color(92, 30, 228),
            Color(136, 20, 176),
            Color(160, 20, 100),
            Color(152, 34, 32),
            Color(120, 60, 0),
            Color(84, 90, 0),
            Color(40, 114, 0),
            Color(8, 124, 0),
            Color(0, 118, 40),
            Color(0, 102, 120),
            Color(0, 0, 0),
            Color(0, 0, 0),
            Color(0, 0, 0),
            Color(236, 238, 236),
            Color(76, 154, 236),
            Color(120, 124, 236),
            Color(176, 98, 236),
            Color(228, 84, 236),
            Color(236, 88, 180),
            Color(236, 106, 100),
            Color(212, 136, 32),
            Color(160, 170, 0),
            Color(116, 196, 0),
            Color(76, 208, 32),
            Color(56, 204, 108),
            Color(56, 180, 204),
            Color(60, 60, 60),
            Color(0, 0, 0),
            Color(0, 0, 0),
            Color(236, 238, 236),
            Color(168, 204, 236),
            Color(188, 188, 236),
            Color(212, 178, 236),
            Color(236, 174, 236),
            Color(236, 174, 212),
            Color(236, 180, 176),
            Color(228, 196, 144),
            Color(204, 210, 120),
            Color(180, 222, 120),
            Color(168, 226, 144),
            Color(152, 226, 180),
            Color(160, 214, 228),
            Color(160, 162, 160),
            Color(0, 0, 0),
            Color(0, 0, 0),
        ];
        if color > 0x3f {
            return Color(255, 0, 255);
        }
        PALETTE_TABLE[color as usize]
    }

    fn increment_scroll_x(&mut self, registers: &PPURegisters) {
        if registers.ppu_mask & 0x18 != 0 {
            // Check if rendering is enabled
            let mut coarse_x = self.vram_addr & 0b000_00_00000_11111;
            coarse_x += 1;
            if coarse_x > 31 {
                coarse_x = 0;
                self.vram_addr ^= 0b000_01_00000_00000; // Flip nametable X bit
            }
            self.vram_addr &= 0b111_11_11111_00000;
            self.vram_addr |= coarse_x;
        }
    }
    fn increment_scroll_y(&mut self, registers: &mut PPURegisters) {
        if registers.ppu_mask & 0x18 != 0 {
            // Check if rendering is enabled
            if (self.vram_addr & 0b111_00_00000_00000) >> 12 < 7 {
                self.vram_addr += 0b001_00_00000_00000;
            } else {
                self.vram_addr &= 0b000_11_11111_11111; // Set fine_y to 0
                let mut coarse_y = (self.vram_addr & 0b000_00_11111_00000) >> 5;
                coarse_y += 1;
                if coarse_y > 29 {
                    coarse_y = 0;
                    self.vram_addr ^= 0b000_10_00000_00000; // Flip nametable Y bit
                }
                self.vram_addr &= 0b111_11_00000_11111;
                self.vram_addr |= coarse_y << 5;
            }
        }
    }

    fn get_oam_entry(&self, id: u8) -> OAMEntry {
        let bytes = &self.oam[id as usize * 4..(id as usize + 1) * 4];
        OAMEntry {
            y: bytes[0],
            tile_id: bytes[1],
            attr: bytes[2],
            x: bytes[3],
        }
    }
    pub fn print_oam(&self) -> String {
        let mut out = String::new();
        for i in 0..64 {
            let entry = self.get_oam_entry(i);
            out.push_str(&format!(
                "{:02}: {{ y: {}, tile_id: ${:02X}, attr: {:08b}b, x: {} }} \n",
                i, entry.y, entry.tile_id, entry.attr, entry.x
            ));
        }
        out
    }
}
