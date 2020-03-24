use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::rect::Rect;
use super::super::PIXEL_SCALE;

const WINDOW_WIDTH: u32 = 15 + 128*PIXEL_SCALE + 15 + 128*PIXEL_SCALE + 15;
const WINDOW_HEIGHT: u32 = 15 + 128*PIXEL_SCALE + 15 + PALETTE_SCALE + 15;
const PALETTE_X: i32 = 15;
const PALETTE_Y: i32 = 15 + (128 * PIXEL_SCALE as i32) + 15;
const PALETTE_SCALE: u32 = PIXEL_SCALE * 4;

pub struct PatternTableViewer {
    canvas: WindowCanvas,
    tc: TextureCreator<WindowContext>,
    pub window_id: u32,
    pub selected_palette: u16
}

impl PatternTableViewer {
    pub fn new(video: &sdl2::VideoSubsystem) -> Result<Self, Box<dyn std::error::Error>> {
        let window = video.window("Pattern table viewer", WINDOW_WIDTH, WINDOW_HEIGHT).build()?;
        let window_id = window.id();
        let canvas = window.into_canvas().build()?;
        let tc = canvas.texture_creator();
        Ok(PatternTableViewer {
            canvas,
            tc,
            window_id,
            selected_palette: 0
        })
    }
    
    pub fn update(&mut self, nes: &super::super::Nes<'_>) {
        let mut pt1 = self.tc.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, 128,128).unwrap();
        let mut pt2 = self.tc.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, 128,128).unwrap();
        let pattern_table = nes.pattern_table();
        let palette = nes.get_palette(self.selected_palette);
        let bg_color = nes.get_palette(0)[0];
        pt1.with_lock(None, |buf, pitch| {
            for tr in 0..16 {
                for tc in 0..16 {
                    let tile_addr = (tr << 8) | (tc << 4);
                    let bp_hi = &pattern_table[tile_addr + 8 .. tile_addr + 16];
                    let bp_lo = &pattern_table[tile_addr + 0 .. tile_addr + 8];
                    for y in 0..8 {
                        let byte_hi = bp_hi[y];
                        let byte_lo = bp_lo[y];
                        for x in 0..8 {
                            let mask = 0x80 >> x;
                            let px_hi = (byte_hi & mask) >> (7-x);
                            let px_lo = (byte_lo & mask) >> (7-x);
                            let px = (px_hi << 1) | px_lo;
                            let (r,g,b) = if px == 0 {
                                bg_color
                            } else {
                                palette[px as usize]
                            };

                            let ty = (tr*8) + y;
                            let tx = (tc*8) + x;
                            buf[(ty*pitch + tx*3) + 0] = r;
                            buf[(ty*pitch + tx*3) + 1] = g;
                            buf[(ty*pitch + tx*3) + 2] = b;
                        }
                    }
                }
            }
        }).unwrap();
        pt2.with_lock(None, |buf, pitch| {
            for tr in 0..16 {
                for tc in 0..16 {
                    let tile_addr = 0x1000 | (tr << 8) | (tc << 4);
                    let bp_hi = &pattern_table[tile_addr + 8 .. tile_addr + 16];
                    let bp_lo = &pattern_table[tile_addr + 0 .. tile_addr + 8];
                    for y in 0..8 {
                        let byte_hi = bp_hi[y];
                        let byte_lo = bp_lo[y];
                        for x in 0..8 {
                            let mask = 0x80 >> x;
                            let px_hi = (byte_hi & mask) >> (7-x);
                            let px_lo = (byte_lo & mask) >> (7-x);
                            let px = (px_hi << 1) | px_lo;
                            let (r,g,b) = if px == 0 {
                                bg_color
                            } else {
                                palette[px as usize]
                            };

                            let ty = (tr*8) + y;
                            let tx = (tc*8) + x;
                            buf[(ty*pitch + tx*3) + 0] = r;
                            buf[(ty*pitch + tx*3) + 1] = g;
                            buf[(ty*pitch + tx*3) + 2] = b;
                        }
                    }
                }
            }
        }).unwrap();

        self.canvas.set_draw_color((0,0,255));
        self.canvas.clear();

        self.canvas.copy(&pt1, None, Rect::new(15                              , 15, 128*PIXEL_SCALE, 128*PIXEL_SCALE)).unwrap();
        self.canvas.copy(&pt2, None, Rect::new(15 + 128*PIXEL_SCALE as i32 + 15, 15, 128*PIXEL_SCALE, 128*PIXEL_SCALE)).unwrap();

        // Draw the palette selector
        for pi in 0..8 {
            let offset_x = PALETTE_X + pi as i32 * ((4*PALETTE_SCALE as i32) + 5);
            if pi == self.selected_palette {
                self.canvas.set_draw_color((255,255,255));
                self.canvas.fill_rect(Rect::new(offset_x-1, PALETTE_Y-1, 4*PALETTE_SCALE + 2, PALETTE_SCALE + 2)).unwrap();
            } else {
                self.canvas.set_draw_color((0,0,255));
                self.canvas.fill_rect(Rect::new(offset_x-1, PALETTE_Y-1, 4*PALETTE_SCALE + 2, PALETTE_SCALE + 2)).unwrap();
            }
            for i in 0..4 {
                self.canvas.set_draw_color(nes.get_palette(pi)[i as usize]);
                self.canvas.fill_rect(Rect::new(offset_x + i*PALETTE_SCALE as i32, PALETTE_Y, PALETTE_SCALE, PALETTE_SCALE)).unwrap();
            }
        }

        self.canvas.present();
    }
}