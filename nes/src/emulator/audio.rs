use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

use anyhow::{Context, Result};
use cpal::traits::*;

type CpalDataCallback =
    Box<dyn for<'a, 'b> FnMut(&'a mut [f32], &'b cpal::OutputCallbackInfo) + Send>;

pub struct AudioPlayer {
    stream: Box<dyn cpal::traits::StreamTrait>,
    volume: Arc<AtomicU16>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("No output device available")?;
        dbg!(device.default_output_config()?.buffer_size());
        let config = cpal::StreamConfig {
            ..device.default_output_config()?.config()
        };

        let volume = Arc::new(AtomicU16::new(100));

        let error_callback = |e| panic!("{}", e);

        let data_callback: CpalDataCallback = {
            let elapsed = Box::leak(Box::new(0.0));
            let t_per_sample = 1.0 / (config.sample_rate.0 as f32);
            let volume = volume.clone();
            Box::new(move |buffer, _out_info| {
                let v = volume.load(Ordering::SeqCst) as f32 / 1000.0;
                // println!("audio callback, t = {}, v = {}", t, v);
                #[inline]
                fn square_wave(t: f32) -> f32 {
                    if t % 1.0 < 0.5 {
                        2.0 * (t % 1.0)
                    } else {
                        2.0 - 2.0 * (t % 1.0)
                    }
                }
                buffer.iter_mut().enumerate().for_each(|(i, b)| {
                    let t = *elapsed + i as f32 * t_per_sample;
                    *b = square_wave(t * 220.0) * (f32::sin(t / 2.).powi(2)) * v * 0.33;
                });
                *elapsed += buffer.len() as f32 * t_per_sample;
            })
        };
        let stream = device.build_output_stream(&config, data_callback, error_callback)?;
        stream.play()?;

        Ok(AudioPlayer {
            stream: Box::new(stream),
            volume,
        })
    }

    /// Set the volume. Automatically clamps volume between 0..=1000.
    pub fn set_volume(&self, v: u16) {
        let v = v.clamp(0, 1000);
        self.volume.store(v, Ordering::SeqCst);
    }

    /// Add `dv` to the volume. Returns new volume. Automatically clamps voluime between 0..=1000.
    pub fn change_volume(&self, dv: i16) -> u16 {
        let v = self.volume.load(Ordering::Acquire) as i16 + dv;
        let v = v.clamp(0, 1000) as u16;
        self.volume.store(v, Ordering::Release);
        v
    }

    pub fn get_volume(&self) -> u16 {
        self.volume.load(Ordering::SeqCst)
    }
}

#[derive(Default)]
pub struct Audio {}

impl nes_core::apu::AudioOutput for Audio {
    fn queue_audio(&mut self, samples: &[f32]) -> Result<(), String> {
        todo!()
    }
    fn sample_rate(&self) -> usize {
        todo!()
    }
}
