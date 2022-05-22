use crate::notation::zoot_allures::Notation;
use Notation::*;
mod mode;
use crate::scales::mode::Mode;

pub mod chordlib;
use chordlib::ChordLib;
///ScaleConstructor

//todo

// let major = parse my input. 0123456789xy to construct a vec of frequencies
//char by char input

/// Zetta Notation
/// The point of this is for you to understand alternate music notation
/// 024579Y is a major scale of any tonic, WWHWWWH
/// 023578X is a minor scale; No matter what tonic!
/// Modes are built as such,
///
/// ///   Ex. Tonic[0] == C   &Eb relative minor
/// C Ionian     024579Y
/// D Dorian     24579Y0
/// E Phrygian   4579Y02
/// F Lydian     579Y024
/// G Mixolydian 79Y0245
/// A Aeolian    9Y02457 -> Relative minor
/// B Locrian    Y024579
///     for Tonic as T in A..G#          !EXAMPLE::C!
/// *** *** | WWHWWWH T Ionian      0|24579Y|0  => C  D  E  F  G  A  B  C
///         | WHWWWHW T Dorian      0|23579X|0  => C  D  Eb F  G  A  Bb C
///         | HWWWHWW T Phyrgian    0|13578X|0  => C  Db Eb F  G  A  Bb C
///         | WWWHWWH T Lydian      0|24679Y|0  => C  D  E  Gb G  A  B  C
///         | WWHWWHW T Mixolydian  0|24579X|0  => C  D  E  F  G  A  Bb C
///         | WHWWHWW T Aeolian     0|23578X|0  => C  D  Eb F  G  Ab Bb C
///         | HWWHWWW T Locrian     0|13568X|0  => C  Db Eb F  Gb Ab Bb C
///
/// There are all your church modes world.
///                 
/// TODO: add blues scale, harmonic scales, all scales,
/// -> Register the result my encoding is accounting for the cyclical nature of the chromatic scale

pub struct Scale {
    definition: Vec<Notation>,
    values: Vec<f64>,
}

impl Scale {
    fn ionian(&mut self) {
        self._ionian();
    }
    fn dorian(&mut self) {
        self._dorian();
    }
    fn phyrgian(&mut self) {
        self._phyrgian();
    }
    fn lydian(&mut self) {
        self._lydian();
    }
    fn mixolydian(&mut self) {
        self._mixolydian();

    }
    fn aeolian(&mut self) {
        self._aeolian();

    }
    fn locrian(&mut self) {
        self._locrian();

    }
}

impl Mode for Scale {
    ///Major
    fn _ionian(&mut self) {
        self.definition = vec![Z0, Z2, Z4, Z5, Z7, Z9, Y];
    }

    fn _dorian(&mut self) {
        self.definition = vec![Z0, Z2, Z3, Z5, Z7, Z9, X];
    }

    fn _phyrgian(&mut self) {
        self.definition = vec![Z0, Z1, Z3, Z5, Z7, Z8, X];
    }
    fn _lydian(&mut self) {
        self.definition = vec![Z0, Z2, Z4, Z6, Z7, Z9, Y];
    }
    fn _mixolydian(&mut self) {
        self.definition = vec![Z0, Z2, Z4, Z5, Z7, Z9, X];
    }
    ////Minor
    fn _aeolian(&mut self) {
        self.definition = vec![Z0, Z2, Z3, Z5, Z7, Z8, X];
    }
    fn _locrian(&mut self) {
        self.definition = vec![Z0, Z1, Z3, Z5, Z6, Z8, X];
    }
}

