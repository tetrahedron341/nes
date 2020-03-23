use super::length_counter::LengthCounter;

const SEQUENCE: [u8; 32] = [
    0xf, 0xe, 0xd, 0xc, 0xb, 0xa, 0x9, 0x8, 0x7, 0x6, 0x5, 0x4, 0x3, 0x2, 0x1, 0x0,
    0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf
];

pub struct Triangle {
    pub enabled: bool,

    len_ctr: LengthCounter,

    lin_ctr_reload: bool,
    lin_ctr_reload_val: u8,
    lin_ctr_control: bool,
    lin_ctr_val: u8,

    sequencer: usize,

    raw_timer_period: u16,
    timer_div: u16,
}

impl Triangle {
    pub fn new() -> Self {
        Triangle {
            enabled: false,

            len_ctr: LengthCounter::new(),

            lin_ctr_reload: false,
            lin_ctr_reload_val: 0,
            lin_ctr_control: false,
            lin_ctr_val: 0,

            sequencer: 0,

            raw_timer_period: 0,
            timer_div: 0,
        }
    }

    pub fn write_to_registers(&mut self, i: usize, v: u8) {
        match i {
            0 => {
                self.len_ctr.halt = v & 0x80 != 0;
                self.lin_ctr_control = v & 0x80 != 0;
                self.lin_ctr_reload_val = v & 0x7f;
            },
            1 => {
                self.raw_timer_period &= 0b111_0000_0000;
                self.raw_timer_period |= v as u16;
            },
            2 => {
                if self.enabled {
                    self.len_ctr.load_counter(v >> 3);
                }
                self.raw_timer_period &= 0b000_1111_1111;
                self.raw_timer_period |= ((v & 0b111) as u16) << 8;
                self.lin_ctr_reload = true;
            }
            _ => panic!("Invalid triangle register index {}", i)
        }
    }

    pub fn tick_timer(&mut self) {
        if self.raw_timer_period < 2 {
            // At the moment, the audio is completely unfiltered. To prevent wierd popping noises, 
            // halt the channel whenever the period is below 2
            return;
        }
        match self.timer_div.checked_sub(1) {
            Some(n) => self.timer_div = n,
            None => {
                if !self.len_ctr.is_zero() && self.lin_ctr_val != 0 {
                    self.sequencer += 1;
                    self.sequencer %= 32;
                }
                self.timer_div = self.raw_timer_period;
            }
        }
    }

    pub fn tick_lin_ctr(&mut self) {
        if self.lin_ctr_reload {
            self.lin_ctr_val = self.lin_ctr_reload_val;
        } else {
            if self.lin_ctr_val != 0 { self.lin_ctr_val -= 1; }
        }
        if !self.lin_ctr_control { self.lin_ctr_reload = false; }
    }

    pub fn tick_length(&mut self) {
        self.len_ctr.tick();
    }

    pub fn digital_sample(&self) -> u8 {
        SEQUENCE[self.sequencer]
    }

    pub fn disable(&mut self) {
        self.len_ctr.set_zero();
    }

    #[inline]
    pub fn length_counter_gt_zero(&self) -> bool {
        !self.len_ctr.is_zero()
    }
}