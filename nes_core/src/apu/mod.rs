mod apu_registers;
mod audio_output;
mod envelope;
mod length_counter;
mod sweep;
mod pulse;

pub use audio_output::*;
pub use apu_registers::APURegisters;

use crate::error::Result;
use std::collections::VecDeque;
use pulse::Pulse;

const SAMPLE_OUT: usize = 4096;

const QUARTER_FRAME_DIVIDER_PERIOD: u32 = 89490;

pub struct APU<T: AudioOutput> {
    pub volume: f32,

    pulse_1: Pulse,
    pulse_2: Pulse,

    quarter_frame_divider: u32,
    frame_seq_mode: bool,
    frame_seq: u8,

    timer_div: u8,

    sample_buffer: VecDeque<f32>,
    sample_out: T,
    sample_divider: f64,

    frame_irq: bool,
    irq_inhibit: bool,
}

impl<T: AudioOutput> APU<T> {
    pub fn new(output: T) -> Self {
        APU {
            volume: 1.0,

            pulse_1: Pulse::new(0),
            pulse_2: Pulse::new(1),

            quarter_frame_divider: QUARTER_FRAME_DIVIDER_PERIOD,
            frame_seq_mode: false,
            frame_seq: 0,

            timer_div: 0,

            sample_buffer: VecDeque::with_capacity(2*SAMPLE_OUT),
            sample_out: output,
            sample_divider: 0.0,

            frame_irq: false,
            irq_inhibit: false,
        }
    }

    // pub fn read(&self, addr: u16) -> u8 {
    //     match addr {
    //         0x4000..=0x4017 => 0x00,
    //         _ => unreachable!()
    //     }
    // }

    // pub fn write(&mut self, addr: u16, v: u8) {
    //     match addr {
    //         0x4000 => self.pulse_1.write_to_registers(0, v),
    //         0x4001 => self.pulse_1.write_to_registers(1, v),
    //         0x4002 => self.pulse_1.write_to_registers(2, v),
    //         0x4003 => self.pulse_1.write_to_registers(3, v),

    //         0x4004 => self.pulse_2.write_to_registers(0, v),
    //         0x4005 => self.pulse_2.write_to_registers(1, v),
    //         0x4006 => self.pulse_2.write_to_registers(2, v),
    //         0x4007 => self.pulse_2.write_to_registers(3, v),

    //         0x4008..=0x4017 => (),
    //         _ => unreachable!()
    //     }
    // }

    // This function is called at 1/4 the master clock cycle
    pub fn tick(&mut self, registers: &mut APURegisters) {
        self.update_from_registers(registers);

        // The quarter frame divider is run at the full 21.477272 MHz master clock cycle
        match self.quarter_frame_divider.checked_sub(4) {
            Some(n) => self.quarter_frame_divider = n,
            None => {
                match self.frame_seq_mode {
                    // Mode 0 - 4-step sequence
                    false => {
                        match self.frame_seq {
                            0 | 2 => {
                                self.pulse_1.tick_envelope();
                                self.pulse_2.tick_envelope();
                            },
                            1 => {
                                self.pulse_1.tick_envelope();
                                self.pulse_2.tick_envelope();
                                self.pulse_1.tick_length_and_sweep();
                                self.pulse_2.tick_length_and_sweep();
                            },
                            3 => {
                                self.pulse_1.tick_envelope();
                                self.pulse_2.tick_envelope();
                                self.pulse_1.tick_length_and_sweep();
                                self.pulse_2.tick_length_and_sweep();
                                if !self.irq_inhibit {
                                    self.frame_irq = true;
                                }
                            },
                            _ => unreachable!()
                        }
                        self.frame_seq += 1;
                        self.frame_seq %= 4;
                    }
                    // Mode 1 - 5-step sequence
                    true => {
                        match self.frame_seq {
                            0 | 2 => {
                                self.pulse_1.tick_envelope();
                                self.pulse_2.tick_envelope();
                                self.pulse_1.tick_length_and_sweep();
                                self.pulse_2.tick_length_and_sweep();
                            },
                            1 | 3 => {
                                self.pulse_1.tick_envelope();
                                self.pulse_2.tick_envelope();
                            },
                            4 => {},
                            _ => unreachable!()
                        }
                        self.frame_seq += 1;
                        self.frame_seq %= 5;
                    }
                }
                self.quarter_frame_divider = QUARTER_FRAME_DIVIDER_PERIOD - 1;
            }
        }

        // The timers run at 1/12 the master clock speed
        match self.timer_div.checked_sub(1) {
            Some(n) => self.timer_div = n,
            None => {
                self.pulse_1.tick_timer();
                self.pulse_2.tick_timer();
                self.timer_div = 2;
            }
        }

        // Use the sample divider to calculate when to generate samples
        self.sample_divider -= 1.0;
        if self.sample_divider.is_sign_negative() {
            self.sample_buffer.push_back(self.single_sample());
            if self.sample_buffer.len() > SAMPLE_OUT {
                self.queue_samples().unwrap();
            }
            self.sample_divider += 5_369_318.0 / self.sample_out.sample_rate() as f64; // Try to generate samples at the sample rate
        }
    }

