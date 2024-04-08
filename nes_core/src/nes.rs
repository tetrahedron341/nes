use crate::apu::{AudioOutput, APU};
use crate::cart::Cart;
use crate::controller::NESController;
use crate::error::*;
use crate::mmu::{MMUSaveState, MMU};
use crate::mos6502::MOS6502;
use crate::ppu::{Color, PPUSaveState, VideoInterface, PPU};
use bitflags::bitflags;

bitflags! {
    pub struct NESConfig : u16 {
        const DEBUG = 1 << 0;
        const DEBUG_OUTPUT = 1 << 1 | Self::DEBUG.bits;
    }
}

impl From<&NESConfig> for crate::mos6502::CPUConfig {
    fn from(val: &NESConfig) -> Self {
        use crate::mos6502::CPUConfig;
        let mut out = CPUConfig::empty();
        out.set(CPUConfig::DEBUG, val.contains(NESConfig::DEBUG));
        out.set(
            CPUConfig::DEBUG_OUTPUT,
            val.contains(NESConfig::DEBUG_OUTPUT),
        );
        out
    }
}
impl From<&NESConfig> for crate::mmu::MMUConfig {
    fn from(val: &NESConfig) -> Self {
        use crate::mmu::MMUConfig;
        let mut out = MMUConfig::empty();
        out.set(MMUConfig::DEBUG, val.contains(NESConfig::DEBUG));
        out.set(
            MMUConfig::DEBUG_OUTPUT,
            val.contains(NESConfig::DEBUG_OUTPUT),
        );
        out
    }
}

struct NesVideoWrapper<V> {
    screen: V,
    frame_completed: std::cell::Cell<bool>,
}

impl<V: VideoInterface> VideoInterface for NesVideoWrapper<V> {
    #[inline]
    fn draw_pixel(&mut self, x: u16, y: u16, color: Color) {
        self.screen.draw_pixel(x, y, color);
    }
    #[inline]
    fn end_of_frame(&mut self) {
        self.frame_completed.set(true);
        self.screen.end_of_frame();
    }
}

#[derive(Clone)]
pub struct NesSaveState {
    cpu_state: MOS6502,
    mmu_state: MMUSaveState,
    ppu_state: PPUSaveState,
}

/// Represents the NES system.
pub struct Nes<V: VideoInterface, C: NESController, A: AudioOutput> {
    pub cpu: MOS6502,
    pub ppu: PPU,
    pub mmu: MMU<C>,
    pub apu: APU<A>,
    screen: NesVideoWrapper<V>,
    cycles_counter: u32,
    oam_write: Option<u8>,
    _config: NESConfig,
}

impl<V: VideoInterface, C: NESController, A: AudioOutput> Nes<V, C, A> {
    pub fn new<T: Into<Option<Cart>>>(
        cart: T,
        screen: V,
        controller: C,
        audio: A,
        config: Option<NESConfig>,
    ) -> Self {
        let mut cpu = MOS6502::new(config.as_ref().map(|c| c.into()));
        cpu.reset();
        let ppu = PPU::default();
        let apu = APU::new(audio);
        let mmu = MMU::new(cart, controller, config.as_ref().map(|c| c.into()));

        Nes {
            cpu,
            ppu,
            apu,
            mmu,
            screen: NesVideoWrapper {
                screen,
                frame_completed: std::cell::Cell::new(false),
            },
            cycles_counter: 0,
            oam_write: None,
            _config: config.unwrap_or_else(NESConfig::empty),
        }
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.mmu.reset();
        self.apu.reset();
    }

    pub fn save_state(&self) -> NesSaveState {
        NesSaveState {
            cpu_state: self.cpu.clone(),
            mmu_state: self.mmu.save_state(),
            ppu_state: self.ppu.save_state(),
        }
    }

    pub fn load_state(&mut self, s: NesSaveState) {
        self.cpu = s.cpu_state;
        self.mmu.load_state(s.mmu_state);
        self.ppu.load_state(s.ppu_state);
    }

