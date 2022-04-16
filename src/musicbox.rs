use crate::notary::fills_notary::FillsNotary;

#[derive(Default)]
pub struct MusicBox {
    tonic: f64,
    notary: HashMap<String, f64>,
}

impl MusicBox {
    pub fn new() -> Self {
        let mut music = Self {
            tonic: TONIC,

            notary: HashMap::new(),
        };
        music.scoped_fill();
        music
    }
    pub fn piano() -> Self {
        let music = Self {
            tonic: TONIC, 
            notary: HashMap::new(),
        };
        music.piano();
        music
    }

}


impl FillsNotary for MusicBox {
    fn piano(&mut self) {
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
    fn scoped_fill(&mut self) {
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

    fn scoped_lesser(&mut self) {
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
