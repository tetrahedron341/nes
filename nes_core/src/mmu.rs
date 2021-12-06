use crate::apu::APURegisters;
use crate::cart::{Cart, CartState, Mirroring};
use crate::controller::NESController;
use crate::mos6502::MOS6502Memory;
use crate::ppu::PPUMemory;
use crate::ppu::PPURegisters;
use bitflags::bitflags;
use std::cell::Cell;

bitflags! {
    pub struct MMUConfig : u16 {
        const DEBUG = 1 << 0;
        const DEBUG_OUTPUT = 1 << 1;
    }
}

pub struct MMUSaveState {
    ram: [u8; 2048],
    vram: [[u8; 0x400]; 4],
    cart_state: Option<CartState>,
}

impl Clone for MMUSaveState {
    fn clone(&self) -> Self {
        MMUSaveState {
            ram: {
                let mut ram = [0; 2048];
                ram.as_mut().copy_from_slice(&self.ram);
                ram
            },
            vram: {
                let mut vram = [[0; 0x400]; 4];
                vram.as_mut().copy_from_slice(&self.vram);
                vram
            },
            cart_state: self.cart_state.as_ref().map(|c| (**c).clone()),
        }
    }
}

pub struct MMU<C: NESController> {
    pub cart: Option<Cart>,
    pub ram: [u8; 2048],
    pub ppu_registers: PPURegisters,
    pub apu_registers: APURegisters,
    pub vram: [[u8; 0x400]; 4],
    pub(crate) controller: C,
    controller_shift: Cell<u8>,

    pub oam_transfer: bool,
    pub oam_page: u16,
    pub oam_offset: u16,

    config: MMUConfig,
    blargg_debug_state: Option<BlarggDebug>,
}

impl<C: NESController> MMU<C> {
    pub fn new<T: Into<Option<Cart>>>(cart: T, controller: C, config: Option<MMUConfig>) -> Self {
        let config = config.unwrap_or_else(MMUConfig::empty);
        MMU {
            cart: cart.into(),
            ram: [0; 2048],
            ppu_registers: PPURegisters::default(),
            apu_registers: APURegisters::default(),
            vram: [[0; 0x400]; 4],
            controller,
            controller_shift: Cell::new(0),

            oam_transfer: false,
            oam_page: 0,
            oam_offset: 0,

            config,
            blargg_debug_state: if config.contains(MMUConfig::DEBUG) {
                Some(BlarggDebug {
                    code: 0xff,
                    magic_number: [0xff; 3],
                    test_output: [0x00; 0x200 - 4],
                })
            } else {
                None
            },
        }
    }

    pub fn reset(&mut self) {
        if let Some(c) = self.cart.as_mut() {
            c.reset()
        }
        // self.ram = [0; 2048];
        self.vram = [[0; 0x400]; 4];
        self.controller_shift.set(0);
        self.ppu_registers = PPURegisters::default();
    }

    pub fn save_state(&self) -> MMUSaveState {
        MMUSaveState {
            ram: {
                let mut ram = [0; 2048];
                ram.as_mut().copy_from_slice(&self.ram);
                ram
            },
            vram: {
                let mut vram = [[0; 0x400]; 4];
                vram.as_mut().copy_from_slice(&self.vram);
                vram
            },
            cart_state: self.cart.as_ref().map(|c| c.save_state()),
        }
    }

    pub fn load_state(&mut self, s: MMUSaveState) {
        self.ram.as_mut().copy_from_slice(&s.ram);
        self.vram.as_mut().copy_from_slice(&s.vram);
        s.cart_state
            .map(|s| self.cart.as_mut().map(|c| c.load_state(s)));
    }

