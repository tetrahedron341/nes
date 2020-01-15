use super::Mapper;
use super::Ines;

/// An empty Cartridge mapper.
/// Reads only zeroes, `/dev/null`s writes
pub struct Dummy { }

impl Mapper for Dummy {
    fn name(&self) -> &'static str {
        "DUMMY"
    }
    fn read(&self, _: &Ines, _: u16) -> u8 {
        0
    }
    fn write(&mut self, _: &Ines, _: u16, _: u8) { }
}