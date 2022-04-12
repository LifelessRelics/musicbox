use crate::SCOPE;
use crate::harmonics::Harmonic;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MusString{
    tonic: f64,
    western_degree: i64,
    notes:[String; 12],
    pub harmonic_box: HashMap<String, f64>,

}

impl MusString {
    pub fn new() -> Self {
        Self{
        tonic: 432.0,
        western_degree: 0,
        harmonic_box: HashMap::new(),
        
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
        }
    }
   
}


impl Harmonic for MusString {
    fn phi_over(&mut self) {
        let mut j: i64 = 0;
        let mut k: usize =0;
        for i in 0..SCOPE {
            if k == 12 {
                k = 0;
                j += 1;
            }
            
            self.western_degree = i;
            let twelth_root_ratio: f64 = self.western_degree as f64 /12.0;
            let lambda: f64 = self.tonic * ( 2.0_f64.powf( twelth_root_ratio ) );
            self.harmonic_box.insert(format!("{}{}", self.notes[k], j),lambda.to_owned());
           
            k += 1;
        }
        
    }
    fn natural_harmonics_over(&mut self) {
        for i in 1..SCOPE {
            let harmonic_series:f64 = ((i as f64) + 1.0) / (i as f64);
            let overtone = self.tonic * harmonic_series;
            self.harmonic_box.insert(format!("{}/{}", i, i+1), overtone.to_owned());
        }
    }
    fn phi_under(&mut self) {
        let mut j: i64 = 0;
        let mut k: usize =0;

        for i in 0..SCOPE {
            if k == 12 {
                k = 0;
                j += 1;
            }
            
            self.western_degree = i;
            let twelth_root_ratio: f64 = self.western_degree as f64 /12.0;
            let lambda: f64 = self.tonic / ( 2.0_f64.powf( twelth_root_ratio ) );
            self.harmonic_box.insert(format!("{}{}", self.notes[k], j),lambda);
            

            k += 1;
        }
    }
    fn construct(&mut self) {
        self.natural_harmonics_over();
        self.phi_over();
        // self.phi_under();
    }
}

