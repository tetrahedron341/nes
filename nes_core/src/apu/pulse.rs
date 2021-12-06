use super::envelope::Envelope;
use super::length_counter::LengthCounter;
use super::sweep::Sweep;

const SEQUENCER_STEPS: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

pub struct Pulse {
    pub enabled: bool,

    duty_cycle: u8,
    envelope: Envelope,
    sweep: Sweep,
    length_counter: LengthCounter,
    // NOTE: The sequencer is ticked every other timer cycle, so it goes from 0-15 instead of 0-7
    sequencer: usize,

    register_four_write: bool,
    register_two_write: bool,

    raw_timer_period: u16,
    timer_div: u16,
}

impl Pulse {
    pub fn write_to_registers(&mut self, i: usize, v: u8) {
        match i {
            0 => {
                self.duty_cycle = v >> 6;
                self.length_counter.halt = v & 0x20 != 0;
                self.envelope.loop_env = v & 0x20 != 0;
                self.envelope.disable = v & 0x10 != 0;
                self.envelope.raw_period = v & 0x0f;
            }
            1 => {
                self.sweep.enable = v & 0x80 != 0;
                self.sweep.raw_period = (v & 0x70) >> 4;
                self.sweep.negate = v & 0x08 != 0;
                self.sweep.shift = v & 0x07;

                self.register_two_write = true;
            }
            2 => {
                self.raw_timer_period &= 0b111_0000_0000;
                self.raw_timer_period |= v as u16;
            }
            3 => {
                self.raw_timer_period &= 0b000_1111_1111;
                self.raw_timer_period |= ((v & 0b111) as u16) << 8;
                if self.enabled {
                    self.length_counter.load_counter(v >> 3);
                }

                self.register_four_write = true;
            }
            _ => unreachable!(),
        }
    }

    pub fn new(channel_number: u8) -> Self {
        Pulse {
            enabled: false,

            duty_cycle: 0,
            envelope: Envelope::new(),
            sweep: Sweep::new(channel_number),
            length_counter: LengthCounter::new(),
            sequencer: 0,

            register_four_write: false,
            register_two_write: false,

            raw_timer_period: 0,
            timer_div: 0,
        }
    }

    pub fn digital_sample(&self) -> u8 {
        let envelope = self.envelope.get_volume();
        let sweep_mute = self.sweep.is_muted(self.raw_timer_period);
        let sequencer_val = SEQUENCER_STEPS[self.duty_cycle as usize][self.sequencer / 2];
        let length_mute: bool = self.length_counter.is_zero();

        if !sweep_mute && sequencer_val == 1 && !length_mute {
            envelope
        } else {
            0
        }
    }

    pub fn tick_envelope(&mut self) {
        self.envelope.tick(self.register_four_write);
        self.register_four_write = false;
    }

    pub fn tick_length_and_sweep(&mut self) {
        self.sweep
            .tick(self.register_two_write, &mut self.raw_timer_period);
        self.length_counter.tick();
        self.register_two_write = false;
    }

    pub fn tick_timer(&mut self) {
        match self.timer_div.checked_sub(1) {
            Some(n) => self.timer_div = n,
            None => {
                self.sequencer += 1;
                self.sequencer %= 16;
                self.timer_div = self.raw_timer_period;
            }
        }
    }

    #[inline]
    pub fn length_counter_gt_zero(&self) -> bool {
        !self.length_counter.is_zero()
    }

    pub fn disable(&mut self) {
        self.length_counter.set_zero();
    }
}
