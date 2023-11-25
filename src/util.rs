use std::fs::File;
use byteorder::{WriteBytesExt, LittleEndian};

pub fn save_f32_le(output_path: &str, data: &[f32]) -> std::io::Result<()> {
    let mut f = File::create(output_path)?;
    for &entry in data.iter() {
        f.write_f32::<LittleEndian>(entry)?;
    }
    Ok(())
}

// Frequency of nth key on ideal keyboard with A4 tuned to 440Hz
// Source: https://en.wikipedia.org/wiki/Piano_key_frequencies
pub fn ideal_frequency(key: usize) -> f32 {
    f32::powf(2., (key as isize - 49) as f32 / 12.) * 440.
}