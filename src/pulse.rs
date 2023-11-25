use std::f32::consts::PI;

pub struct Pulse {
    frequency: f32,
    duration: f32,
    volume: f32,
}

impl Pulse {
    pub fn new(frequency: f32, duration: f32 , volume: f32) -> Self {
        Self { frequency, duration, volume }
    }
    pub fn get_wave_data(&self, sample_rate: f32) -> Vec<f32> {
        let num_samples = (sample_rate * self.duration) as usize;
        (0 .. num_samples)
            .map(|sample| (sample as f32 * self.frequency * 2. * PI / sample_rate).sin() * self.volume)
            .collect()
    }
    pub fn with_attack_decay(&self, attack_dur: f32, decay_dur: f32, sample_rate: f32) -> Vec<f32> {
        let data = self.get_wave_data(sample_rate);
        let modulation = (0 .. data.len())
            .map(|sample| {
                let attack = f32::min(1.0, sample as f32 / (sample_rate * attack_dur));
                let decay = f32::min(1.0, (data.len() - sample) as f32 / (sample_rate * decay_dur));
                attack * decay
            })
            .collect::<Vec<_>>();
        data.iter().zip(modulation.iter()).map(|(d, m)| d * m).collect()
    }
}