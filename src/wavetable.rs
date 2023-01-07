use std::mem::MaybeUninit;

use crate::waveform::*;

// wavetable. includes 256 waveforms
pub struct Wavetable{
    pub wavetable: [Waveform; 256],
    name: String,
}

impl Wavetable {
    pub fn empty() -> Wavetable {
        let wt = {
            let mut wt: [MaybeUninit<Waveform>; 256] =
                unsafe { MaybeUninit::uninit().assume_init() };
            for i in 0..256 {
                wt[i] = MaybeUninit::new(Waveform::empty());
            }
            unsafe { std::mem::transmute::<_, [Waveform; 256]>(wt) }
        };
        Wavetable::new(wt,"untitled".to_string())
    }

    pub fn new(wavetable: [Waveform; 256], name: String) -> Wavetable {
        Wavetable {
            wavetable,
            name: name,
        }
    }

    pub fn from_fn<F, T>(f: F, name: String) -> Wavetable
    where
        T: Fn(f32) -> f32,
        F: Fn(f32) -> T,
    {
        let mut i = 0;
        let wavetable = [0; 256].map(|_| {
            let w = Waveform::from_fn(f(i as f32 / 256.0));
            i += 1;
            w
        });
        Wavetable::new(wavetable, name)
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    // apply normalize to each waveform
    pub fn normalize(&mut self) {
        self.wavetable.iter_mut().for_each(|w| w.normalize());
    }

    pub fn morph_liner(&mut self) {
        for wt in 0..256 {
            for wf in 0..2048 {
                self.wavetable[wt].waveform[wf] = (((255.0 - wt as f32)
                    * self.wavetable[0].waveform[wf])
                    + (wt as f32 * self.wavetable[255].waveform[wf]))
                    / 255.0
            }
        }
    }

    // make .wav from Wavetable. file name is the same as wavetable name.
    pub fn export(&self) -> std::io::Result<()> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let fname = format!("{}{}{}","wavetable/",self.name,".wav");
        let mut writer =
            hound::WavWriter::create(fname, spec)
                .unwrap();
        for w in self.wavetable.iter() {
            for s in w.waveform.iter() {
                let amp = i16::MAX as f32;
                writer.write_sample((s * amp) as i16).unwrap();
            }
        }
        Ok(())
    }
}
