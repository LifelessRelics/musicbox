use crate::{SCOPE, TONIC, FillsNotes};
use std::collections::HashMap;

/// Note of the harmonic series N+1/N where N is 1..inf because dividing by zero is the tonic "quote me"
/// Naming conventions in this index are according to physical position of a string to ring a given harmonic
/// ::the indexing is the inverse of the harmonic series to make visualizing the frequency a physical location along a string
#[derive(Default)]
pub struct NaturalMusicBox {
    tonic: f64,
    pub notes: HashMap<String, f64>,
}

impl NaturalMusicBox {
    pub fn new() -> Self {
        let mut music = Self {
            tonic: TONIC,
            notes: HashMap::new(),
        };
        music.fill_notes();
        music
    }
}

impl FillsNotes for NaturalMusicBox {
    //Indexed by their inverse ratio
    fn fill_notes(&mut self) {
        for i in 1..SCOPE {
            let harmonic_series: f64 = ((i as f64) + 1.0) / (i as f64);
            let overtone = self.tonic * harmonic_series;
            self.notes
                .insert(format!("{}/{}", i, i + 1), overtone.to_owned());
        }
    }
    fn fill_inverse_notes(&mut self) {
        for i in 1..SCOPE {
            let harmonic_series: f64 = ((i as f64) + 1.0) / (i as f64);
            let overtone = self.tonic / harmonic_series;
            self.notes
                .insert(format!("{}/{}", i, i + 1), overtone.to_owned());
        }
    }
}
