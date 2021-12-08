use std::sync::Arc;

use anyhow::Result;
use winit::window::Window;

const PIXEL_SCALE: u32 = 2;
const GAME_WIDTH: u32 = 256 * PIXEL_SCALE;
const GAME_HEIGHT: u32 = 240 * PIXEL_SCALE;

pub struct Screen {
    window: Arc<Window>,
}

impl Screen {
    pub fn new<T>(ev: &winit::event_loop::EventLoopWindowTarget<T>) -> Result<Self> {
        let window = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize {
                height: GAME_HEIGHT,
                width: GAME_WIDTH,
            })
            .with_resizable(false)
            .build(ev)?;

        Ok(Screen {
            window: Arc::new(window),
        })
    }

    pub fn as_window(&self) -> &Window {
        &*self.window
    }
}

impl nes_core::ppu::VideoInterface for Screen {
    fn draw_pixel(&self, x: u16, y: u16, color: nes_core::ppu::Color) {
        todo!()
    }
    fn end_of_frame(&self) {
        todo!()
    }
}
