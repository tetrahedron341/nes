mod screen;
mod controller;
mod audio;
mod debug;

use screen::Screen;
use controller::Controller;
use audio::Audio;
use debug::{
    NametableViewer,
    PatternTableViewer
};

use nes_core::controller::ControllerState;
use nes_core::nes::NesSaveState;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

type Nes<'a> = nes_core::nes::Nes<Screen<'a>, Controller, Audio>;

const TITLE: &'static str = "NES Emulator";

//const FONT: &'static [u8] = include_bytes!(r"../font/DejaVuSansMono.ttf");

const PIXEL_SCALE: u32 = 2;
const GAME_WIDTH: u32 = 256*PIXEL_SCALE;
const GAME_HEIGHT: u32 = 240*PIXEL_SCALE;

pub fn run(rom_name: String) {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let audio_ctx = sdl_ctx.audio().unwrap();
    let mut window = video.window(TITLE, GAME_WIDTH, GAME_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();
        
    window.set_title(&format!("{}: {}x{}", TITLE, GAME_WIDTH, GAME_HEIGHT)).unwrap();
    window.set_display_mode(sdl2::video::DisplayMode::new(
        sdl2::pixels::PixelFormatEnum::RGB24,
        window.display_mode().unwrap().w,
        window.display_mode().unwrap().h,
        60
    )).unwrap();

    let main_window_id = window.id();
    
    let mut canvas = window.into_canvas().build().unwrap();
    let tc = canvas.texture_creator();
    let screen = Screen::new(&tc, GAME_WIDTH/PIXEL_SCALE, GAME_HEIGHT/PIXEL_SCALE).unwrap();
    
    let controller = Controller::new();

    let audio = Audio { 
        device: audio_ctx.open_queue(None, &sdl2::audio::AudioSpecDesired {
                channels: Some(1),
                freq: None,
                samples: None
            }).unwrap()
    };
    audio.device.resume();

    let cart = nes_core::cart::Cart::from_file(rom_name).unwrap();
    let mut nes: Nes = nes_core::nes::Nes::new(cart, screen, controller, audio, None);

    let mut save_state: Option<NesSaveState> = None;

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let mut paused = false;
    let mut muted = false;
    let mut frame_unlocked = false;

    let mut debug_screen: Option<PatternTableViewer> = None;
    let mut update_debug = false;

    let mut fps_count = 0;
    let mut fps_timer = std::time::SystemTime::now();
    let mut fps: f64 = 0.0;

    let mut nametable_viewer: Option<NametableViewer> = None;

    let update_title = |canvas: &mut WindowCanvas, fps: f64, paused: bool, muted: bool, volume: f32| {
        let vol_pct = format!("{:.0}%", 100.0 * volume / 5.0);
        canvas
            .window_mut()
            .set_title(
                &format!("{}: {}x{} FPS: {:.2} {} {}", 
                    TITLE, GAME_WIDTH, GAME_HEIGHT, fps, if paused {"Paused"} else {""}, if muted {"Muted"} else {&vol_pct})
                ).unwrap();
    };

    'running: loop {
        let frame_start_time = std::time::SystemTime::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::Escape), window_id, ..} => {
                    if window_id == main_window_id {
                        break 'running
                    } else if Some(window_id) == nametable_viewer.as_ref().map(|nt_v| nt_v.window_id) {
                        nametable_viewer.take();
                    }
                },
                Event::Window {win_event, window_id, ..} => {
                    use sdl2::event::WindowEvent;
                    match win_event {
                        WindowEvent::Close => {
                            if window_id == main_window_id {
                                break 'running
                            } else if Some(window_id) == nametable_viewer.as_ref().map(|nt_v| nt_v.window_id) {
                                nametable_viewer.take();
                            }
                        },
                        _ => ()
                    }
                },
                Event::KeyDown {keycode: Some(Keycode::R), keymod: sdl2::keyboard::Mod::LCTRLMOD, ..} |
                Event::KeyDown {keycode: Some(Keycode::R), keymod: sdl2::keyboard::Mod::RCTRLMOD, ..} => {
                    nes.reset();
                },
                Event::KeyDown {keycode: Some(Keycode::T), keymod: sdl2::keyboard::Mod::LCTRLMOD, ..} |
                Event::KeyDown {keycode: Some(Keycode::T), keymod: sdl2::keyboard::Mod::RCTRLMOD, ..} => {
                    nametable_viewer.replace(NametableViewer::new(&video));
                },
                Event::KeyDown {keycode: Some(Keycode::P), keymod: sdl2::keyboard::Mod::LCTRLMOD, ..} |
                Event::KeyDown {keycode: Some(Keycode::P), keymod: sdl2::keyboard::Mod::RCTRLMOD, ..} => {
                    debug_screen.replace(PatternTableViewer::new(&video).unwrap());
                },
                Event::KeyDown {keycode: Some(Keycode::S), keymod: sdl2::keyboard::Mod::LCTRLMOD, ..} |
                Event::KeyDown {keycode: Some(Keycode::S), keymod: sdl2::keyboard::Mod::RCTRLMOD, ..} => {
                    save_state = Some(nes.save_state());
                },
                Event::KeyDown {keycode: Some(Keycode::L), keymod: sdl2::keyboard::Mod::LCTRLMOD, ..} |
                Event::KeyDown {keycode: Some(Keycode::L), keymod: sdl2::keyboard::Mod::RCTRLMOD, ..} => {
                    save_state.as_ref().map(|s| nes.load_state(s.clone()));
                },

                Event::KeyDown {keycode: Some(Keycode::LAlt), ..} |
                Event::KeyDown {keycode: Some(Keycode::RAlt), ..} => {
                    frame_unlocked = true;

                    nes.get_audio_device_mut().device.pause();
                    nes.get_audio_device_mut().device.clear();
                },

                Event::KeyUp {keycode: Some(Keycode::LAlt), ..} |
                Event::KeyUp {keycode: Some(Keycode::RAlt), ..} => {
                    frame_unlocked = false;
                    nes.get_audio_device_mut().device.resume();
                },

                Event::KeyDown {keycode: Some(Keycode::P), ..} => {
                    paused = !paused;
                    if paused {
                        nes.get_audio_device_mut().device.clear();
                    }
                    update_title(&mut canvas, fps, paused, muted, nes.apu.volume);
                },

                Event::KeyDown {keycode: Some(Keycode::M), ..} => {
                    muted = !muted;
                    if muted {
                        nes.get_audio_device_mut().device.clear();
                        nes.get_audio_device_mut().device.pause();
                    } else {
                        nes.get_audio_device_mut().device.clear();
                        nes.get_audio_device_mut().device.resume();
                    }
                    update_title(&mut canvas, fps, paused, muted, nes.apu.volume);
                },

                Event::KeyDown {keycode: Some(Keycode::Plus), ..} |
                Event::KeyDown {keycode: Some(Keycode::Equals), keymod: sdl2::keyboard::Mod::LSHIFTMOD, ..} |
                Event::KeyDown {keycode: Some(Keycode::Equals), keymod: sdl2::keyboard::Mod::RSHIFTMOD, ..} |
                Event::KeyDown {keycode: Some(Keycode::KpPlus), ..} => {
                    nes.apu.volume += 0.1;
                    nes.apu.volume = nes.apu.volume.min(5.0);
                    println!("Volume set to: {}", nes.apu.volume);
                    update_title(&mut canvas, fps, paused, muted, nes.apu.volume);
                },
                Event::KeyDown {keycode: Some(Keycode::Minus), ..} |
                Event::KeyDown {keycode: Some(Keycode::KpMinus), ..} => {
                    nes.apu.volume -= 0.1;
                    nes.apu.volume = nes.apu.volume.max(0.0);
                    println!("Volume set to: {}", nes.apu.volume);
                    update_title(&mut canvas, fps, paused, muted, nes.apu.volume);
                },

                Event::KeyDown {keycode: Some(Keycode::RightBracket), ..} => {
                    debug_screen.as_mut().map(|ds| {
                        ds.selected_palette += 1;
                        ds.selected_palette %= 8;
                        update_debug = true;
                    });
                },
                Event::KeyDown {keycode: Some(Keycode::LeftBracket), ..} => {
                    debug_screen.as_mut().map(|ds| {
                        ds.selected_palette += 7;
                        ds.selected_palette %= 8;
                        update_debug = true;
                    });
                },
                Event::KeyDown {keycode: Some(Keycode::O), ..} => {
                    println!("{}", nes.ppu.print_oam());
                    // println!("{}", std::str::from_utf8(nes.mmu.blargg_debug_text()).unwrap())
                },
                Event::KeyDown {keycode: Some(Keycode::R), ..} => {
                    println!("{:X?}", nes.mmu.ppu_registers);
                },

                Event::KeyDown {keycode: Some(Keycode::Z), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.insert(ControllerState::A);
                },
                Event::KeyUp {keycode: Some(Keycode::Z), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.remove(ControllerState::A);
                },
                Event::KeyDown {keycode: Some(Keycode::X), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.insert(ControllerState::B);
                },
                Event::KeyUp {keycode: Some(Keycode::X), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.remove(ControllerState::B);
                },
                Event::KeyDown {keycode: Some(Keycode::G), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.insert(ControllerState::SELECT);
                },
                Event::KeyUp {keycode: Some(Keycode::G), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.remove(ControllerState::SELECT);
                },
                Event::KeyDown {keycode: Some(Keycode::H), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.insert(ControllerState::START);
                },
                Event::KeyUp {keycode: Some(Keycode::H), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.remove(ControllerState::START);
                },
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.insert(ControllerState::LEFT);
                },
                Event::KeyUp {keycode: Some(Keycode::Left), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.remove(ControllerState::LEFT);
                },
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.insert(ControllerState::RIGHT);
                },
                Event::KeyUp {keycode: Some(Keycode::Right), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.remove(ControllerState::RIGHT);
                },
                Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.insert(ControllerState::UP);
                },
                Event::KeyUp {keycode: Some(Keycode::Up), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.remove(ControllerState::UP);
                },
                Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.insert(ControllerState::DOWN);
                },
                Event::KeyUp {keycode: Some(Keycode::Down), ..} => {
                    let mut c = nes.get_controller_mut().buttons.write().unwrap(); 
                    c.remove(ControllerState::DOWN);
                },
                _ => {}
            }
        }

        if !paused {
            if let Err(e) = nes.run_frame() {
                panic!("{}", e);
            };
            let txt = nes.get_screen().txt.read().unwrap();
            canvas.copy(
                &txt, 
                Rect::new(0,0,GAME_WIDTH/PIXEL_SCALE, GAME_HEIGHT/PIXEL_SCALE), 
                Rect::new(0,0, GAME_WIDTH, GAME_HEIGHT)
            ).unwrap();
        }
        
        if !paused || update_debug {
            debug_screen.as_mut().map(|ds| ds.update(&nes));
            update_debug = false;
        }
        
        // Update the nametable viewer (if it exists)
        nametable_viewer.as_mut().map(|nt_v| {
            nt_v.update(&nes);
        });

        canvas.present();
        fps_count += 1;
        if fps_count >= 65 {
            let t = fps_timer.elapsed().unwrap();
            fps = (1.0 / t.as_secs_f64()) * 65.0;
            fps_timer = std::time::SystemTime::now();
            fps_count = 0;
            update_title(&mut canvas, fps, paused, muted, nes.apu.volume);
        }
        if !frame_unlocked {
            std::thread::sleep(std::time::Duration::from_micros(1_000_000 / 65).checked_sub(frame_start_time.elapsed().unwrap()).unwrap_or_default())
        }
    }
}