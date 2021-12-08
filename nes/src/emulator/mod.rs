mod audio;
mod controller;
mod screen;

use audio::Audio;
use controller::Controller;
use screen::Screen;

use anyhow::Result;
use nes_core::controller::ControllerState;
use nes_core::nes::NesSaveState;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

type Nes = nes_core::nes::Nes<Screen, Controller, Audio>;

struct App {
    event_loop: Option<winit::event_loop::EventLoop<AppEvent>>,
    el_proxy: winit::event_loop::EventLoopProxy<AppEvent>,
    audio_thread: audio::AudioPlayer,
    nes: Nes,
}

impl App {
    fn new(rom_path: String) -> Result<Self> {
        let event_loop = winit::event_loop::EventLoop::with_user_event();
        let el_proxy = event_loop.create_proxy();

        let cart = nes_core::cart::Cart::from_file(rom_path)?;
        let screen = Screen::new(&event_loop)?;
        let controller = Controller::new();
        let audio = audio::Audio {};
        let audio_thread = audio::AudioPlayer::new()?;
        let config = None;
        let nes = Nes::new(cart, screen, controller, audio, config);

        Ok(App {
            event_loop: Some(event_loop),
            el_proxy,
            nes,
            audio_thread,
        })
    }

    fn update_title(&self) {
        self.screen().as_window().set_title(&format!(
            "NES - Vol: {}%",
            self.audio_thread.get_volume() / 10
        ));
    }

    fn screen(&self) -> &Screen {
        self.nes.get_screen()
    }

    fn check_window(&self, window_id: winit::window::WindowId) -> Option<AppWindow> {
        if self.screen().as_window().id() == window_id {
            Some(AppWindow::Screen)
        } else {
            None
        }
    }

    fn handle_kbd(&mut self, input: KeyboardInput) {
        match input {
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Plus),
                state: ElementState::Pressed,
                ..
            } => self.el_proxy.send_event(AppEvent::VolumeUp).unwrap(),
            KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Minus),
                state: ElementState::Pressed,
                ..
            } => self.el_proxy.send_event(AppEvent::VolumeDown).unwrap(),
            _ => (),
        }
    }

    fn run(mut self) -> ! {
        self.update_title();
        self.event_loop.take().unwrap().run(move |ev, _, cf| {
            use winit::event::{Event, WindowEvent};
            use winit::event_loop::ControlFlow;
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *cf = ControlFlow::Exit;
                    }
                    WindowEvent::KeyboardInput { input, .. } => self.handle_kbd(input),
                    _ => (),
                },
                Event::RedrawRequested(wid) => match self.check_window(wid) {
                    Some(AppWindow::Screen) => (),
                    None => (),
                },
                Event::UserEvent(event) => match event {
                    AppEvent::VolumeUp => {
                        dbg!(self.audio_thread.change_volume(100));
                        self.update_title();
                    }
                    AppEvent::VolumeDown => {
                        dbg!(self.audio_thread.change_volume(-100));
                        self.update_title();
                    }
                },
                _ => (),
            }
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum AppWindow {
    Screen,
}

#[derive(Debug, Clone, Copy)]
enum AppEvent {
    VolumeUp,
    VolumeDown,
}

pub fn run(rom_name: String) -> Result<()> {
    let app = App::new(rom_name)?;
    app.run()
}
