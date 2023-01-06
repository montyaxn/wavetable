// waveform type on heap
pub struct Waveform {
    pub waveform: Box<[f32; 2048]>,
}

impl Waveform {
    pub fn new(w: [f32; 2048]) -> Self {
        Waveform {
            waveform: Box::new(w),
        }
    }

    pub fn from_fn<F>(f: F) -> Waveform
    where
        F: Fn(f32) -> f32,
    {
        let mut wave: [f32; 2048] = [0.0; 2048];
        for s in 0..2048 {
            wave[s] = f(s as f32 / 2048_f32);
        }

        Waveform::new(wave)
    }

    pub fn empty() -> Waveform {
        let w = [0.0; 2048];
        Waveform::new(w)
    }

    // make sin wave form
    pub fn sin() -> Waveform {
        let sin = |phase: f32| (phase * 2.0 * std::f32::consts::PI).sin();
        Waveform::from_fn(sin)
    }

    // make saw wave form
    pub fn saw() -> Waveform {
        let mut w: [f32; 2048] = [0.0; 2048];
        for s in 0..2048 {
            w[s] = (s as f32 / 1024_f32 * std::f32::consts::PI).sin();
        }

        Waveform::new(w)
    }

    // convert Wave form into raw data
    pub fn render(&self) -> Vec<i16> {
        self.waveform
            .into_iter()
            .map(|w| {
                if w >= 1_f32 {
                    i16::MAX
                } else if w <= -1_f32 {
                    i16::MIN
                } else {
                    (w * i16::MAX as f32) as i16
                }
            })
            .collect()
    }

    pub fn normalize(&mut self) {
        let max = self.waveform.iter().fold(f32::NAN, |m, v| v.max(m));
        if max == 0.0 {
            return;
        }
        for i in 0..2048 {
            self.waveform[i] = self.waveform[i] / max;
        }
    }

    pub fn scale(&mut self, sf: f32) {
        for i in 0..2048 {
            self.waveform[i] = self.waveform[i] * sf;
        }
    }

    pub fn invert(&mut self) {
        for i in 0..1024 {
            let temp = self.waveform[i];
            self.waveform[i] = self.waveform[2047 - i];
            self.waveform[2047 - i] = temp;
        }
    }

    // act as a periodic function based on waveform
    pub fn as_fn(&self, x: i32) -> f32 {
        let i = (x.rem_euclid(2048)) as usize;
        self.waveform[i]
    }
}

pub mod operation {
    use crate::waveform::*;

    pub fn add(r: &Waveform, l: &Waveform) -> Waveform {
        let mut w = [0.0_f32; 2048];
        for i in 0..2048 {
            w[i] = r.waveform[i] + l.waveform[i];
        }
        Waveform::new(w)
    }

    pub fn sub(r: &Waveform, l: &Waveform) -> Waveform {
        let mut w = [0.0_f32; 2048];
        for i in 0..2048 {
            w[i] = r.waveform[i] - l.waveform[i];
        }
        Waveform::new(w)
    }

    pub fn mul(r: &Waveform, l: &Waveform) -> Waveform {
        let mut w = [0.0_f32; 2048];
        for i in 0..2048 {
            w[i] = r.waveform[i] * l.waveform[i];
        }
        Waveform::new(w)
    }

    pub fn div(r: &Waveform, l: &Waveform) -> Waveform {
        let mut w = [0.0_f32; 2048];
        for i in 0..2048 {
            w[i] = r.waveform[i] / l.waveform[i];
        }
        Waveform::new(w)
    }

    pub fn fm(carrier: &Waveform, amp: f32, harmonic: i32, modulator: &Waveform) -> Waveform {
        let mut w = [0.0_f32; 2048];
        for i in 0..2048 {
            w[i] = carrier.as_fn(
                i as i32 + (amp * modulator.as_fn(i as i32 * harmonic) * 2048.0).round() as i32,
            );
        }
        Waveform::new(w)
    }
}