    /// Loads a single bytes from the specified address.
    /// Different address ranges may load from different devices.
    /// See https://wiki.nesdev.com/w/index.php/CPU_memory_map
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            (0x0000..=0x1fff) => self.ram[(addr as usize) % 2048],
            (0x2000..=0x3fff) => self.ppu_registers.read_by_index((addr - 0x2000) % 8),
            0x4016 => {
                let c = self.controller_shift.get() & 0x01 != 0;
                let sh = self.controller_shift.get() >> 1;
                self.controller_shift.set(sh);
                if c {
                    1
                } else {
                    0
                }
            }
            0x4017 => 0,
            0x4000..=0x4015 | 0x4018..=0x401f => self.apu_registers.read(addr),
            (0x4020..=0xffff) => self
                .cart
                .as_ref()
                .expect("Cartridge is not inserted!")
                .read(addr),
        }
    }

    pub fn write(&mut self, addr: u16, v: u8) {
        match addr {
            (0x0000..=0x1fff) => self.ram[(addr as usize) % 2048] = v,
            (0x2000..=0x3fff) => self.ppu_registers.write_by_index((addr - 0x2000) % 8, v),
            0x4014 => {
                // OAMDMA
                self.oam_transfer = true;
                self.oam_page = (v as u16) << 8;
                self.oam_offset = 0;
            }
            0x4016 => {
                // JOY1
                if v & 0x01 != 0 {
                    self.controller_shift
                        .set(self.controller.poll_controller().bits())
                }
            }
            0x4000..=0x4013 | 0x4015 | 0x4017..=0x401f => {
                self.apu_registers.write(addr, v);
            }
            (0x6000..=0x61ff) if self.config.contains(MMUConfig::DEBUG) => match addr {
                0x6001..=0x6003 => {
                    if let Some(bds) = self.blargg_debug_state.as_mut() {
                        bds.magic_number[(addr - 0x6001) as usize] = v;
                    }
                }
                0x6000 => {
                    if let Some(bds) = self.blargg_debug_state.as_mut() {
                        bds.code = v;
                    }
                }
                0x6004..=0x61ff => {
                    if let Some(bds) = self.blargg_debug_state.as_mut() {
                        bds.test_output[(addr - 0x6004) as usize] = v;
                    }
                }
                _ => unreachable!(),
            },
            (0x4020..=0xffff) => self
                .cart
                .as_mut()
                .expect("Cartridge is not inserted!")
                .write(addr, v),
        }
    }

    pub fn blargg_debug_text(&self) -> &[u8] {
        let state = self
            .blargg_debug_state
            .as_ref()
            .expect("Blargg test output can only be used in DEBUG mode.");
        &state.test_output
    }
    pub fn blargg_debug_status(&self) -> u8 {
        let state = self
            .blargg_debug_state
            .as_ref()
            .expect("Blargg test output can only be used in DEBUG mode.");
        state.code
    }

    pub fn has_cartridge(&self) -> bool {
        self.cart.is_some()
    }
    pub fn insert_cartridge(&mut self, cart: Cart) {
        self.cart = Some(cart);
    }

    fn get_vram_bank_from_nametable_addr(mirroring: Mirroring, addr: u16) -> Option<usize> {
        // VRAM is divided into 4 1KB banks A,B,C,D
        // Banks C and D are only used in 4 Screen mode
        // Mirroring is implemented by translating each nametable to a bank
        match addr {
            0x2000..=0x23ff => match mirroring {
                Mirroring::OneScreenUpperBank => Some(1),
                _ => Some(0),
            },
            0x2400..=0x27ff => match mirroring {
                Mirroring::Vertical => Some(1),
                Mirroring::Horizontal => Some(0),
                Mirroring::OneScreenLowerBank => Some(0),
                Mirroring::OneScreenUpperBank => Some(1),
                Mirroring::FourScreen => Some(1),
            },
            0x2800..=0x2bff => match mirroring {
                Mirroring::Vertical => Some(0),
                Mirroring::Horizontal => Some(1),
                Mirroring::OneScreenLowerBank => Some(0),
                Mirroring::OneScreenUpperBank => Some(1),
                Mirroring::FourScreen => Some(2),
            },
            0x2c00..=0x2fff => match mirroring {
                Mirroring::Vertical => Some(1),
                Mirroring::Horizontal => Some(1),
                Mirroring::OneScreenLowerBank => Some(0),
                Mirroring::OneScreenUpperBank => Some(1),
                Mirroring::FourScreen => Some(3),
            },
            _ => None,
        }
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
    #[inline]
    fn read_ppu(&self, addr: u16) -> u8 {
        let cart = self.cart.as_ref().expect("Cartridge is not inserted!");
        if (0x0000..=0x1fff).contains(&addr) {
            cart.read(addr)
        } else if (0x2000..=0x2fff).contains(&addr) {
            let trunc_addr = (addr % 0x400) as usize;
            if let Some(bank) = Self::get_vram_bank_from_nametable_addr(cart.mirroring(), addr) {
                self.vram[bank][trunc_addr]
            } else {
                unreachable!()
            }
        } else {
            0xff
        }
    }
    fn write_ppu(&mut self, addr: u16, v: u8) {
        let cart = self.cart.as_mut().expect("Cartridge is not inserted!");
        if (0x0000..=0x1fff).contains(&addr) {
            cart.write(addr, v)
        } else if (0x2000..=0x2fff).contains(&addr) {
            let trunc_addr = (addr % 0x400) as usize;
            if let Some(bank) = Self::get_vram_bank_from_nametable_addr(cart.mirroring(), addr) {
                self.vram[bank][trunc_addr] = v;
            } else {
                unreachable!();
            }
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
    test_output: [u8; 0x200 - 4],
}
