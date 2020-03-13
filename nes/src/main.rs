extern crate nes_core;
extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use nes_core::controller::ControllerState;
use nes_core::nes::NesSaveState;
use nes_core::apu::AudioOutput;
use std::env;
use std::sync::RwLock;


const TITLE: &'static str = "NES Emulator";

//const FONT: &'static [u8] = include_bytes!(r"../font/DejaVuSansMono.ttf");

const PIXEL_SCALE: u32 = 2;
const GAME_WIDTH: u32 = 256*PIXEL_SCALE;
const GAME_HEIGHT: u32 = 240*PIXEL_SCALE;
const DEBUG_WIDTH: u32 = 300*PIXEL_SCALE;

struct Screen<'a> {
    txt: RwLock<sdl2::render::Texture<'a>>,
    buffer: RwLock<Vec<(u8,u8,u8)>>,
    w: u32, h: u32
}
impl<'a> Screen<'a> {
    fn new<T>(tc: &'a sdl2::render::TextureCreator<T>, w: u32, h: u32) -> Result<Self,sdl2::render::TextureValueError> {
        let txt = tc.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, GAME_WIDTH, GAME_HEIGHT)?;
        let txt = RwLock::new(txt);
        let buffer = vec![(255,255,255); (w*h) as usize];
        let buffer = RwLock::new(buffer);
        Ok(Screen {
            txt, buffer, w, h
        })
    }
}

impl<'a> nes_core::ppu::VideoInterface for Screen<'a> {
    fn draw_pixel(&self, x: u16, y: u16, color: nes_core::ppu::Color) {
        if (x as u32) < self.w && (y as u32) < self.h {
            let mut buffer = self.buffer.write().unwrap();
            buffer[y as usize*self.w as usize + x as usize] = color.into_tuple();
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
                    buf[txt_off+1] = c.1;
                    buf[txt_off+2] = c.2;
                }
            }
        }).unwrap();
    }
}

struct DebugScreen<'a> {
    pt1: sdl2::render::Texture<'a>,
    pt2: sdl2::render::Texture<'a>,
}

impl<'a> DebugScreen<'a> {
    fn new<T>(tc: &'a sdl2::render::TextureCreator<T>) -> Result<Self, Box<dyn std::error::Error>> {
        let pt1 = tc.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, 128,128)?;
        let pt2 = tc.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, 128,128)?;
        Ok(DebugScreen {
            pt1,pt2
        })
    }
}

struct NametableViewer {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    tc: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    window_id: u32
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
}

impl nes_core::controller::NESController for Controller {
    fn poll_controller(&self) -> ControllerState {
        *self.buttons.read().unwrap()
    }
}

struct Audio {
    device: sdl2::audio::AudioQueue<f32>
}