    pub fn master_clock_tick(&mut self) -> Result<()> {
        if !self.mmu.has_cartridge() {
            return Err(Error::missing_cart());
        }

        if self.mmu.oam_transfer {
            if let Some(v) = self.oam_write.take() {
                self.ppu.oam[self.mmu.oam_offset as usize] = v;
                self.mmu.oam_offset += 1;
                if self.mmu.oam_offset >= 256 {
                    self.mmu.oam_transfer = false;
                    self.mmu.oam_page = 0;
                    self.mmu.oam_offset = 0;
                }
            } else {
                let addr = self.mmu.oam_page + self.mmu.oam_offset;
                self.oam_write = Some(self.mmu.read(addr));
            }
        } else {
            if self.ppu.nmi {
                self.cpu.nmi();
                self.ppu.nmi = false;
            } else if self.apu.get_irq() {
                self.cpu.irq();
            }
            if self.cycles_counter == 0 {
                self.cycles_counter += self.cpu.tick(&mut self.mmu)?;
            }
            self.cycles_counter -= 1;
        }

        self.ppu.tick(&mut self.mmu, &mut self.screen);
        self.ppu.tick(&mut self.mmu, &mut self.screen);
        self.ppu.tick(&mut self.mmu, &mut self.screen);
        self.apu.tick(&mut self.mmu.apu_registers);
        self.apu.tick(&mut self.mmu.apu_registers);
        self.apu.tick(&mut self.mmu.apu_registers);

        Ok(())
    }

    /// Runs the CPU until it recieves an NMI, signaling the end of a frame.
    pub fn run_frame(&mut self) -> Result<()> {
        loop {
            self.master_clock_tick()?;
            if self.screen.frame_completed.get() {
                self.screen.frame_completed.set(false);
                break;
            }
        }

        Ok(())
    }

    pub fn insert_cartridge(&mut self, cart: Cart) {
        self.mmu.insert_cartridge(cart)
    }

    pub fn pattern_table(&self) -> [u8; 0x2000] {
        use crate::ppu::PPUMemory;
        let mut r = [0; 0x2000];
        for i in 0..0x2000 {
            r[i as usize] = self.mmu.read_ppu(i as u16);
        }
        r
    }

    pub fn get_palette(&self, id: u16) -> [(u8, u8, u8); 4] {
        let index = (4 * id) as usize;
        let palette = &self.ppu.palette_ram[index..index + 4];
        let mut out = [(0, 0, 0); 4];
        for i in 0..4 {
            let c = palette[i as usize];
            out[i as usize] = self.ppu.convert_color_to_rgb(c).into_tuple();
        }
        out
    }

    pub fn palette_table(&self) -> [(u8, u8, u8); 32] {
        let mut out = [(0, 0, 0); 32];
        let palette_table = &self.ppu.palette_ram[..];
        for i in 0..32 {
            let c = palette_table[i];
            out[i] = self.ppu.convert_color_to_rgb(c).into_tuple();
        }
        out
    }

    /// Returns a clone of 0x2000-0x2fff of PPU memory
    pub fn get_nametables(&self) -> [u8; 0x1000] {
        use crate::ppu::PPUMemory;
        let mut buf = [0; 0x1000];
        buf.iter_mut().enumerate().for_each(|(i, b)| {
            *b = self.mmu.read_ppu(0x2000 + i as u16);
        });
        buf
    }

    #[inline]
    pub fn get_screen(&self) -> &V {
        &self.screen.screen
    }
    #[inline]
    pub fn get_screen_mut(&mut self) -> &mut V {
        &mut self.screen.screen
    }

    #[inline]
    pub fn get_audio_device(&self) -> &A {
        self.apu.audio_device()
    }
    #[inline]
    pub fn get_audio_device_mut(&mut self) -> &mut A {
        self.apu.audio_device_mut()
    }

    #[inline]
    pub fn get_controller(&self) -> &C {
        &self.mmu.controller
    }
    #[inline]
    pub fn get_controller_mut(&mut self) -> &mut C {
        &mut self.mmu.controller
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_ram() {
//         let mut nes = Nes::new(Cart::dummy(), None);
//         for i in 0..2048 {
//             nes.mmu.write(i, (i & 0xff) as u8);
//         }
//         for i in 0..2048 {
//             assert_eq!(nes.mmu.read(i), (i & 0xff) as u8);
//             // RAM Mirrors
//             assert_eq!(nes.mmu.read(i+2048), (i & 0xff) as u8);
//             assert_eq!(nes.mmu.read(i+4096), (i & 0xff) as u8);
//             assert_eq!(nes.mmu.read(i+6144), (i & 0xff) as u8);
//         }
//     }
// }
