pub trait MOS6502Memory {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, v: u8);
    fn read_double(&self, addr: u16) -> u16 {
        let ah = addr & 0xff00;
        let al = (addr & 0xff) as u8;
        let lo = self.read(addr);
        let hi = self.read(ah + al.wrapping_add(1) as u16);
        ((hi as u16) << 8) | (lo as u16)
    }
    fn write_double(&mut self, addr: u16, v: u16) {
        self.write(addr, (v & 0x00ff) as u8);
        self.write(addr, ((v & 0xff00) >> 8) as u8);
    }
}
