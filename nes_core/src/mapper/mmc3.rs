pub struct MMC3 {}

impl super::Mapper for MMC3 {
    fn name(&self) -> &'static str {
        "MMC3"
    }

    fn read(&self, ines: &crate::cart::Ines, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, ines: &crate::cart::Ines, addr: u16, v: u8) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn clone(&self) -> Box<dyn super::Mapper + Send + Sync> {
        todo!()
    }
}
