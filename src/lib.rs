pub mod chord_depr;
pub mod instrument;
pub mod makam;
pub mod natural_musicbox;
pub mod notary;
pub mod notation;
pub mod note;
pub mod scales;
pub mod score;
pub mod mubot;
pub mod fetchuser;
use scales::chordlib::major::_major;
use notary::fills_notary::FillsNotary;
use notation::zoot_allures::Notation;
use fetchuser::{fetch_input};
use mubot::Mubot;
use std::collections::HashMap;


/// Adjusts quantity of notes in hashmap
pub const SCOPE: i64 = 365;
pub const PIANO: i64 = 89;

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
        music.piano();
        music
    }
    // fn chord_fetch(input: Notation) -> Chord {
    //     println!("Welcome to Musicbox. EnterScaleToReceiveChords [ |0| 123456789XY, |A| BCDEFGHIJKL, |M| ]");
    //     let scale = fetch_input();
    // }
}




struct Chord {
    scale: Vec<Notation>,
    buffer: Vec<Notation>,
}



impl FillsNotary for MusicBox {
    fn piano(&mut self) {
        let notes = [
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
        for i in 0..PIANO {
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
        let notes = [
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
            self.notary.insert(format!("{}{}", notes[k], j), lambda);

            k += 1;
        }
    }

    fn scoped_lesser(&mut self) {
        let notes = [
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
            self.notary.insert(format!("{}{}", notes[k], j), lambda);

            k += 1;
        }
    }
}


///[0]Tonic
pub fn _0(tonic: f64) {
   semitone(tonic , 0);
}
///[1]Semitone
pub fn _1(tonic: f64) {
    semitone(tonic , 1);
 }
///[2]Wholetone
pub fn _2(tonic: f64) {
    semitone(tonic , 2);
 }
///[3]Minor3
pub fn _3(tonic: f64) {
    semitone(tonic , 3);
 }
///[4]Major3
pub fn _4(tonic: f64) {
    semitone(tonic , 4);
 }
///[5]Fourth(prime)inv[7]
pub fn _5(tonic: f64) {
    semitone(tonic , 5);
 }
///[6]Tritone***
pub fn _6(tonic: f64) {
    semitone(tonic , 6);
 }
///[7] Fifth(prime)inv[5],
pub fn _7(tonic: f64) {
    semitone(tonic , 7);
 }
///[8]Inverse[4]&&AugFifth
pub fn _8(tonic: f64) {
    semitone(tonic , 8);
 }
///[9]Inverse[3]Major6
pub fn _9(tonic: f64) {
    semitone(tonic , 9);
 }
///[x]Inverse[2]Dominant7
pub fn _x(tonic: f64) {
    semitone(tonic , 10);
 }
///[y]Inverse[1]Major7&&LeadTone
pub fn _y(tonic: f64) {
    semitone(tonic , 11);
 }
///[a] Octave(prime)
pub fn _a(tonic: f64) {
    semitone(tonic , 12);
 }
///[b] Minor9(invert[9])
pub fn _b(tonic: f64) {
    semitone(tonic , 13);
 }
///[c] Major9(invert[x])
pub fn _c(tonic: f64) {
    semitone(tonic , 14);
 }
///[d] Sharp9(invert[9])
pub fn _d(tonic: f64) {
    semitone(tonic , 15);
 }
///[e] Tenth(invert[8])
pub fn _e(tonic: f64) {
    semitone(tonic , 16);
 }
///[f] Eleventh(invert[7])
pub fn _f(tonic: f64) {
    semitone(tonic , 17);
 }
///[g] Sharp11(invert[6])
pub fn _g(tonic: f64) {
    semitone(tonic , 18);
 }
///[h] Twelfth(inv[5])
pub fn _h(tonic: f64) {
    semitone(tonic , 19);
 }
///[i] Flat13(invert[4])
pub fn _i(tonic: f64) {
    semitone(tonic , 20);
 }
///[j] Thirteen(inv[3])
pub fn _j(tonic: f64) {
    semitone(tonic , 21);
 }
///[k] Sharp13(inv[2])
pub fn _k(tonic: f64) {
    semitone(tonic , 21);
 }
///[l] Flat15 (invert[1])
pub fn _l(tonic: f64) {
    semitone(tonic , 22);
 }
///[m] Fifteen && DoubleOctave
pub fn _m(tonic: f64) {
    semitone(tonic , 23);
 }


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
