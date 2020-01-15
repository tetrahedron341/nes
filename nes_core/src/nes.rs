use crate::mos6502::MOS6502;
use crate::ppu::{PPU, VideoInterface};
use crate::mmu::MMU;
use crate::cart::Cart;
use crate::error::*;
use crate::controller::NESController;
use bitflags::bitflags;

bitflags! {
    pub struct NESConfig : u16 {
        const DEBUG = 1 << 0;
        const DEBUG_OUTPUT = 1 << 1 | Self::DEBUG.bits;
    }
}

impl Into<crate::mos6502::CPUConfig> for &NESConfig {
    fn into(self) -> crate::mos6502::CPUConfig {
        use crate::mos6502::CPUConfig;
        let mut out = CPUConfig::empty();
        out.set(CPUConfig::DEBUG, self.contains(NESConfig::DEBUG));
        out.set(CPUConfig::DEBUG_OUTPUT, self.contains(NESConfig::DEBUG_OUTPUT));
        out
    }
}
impl Into<crate::mmu::MMUConfig> for &NESConfig {
    fn into(self) -> crate::mmu::MMUConfig {
        use crate::mmu::MMUConfig;
        let mut out = MMUConfig::empty();
        out.set(MMUConfig::DEBUG, self.contains(NESConfig::DEBUG));
        out.set(MMUConfig::DEBUG_OUTPUT, self.contains(NESConfig::DEBUG_OUTPUT));
        out
    }
}

/// Represents the NES system.
pub struct Nes<V: VideoInterface, C: NESController> {
    pub cpu: MOS6502,
    pub ppu: PPU,
    pub mmu: MMU<C>,
    pub screen: V,
    cycles_counter: u32,
    oam_write: Option<u8>,
    _config: NESConfig
}

impl<'a, V: VideoInterface, C: NESController> Nes<V,C> {
    pub fn new(cart: Cart, screen: V, controller: C, config: Option<NESConfig>) -> Self {
        let mut cpu = MOS6502::new(config.as_ref().map(|c| c.into()));
        cpu.reset();
        let ppu = PPU::new();
        let mmu = MMU::new(cart, controller, config.as_ref().map(|c| c.into()));
        Nes {
            cpu,
            ppu,
            mmu,
            screen,
            cycles_counter: 0,
            oam_write: None,
            _config: config.unwrap_or(NESConfig::empty())
        }
    }

    pub fn master_clock_tick(&mut self) -> Result<()> {
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
            }
            if self.cycles_counter == 0 {
                self.cycles_counter += self.cpu.tick(&mut self.mmu)?;
            }
            self.cycles_counter -= 1;
        }

        self.ppu.tick(&mut self.mmu, &mut self.screen);
        self.ppu.tick(&mut self.mmu, &mut self.screen);
        self.ppu.tick(&mut self.mmu, &mut self.screen);
        
        Ok(())
    }

    /// Runs the CPU until it recieves an NMI, signaling the end of a frame.
    pub fn run_frame(&mut self) -> Result<()> {
        let mut prev_nmi = true;
        loop {
            self.master_clock_tick()?;
            let nmi = self.ppu.nmi;
            if !prev_nmi && nmi { break }
            prev_nmi = nmi;
        }

        Ok(())
    }

    pub fn pattern_table(&self) -> [u8; 0x2000] {
        use crate::ppu::PPUMemory;
        let mut r = [0; 0x2000];
        for i in 0..0x2000 {
            r[i as usize] = self.mmu.read_ppu(i as u16);
        }
        r
    }

    pub fn get_palette(&self, id: u16) -> [(u8,u8,u8); 4] {
        let index = (4*id) as usize;
        let palette = &self.ppu.palette_ram[index..index+4];
        let mut out = [(0,0,0); 4];
        for i in 0..4 {
            let c = palette[i as usize];
            out[i as usize] = self.ppu.convert_color_to_rgb(c).into_tuple();
        }
        out
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