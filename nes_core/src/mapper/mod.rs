mod nrom;
pub mod dummy;
mod mmc1;

use crate::error::*;
use crate::cart::Ines;

pub trait Mapper {
    fn name(&self) -> &'static str;
    fn read(&self, ines: &Ines, addr: u16) -> u8;
    fn write(&mut self, ines: &Ines, addr: u16, v: u8);
}

pub fn from_ines_id(id: u16) -> Result<Box<dyn Mapper>> {
    match id {
        0 => Ok(Box::new(nrom::NROM::new())),
        1 => {println!("WARNING: MMC1 support is not finished!"); Ok(Box::new(mmc1::MMC1::new()))},
        _ => Err(Error::format_err(format!("Invalid mapper ID: {}", id)))
    }
}