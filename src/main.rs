pub mod waveform;
pub mod wavetable;

use waveform::*;
use wavetable::*;

fn main() -> std::io::Result<()> {
    let mut wt = Wavetable::empty(); //空のwavetable作る
    let sin = Waveform::sin(); //sin波の波形を作る
    let fm_sin = operation::fm(&sin, 0.5, 3, &sin); //sin波でsin波をfmした波形をつくる（モジュレーターに(整数/2)倍音がくると破綻しない）
    wt.wavetable[0] = sin; //sin波をはじめにセット
    wt.wavetable[255] = fm_sin; //fmしたやつを最後にセット
    wt.morph_liner(); //間を埋める
    wt.set_name("fm_sin".to_string());
    wt.export()?;
    Ok(())
}
