use crate::interval::Interval;
use crate::{SCOPE, TONIC, FillsNotes};
use std::collections::HashMap;

/// Hashmap containing notes congruent with western music theory,
/// not arranged with middle C as C0, with reason that no natural string may produce a harmonic below its tonic.
#[derive(Default)]
pub struct WesternMusicBox {
    tonic: f64,
    western_degree: i64,
    pub notes: HashMap<String, f64>,
    chord: Vec<Interval>,
}

impl WesternMusicBox {
    pub fn new() -> Self {
        let mut music = Self {
            tonic: TONIC,
            western_degree: 0,
            notes: HashMap::new(),

            
            chord: vec![],
        };
        music.fill_notes();
        music.fill_inverse_notes();
        music
    }
}

impl FillsNotes for WesternMusicBox {
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

            self.western_degree = i;
            let twelth_root_ratio: f64 = self.western_degree as f64 / 12.0;
            let lambda: f64 = self.tonic * (2.0_f64.powf(twelth_root_ratio));
            self.notes
                .insert(format!("{}{}", notes[k], j), lambda.to_owned());

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

            self.western_degree = i;
            let twelth_root_ratio: f64 = self.western_degree as f64 / 12.0;
            let lambda: f64 = self.tonic / (2.0_f64.powf(twelth_root_ratio));
            self.notes
                .insert(format!("{}{}", notes[k], j), lambda);

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
