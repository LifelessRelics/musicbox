use crate::western_music_box;
use crate::lambda;
pub mod chord_library;

use crate::chords::chord_library::augmented::*;
use crate::chords::chord_library::minor::*;
use crate::chords::chord_library::atonal::*;
use crate::chords::chord_library::diminished::*;
use crate::chords::chord_library::dominant::*;
use crate::chords::chord_library::major::*;
use crate::chords::chord_library::bitonal::*;
use crate::chords::chord_library::just::*;

#[derive(Debug)]
pub struct Chord {
    pub tonic: f64,
    pub notes: Vec<Interval>,
    pub frequencies: Vec<f64>,
}


#[derive(Debug)]
pub enum Interval {
    Tonic,
    Semitone,
    Wholetone,
    Min3,
    Maj3,
    Subdominant,
    Dim5,
    Dominant,
    Aug5,
    Maj6,
    Dom7,
    LeadTone,
    Octave,
    Min9,
    Maj9,
    Aug9,
    Tenth, //maj3 + octave
    Eleventh, //fourth + octave 
    Aug11, //tritone + octave
    Twelfth, //Fifth + octave
    Aug12, //Aug5 + octave
    Thirteenth, //Maj6 + octave
    Aug13,  //Dom7 + octave
    Maj14, //leadtone two octaves over
    Fifteenth,
    Mystic, //two octaves + wholetone
    OctaveMaj10, //octave + Tenth
    OctaveTwelfth, //Octave over twelfth
}

/// Seperated functionality for building chords
impl Chord {
    pub fn new() -> Self {
        Self {
            tonic: 432.0,
            notes: _major(),
            frequencies: vec![],
        }
    }

pub fn get_freq(&mut self) {
    for i in &self.notes {
    
       match i {
        Tonic=> self.frequencies.push(lambda(self.tonic, 0)),
        Semitone=> self.frequencies.push(lambda(self.tonic, 1)),
        Wholetone=> self.frequencies.push(lambda(self.tonic, 2)),
        Min3=> self.frequencies.push(lambda(self.tonic, 3)),
        Maj3=> self.frequencies.push(lambda(self.tonic, 4)),
        Subdominant=> self.frequencies.push(lambda(self.tonic, 5)),
        Dim5=> self.frequencies.push(lambda(self.tonic, 6)),
        Dominant=> self.frequencies.push(lambda(self.tonic, 7)),
        Aug5=> self.frequencies.push(lambda(self.tonic, 8)),
        Maj6=> self.frequencies.push(lambda(self.tonic, 9)),
        Dom7=> self.frequencies.push(lambda(self.tonic, 10)),
        LeadTone=> self.frequencies.push(lambda(self.tonic, 11)),
        Octave=> self.frequencies.push(lambda(self.tonic, 12)),
        Min9=> self.frequencies.push(lambda(self.tonic, 13)),
        Maj9=> self.frequencies.push(lambda(self.tonic, 14)),
        Aug9=> self.frequencies.push(lambda(self.tonic, 15)),
        Tenth=> self.frequencies.push(lambda(self.tonic, 16)), //maj3 + octave
        Eleventh=> self.frequencies.push(lambda(self.tonic, 17)), //fourth + octave 
        Aug11=> self.frequencies.push(lambda(self.tonic, 18)), //tritone + octave
        Twelfth=> self.frequencies.push(lambda(self.tonic, 19)), //Fifth + octave
        Aug12=> self.frequencies.push(lambda(self.tonic, 20)), //Aug5 + octave
        Thirteenth=> self.frequencies.push(lambda(self.tonic, 21)), //Maj6 + octave
        Aug13=> self.frequencies.push(lambda(self.tonic, 22)),  //Dom7 + octave
        Maj14=> self.frequencies.push(lambda(self.tonic, 23)), 
        Fifteenth=> self.frequencies.push(lambda(self.tonic, 24)),
        Mystic=> self.frequencies.push(lambda(self.tonic, 26)), //two octaves + wholetone
        OctaveMaj10=> self.frequencies.push(lambda(self.tonic, 28)), //octave + Tenth
        OctaveTwelfth=> self.frequencies.push(lambda(self.tonic, 31)), //Octave over twelfth
    
       }
            
    }
}
    
