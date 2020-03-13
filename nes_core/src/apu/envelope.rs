/// Represents the envelope component of an APU channel
pub struct Envelope {
    pub loop_env: bool,
    pub disable: bool,
    pub raw_period: u8,
    divider_time: u8,
    counter: u8,
}

impl Envelope {
    pub fn new() -> Self {
        Envelope{
            loop_env: false,
            disable: false,
            raw_period: 1,
            divider_time: 1,
            counter: 0,
        }
    }

    pub fn tick(&mut self, register_four_write: bool) {
        if register_four_write {
            self.divider_time = self.raw_period;
            self.counter = 15;
        } else {
            match self.divider_time.checked_sub(1) {
                Some(n) => self.divider_time = n,
                None => {
                    if self.loop_env && self.counter == 0 {
                        self.counter = 15;
                    } else if self.counter > 0 {
                        self.counter -= 1;
                    }
                    self.divider_time = self.raw_period;
                }
            }
        }
    }

    pub fn get_volume(&self) -> u8 {
        if self.disable {
            self.raw_period
        } else {
            self.counter
        }
    }
}