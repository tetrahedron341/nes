use crate::cart::{Cart, Mirroring};
use crate::mos6502::MOS6502Memory;
use crate::ppu::PPUMemory;
use crate::ppu::PPURegisters;
use crate::controller::NESController;
use std::cell::Cell;
use bitflags::bitflags;

bitflags! {
    pub struct MMUConfig : u16 {
        const DEBUG = 1 << 0;
        const DEBUG_OUTPUT = 1 << 1 | Self::DEBUG.bits;
    }
}

pub struct MMU<C: NESController> {
    pub cart: Cart,
    pub ram: [u8; 2048],
    pub ppu_registers: PPURegisters,
    pub vram: [u8; 0x1000],
    pub registers: [u8; 0x20],
    pub controller: C,
    controller_shift: Cell<u8>,

    pub oam_transfer: bool,
    pub oam_page: u16,
    pub oam_offset: u16,

    config: MMUConfig,
    blargg_debug_state: Option<BlarggDebug>
}

impl<C: NESController> MMU<C> {
    pub fn new(cart: Cart, controller: C, config: Option<MMUConfig>) -> Self {
        let config = config.unwrap_or(MMUConfig::empty());
        MMU {
            cart,
            ram: [0; 2048],
            ppu_registers: PPURegisters::new(),
            registers: [0; 0x20],
            vram: [0; 0x1000],
            controller: controller,
            controller_shift: Cell::new(0),

            oam_transfer: false,
            oam_page: 0,
            oam_offset: 0,

            config,
            blargg_debug_state: if config.contains(MMUConfig::DEBUG) {
                Some(BlarggDebug {
                    code: 0xff,
                    magic_number: [0xff; 3],
                    test_output: [0x00; 0x200-4]
                })
            } else {None}
        }
    }

    /// Loads a single bytes from the specified address.
    /// Different address ranges may load from different devices.
    /// See https://wiki.nesdev.com/w/index.php/CPU_memory_map
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            (0x0000 ..= 0x1fff) => self.ram[(addr as usize) % 2048],
            (0x2000 ..= 0x3fff) => self.ppu_registers.read_by_index(((addr-0x2000)) % 8),
            0x4016 => {
                let c = self.controller_shift.get() & 0x01 != 0;
                let sh = self.controller_shift.get() >> 1;
                self.controller_shift.set(sh);
                if c {1} else {0}
            },
            (0x4000 ..= 0x401f) => self.registers[((addr-0x4000) as usize) % 0x20],
            (0x4020 ..= 0xffff) => self.cart.read(addr)
        }
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        match addr {
            (0x0000 ..= 0x1fff) => self.ram[(addr as usize) % 2048] = v,
            (0x2000 ..= 0x3fff) => self.ppu_registers.write_by_index(((addr-0x2000)) % 8, v),
            0x4014 => { // OAMDMA
                self.oam_transfer = true;
                self.oam_page = (v as u16) << 8;
                self.oam_offset = 0;
            },
            0x4016 => { // JOY1
                if v & 0x01 != 0 {
                    self.controller_shift.set(self.controller.poll_controller().bits())
                }
            },
            (0x4000 ..= 0x401f) => self.registers[((addr-0x4000) as usize) % 0x20] = v,
            (0x6000 ..= 0x61ff) if self.config.contains(MMUConfig::DEBUG) => {
                match addr {
                    0x6001..=0x6003 => { 
                        self.blargg_debug_state
                            .as_mut()
                            .map(|bds| {
                                bds.magic_number[(addr-0x6001) as usize] = v;
                            } );
                    },
                    0x6000 => { self.blargg_debug_state.as_mut().map(|bds| bds.code = v ); },
                    0x6004..=0x61ff => { self.blargg_debug_state.as_mut().map(|bds| bds.test_output[(addr-0x6004) as usize] = v ); }
                    _ => unreachable!()
                }
            },
            (0x4020 ..= 0xffff) => self.cart.write(addr, v)
        }
    }

    pub fn blargg_debug_text(&self) -> &[u8] {
        let state = self.blargg_debug_state.as_ref().expect("Blargg test output can only be used in DEBUG mode.");
        &state.test_output
    }
    pub fn blargg_debug_status(&self) -> u8 {
        let state = self.blargg_debug_state.as_ref().expect("Blargg test output can only be used in DEBUG mode.");
        state.code
    }
}

impl<C: NESController> MOS6502Memory for MMU<C> {
    fn read(&self, addr: u16) -> u8 {
        self.read(addr)
    }
    fn write(&mut self, addr: u16, v: u8) {
        self.write(addr, v)
    }
}

impl<C: NESController> PPUMemory for MMU<C> {
    fn read_ppu(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.cart.read(addr),
            0x2000..=0x23ff => self.vram[(addr-0x2000) as usize],
            0x2400..=0x27ff => match self.cart.mirroring() {
                Mirroring::Vertical => self.vram[(addr-0x2000) as usize],
                Mirroring::Horizontal => self.vram[(addr-0x2400) as usize] // Mirror to 0x2000-0x23ff
            },
            0x2800..=0x2bff => match self.cart.mirroring() {
                Mirroring::Vertical => self.vram[(addr-0x2800) as usize], // Mirror to 0x2000-0x23ff
                Mirroring::Horizontal => self.vram[(addr-0x2000) as usize]
            },
            0x2c00..=0x2fff => match  self.cart.mirroring() {
                Mirroring::Vertical => self.vram[(addr-0x2800) as usize], // Mirror to 0x2400-0x27ff
                Mirroring::Horizontal => self.vram[(addr-0x2400) as usize] // Mirror to 0x2800-0x2bff
            },
            _ => 0xff
        }
    }
    fn write_ppu(&mut self, addr: u16, v: u8) {
        match addr {
            0x0000..=0x1fff => (), // TODO: Add CHR RAM support
            0x2000..=0x23ff => self.vram[(addr-0x2000) as usize] = v,
            0x2400..=0x27ff => match self.cart.mirroring() {
                Mirroring::Vertical => self.vram[(addr-0x2000) as usize] = v,
                Mirroring::Horizontal => self.vram[(addr-0x2400) as usize] = v // Mirror to 0x2000-0x23ff
            },
            0x2800..=0x2bff => match self.cart.mirroring() {
                Mirroring::Vertical => self.vram[(addr-0x2800) as usize] = v, // Mirror to 0x2000-0x23ff
                Mirroring::Horizontal => self.vram[(addr-0x2000) as usize] = v
            },
            0x2c00..=0x2fff => match  self.cart.mirroring() {
                Mirroring::Vertical => self.vram[(addr-0x2800) as usize] = v, // Mirror to 0x2400-0x27ff
                Mirroring::Horizontal => self.vram[(addr-0x2400) as usize] = v // Mirror to 0x2800-0x2bff
            },
            _ => ()
        }
    }
    fn registers(&self) -> &PPURegisters {
        &self.ppu_registers
    }
    fn registers_mut(&mut self) -> &mut PPURegisters {
        &mut self.ppu_registers
    }
}

struct BlarggDebug {
    code: u8,
    magic_number: [u8; 3],
    test_output: [u8; 0x200 - 4]
}