    //Returns Major Chord Intervals
    pub fn aug(&mut self) {
        self.notes = _aug();
    }
    pub fn aug11(&mut self) {
        self.notes = _aug11();
    }
    pub fn dream(&mut self) {
        self.notes = _dream();
    }
    pub fn elektra(&mut self) {
        self.notes = _elektra();
    }
    pub fn farben(&mut self) {
        self.notes = _farben();
    }
    pub fn halfdim7(&mut self) {
        self.notes = _halfdim7();
    }
    pub fn harmonic7(&mut self) {
        self.notes = _harmonic7();
    }
    pub fn lead_tone_triad(&mut self) {
        self.notes = _lead_tone_triad();
    }
    pub fn lydian(&mut self) {
        self.notes = _lydian();
    }
    pub fn magic(&mut self) {
        self.notes = _magic();
    }

    
    pub fn major(&mut self) {
        self.notes = _major();
    }
    pub fn minor(&mut self) {
        self.notes = _minor();
    }
    //Returns Major 7 Chord Intervals

    pub fn maj7(&mut self) {
        self.notes = _maj7();
    }

    pub fn augmaj7(&mut self) {
        self.notes = _augmaj7();
    }
    pub fn dominant9(&mut self) {
        self.notes = _dominant();
    }
    pub fn dominant11(&mut self) {
        self.notes = _dominant11();
    }
    pub fn dominantmin9(&mut self){
        self.notes = _dominantmin9();
    }
    pub fn maj7sharp11(&mut self){
        self.notes = _maj7sharp11();
    }
    pub fn dominant7(&mut self) {
        self.notes = _dominant7();
    }
    pub fn dominant7sharp9(&mut self) {
        self.notes = _dominant7sharp9();
    }
    pub fn dominant13(&mut self) {
        self.notes = _dominant13();
    }
    pub fn maj11(&mut self) {
        self.notes = _maj11();
    }
    pub fn dominantparallel(&mut self) {
        self.notes = _dominantparallel();
    }
    pub fn aug7(&mut self) {
        self.notes = _aug7();
    }
    pub fn aug6_italian(&mut self) {
        self.notes = _aug6_italian();
    }
    pub fn aug6_french(&mut self) {
        self.notes = _aug6_french();
    }
    pub fn aug6_german(&mut self) {
        self.notes = _aug6_german();
    }
    
    pub fn dim(&mut self) {
        self.notes = _dim();
    }
    pub fn dimmaj7(&mut self) {
        self.notes = _dimmaj7();
    }
    pub fn dim7(&mut self) {
        self.notes = _dim7();
    }
    pub fn dominant7flat5(&mut self) {
        self.notes = _dominant7flat5();
    }
    
    pub fn dominant(&mut self) {
        self.notes = _dominant();
    }
    pub fn min11(&mut self) {
        self.notes = _min11();
    }
    pub fn minmaj7(&mut self) {
        self.notes = _minmaj7();
    }
    pub fn min9(&mut self) {
        self.notes = _min9();
    }
    pub fn min7(&mut self) {
        self.notes = _min7();
    }
    pub fn min6(&mut self) {
        self.notes = _min6();
    }
    pub fn min69(&mut self) {
        self.notes = _min69();
    }
    pub fn min13(&mut self) {
        self.notes = _min13();
    }
    pub fn mystic(&mut self) {
        self.notes = _mystic();
    }
    pub fn neopolitan(&mut self) {
        self.notes = _neopolitan();
    }
    pub fn ninthaug5(&mut self) {
        self.notes = _ninthaug5();
    }
    pub fn ninthdim5(&mut self) {
        self.notes = _ninthdim5();
    }
    pub fn northern_lights(&mut self) {
        self.notes = _northern_lights();
    }
    //TODO: Add the rest of the chords: https://en.wikipedia.org/wiki/List_of_chords
}

// Dream, //Tonic,fourth,dim5 ,fifth
// Elektra, //Tonic, dim5, 6, octave, maj3
// Farben, //Tonic, min6, maj7, maj3, fifth
// HalfDim7, //Tonic, min3, dim5, dom7
// Harmonic7, //Tonic, maj3, fifth, maj6,
// LeadToneTriad, // leading tone, 2nd, maj7
// Lydian, //Tonic, maj3, fifth, maj7, fourth
// Magic, // tonic, minor2, min3, dim5, dom7, octave, min3, fourth
// Maj11, //Tonic, maj3, fifth, maj7, maj9,