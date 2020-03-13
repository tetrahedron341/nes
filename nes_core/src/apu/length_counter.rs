const COUNTER_LOOKUP: [u8; 32] = [
    0x0A, 0xFE,
    0x14, 0x02,
    0x28, 0x04,
    0x50, 0x06,
    0xA0, 0x08,
    0x3C, 0x0A,
    0x0E, 0x0C,
    0x1A, 0x0E,
    0x0C, 0x10,
    0x18, 0x12,
    0x30, 0x14,
    0x60, 0x16,
    0xC0, 0x18,
    0x48, 0x1A,
    0x10, 0x1C,
    0x20, 0x1E,
];

pub struct LengthCounter {
    pub halt: bool,
    counter: u8,
}

impl LengthCounter {
    pub fn new() -> Self {
        LengthCounter {
            halt: false,
            counter: 0,
        }
    }

    pub fn tick(&mut self) {
        if !self.halt && self.counter > 0 {
            self.counter -= 1;
        }
    }

    pub fn is_zero(&self) -> bool {
        self.counter == 0
    }

    /// Loads the length counter with a value determined by a table
    /// https://wiki.nesdev.com/w/index.php/APU_Length_Counter
    pub fn load_counter(&mut self, i: u8) {
        self.counter = COUNTER_LOOKUP[i as usize];
    }

    pub fn set_zero(&mut self) {
        self.counter = 0;
    }
}