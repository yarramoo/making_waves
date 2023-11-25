use making_waves::tunes;
use making_waves::pulse::Pulse;
use making_waves::util::save_f32_le;

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

fn main() -> std::io::Result<()> {
    let wave_specs = tunes::happy_birthday(0.8);
    let attack_dur = 0.01;
    let decay_dur  = 0.01;
    let wave_data = make_waves_attack_decay(&wave_specs, attack_dur, decay_dur, SAMPLE_RATE);
    save_f32_le(OUTPUT_PATH, &wave_data)?;
    std::process::Command::new("ffplay")
        .args(vec!["-f", "f32le", "-ar", "48000", OUTPUT_PATH])
        .spawn()?;
    Ok(())
}
