use sdl2::render::WindowCanvas;

pub struct NametableViewer {
    pub canvas: WindowCanvas,
    pub tc: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    pub window_id: u32,
}

impl NametableViewer {
    pub fn update(&mut self, nes: &super::super::Nes) {
        let mut txt = self
            .tc
            .create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, 512, 480)
            .unwrap();
        txt.with_lock(None, |buf, pitch| {
            let pattern_table = nes.pattern_table();
            let pattern_table = if nes.mmu.ppu_registers.ppu_ctrl & 0b00010000 == 0 {
                &pattern_table[0..0x1000]
            } else {
                &pattern_table[0x1000..]
            };
            let palette_table = nes.palette_table();
            let bg_color = palette_table[0];
            let vram = nes.get_nametables();
            for table in 0..4 {
                let table_off = 0x400 * table;
                let (table_x, table_y) = match table {
                    0 => (0, 0),
                    1 => (256, 0),
                    2 => (0, 240),
                    3 => (256, 240),
                    _ => unreachable!(),
                };
                let nametable = &vram[table_off..table_off + 0x400];
                let attr_table = &nametable[0x3c0..];
                for row in 0..30 {
                    for tile in 0..32 {
                        let attr = attr_table[(row / 4) * 0x8 + (tile / 4)];
                        let palette = match ((row / 2) % 2) * 2 + ((tile / 2) % 2) {
                            0b00 => attr & 0b00000011,
                            0b01 => (attr & 0b00001100) >> 2,
                            0b10 => (attr & 0b00110000) >> 4,
                            0b11 => (attr & 0b11000000) >> 6,
                            _ => unreachable!(),
                        };
                        let pattern_byte = nametable[row * 0x20 + tile];
                        for p_row in 0..8 {
                            let lo = pattern_table[((pattern_byte as u16) << 4) as usize + p_row];
                            let hi =
                                pattern_table[((pattern_byte as u16) << 4) as usize + p_row + 8];
                            for pixel in 0..8 {
                                let px_hi = (hi >> (7 - pixel)) & 1;
                                let px_lo = (lo >> (7 - pixel)) & 1;
                                let px = px_hi << 1 | px_lo;
                                let (r, g, b) = if px == 0 {
                                    bg_color
                                } else {
                                    palette_table[(palette << 2 | px) as usize]
                                };
                                // print!("{:?}",(r,g,b));
                                let pix_offset = pitch * ((table_y + row * 8) + p_row)
                                    + (table_x + tile * 8 + pixel) * 3;
                                buf[pix_offset] = r;
                                buf[pix_offset + 1] = g;
                                buf[pix_offset + 2] = b;
                            }
                        }
                    }
                }
            }
        })
        .unwrap();
        self.canvas
            .copy(&txt, None, sdl2::rect::Rect::new(0, 0, 512, 480))
            .unwrap();
        self.canvas.present();
    }

    pub fn new(video: &sdl2::VideoSubsystem) -> Self {
        let mut window = video.window("Nametable viewer", 512, 480).build().unwrap();
        window.set_minimum_size(512, 480).unwrap();
        let window_id = window.id();
        let canvas = window.into_canvas().build().unwrap();
        let tc = canvas.texture_creator();
        NametableViewer {
            canvas,
            tc,
            window_id,
        }
    }
}
