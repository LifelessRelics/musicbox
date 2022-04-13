pub mod chords;
pub mod natural_music_box;
mod traits;
pub mod western_music_box;

/// Adjusts quantity of notes in hashmap
pub const SCOPE: i64 = 365;
/// Music box tuning
pub const TONIC: f64 = 432.0;

///Foundation for western music theory half-step
pub fn phi() -> f64 {
    (1.0 + 5.0_f64.sqrt()) / 2.0
}

pub fn twlv_root_two() -> f64 {
    2.0_f64.powf(1.0 / 12.0)
}

/// This function takes in a tonic frequency and returns the frequency corresponding to the math equation Interval_frequency(degree) = TONIC* 2.0^(degree/12)
pub fn lambda(tonic_freq: f64, degree: i64) -> f64 {
    tonic_freq * 2.0_f64.powf(degree as f64/ 12.0)
}