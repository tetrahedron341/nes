const PIX_SCALE: usize = 3;
pub const SCREEN_WIDTH: usize = PIX_SCALE * 256;
pub const SCREEN_HEIGHT: usize = PIX_SCALE * 240;

/// A BGRA pixel array that fits exactly one NES video frame.
pub type NesFrame = [u8; SCREEN_HEIGHT * SCREEN_WIDTH * 4];

const BLANK_FRAME: NesFrame = [0u8; SCREEN_HEIGHT * SCREEN_WIDTH * 4];

/// Acts as middleman between the emulated PPU and the image drawn on screen
pub struct Screen {
    /// The frame that is shown to the user.
    front: Box<NesFrame>,
    /// The frame currently being drawn.
    back: Box<NesFrame>,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            front: Box::new(BLANK_FRAME),
            back: Box::new(BLANK_FRAME),
        }
    }

    pub fn get_frame(&self) -> &NesFrame {
        &self.front
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

impl nes_core::ppu::VideoInterface for Screen {
    #[inline(always)]
    fn draw_pixel(&mut self, x: u16, y: u16, color: nes_core::ppu::Color) {
        let (x, y) = (x as usize, y as usize);
        for y in PIX_SCALE * y..PIX_SCALE * (y + 1) {
            for x in PIX_SCALE * x..PIX_SCALE * (x + 1) {
                let offset = (y * SCREEN_WIDTH + x) * 4;
                self.back[offset] = color.0; // R
                self.back[offset + 1] = color.1; // G
                self.back[offset + 2] = color.2; // B
                self.back[offset + 3] = 0xFF; // A
            }
        }
    }
    fn end_of_frame(&mut self) {
        std::mem::swap::<Box<NesFrame>>(&mut self.front, &mut self.back);
    }
}
