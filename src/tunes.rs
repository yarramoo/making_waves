use crate::Pulse;
use sound_wave::ideal_frequency;

#[allow(dead_code)]
pub(crate) fn spidermannnnnn(volume: f32) -> Vec<Pulse> {
    vec![
        Pulse::new(ideal_frequency(54), 0.3, volume),
        Pulse::new(ideal_frequency(57), 0.2, volume),
        Pulse::new(ideal_frequency(61), 0.7, volume),
        Pulse::new(100., 0.05, 0.),
        Pulse::new(ideal_frequency(60), 0.3, volume),
        Pulse::new(ideal_frequency(57), 0.2, volume),
        Pulse::new(ideal_frequency(54), 0.7, volume),
        Pulse::new(100., 0.1, 0.),
        Pulse::new(ideal_frequency(54), 0.3, volume),
        Pulse::new(ideal_frequency(57), 0.2, volume),
        Pulse::new(ideal_frequency(61), 0.4, volume),
        Pulse::new(100., 0.03, 0.),
        Pulse::new(ideal_frequency(62), 0.2, volume),
        Pulse::new(ideal_frequency(61), 0.2, volume),
        Pulse::new(100., 0.1, 0.),
        Pulse::new(ideal_frequency(60), 0.3, volume),
        Pulse::new(ideal_frequency(57), 0.2, volume),
        Pulse::new(ideal_frequency(54), 1.0, volume),
    ]
}

#[allow(dead_code)]
pub(crate) fn happy_birthday(volume: f32) -> Vec<Pulse> {
    vec![
        Pulse::new(ideal_frequency(47), 0.3, volume),
        Pulse::new(0., 0.05, 0.),
        Pulse::new(ideal_frequency(47), 0.2, volume),
        Pulse::new(ideal_frequency(49), 0.5, volume),
        Pulse::new(ideal_frequency(47), 0.3, volume),
        Pulse::new(0., 0.2, 0.),
        Pulse::new(ideal_frequency(52), 0.5, volume),
        Pulse::new(0., 0.05, 0.),
        Pulse::new(ideal_frequency(51), 0.8, volume),
        Pulse::new(0., 0.2, 0.),

        Pulse::new(ideal_frequency(47), 0.3, volume),
        Pulse::new(0., 0.05, 0.),
        Pulse::new(ideal_frequency(47), 0.2, volume),
        Pulse::new(ideal_frequency(49), 0.5, volume),
        Pulse::new(ideal_frequency(47), 0.3, volume),
        Pulse::new(0., 0.2, 0.),
        Pulse::new(ideal_frequency(52), 0.5, volume),
        Pulse::new(0., 0.05, 0.),
        Pulse::new(ideal_frequency(51), 0.8, volume),
        Pulse::new(0., 0.2, 0.),

        Pulse::new(ideal_frequency(47), 0.3, volume),
        Pulse::new(0., 0.05, 0.),
        Pulse::new(ideal_frequency(47), 0.2, volume),
        Pulse::new(ideal_frequency(59), 0.5, volume),
        Pulse::new(ideal_frequency(56), 0.5, volume),
        Pulse::new(ideal_frequency(52), 0.5, volume),
        Pulse::new(ideal_frequency(51), 0.5, volume),
        Pulse::new(ideal_frequency(49), 0.8, volume),
        Pulse::new(0., 0.2, 0.),

        Pulse::new(ideal_frequency(57), 0.3, volume),
        Pulse::new(0., 0.05, 0.),
        Pulse::new(ideal_frequency(57), 0.2, volume),
        Pulse::new(ideal_frequency(56), 0.5, volume),
        Pulse::new(ideal_frequency(52), 0.3, volume),
        Pulse::new(0., 0.2, 0.),
        Pulse::new(ideal_frequency(54), 0.5, volume),
        Pulse::new(0., 0.05, 0.),
        Pulse::new(ideal_frequency(52), 0.8, volume),
        Pulse::new(0., 0.2, 0.),
    ]
}


