use std::sync::RwLock;

pub struct Screen<'a> {
    pub txt: RwLock<sdl2::render::Texture<'a>>,
    buffer: RwLock<Vec<(u8, u8, u8)>>,
    w: u32,
    h: u32,
}
impl<'a> Screen<'a> {
    pub fn new<T>(
        tc: &'a sdl2::render::TextureCreator<T>,
        w: u32,
        h: u32,
    ) -> Result<Self, sdl2::render::TextureValueError> {
        let txt = tc.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24,
            super::GAME_WIDTH,
            super::GAME_HEIGHT,
        )?;
        let txt = RwLock::new(txt);
        let buffer = vec![(255, 255, 255); (w * h) as usize];
        let buffer = RwLock::new(buffer);
        Ok(Screen { txt, buffer, w, h })
    }
}

impl<'a> nes_core::ppu::VideoInterface for Screen<'a> {
    fn draw_pixel(&self, x: u16, y: u16, color: nes_core::ppu::Color) {
        if (x as u32) < self.w && (y as u32) < self.h {
            let mut buffer = self.buffer.write().unwrap();
            buffer[y as usize * self.w as usize + x as usize] = color.into_tuple();
        }
    }
    fn end_of_frame(&self) {
        let mut txt = self.txt.write().unwrap();
        let buffer = self.buffer.read().unwrap();
        txt.with_lock(None, |buf, s| {
            for y in 0..self.h {
                for x in 0..self.w {
                    let off = (y * self.w + x) as usize;
                    let c = buffer[off];

                    let txt_off = y as usize * s + (x as usize * 3);
                    buf[txt_off] = c.0;
                    buf[txt_off + 1] = c.1;
                    buf[txt_off + 2] = c.2;
                }
            }
        })
        .unwrap();
    }
}
