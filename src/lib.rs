pub mod instrument;
pub mod chord;
pub mod makam;
pub mod notary;
pub mod core;
pub mod notation;
pub mod natural_musicbox;
use std::collections::HashMap;
use notation::zoot_allures::Notation;


/// Adjusts quantity of notes in hashmap
pub const SCOPE: i64 = 365;
/// Recently changed such that the A0 corresponds to the lowest note on a piano. Therefore middle-c is C3
pub const TONIC: f64 = 27.0; //432.0 A

///Core data.
pub struct MusicBox {
    pub tonic: f64,
    pub notary: HashMap<String, f64>,
    pub chord: Vec<Notation>,
}

impl MusicBox {
    pub fn new() -> Self {
        let mut music = Self {
            tonic: TONIC,

            notary: HashMap::new(),
            
            chord: vec![],
        };
        music.fill_notes();
        music.fill_inverse_notes();
        music
    }
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

           
            let twelth_root_ratio: f64 = i as f64 / 12.0;
            let lambda: f64 = self.tonic / (2.0_f64.powf(twelth_root_ratio));
            self.notary
                .insert(format!("{}{}", notes[k], j), lambda);

            k += 1;
        }
    }
}

///Used to construct musicbox harmonics
pub trait FillsNotary {
    fn fill_notes(&mut self);
    fn fill_inverse_notes(&mut self);
}