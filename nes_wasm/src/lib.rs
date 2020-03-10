#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::{Mutex, MutexGuard};
use std::sync::RwLock;
use std::convert::TryFrom;
use nes_core::{
    nes::NesSaveState,
    ppu::{VideoInterface, Color}, 
    controller::{NESController, ControllerState},
    cart::Cart
};
use lazy_static::lazy_static;

type Nes = nes_core::nes::Nes<CanvasOutput, &'static Controller>;

lazy_static! {
    static ref EMULATOR: Mutex<Option<Nes>> = Mutex::new(None);
    static ref CONTROLLER: Controller = Controller::new();
    static ref SAVE_STATE: RwLock<Option<NesSaveState>> = RwLock::new(None);
}

struct Controller {
    buttons: RwLock<ControllerState>
}

impl Controller {
    fn new() -> Self {
        Controller {
            buttons: RwLock::new(ControllerState::empty())
        }
    }
    fn buttons_down(&self, buttons: ControllerState) {
        self.buttons.write().unwrap().insert(buttons);
    }
    fn buttons_up(&self, buttons: ControllerState) {
        self.buttons.write().unwrap().remove(buttons);
    }
}

impl NESController for Controller {
    fn poll_controller(&self) -> ControllerState {
        *self.buttons.read().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
enum Button {
    A, B,
    Up, Down, Left, Right,
    Start, Select
}

impl Into<JsValue> for Button {
    fn into(self) -> JsValue {
        use Button::*;
        match self {
            A => "A".into(),
            B => "B".into(),
            Start => "Start".into(),
            Select => "Select".into(),
            Left => "Left".into(),
            Right => "Right".into(),
            Up => "Up".into(),
            Down => "Down".into()
        }
    }
}

impl Into<ControllerState> for Button {
    fn into(self) -> ControllerState {
        use Button::*;
        match self {
            A => ControllerState::A,
            B => ControllerState::B,
            Start => ControllerState::START,
            Select => ControllerState::SELECT,
            Left => ControllerState::LEFT,
            Right => ControllerState::RIGHT,
            Up => ControllerState::UP,
            Down => ControllerState::DOWN,
        }
    }
}

impl TryFrom<JsValue> for Button {
    type Error = JsValue;
    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        use Button::*;
        let str = value.as_string().ok_or("Expected string")?;
        match str.to_lowercase().as_ref() {
            "a" => Ok(A),
            "b" => Ok(B),
            "start" => Ok(Start),
            "select" => Ok(Select),
            "left" => Ok(Left),
            "right" => Ok(Right),
            "up" => Ok(Up),
            "down" => Ok(Down),
            _ => Err("Invalid input".into())
        }
    }
}

struct CanvasOutput {
    frame: RwLock<Vec<u8>>
}

impl VideoInterface for CanvasOutput {
    fn draw_pixel(&self, x: u16, y: u16, color: Color) {
        if y >= 240 {
            return
        }
        if x >= 256 {
            return
        }
        let offset1 = (2*y as usize*512 + 2*x as usize) * 4;
        let offset2 = ((2*y as usize + 1)*512 + 2*x as usize) * 4;
        let mut frame = self.frame.write().unwrap_or_else(|_| {
            web_sys::console::log_1(&"Failed to lock write access".into());
            panic!();
        });
        frame[offset1 + 0] = color.0;
        frame[offset1 + 1] = color.1;
        frame[offset1 + 2] = color.2;
        frame[offset1 + 3] = 255;
        frame[offset1 + 4] = color.0;
        frame[offset1 + 5] = color.1;
        frame[offset1 + 6] = color.2;
        frame[offset1 + 7] = 255;
        frame[offset2 + 0] = color.0;
        frame[offset2 + 1] = color.1;
        frame[offset2 + 2] = color.2;
        frame[offset2 + 3] = 255;
        frame[offset2 + 4] = color.0;
        frame[offset2 + 5] = color.1;
        frame[offset2 + 6] = color.2;
        frame[offset2 + 7] = 255;
    }
    fn end_of_frame(&self) {
        let ctx = get_canvas_context();
        let mut frame = self.frame.write().expect_throw("Failed to lock mutex");
        let clamped = wasm_bindgen::Clamped(&mut frame[..]);
        let image_data = web_sys::ImageData::new_with_u8_clamped_array(clamped, 512).unwrap();
        ctx.put_image_data(&image_data, 0.,0.).unwrap();
    }
}

#[wasm_bindgen]
pub fn init_emulator() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let canvas = CanvasOutput { frame: RwLock::new(vec!(0 ; 512 * 480 * 4)) };
    let nes = nes_core::nes_builder()
        .video(canvas)
        .controller(&*CONTROLLER)
        .build(None,None);

    let mut global_emu = get_nes()?;
    global_emu.replace(nes);
    
    Ok(())
}

#[wasm_bindgen]
pub fn advance_frame() -> Result<(), JsValue> {
    let mut global_emu = get_nes()?;
    let nes = global_emu.as_mut().ok_or("Emulator has not been initialized yet")?;

    nes.run_frame().map_err(|e| format!("{}", e))?;

    Ok(())
}

#[wasm_bindgen]
pub fn insert_cartridge(rom: Box<[u8]>) -> Result<(), JsValue> {
    let rom = rom.into_vec();
    let cart = Cart::from_bytes(rom).map_err(|e| format!("{}", e))?;

    let mut global_emu = get_nes()?;
    let nes = global_emu.as_mut().ok_or("Emulator has not been initialized yet")?;
    nes.insert_cartridge(cart);

    Ok(())
}

#[wasm_bindgen]
pub fn reset() -> Result<(), JsValue> {
    let mut global_emu = get_nes()?;
    let nes = global_emu.as_mut().ok_or("Emulator has not been initialized yet")?;
    nes.reset();
    Ok(())
}

#[wasm_bindgen]
pub fn key_down(button: JsValue) -> Result<(), JsValue> {
    let button = Button::try_from(button)?;
    CONTROLLER.buttons_down(button.into());
    Ok(())
}

#[wasm_bindgen]
pub fn key_up(button: JsValue) -> Result<(), JsValue> {
    let button = Button::try_from(button)?;
    CONTROLLER.buttons_up(button.into());
    Ok(())
}

#[wasm_bindgen]
pub fn save_state() -> Result<(), JsValue> {
    let global_emu = get_nes()?;
    global_emu.as_ref().map(|nes| {
        let s = nes.save_state();
        let mut g_s = SAVE_STATE.write().expect("Failed to write to SAVE_STATE");
        g_s.replace(s);
    });
    
    Ok(())
}

#[wasm_bindgen]
pub fn load_state() -> Result<(), JsValue> {
    let mut global_emu = get_nes()?;
    global_emu.as_mut().map(|nes| {
        let g_s = SAVE_STATE.read().expect("Failed to read from SAVE_STATE");
        g_s.as_ref().map(|s| nes.load_state(s.clone()));
    });
    
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

fn get_canvas() -> web_sys::HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let html_canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("nes_canvas")
        .unwrap()
        .dyn_into()
        .unwrap();
    html_canvas
}

fn get_nes() -> Result<MutexGuard<'static, Option<Nes>>, JsValue> {
    EMULATOR.lock().map_err(|_| "Failed to acquire mutex".into())
}