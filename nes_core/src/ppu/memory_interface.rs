use super::PPURegisters;

pub trait PPUMemory {
    fn read_ppu(&self, addr: u16) -> u8;
    fn write_ppu(&mut self, addr: u16, v: u8);
    fn registers(&self) -> &PPURegisters;
    fn registers_mut(&mut self) -> &mut PPURegisters;
}