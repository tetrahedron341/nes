use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::Mutex;
use std::sync::RwLock;
use nes_core::{
    nes::Nes, 
    ppu::{VideoInterface, Color}, 
    controller::{NESController, ControllerState},
    cart::Cart
};
use lazy_static::lazy_static;

lazy_static! {
    static ref EMULATOR: Mutex<Option<Nes<CanvasOutput, Controller>>> = Mutex::new(None);
}

struct Controller {

}

impl NESController for Controller {
    fn poll_controller(&self) -> ControllerState {
        ControllerState::empty()
    }
}

struct CanvasOutput {
    frame: RwLock<Vec<u8>>
}

impl VideoInterface for CanvasOutput {
    fn draw_pixel(&self, x: u16, y: u16, color: Color) {
        let offset = ((y*256 + x) * 4) as usize;
        let mut frame = self.frame.write().expect_throw("Failed to lock write access");
        frame[offset + 0] = color.0;
        frame[offset + 1] = color.1;
        frame[offset + 2] = color.2;
        frame[offset + 3] = 255;
    }
    fn end_of_frame(&self) {
        let ctx = get_canvas_context();
        ctx.scale(2.,2.).unwrap();
        let mut frame = self.frame.write().expect_throw("Failed to lock mutex");
        let clamped = wasm_bindgen::Clamped(&mut frame[..]);
        let image_data = web_sys::ImageData::new_with_u8_clamped_array(clamped, 256).unwrap();
        ctx.put_image_data(&image_data, 0.,0.).unwrap();
    }
}

#[wasm_bindgen]
pub fn init_emulator() -> Result<(), JsValue> {
    let canvas = CanvasOutput { frame: RwLock::new(Vec::with_capacity(256 * 240 * 4)) };
    let controller = Controller {};
    let nes = nes_core::nes_builder()
        .video(canvas)
        .controller(controller)
        .build(None,None);

    let mut global_emu = EMULATOR.lock().map_err(|_| "Failed to acquire mutex")?;
    global_emu.replace(nes);
    
    Ok(())
}

#[wasm_bindgen]
pub fn advance_frame() -> Result<(), JsValue> {
    let mut global_emu = EMULATOR.lock().map_err(|_| "Failed to acquire mutex")?;
    let nes: &mut nes_core::nes::Nes<_,_> = global_emu.as_mut().ok_or("Emulator has not been initialized yet")?;

    nes.run_frame().map_err(|e| format!("{}", e))?;

    Ok(())
}

#[wasm_bindgen]
pub fn insert_cartridge(rom: Box<[u8]>) -> Result<(), JsValue> {
    let rom = rom.into_vec();
    let cart = Cart::from_bytes(rom).map_err(|e| format!("{}", e))?;

    let mut global_emu = EMULATOR.lock().map_err(|_| "Failed to acquire mutex")?;
    let nes: &mut nes_core::nes::Nes<_,_> = global_emu.as_mut().ok_or("Emulator has not been initialized yet")?;
    nes.insert_cartridge(cart);

    Ok(())
}

fn get_canvas_context() -> web_sys::CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let html_canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("nes_canvas")
        .unwrap()
        .dyn_into()
        .unwrap();
    let context: web_sys::CanvasRenderingContext2d = html_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    context
}