use crate::chords::{Interval};
use crate::traits::FillHarmonics;
use crate::{SCOPE, TONIC};
use std::collections::HashMap;

/// Hashmap containing notes congruent with western music theory,
/// not arranged with middle C as C0, with reason that no natural string may produce a harmonic below its tonic.
#[derive(Default)]
pub struct WesternMusicBox {
    tonic: f64,
    western_degree: i64,
    notes: [String; 12],
    pub harmonics: HashMap<String, f64>,
    chord: Vec<Interval>, //TODO, impl chord library over this field in the musicbox
}

impl WesternMusicBox {
    pub fn new() -> Self {
        let mut music = Self {
            tonic: TONIC,
            western_degree: 0,
            harmonics: HashMap::new(),

            notes: [
                "A".to_owned(),
                "Bb".to_owned(),
                "B".to_owned(),
                "C".to_owned(),
                "Db".to_owned(),
                "D".to_owned(),
                "Eb".to_owned(),
                "E".to_owned(),
                "F".to_owned(),
                "Gb".to_owned(),
                "G".to_owned(),
                "Ab".to_owned(),
            ],
            chord: vec![],
        };
        music.fill_harmonics();
        music.fill_inverse_harmonics();
        music
    }
}

impl FillHarmonics for WesternMusicBox {
    fn fill_harmonics(&mut self) {
        let mut j: i64 = 0;
        let mut k: usize = 0;
        for i in 0..SCOPE {
            if k == 12 {
                k = 0;
                j += 1;
            }

            self.western_degree = i;
            let twelth_root_ratio: f64 = self.western_degree as f64 / 12.0;
            let lambda: f64 = self.tonic * (2.0_f64.powf(twelth_root_ratio));
            self.harmonics
                .insert(format!("{}{}", self.notes[k], j), lambda.to_owned());

            k += 1;
        }
    }
    fn fill_inverse_harmonics(&mut self) {
        let mut j: i64 = 0;
        let mut k: usize = 0;

        for i in 0..SCOPE {
            if k == 12 {
                k = 0;
                j -= 1;
            }

            self.western_degree = i;
            let twelth_root_ratio: f64 = self.western_degree as f64 / 12.0;
            let lambda: f64 = self.tonic / (2.0_f64.powf(twelth_root_ratio));
            self.harmonics
                .insert(format!("{}{}", self.notes[k], j), lambda);

            k += 1;
        }
    }
}


// enum ChordTonic {
//     A,
//     B,
//     C,
//     D,
//     E,
//     F,
//     G,
//     Ab,
//     Bb,
//     Db,
//     Eb,
//     Gb,
//     Ab,
// }