impl ChordLib for Scale {
    fn _major() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7]
    }

    fn _maj7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7, Notation::Y]
    }

    fn _dominant9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z6,
            Notation::X,
            Notation::C,
        ]
    }
    fn _dominant11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::X,
            Notation::C,
            Notation::F,
        ]
    }
    fn _dominantmin9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::X,
            Notation::B,
        ]
    }
    fn _maj7sharp11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z6,
            Notation::Y,
            Notation::G,
        ]
    }

    fn _dominant7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7, Notation::X]
    }

    fn _dominant7sharp9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::X,
            Notation::D,
        ]
    }
    fn _dominant13() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::X,
            Notation::C,
            Notation::F,
            Notation::J,
        ]
    }

    fn _maj11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Y,
            Notation::C,
            Notation::F,
        ]
    }
    fn _harmonic7() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Z9,
            Notation::C,
        ]
    }
    fn _maj6() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7, Notation::Z6]
    }
    fn _maj69() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Z6,
            Notation::C,
        ]
    }
    fn _maj9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Y,
            Notation::C,
        ]
    }
    fn _maj13() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Y,
            Notation::C,
            Notation::F,
            Notation::J,
        ]
    }
    fn _neopolitan() -> Vec<Notation> {
        vec![Notation::Z1, Notation::Z5, Notation::Y]
    }

    fn _minor() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z7]
    }
    fn _minmaj7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z7, Notation::Y]
    }
    fn _min9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z3,
            Notation::Z7,
            Notation::X,
            Notation::C,
        ]
    }
    fn _min7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z7, Notation::X]
    }
    fn _min6() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z7, Notation::Z6]
    }
    fn _min69() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z3,
            Notation::Z7,
            Notation::Z6,
            Notation::C,
        ]
    }
    fn _min13() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z3,
            Notation::Z7,
            Notation::X,
            Notation::C,
            Notation::F,
            Notation::J,
        ]
    }

    fn _mediant() -> Vec<Notation> {
        vec![Notation::Z4, Notation::Z7, Notation::Y]
    }
    fn _min11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z3,
            Notation::Z7,
            Notation::X,
            Notation::C,
            Notation::F,
        ]
    }

    fn _dominantparallel() -> Vec<Notation> {
        //dominant relative minor

        vec![Notation::Z4, Notation::Z7, Notation::Y]
    }

    fn _aug() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6]
    }
    fn _aug7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6, Notation::X]
    }

    fn _aug6_italian() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z9]
    }

    fn _aug6_french() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6, Notation::Z9]
    }

    fn _aug6_german() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7, Notation::Z9]
    }
    fn _aug11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::X,
            Notation::C,
            Notation::F,
        ]
    }
    fn _lydian() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Y,
            Notation::I,
        ]
    }
    fn _augmaj7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6, Notation::Y]
    }
    fn _ninthaug5() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z6,
            Notation::X,
            Notation::C,
        ]
    }
    fn _ninthdim5() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z6,
            Notation::Y,
            Notation::C,
        ]
    }

    fn _dim() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z6]
    }

    fn _dimmaj7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z6, Notation::Y]
    }

    fn _dim7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z6, Notation::Z9]
    }

    fn _dominant7flat5() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6, Notation::X]
    }
    fn _lead_tone_triad() -> Vec<Notation> {
        vec![Notation::Z2, Notation::Z5, Notation::Y]
    }

    fn _halfdim7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z6, Notation::X]
    }

    fn _dominant() -> Vec<Notation> {
        vec![Notation::Z7, Notation::Y, Notation::Z2]
    }

    fn _magic() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z1,
            Notation::Z5,
            Notation::Z6,
            Notation::Y,
            Notation::C,
            Notation::E,
            Notation::H,
            Notation::J,
        ]
    }
    fn _dream() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z5, Notation::Z6, Notation::Z7]
    }

    fn _elektra() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z7,
            Notation::Z6,
            Notation::B,
            Notation::E,
        ] //test
    }

    fn _farben() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z6,
            Notation::Y,
            Notation::E,
            Notation::J,
        ]
    }
    fn _mystic() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z6,
            Notation::X,
            Notation::E,
            Notation::J,
            Notation::ZM,
        ]
    }
    // pub fn _northern_lights() -> Vec<Notation> {
    //     vec![
    //         Notation::Z1,
    //         Notation::Wholetone,
    //         Notation::Z6,
    //         Notation::A,
    //         Notation::D,
    //         Notation::G,
    //         Notation::H,
    //         Notation::Aug13,
    //         Notation::M,
    //         Notation::OctaveMaj10,
    //         Notation::,
    //     ]
    // }

    //TODO: finish implementing starting at "Ode-toNapoleon hexachord": https://en.wikipedia.org/wiki/List_of_chords
}
