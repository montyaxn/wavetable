pub mod waveform;
pub mod wavetable;
pub mod preview;

use waveform::*;
use wavetable::*;

fn main() -> std::io::Result<()> {
    let mut wt = Wavetable::empty();
    let sin = Waveform::sin(); 
    let fm_sin = operation::fm(&sin, 0.5, 3, &sin); //sin波でsin波をfmした波形をつくる（モジュレーターに(整数/2)倍音がくると破綻しない）
    wt.wavetable[0] = sin; 
    wt.wavetable[255] = fm_sin; 
    wt.morph_liner(); 
    wt.set_name("fm_sin".to_string());
    wt.export()?;
    wt.preview().unwrap();
    Ok(())
}
