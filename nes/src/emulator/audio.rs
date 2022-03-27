use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

use anyhow::{Context, Result};
use cpal::{traits::*, SampleFormat, SampleRate};
use rtrb::chunks::ChunkError;

type CpalDataCallback =
    Box<dyn for<'a, 'b> FnMut(&'a mut [f32], &'b cpal::OutputCallbackInfo) + Send>;

pub struct AudioPlayer {
    stream: Box<dyn cpal::traits::StreamTrait>,
    pub volume: Arc<AtomicU16>,
}

impl AudioPlayer {
    pub fn new() -> Result<(Self, Audio)> {
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

        let (p, mut c) = rtrb::RingBuffer::<f32>::new(65536);

        let data_callback: CpalDataCallback = {
            let volume = volume.clone();
            let channels = config.channels as usize;
            Box::new(move |buffer, _out_info| {
                let v = volume.load(Ordering::SeqCst) as f32 / 500.0;
                let chunk = match c.read_chunk(buffer.len() / channels) {
                    Ok(chunk) => chunk,
                    Err(ChunkError::TooFewSlots(ready_samples)) => {
                        // println!("[{:?}] stalling!", std::time::Instant::now());
                        c.read_chunk(ready_samples).unwrap()
                    }
                };
                for (i, s) in chunk.into_iter().enumerate() {
                    for c in 0..channels {
                        buffer[channels * i + c] = s * v;
                    }
                }
            })
        };
        let stream = device.build_output_stream(&config, data_callback, error_callback)?;
        stream.play()?;

        let player = AudioPlayer {
            stream: Box::new(stream),
            volume,
        };

        let audio = Audio {
            sample_rate: dbg!(config).sample_rate.0 as usize,
            buffer: p,
            record: std::fs::File::create("nes_apu_log.raw").unwrap(),
        };

        Ok((player, audio))
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

pub struct Audio {
    buffer: rtrb::Producer<f32>,
    record: std::fs::File,
    sample_rate: usize,
}

impl nes_core::apu::AudioOutput for Audio {
    fn queue_audio(&mut self, samples: &[f32]) -> Result<(), String> {
        match self.buffer.write_chunk_uninit(samples.len()) {
            Ok(chunk) => {
                chunk.fill_from_iter(samples.iter().copied());
            }
            Err(ChunkError::TooFewSlots(available)) => {
                // println!("[{:?}] rushing!", std::time::Instant::now());
                self.buffer
                    .write_chunk_uninit(available)
                    .unwrap()
                    .fill_from_iter(samples.iter().copied());
            }
        }
        Ok(())
    }
    fn sample_rate(&self) -> usize {
        self.sample_rate
    }
}
