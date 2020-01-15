extern crate nes_core;
extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use nes_core::controller::ControllerState;
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

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let rom_name = args.get(1).expect("Provide a path to a ROM as an argument.");


    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
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
    
    let mut canvas = window.into_canvas().build().unwrap();
    let tc = canvas.texture_creator();
    let screen = Screen::new(&tc, GAME_WIDTH/PIXEL_SCALE, GAME_HEIGHT/PIXEL_SCALE).unwrap();
    println!("{}x{}", screen.w, screen.h);
    
    let controller = Controller::new();

    let cart = nes_core::cart::Cart::from_file(rom_name).unwrap();
    let mut nes = nes_core::nes::Nes::new(cart, &screen, &controller, None);

    println!("VRAM: {:p}", &nes.mmu.vram);

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    let mut selected_palette = 0;
    let mut paused = false;

    let mut update_debug = true;

    let mut fps_count = 0;
    let mut fps_timer = std::time::SystemTime::now();
    let mut fps: f64;

    'running: loop {
        let frame_start_time = std::time::SystemTime::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::R), keymod: sdl2::keyboard::Mod::LCTRLMOD, ..} |
                Event::KeyDown {keycode: Some(Keycode::R), keymod: sdl2::keyboard::Mod::RCTRLMOD, ..} => {
                    nes.cpu.reset();
                },
                Event::KeyDown {keycode: Some(Keycode::U), ..} => {
                    update_debug = true;
                },
                Event::KeyDown {keycode: Some(Keycode::P), ..} => {
                    paused = !paused;
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

        if update_debug {
            canvas.set_draw_color((0,0,255));
            canvas.fill_rect(Rect::new(GAME_WIDTH as i32, 0, DEBUG_WIDTH, GAME_HEIGHT)).unwrap();

            // Draw the pattern table
            let pattern_table = nes.pattern_table();
            let table_left = &pattern_table[0..0x1000];
            let table_right = &pattern_table[0x1000..0x2000];

            const PATTERN_X: i32 = GAME_WIDTH as i32 + 15;
            const PATTERN_Y: i32 = 15;

            draw_pattern_table(&mut canvas, table_left, &nes.get_palette(selected_palette), PATTERN_X, PATTERN_Y, PIXEL_SCALE);
            draw_pattern_table(&mut canvas, table_right, &nes.get_palette(selected_palette), PATTERN_X+128*PIXEL_SCALE as i32+15, PATTERN_Y, PIXEL_SCALE);

            // Draw the palette selector
            const PALETTE_X: i32 = PATTERN_X;
            const PALETTE_Y: i32 = PATTERN_Y + (128 * PIXEL_SCALE as i32) + 15;
            const PALETTE_SCALE: u32 = PIXEL_SCALE * 4;
            for pi in 0..8 {
                let offset_x = PALETTE_X + pi as i32 * ((4*PALETTE_SCALE as i32) + 5);
                if pi == selected_palette {
                    canvas.set_draw_color((255,255,255));
                    canvas.fill_rect(Rect::new(offset_x-1, PALETTE_Y-1, 4*PALETTE_SCALE + 2, PALETTE_SCALE + 2)).unwrap();
                }
                for i in 0..4 {
                    canvas.set_draw_color(nes.get_palette(pi)[i as usize]);
                    canvas.fill_rect(Rect::new(offset_x + i*PALETTE_SCALE as i32, PALETTE_Y, PALETTE_SCALE, PALETTE_SCALE)).unwrap();
                }
            }

            update_debug = false;
        }

        canvas.present();
        fps_count += 1;
        if fps_count >= 60 {
            let t = fps_timer.elapsed().unwrap();
            fps = (1.0 / t.as_secs_f64()) * 60.0;
            fps_timer = std::time::SystemTime::now();
            fps_count = 0;
            canvas.window_mut().set_title(&format!("{}: {}x{} FPS: {:.2} {}", TITLE, GAME_WIDTH + DEBUG_WIDTH, GAME_HEIGHT, fps, if paused {"Paused"} else {""})).unwrap();
        }
        std::thread::sleep(std::time::Duration::from_micros(1_000_000 / 60).checked_sub(frame_start_time.elapsed().unwrap()).unwrap_or_default())
    }
}

fn draw_pattern_table(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, 
    pattern_table: &[u8], 
    palette: &[(u8,u8,u8)],
    start_x: i32,
    start_y: i32, 
    scale: u32
) {
    for ty in 0..16usize {
        for tx in 0..16usize {
            let tile_addr = ty<<8 | tx<<4;
            let bp_upper = &pattern_table[tile_addr | 0x8 ..= tile_addr | 0xf];
            let bp_lower = &pattern_table[tile_addr | 0x0 ..= tile_addr | 0x7];
            for y in 0..8 {
                let byte_upper = bp_upper[y as usize];
                let byte_lower = bp_lower[y as usize];
                for x in 0..8 {
                    let bit_mask = 0x80 >> x;
                    let bit_high = (byte_upper & bit_mask) >> (7-x);
                    let bit_low = (byte_lower & bit_mask) >> (7-x);
                    let palette_select = (bit_high<<1) | bit_low;

                    // TODO: Use palette colors in debug pattern table display
                    let color: (u8,u8,u8) = palette[palette_select as usize];

                    canvas.set_draw_color(color);
                    let offset_x = (8*tx as i32+x) * scale as i32;
                    let offset_y = (8*ty as i32+y) * scale as i32;
                    canvas.fill_rect(Rect::new(start_x+offset_x, start_y+offset_y, scale, scale)).unwrap();
                }
            }
        }
    }
}