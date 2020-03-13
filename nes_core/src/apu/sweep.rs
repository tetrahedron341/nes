pub struct Sweep {
    pub enable: bool,
    pub raw_period: u8,
    pub negate: bool,
    pub shift: u8,

    negate_mode: u8,
    divider: u8,
}

impl Sweep {
    pub fn new(negate_mode: u8) -> Self {
        Sweep {
            enable: false,
            raw_period: 0,
            negate: false,
            shift: 0,

            negate_mode,
            divider: 0,
        }
    }

    fn calculate_new_period(&self, raw_timer_period: u16) -> u16 {
        let mut change_amount = (raw_timer_period >> self.shift) as i16;
        if self.negate {
            match self.negate_mode {
                1 => change_amount = -change_amount,
                0 => change_amount = -change_amount - 1,
                _ => unreachable!()
            }
        }
        (raw_timer_period as i16 + change_amount) as u16
    }

    pub fn is_muted(&self, raw_timer_period: u16) -> bool {
        self.calculate_new_period(raw_timer_period) > 0x7ff || raw_timer_period < 8
    }

    pub fn tick(&mut self, register_two_write: bool, timer_period: &mut u16) {
        match self.divider.checked_sub(1) {
            Some(n) => self.divider = n,
            None => {
                if !self.is_muted(*timer_period) && self.enable && self.shift > 0 {
                    *timer_period = self.calculate_new_period(*timer_period);
                }
                self.divider = self.raw_period;
            }
        }
        if register_two_write {
            self.divider = self.raw_period;
        }
    }
}