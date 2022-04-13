pub mod interval;
pub mod chords;
pub mod natural_music_box;
pub mod western_music_box;
mod traits;
pub mod makam;
/// Adjusts quantity of notes in hashmap
pub const SCOPE: i64 = 365;
/// Music box tuning
pub const TONIC: f64 = 432.0;

///The golden Ratio
pub fn phi() -> f64 {
    (1.0 + 5.0_f64.sqrt()) / 2.0
}



///For Makam
///Calculate stacked Quartertones result
pub fn quartertone(tonic_freq: f64, stack: i64) -> f64{
    tonic_freq * 2.0_f64.powf(stack as f64/ 24.0)

}

///Arabian music uses quartertones
///Calculate (Wholetone - Quartertone) *degree
pub fn three4tone(tonic_freq: f64, stack: i64)-> f64 {
    tonic_freq * 2.0_f64.powf((3 * stack) as f64/ 24.0)

}
///experimental for cultural appreciation
///Calculate 8th of a Wholetone
pub fn eigth_tone(tonic_freq: f64, stack: i64)-> f64 {
    tonic_freq * 2.0_f64.powf(( stack) as f64/ 48.0)

}
///Deeper cultural appreciation, Also the sound can be said to vary by an amount 2^N, where N is the scope of detail.
///Calculate 16th of a Wholetone
pub fn sixteenth_tone(tonic_freq: f64, stack: i64)-> f64 {
    tonic_freq * 2.0_f64.powf((3 * stack) as f64/ 96.0)

}



/// Western theory function takes in a tonic frequency and returns the frequency corresponding to:  Interval_frequency(degree) = TONIC* 2.0^(degree/12)
pub fn semitone(tonic_freq: f64, degree: i64) -> f64 {
    tonic_freq * 2.0_f64.powf(degree as f64/ 12.0)
}