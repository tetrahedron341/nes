use super::length_counter::LengthCounter;
use super::envelope::Envelope;

pub struct Noise {
    pub enabled: bool,
    len_ctr: LengthCounter,
    envelope: Envelope,
    restart_envelope: bool,

    shift_register: u16,
    mode: bool,

    raw_timer_period: u16,
    timer_div: u16,
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            enabled: false,
            len_ctr: LengthCounter::new(),
            envelope: Envelope::new(),
            restart_envelope: false,

            shift_register: 1,
            mode: false,

            raw_timer_period: 0,
            timer_div: 0
        }
    }

    pub fn write_to_registers(&mut self, i: usize, v: u8) {
        match i {
            0 => {
                self.envelope.loop_env = v & 0x20 != 0;
                self.len_ctr.halt = v & 0x20 != 0;
                self.envelope.disable = v & 0x10 != 0;
                self.envelope.raw_period = v & 0x0f;
            },
            1 => {
                // https://wiki.nesdev.com/w/index.php/APU_Noise
                const PERIOD_LOOKUP: [u16; 16] = [
                    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068
                ];

                self.mode = v & 0x80 != 0;
                self.raw_timer_period = PERIOD_LOOKUP[v as usize & 0x0f];
            },
            2 => {
                self.len_ctr.load_counter((v & 0xf8) >> 3);
                self.restart_envelope = true;
            },
            _ => panic!("Invalid noise register index {}", i)
        }
    }

    pub fn tick_envelope(&mut self) {
        self.envelope.tick(self.restart_envelope);
        self.restart_envelope = false;
    }

    pub fn tick_length(&mut self) {
        self.len_ctr.tick();
    }

    pub fn tick_timer(&mut self) {
        match self.timer_div.checked_sub(1) {
            Some(n) => self.timer_div = n,
            None => {
                let feedback = if !self.mode {
                    // Mode 0
                    let bit1 = (self.shift_register & 0x0002) >> 1;
                    let bit0 = self.shift_register & 0x0001;
                    bit0 ^ bit1
                } else {
                    // Mode 1
                    let bit6 = (self.shift_register & 0x0040) >> 1;
                    let bit0 = self.shift_register & 0x0001;
                    bit0 ^ bit6
                };
                self.shift_register >>= 1;
                self.shift_register |= feedback << 14;
                self.timer_div = self.raw_timer_period;
            }
        }
    }

    pub fn digital_sample(&self) -> u8 {
        if self.shift_register & 0x0001 != 0 && !self.len_ctr.is_zero() {
            self.envelope.get_volume()
        } else {
            0
        }
    }

    pub fn disable(&mut self) {
        self.len_ctr.set_zero();
    }

    #[inline]
    pub fn length_counter_gt_zero(&self) -> bool {
        !self.len_ctr.is_zero()
    }
}