#![allow(dead_code)]

use nes_core::{
    apu::AudioOutput,
    cart::Cart,
    controller::{ControllerState, NESController},
    ppu::{Color, VideoInterface},
};
use std::convert::TryFrom;
use std::sync::RwLock;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "queueAudio")]
    fn queue_audio(samples: &[f32]);
}

#[wasm_bindgen]
pub fn initialize() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Nes(nes_core::nes::Nes<CanvasOutput, Controller, Audio>);

#[wasm_bindgen]
#[derive(Clone)]
pub struct NesSaveState(nes_core::nes::NesSaveState);

struct Controller {
    buttons: ControllerState,
}

impl Controller {
    fn new() -> Self {
        Controller {
            buttons: ControllerState::empty(),
        }
    }
    fn buttons_down(&mut self, buttons: ControllerState) {
        self.buttons.insert(buttons);
    }
    fn buttons_up(&mut self, buttons: ControllerState) {
        self.buttons.remove(buttons);
    }
}

impl NESController for Controller {
    fn poll_controller(&self) -> ControllerState {
        self.buttons
    }
}

#[derive(Debug, Copy, Clone)]
enum Button {
    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
}

impl From<Button> for JsValue {
    fn from(val: Button) -> Self {
        use Button::*;
        match val {
            A => "A".into(),
            B => "B".into(),
            Start => "Start".into(),
            Select => "Select".into(),
            Left => "Left".into(),
            Right => "Right".into(),
            Up => "Up".into(),
            Down => "Down".into(),
        }
    }
}

impl From<Button> for ControllerState {
    fn from(val: Button) -> Self {
        use Button::*;
        match val {
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
            _ => Err("Invalid input".into()),
        }
    }
}

struct CanvasOutput {
    frame: RwLock<Vec<u8>>,
}

impl VideoInterface for CanvasOutput {
    fn draw_pixel(&mut self, x: u16, y: u16, color: Color) {
        if y >= 240 {
            return;
        }
        if x >= 256 {
            return;
        }
        let offset1 = (2 * y as usize * 512 + 2 * x as usize) * 4;
        let offset2 = offset1 + 4;
        let offset3 = ((2 * y as usize + 1) * 512 + 2 * x as usize) * 4;
        let offset4 = offset3 + 4;

        let mut frame = self.frame.write().unwrap();
        for pix_off in [offset1, offset2, offset3, offset4] {
            frame[pix_off] = color.0;
            frame[pix_off + 1] = color.1;
            frame[pix_off + 2] = color.2;
            frame[pix_off + 3] = 255;
        }
    }
    fn end_of_frame(&mut self) {
        let ctx = get_canvas_context();
        let frame = self.frame.read().unwrap();
        let frame_copy = {
            let mut _fc = [0; 512 * 480 * 4];
            assert!(frame.len() == 512 * 480 * 4);
            _fc.copy_from_slice(&frame[..512 * 480 * 4]);
            _fc
        };
        let clamped = wasm_bindgen::Clamped(&frame_copy[..]);
        let image_data = web_sys::ImageData::new_with_u8_clamped_array(clamped, 512).unwrap();
        ctx.put_image_data(&image_data, 0., 0.).unwrap();
    }
}

#[wasm_bindgen]
pub struct Audio {}

#[wasm_bindgen]
impl Audio {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Audio {}
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioOutput for Audio {
    fn queue_audio(&mut self, samples: &[f32]) -> Result<(), String> {
        queue_audio(samples);

        Ok(())
    }
    fn sample_rate(&self) -> usize {
        44100
    }
}

#[wasm_bindgen]
pub fn init_emulator(audio: Audio) -> Result<Nes, JsValue> {
    let canvas = CanvasOutput {
        frame: RwLock::new(vec![0; 512 * 480 * 4]),
    };
    let controller = Controller {
        buttons: ControllerState::empty(),
    };
    let nes = nes_core::nes_builder()
        .video(canvas)
        .controller(controller)
        .audio(audio)
        .build(None, None);

    Ok(Nes(nes))
}

#[wasm_bindgen]
pub fn advance_frame(nes: &mut Nes) -> Result<(), JsValue> {
    nes.0.run_frame().map_err(|e| format!("{}", e).into())
}

#[wasm_bindgen]
pub fn insert_cartridge(nes: &mut Nes, rom: Box<[u8]>) -> Result<(), JsValue> {
    let rom = rom.into_vec();
    let cart = Cart::from_bytes(rom).map_err(|e| format!("{}", e))?;

    nes.0.insert_cartridge(cart);

    Ok(())
}

#[wasm_bindgen]
pub fn reset(nes: &mut Nes) {
    nes.0.reset();
}

#[wasm_bindgen]
pub fn key_down(nes: &mut Nes, button: JsValue) -> Result<(), JsValue> {
    let button = Button::try_from(button)?;
    nes.0.get_controller_mut().buttons_down(button.into());
    Ok(())
}

#[wasm_bindgen]
pub fn key_up(nes: &mut Nes, button: JsValue) -> Result<(), JsValue> {
    let button = Button::try_from(button)?;
    nes.0.get_controller_mut().buttons_up(button.into());
    Ok(())
}

#[wasm_bindgen]
pub fn save_state(nes: &Nes) -> NesSaveState {
    NesSaveState(nes.0.save_state())
}

#[wasm_bindgen]
pub fn load_state(nes: &mut Nes, s: &NesSaveState) {
    nes.0.load_state(s.0.clone());
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
