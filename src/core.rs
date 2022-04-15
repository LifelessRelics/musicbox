use crate::{TONIC,SCOPE};
use std::collections::HashMap;


///The golden Ratio
pub fn phi() -> f64 {
    (1.0 + 5.0_f64.sqrt()) / 2.0
}

///For Makam
///Calculate stacked Quartertones result
pub fn quartertone(tonic_freq: f64, stack: i64) -> f64 {
    tonic_freq * 2.0_f64.powf(stack as f64 / 24.0)
}

///Arabian music uses quartertones
///Calculate (Wholetone - Quartertone) *degree
pub fn three4tone(tonic_freq: f64, stack: i64) -> f64 {
    tonic_freq * 2.0_f64.powf((3 * stack) as f64 / 24.0)
}
///experimental for cultural appreciation
///Calculate 8th of a Wholetone
pub fn eigth_tone(tonic_freq: f64, stack: i64) -> f64 {
    tonic_freq * 2.0_f64.powf((stack) as f64 / 48.0)
}
///Deeper cultural appreciation, Also the sound can be said to vary by an amount 2^N, where N is the scope of detail.
///Calculate 16th of a Wholetone
pub fn sixteenth_tone(tonic_freq: f64, stack: i64) -> f64 {
    tonic_freq * 2.0_f64.powf((3 * stack) as f64 / 96.0)
}

/// Western theory function takes in a tonic frequency and returns the frequency corresponding to:  Interval_frequency(degree) = TONIC* 2.0^(degree/12)
pub fn semitone(tonic_freq: f64, degree: i64) -> f64 {
    tonic_freq * 2.0_f64.powf(degree as f64 / 12.0)
}


#[derive(Default)]
pub struct MusicBox {
    key: String,
    tonic: f64,
    pub notary: HashMap<String, f64>,
    pub chord_buf: Vec<f64>,
}

impl MusicBox {
    pub fn new() -> Self {
        let mut music = Self {
            key: String::from("Welcome to Music box!"),
            tonic: TONIC,
            notary: HashMap::new(),

            
            chord_buf: vec![],
        };
        music.fill_notes();
        music.fill_inverse_notes();
        music
    }
}

pub trait FillsNotary {
    fn fill_notes(&mut self);
    fn fill_inverse_notes(&mut self);
}

impl FillsNotary for MusicBox {
    fn fill_notes(&mut self) {
        let notes =[
            "A".to_string(),
            "Bb".to_string(),
            "B".to_string(),
            "C".to_string(),
            "Db".to_string(),
            "D".to_string(),
            "Eb".to_string(),
            "E".to_string(),
            "F".to_string(),
            "Gb".to_string(),
            "G".to_string(),
            "Ab".to_string(),
        ];

        let mut j: i64 = 0;
        let mut k: usize = 0;
        for i in 0..SCOPE {
            if k == 12 {
                k = 0;
                j += 1;
            }

            let twelth_root_ratio: f64 = i as f64 / 12.0;
            let lambda: f64 = self.tonic * (2.0_f64.powf(twelth_root_ratio));
            self.notary
                .insert(format!("{}{}", notes[k], j) , lambda.to_owned());

            k += 1;
        }
    }

    fn fill_inverse_notes(&mut self) {
        let notes =[
            "A".to_string(),
            "Bb".to_string(),
            "B".to_string(),
            "C".to_string(),
            "Db".to_string(),
            "D".to_string(),
            "Eb".to_string(),
            "E".to_string(),
            "F".to_string(),
            "Gb".to_string(),
            "G".to_string(),
            "Ab".to_string(),
        ];
        let mut j: i64 = 0;
        let mut k: usize = 0;

        for i in 0..SCOPE {
            if k == 12 {
                k = 0;
                j -= 1;
            }

           
            let twelth_root_ratio: f64 = i as f64 / 12.0;
            let lambda: f64 = self.tonic / (2.0_f64.powf(twelth_root_ratio));
            self.notary
                .insert(format!("{}{}", notes[k], j), lambda);

            k += 1;
        }
    }
}
