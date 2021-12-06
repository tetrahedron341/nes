pub trait AudioOutput {
    /// Called by the APU to queue audio samples.
    fn queue_audio(&mut self, samples: &mut [f32]) -> Result<(), String>;
    fn sample_rate(&self) -> usize;
}

pub struct DummyAudio();

impl AudioOutput for DummyAudio {
    fn queue_audio(&mut self, _samples: &mut [f32]) -> Result<(), String> {
        Ok(())
    }
    fn sample_rate(&self) -> usize {
        1
    }
}
