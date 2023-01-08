use crate::waveform::Waveform;

#[derive(Clone, Debug)]
pub struct WtWave {
    freq: f32,
    index:u8,
    wt : [Waveform;256],
    num_sample: usize,
}

impl WtWave {
    pub fn new(wt:[Waveform;256]) -> WtWave {
        WtWave {
            freq: 440.0,
            index: 0,
            wt:wt,
            num_sample: 0,
        }
    }
}

impl Iterator for WtWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        let value = (( self.freq * self.num_sample as f32 / 48000.0 * 2048.0) as i64).rem_euclid(2048);
        Some(self.wt[self.index as usize].waveform[value as usize])
    }
}

impl rodio::Source for WtWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}