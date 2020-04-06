pub struct Audio {
    pub device: sdl2::audio::AudioQueue<f32>
}

impl nes_core::apu::AudioOutput for Audio {
    fn queue_audio(&mut self, samples: &mut [f32]) -> Result<(), String> {
        self.device.queue(samples);
        Ok(())
    }
    fn sample_rate(&self) -> usize {
        self.device.spec().freq as usize
    }
}