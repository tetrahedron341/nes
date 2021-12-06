mod axrom;
pub mod dummy;
mod mmc1;
mod nrom;
mod uxrom;

use crate::cart::{Ines, Mirroring};
use crate::error::*;

/// Represents a memory banking method for a cartridge.
pub trait Mapper {
    /// Returns the name of the mapper
    fn name(&self) -> &'static str;
    /// Performs a read from the cartridge
    fn read(&self, ines: &Ines, addr: u16) -> u8;
    /// Performs a write to the cartridge
    fn write(&mut self, ines: &Ines, addr: u16, v: u8);
    /// Resets the mapper
    fn reset(&mut self);
    /// Clones the mapper's state
    fn clone(&self) -> Box<dyn Mapper + Send + Sync>;
    /// Gets the nametable mirroring mode.
    ///
    /// The default implementation always selects the mirroring mode defined
    /// in the iNES header.
    fn mirroring(&self) -> Option<Mirroring> {
        None
    }
}

pub fn from_ines_id(id: u16) -> Result<Box<dyn Mapper + Send + Sync>> {
    match id {
        0 => Ok(Box::new(nrom::NROM::new())),
        1 => Ok(Box::new(mmc1::MMC1::new())),
        2 => Ok(Box::new(uxrom::UxROM::new())),

        7 => Ok(Box::new(axrom::AxROM::new())),
        _ => Err(Error::format_err(format!("Invalid mapper ID: {}", id))),
    }
}