    pub fn reset(&mut self) {
        self.pulse_1.enabled = false;
        self.pulse_2.enabled = false;
    }

    pub fn get_irq(&mut self) -> bool {
        self.frame_irq
    }

    pub fn audio_device(&mut self) -> &mut T {
        &mut self.sample_out
    }

    fn update_from_registers(&mut self, registers: &mut APURegisters) {
        if let Some((addr, is_write)) = registers.last_access.get() {
            match addr {
                0..=3 => self.pulse_1.write_to_registers(addr, registers.registers[addr]),
                4..=7 => self.pulse_2.write_to_registers(addr - 4, registers.registers[addr]),

                0x15 if is_write => {
                    let v = registers.registers[0x15];
                    self.pulse_1.enabled = v & 0b0000_0001 != 0;
                    if !self.pulse_1.enabled { self.pulse_1.disable() }
                    self.pulse_2.enabled = v & 0b0000_0010 != 0;
                    if !self.pulse_2.enabled { self.pulse_2.disable() }
                },
                0x15 if !is_write => {
                    self.frame_irq = false;
                },
                0x17 if is_write => {
                    let v = registers.registers[0x17];
                    self.frame_seq_mode = v & 0b1000_0000 != 0;
                    if v & 0b1000_0000 != 0 {
                        self.pulse_1.tick_envelope();
                        self.pulse_2.tick_envelope();
                        self.pulse_1.tick_length_and_sweep();
                        self.pulse_2.tick_length_and_sweep();
                    }
                    if v & 0b0100_0000 != 0 {
                        self.frame_irq = false;
                    }
                    self.irq_inhibit = v & 0b0100_0000 != 0;
                    self.quarter_frame_divider = QUARTER_FRAME_DIVIDER_PERIOD - 1;
                },
                _ => ()
            }
        }

        // Update the status register
        let mut status = 0u8;
        status |= if self.pulse_1.length_counter_gt_zero() { 1<<0 } else { 0 };
        status |= if self.pulse_2.length_counter_gt_zero() { 1<<1 } else { 0 };
        status |= if !self.irq_inhibit && self.frame_irq { 1<<6 } else { 0 };

        registers.status_out = status;

        registers.last_access.set(None);
    }

    // https://wiki.nesdev.com/w/index.php/APU_Mixer
    fn single_sample(&self) -> f32 {
        let p1 = if self.pulse_1.enabled {self.pulse_1.digital_sample() as f32} else {0.0};
        let p2 = if self.pulse_2.enabled {self.pulse_2.digital_sample() as f32} else {0.0};
        let _t = 0 as f32;
        let _n = 0 as f32;
        let _d = 0 as f32;
        
        let mut square_out = 95.88 / ((8128.0 as f32) / (p1+p2) + 100.0);
        if !square_out.is_normal() {
            square_out = 0.0;
        }
        let tnd_out = 0.0;

        (square_out + tnd_out) * self.volume
    }

    fn queue_samples(&mut self) -> Result<()> {
        let samples = self.sample_buffer.drain(..).collect::<Vec<_>>();
        self.sample_out.queue_audio(&samples[..])?;
        Ok(())
    }
}