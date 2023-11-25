use std::fs::File;
use byteorder::{WriteBytesExt, LittleEndian};

mod tunes;
use tunes::*;

mod pulse;
use pulse::*;

const SAMPLE_RATE: f32 = 48_000.;
const OUTPUT_PATH: &str = "output.bin";


// fn make_waves(specs: &[Pulse], sample_rate: f32) -> Vec<f32> {
//     specs.iter()
//         .map(|spec| spec.get_wave_data(sample_rate))
//         .flatten()
//         .collect()
// }

fn make_waves_attack_decay(specs: &[Pulse], attack_dur: f32, decay_dur: f32, sample_rate: f32) -> Vec<f32> {
    specs.iter()
        .map(|spec| spec.with_attack_decay(attack_dur, decay_dur, sample_rate))
        .flatten()
        .collect()
}



fn save() -> std::io::Result<()> {
    let mut f = File::create(OUTPUT_PATH)?;
    let volume = 0.8;
    let wave_specs = happy_birthday(volume);
    // let wave_data = make_waves(&wave_specs[..], SAMPLE_RATE);
    let wave_data = make_waves_attack_decay(&wave_specs[..], 0.01, 0.01, SAMPLE_RATE);
    for &entry in wave_data.iter() {
        f.write_f32::<LittleEndian>(entry)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    save()?;
    std::process::Command::new("ffplay")
        .args(vec!["-f", "f32le", "-ar", "48000", OUTPUT_PATH])
        .spawn()?;
    Ok(())
}