impl AudioOutput for Audio {
    fn queue_audio(&mut self, samples: &[f32]) -> Result<(), String> {
        self.device.queue(samples);
        Ok(())
    }
    fn sample_rate(&self) -> usize {
        self.device.spec().freq as usize
    }
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let rom_name = args.get(1).expect("Provide a path to a ROM as an argument.");


    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let audio_ctx = sdl_ctx.audio().unwrap();
    let mut window = video.window(TITLE, GAME_WIDTH + DEBUG_WIDTH, GAME_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();
        
    window.set_title(&format!("{}: {}x{}", TITLE, GAME_WIDTH + DEBUG_WIDTH, GAME_HEIGHT)).unwrap();
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
    let mut nes = nes_core::nes::Nes::new(cart, &screen, &controller, audio, None);

    let mut save_state: Option<NesSaveState> = None;

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let mut selected_palette = 0;
    let mut paused = false;
    let mut muted = false;
    let mut frame_unlocked = false;

    let mut debug_screen = DebugScreen::new(&tc).unwrap();
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
                    TITLE, GAME_WIDTH + DEBUG_WIDTH, GAME_HEIGHT, fps, if paused {"Paused"} else {""}, if muted {"Muted"} else {&vol_pct})
                ).unwrap();
    };

    canvas.set_draw_color((0,0,255));
    canvas.clear();

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
                    nametable_viewer.replace(create_nametable_viewer(&video));
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
                    frame_unlocked = !frame_unlocked;
                    if frame_unlocked {
                        nes.apu.audio_device().device.pause();
                        nes.apu.audio_device().device.clear();
                    } else {
                        nes.apu.audio_device().device.resume();
                    }
                },

                Event::KeyDown {keycode: Some(Keycode::P), ..} => {
                    paused = !paused;
                    if paused {
                        nes.apu.audio_device().device.clear();
                    }
                    update_title(&mut canvas, fps, paused, muted, nes.apu.volume);
                },

                Event::KeyDown {keycode: Some(Keycode::M), ..} => {
                    muted = !muted;
                    if muted {
                        nes.apu.audio_device().device.clear();
                        nes.apu.audio_device().device.pause();
                    } else {
                        nes.apu.audio_device().device.clear();
                        nes.apu.audio_device().device.resume();
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
                    selected_palette += 1;
                    selected_palette %= 8;
                    update_debug = true;
                },
                Event::KeyDown {keycode: Some(Keycode::LeftBracket), ..} => {
                    selected_palette += 7;
                    selected_palette %= 8;
                    update_debug = true;
                },
                Event::KeyDown {keycode: Some(Keycode::O), ..} => {
                    println!("{}", nes.ppu.print_oam());
                },
                Event::KeyDown {keycode: Some(Keycode::R), ..} => {
                    println!("{:X?}", nes.mmu.ppu_registers);
                },

                Event::KeyDown {keycode: Some(Keycode::Z), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.insert(ControllerState::A);
                },
                Event::KeyUp {keycode: Some(Keycode::Z), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.remove(ControllerState::A);
                },
                Event::KeyDown {keycode: Some(Keycode::X), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.insert(ControllerState::B);
                },
                Event::KeyUp {keycode: Some(Keycode::X), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.remove(ControllerState::B);
                },
                Event::KeyDown {keycode: Some(Keycode::G), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.insert(ControllerState::SELECT);
                },
                Event::KeyUp {keycode: Some(Keycode::G), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.remove(ControllerState::SELECT);
                },
                Event::KeyDown {keycode: Some(Keycode::H), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.insert(ControllerState::START);
                },
                Event::KeyUp {keycode: Some(Keycode::H), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.remove(ControllerState::START);
                },
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.insert(ControllerState::LEFT);
                },
                Event::KeyUp {keycode: Some(Keycode::Left), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.remove(ControllerState::LEFT);
                },
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.insert(ControllerState::RIGHT);
                },
                Event::KeyUp {keycode: Some(Keycode::Right), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.remove(ControllerState::RIGHT);
                },
                Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.insert(ControllerState::UP);
                },
                Event::KeyUp {keycode: Some(Keycode::Up), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.remove(ControllerState::UP);
                },
                Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.insert(ControllerState::DOWN);
                },
                Event::KeyUp {keycode: Some(Keycode::Down), ..} => {
                    let mut c = controller.buttons.write().unwrap(); 
                    c.remove(ControllerState::DOWN);
                },
                _ => {}
            }
        }

        if !paused {
            nes.run_frame().expect("CPU Error");
            {
                let txt = screen.txt.read().unwrap();
                canvas.copy(
                    &txt, 
                    Rect::new(0,0,GAME_WIDTH/PIXEL_SCALE, GAME_HEIGHT/PIXEL_SCALE), 
                    Rect::new(0,0, GAME_WIDTH, GAME_HEIGHT)
                ).unwrap();
            }
        }
        if !paused || update_debug {
            // Update the debug screen
            let pattern_table = nes.pattern_table();
            let palette = nes.get_palette(selected_palette);
            let bg_color = nes.get_palette(0)[0];
            debug_screen.pt1.with_lock(None, |buf, pitch| {
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
            debug_screen.pt2.with_lock(None, |buf, pitch| {
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
            update_debug = false;
        }

        // Draw the palette selector
        const PALETTE_X: i32 = GAME_WIDTH as i32 + 15;
        const PALETTE_Y: i32 = 15 + (128 * PIXEL_SCALE as i32) + 15;
        const PALETTE_SCALE: u32 = PIXEL_SCALE * 4;
        for pi in 0..8 {
            let offset_x = PALETTE_X + pi as i32 * ((4*PALETTE_SCALE as i32) + 5);
            if pi == selected_palette {
                canvas.set_draw_color((255,255,255));
                canvas.fill_rect(Rect::new(offset_x-1, PALETTE_Y-1, 4*PALETTE_SCALE + 2, PALETTE_SCALE + 2)).unwrap();
            } else {
                canvas.set_draw_color((0,0,255));
                canvas.fill_rect(Rect::new(offset_x-1, PALETTE_Y-1, 4*PALETTE_SCALE + 2, PALETTE_SCALE + 2)).unwrap();
            }
            for i in 0..4 {
                canvas.set_draw_color(nes.get_palette(pi)[i as usize]);
                canvas.fill_rect(Rect::new(offset_x + i*PALETTE_SCALE as i32, PALETTE_Y, PALETTE_SCALE, PALETTE_SCALE)).unwrap();
            }
        }

        canvas.copy(&debug_screen.pt1, None, Rect::new(GAME_WIDTH as i32 + 15                         , 15, 128*PIXEL_SCALE, 128*PIXEL_SCALE)).unwrap();
        canvas.copy(&debug_screen.pt2, None, Rect::new(GAME_WIDTH as i32 + 128*PIXEL_SCALE as i32 + 30, 15, 128*PIXEL_SCALE, 128*PIXEL_SCALE)).unwrap();

        // Update the nametable viewer (if it exists)
        nametable_viewer.as_mut().map(|nt_v| {
            let mut txt = nt_v.tc.create_texture_streaming(sdl2::pixels::PixelFormatEnum::RGB24, 512, 480).unwrap();
            txt.with_lock(None, |buf, pitch| {
                let pattern_table = nes.pattern_table();
                let pattern_table = if nes.mmu.ppu_registers.ppu_ctrl & 0b00010000 == 0 {
                    &pattern_table[0..0x1000]
                } else {
                    &pattern_table[0x1000 ..]
                };
                let palette_table = nes.palette_table();
                let bg_color = palette_table[0];
                let vram = nes.get_nametables();
                for table in 0..4 {
                    let table_off = 0x400 * table;
                    let (table_x, table_y) = match table {
                        0 => (0,0),
                        1 => (256,0),
                        2 => (0,240),
                        3 => (256,240),
                        _ => unreachable!()
                    };
                    let nametable = &vram[table_off .. table_off + 0x400];
                    let attr_table = &nametable[0x3c0 ..];
                    for row in 0..30 {
                        for tile in 0..32 {
                            let attr = attr_table[(row/4)*0x8 + (tile/4)];
                            let palette = match ((row/2)%2)*2 + ((tile/2)%2) {
                                0b00 => attr & 0b00000011,
                                0b01 => (attr & 0b00001100) >> 2,
                                0b10 => (attr & 0b00110000) >> 4,
                                0b11 => (attr & 0b11000000) >> 6,
                                _ => unreachable!()
                            };
                            let pattern_byte = nametable[row*0x20 + tile];
                            for p_row in 0..8 {
                                let lo = pattern_table[((pattern_byte as u16)<<4) as usize + p_row];
                                let hi = pattern_table[((pattern_byte as u16)<<4) as usize + p_row + 8];
                                for pixel in 0..8 {
                                    let px_hi = (hi >> (7-pixel)) & 1;
                                    let px_lo = (lo >> (7-pixel)) & 1;
                                    let px = px_hi << 1 | px_lo;
                                    let (r,g,b) = if px == 0 {
                                        bg_color
                                    } else {
                                        palette_table[(palette<<2 | px) as usize]
                                    };
                                    // print!("{:?}",(r,g,b));
                                    buf[pitch*((table_y + row*8) + p_row) + (table_x + tile*8 + pixel)*3 + 0] = r;
                                    buf[pitch*((table_y + row*8) + p_row) + (table_x + tile*8 + pixel)*3 + 1] = g;
                                    buf[pitch*((table_y + row*8) + p_row) + (table_x + tile*8 + pixel)*3 + 2] = b;
                                }
                            }
                        }
                    }
                }
            }).unwrap();
            nt_v.canvas.copy(&txt, None, Rect::new(0,0,512,480)).unwrap();
            nt_v.canvas.present();
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

fn create_nametable_viewer(video: &sdl2::VideoSubsystem) -> NametableViewer {
    let mut window = video
        .window("Nametable viewer", 512, 480)
        .build().unwrap();
    window.set_minimum_size(512,480).unwrap();
    let window_id = window.id();
    let canvas = window.into_canvas().build().unwrap();
    let tc = canvas.texture_creator();
    NametableViewer {
        canvas, tc, window_id
    }
}