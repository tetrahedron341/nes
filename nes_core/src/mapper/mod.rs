mod nrom;
pub mod dummy;
mod mmc1;

use crate::error::*;
use crate::cart::{Ines, Mirroring};

/// Represents a memory banking method for a cartridge.
pub trait Mapper {
    fn name(&self) -> &'static str;
    fn read(&self, ines: &Ines, addr: u16) -> u8;
    fn write(&mut self, ines: &Ines, addr: u16, v: u8);
    fn reset(&mut self);
    fn clone(&self) -> Box<dyn Mapper + Send + Sync>;
    fn mirroring(&self) -> Option<Mirroring> {
        None
    }
}

pub fn from_ines_id(id: u16) -> Result<Box<dyn Mapper + Send + Sync>> {
    match id {
        0 => Ok(Box::new(nrom::NROM::new())),
        1 => Ok(Box::new(mmc1::MMC1::new())),
        _ => Err(Error::format_err(format!("Invalid mapper ID: {}", id)))
    }
}