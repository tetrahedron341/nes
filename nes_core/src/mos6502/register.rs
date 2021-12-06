use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Register<T>(pub T);

impl<T: Copy> Register<T> {
    pub fn get(&self) -> T {
        self.0
    }

    pub fn set(&mut self, v: T) {
        self.0 = v
    }
}

impl Register<u8> {
    pub fn inc(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
    pub fn dec(&mut self) {
        self.0 = self.0.wrapping_sub(1);
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
    pub fn is_neg(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }
}

impl Register<u16> {
    pub fn inc(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
    pub fn dec(&mut self) {
        self.0 = self.0.wrapping_sub(1);
    }

    pub fn hi(&self) -> u8 {
        ((self.0 & 0xff00) >> 8) as u8
    }
    pub fn lo(&self) -> u8 {
        (self.0 & 0x00ff) as u8
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
    pub fn is_neg(&self) -> bool {
        self.0 & 0b1000_0000_0000_0000 != 0
    }
}

impl std::ops::Add<u8> for Register<u8> {
    type Output = Register<u8>;
    fn add(self, other: u8) -> Self::Output {
        Register(self.0.wrapping_add(other))
    }
}
impl std::ops::Add<u16> for Register<u16> {
    type Output = Register<u16>;
    fn add(self, other: u16) -> Self::Output {
        Register(self.0.wrapping_add(other))
    }
}

impl std::ops::Sub<u8> for Register<u8> {
    type Output = Register<u8>;
    fn sub(self, other: u8) -> Self::Output {
        Register((self.0 as i8).wrapping_sub(other as i8) as u8)
    }
}
impl std::ops::Sub<u16> for Register<u16> {
    type Output = Register<u16>;
    fn sub(self, other: u16) -> Self::Output {
        Register((self.0 as i16).wrapping_sub(other as i16) as u16)
    }
}

bitflags! {
    pub struct StatusRegister: u8 {
        const C = 1 << 0;
        const Z = 1 << 1;
        const I = 1 << 2;
        const D = 1 << 3;
        const B = 1 << 4 | 1 << 5;
        const V = 1 << 6;
        const N = 1 << 7;
    }
}
