mod audio;
mod components;
mod controller;
mod input;
mod screen;

use self::audio::{Audio, AudioPlayer};
use anyhow::Result;
use controller::Controller;
use iced::{Application, Length};
use screen::Screen;

type Nes = nes_core::nes::Nes<Screen, Controller, Audio>;

#[derive(Debug, PartialEq)]
enum AppState {
    Empty,
    Running,
    Paused,
}

#[derive(Default)]
pub struct Flags {
    pub rom_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    NextFrame,
    ControllerButtonPressed(nes_core::controller::ControllerState),
    ControllerButtonReleased(nes_core::controller::ControllerState),
    TogglePause,
}

struct App {
    state: AppState,
    nes: Nes,
    audio_player: audio::AudioPlayer,
    game_title: String,
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Flags = Flags;
    type Message = Message;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (audio_player, nes_audio) = AudioPlayer::new().unwrap();
        let mut app = App {
            state: AppState::Empty,
            nes: Nes::new(
                None,
                Screen::default(),
                Controller::default(),
                nes_audio,
                None,
            ),
            audio_player,
            game_title: flags.rom_path.as_ref().cloned().unwrap_or_default(),
        };

        if let Some(rom_path) = flags.rom_path {
            let cart = nes_core::cart::Cart::from_file(rom_path).unwrap();
            app.state = AppState::Running;
            app.nes.mmu.cart = Some(cart);
        }

        (app, iced::Command::none())
    }

    fn title(&self) -> String {
        let mut t = String::new();
        t += &self.game_title;
        if self.state == AppState::Paused {
            t += " - PAUSE";
        }
        t
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::NextFrame => {
                if self.state == AppState::Running {
                    self.nes.run_frame().unwrap()
                }
            }
            Message::ControllerButtonPressed(b) => self.nes.get_controller_mut().buttons |= b,
            Message::ControllerButtonReleased(b) => self.nes.get_controller_mut().buttons &= !b,
            Message::TogglePause => match self.state {
                AppState::Running => self.state = AppState::Paused,
                AppState::Paused => self.state = AppState::Running,
                _ => (),
            },
        }

        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let screen = self.nes.get_screen().get_frame();
        let pixels = Vec::from(&screen[..]);

        let image = iced::Image::new(iced::image::Handle::from_pixels(
            screen::SCREEN_WIDTH as u32,
            screen::SCREEN_HEIGHT as u32,
            pixels,
        ))
        .height(Length::Fill)
        .width(Length::Fill);

        iced::Container::new(image)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch([
            iced_native::subscription::events_with(input::event_handler),
            iced::time::every(std::time::Duration::from_micros(16667)).map(|_| Message::NextFrame),
        ])
    }
}

pub fn run(flags: Flags) -> Result<()> {
    App::run(iced::Settings {
        flags,
        antialiasing: true,
        default_font: None,
        default_text_size: 20,
        exit_on_close_request: true,
        window: iced::window::Settings {
            size: (256, 240),
            ..Default::default()
        },
        ..Default::default()
    })?;
    Ok(())
}
