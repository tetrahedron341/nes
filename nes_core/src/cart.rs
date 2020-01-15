use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ops::Range;

use crate::error::*;
use crate::mapper::{self, Mapper};

pub struct Cart {
    ines: Ines,
    mapper: Box<dyn Mapper>
}

impl Cart {
    /// Loads an iNES file, parses it, and assigns it a mapper.
    /// 
    /// May fail on: 
    /// * I/O Errors
    /// * iNES format errors
    /// * Unimplemented/Invalid mappers
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let ines = Ines::new(bytes)?;
        let mapper = mapper::from_ines_id(ines.mapper_id())?;
        Ok(Cart{
            ines,
            mapper
        })
    }

    /// Makes a cartridge directly from a byte vector representing an iNes ROM.
    pub fn from_bytes(rom: Vec<u8>) -> Result<Self> {
        let ines = Ines::new(rom)?;
        let mapper = mapper::from_ines_id(ines.mapper_id())?;
        Ok(Cart{
            ines,
            mapper
        })
    }

    /// Creates a dummy cartridge.
    /// This cartridge contains only zeroes.
    /// Any writes are no-ops.
    pub fn dummy() -> Self {
        Cart {
            ines: Ines::dummy(),
            mapper: Box::new(mapper::dummy::Dummy {})
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.mapper.read(&self.ines, addr)
    }
    pub fn write(&mut self, addr: u16, v: u8) {
        self.mapper.write(&self.ines, addr, v)
    }

    pub fn mapper_name(&self) -> &'static str {
        self.mapper.name()
    }
    pub fn header(&self) -> &Ines {
        &self.ines
    }
    pub fn mirroring(&self) -> Mirroring {
        self.ines.mirroring()
    }
}

pub struct Ines {
    // prg_size: u8,
    // chr_size: u8,
    flags6: u8,
    flags7: u8,

    pub prg_rom_range: Range<usize>,
    pub chr_rom_range: Range<usize>,

    data: Vec<u8>
}

impl Ines {
    fn new(data: Vec<u8>) -> Result<Self> {
        if data[0..4] != *b"NES\x1A" {
            return Err(Error::format_err("Invalid iNES header".to_string()));
        }
        let prg_size = data[4];
        let chr_size = data[5];
        let flags6 = data[6];
        let flags7 = data[7];
        
        let mut index: usize = 16;
        let prg_len: usize = prg_size as usize * 16384;
        let chr_len: usize = chr_size as usize * 8192;

        let prg_rom_range: Range<usize> = index .. index + prg_len;
        index += prg_len;
        let chr_rom_range: Range<usize> = index .. index + chr_len;

        Ok(Ines{
            // prg_size, chr_size,
            flags6, flags7,
            prg_rom_range,
            chr_rom_range,
            data
        })
    }

    fn dummy() -> Self {
        Ines {
            // prg_size: 0, chr_size: 0,
            flags6: 0, flags7: 0,
            prg_rom_range: 0..0,
            chr_rom_range: 0..0,
            data: Vec::new()
        }
    }

    fn mapper_id(&self) -> u16 {
        let low = self.flags6 >> 4;
        let hi = self.flags7 & 0xf0;
        (hi | low) as u16
    }

    pub fn prg_rom_slice(&self) -> &[u8] {
        &self.data[self.prg_rom_range.start .. self.prg_rom_range.end]
    }
    pub fn chr_rom_slice(&self) -> &[u8] {
        &self.data[self.chr_rom_range.start .. self.chr_rom_range.end]
    }
    pub fn mirroring(&self) -> Mirroring {
        if self.flags6 & 0x01 != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Mirroring {
    Horizontal,
    Vertical
}