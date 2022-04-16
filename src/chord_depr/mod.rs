pub mod chord_library;
use crate::chord_depr::chord_library::atonal::*;
use crate::chord_depr::chord_library::augmented::*;
use crate::chord_depr::chord_library::bitonal::*;
use crate::chord_depr::chord_library::diminished::*;
use crate::chord_depr::chord_library::dominant::*;
use crate::chord_depr::chord_library::just::*;
use crate::chord_depr::chord_library::major::*;
use crate::chord_depr::chord_library::minor::*;
use crate::{eigth_tone, quartertone, semitone, sixteenth_tone, three4tone};
use crate::notation::Interval;
mod zoot_allures;

#[derive(Default)]
pub struct Chord {
    pub tonic: f64,
    pub notes: Vec<Interval>,
    pub frequencies: Vec<f64>,
}

/// Seperated functionality for building chords
impl Chord {
    pub fn new() -> Self {
        Self {
            tonic: 440.0,
            notes: _major(),
            frequencies: vec![],
        }
    }

    pub fn get_freq(&mut self) {
        for i in &self.notes {
            match i {
                Interval::Tonic => self.frequencies.push(semitone(self.tonic, 0)),

                Interval::Sixteenth => self.frequencies.push(sixteenth_tone(self.tonic, 1)), // 16nd tone
                Interval::Eigth => self.frequencies.push(eigth_tone(self.tonic, 1)), // 8th tone
                Interval::Quartertone => self.frequencies.push(quartertone(self.tonic, 1)), //Quarter tone

                Interval::Semitone => self.frequencies.push(semitone(self.tonic, 1)), //HALFSTEP

                Interval::ThreeQuarter => self.frequencies.push(three4tone(self.tonic, 1)),

                Interval::Wholetone => self.frequencies.push(semitone(self.tonic, 2)),
                Interval::Min3 => self.frequencies.push(semitone(self.tonic, 3)),
                Interval::Maj3 => self.frequencies.push(semitone(self.tonic, 4)),
                Interval::Subdominant => self.frequencies.push(semitone(self.tonic, 5)),
                Interval::Tritone => self.frequencies.push(semitone(self.tonic, 6)),
                Interval::Dominant => self.frequencies.push(semitone(self.tonic, 7)),
                Interval::Aug5 => self.frequencies.push(semitone(self.tonic, 8)),
                Interval::Maj6 => self.frequencies.push(semitone(self.tonic, 9)),
                Interval::Dom7 => self.frequencies.push(semitone(self.tonic, 10)),
                Interval::LeadTone => self.frequencies.push(semitone(self.tonic, 11)),
                Interval::Octave => self.frequencies.push(semitone(self.tonic, 12)),
                Interval::Min9 => self.frequencies.push(semitone(self.tonic, 13)),
                Interval::Maj9 => self.frequencies.push(semitone(self.tonic, 14)),
                Interval::Aug9 => self.frequencies.push(semitone(self.tonic, 15)),
                Interval::Tenth => self.frequencies.push(semitone(self.tonic, 16)), //maj3 + octave
                Interval::Eleventh => self.frequencies.push(semitone(self.tonic, 17)), //fourth + octave
                Interval::Aug11 => self.frequencies.push(semitone(self.tonic, 18)), //tritone + octave
                Interval::Twelfth => self.frequencies.push(semitone(self.tonic, 19)), //Fifth + octave
                Interval::Flat13 => self.frequencies.push(semitone(self.tonic, 20)), //Aug5 + octave
                Interval::Thirteenth => self.frequencies.push(semitone(self.tonic, 21)), //Maj6 + octave
                Interval::Aug13 => self.frequencies.push(semitone(self.tonic, 22)), //Dom7 + octave
                Interval::Maj14 => self.frequencies.push(semitone(self.tonic, 23)),
                Interval::Fifteenth => self.frequencies.push(semitone(self.tonic, 24)),
                Interval::Mystic => self.frequencies.push(semitone(self.tonic, 26)), //two octaves + wholetone
                Interval::OctaveMaj10 => self.frequencies.push(semitone(self.tonic, 28)), //octave + Tenth
                Interval::OctaveTwelfth => self.frequencies.push(semitone(self.tonic, 31)), //Octave over twelfth
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
    pub fn dominantmin9(&mut self) {
        self.notes = _dominantmin9();
    }
    pub fn maj7sharp11(&mut self) {
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
    // pub fn northern_lights(&mut self) {
    //     self.notes = _northern_lights();
    // }
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
