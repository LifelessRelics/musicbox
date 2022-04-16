pub mod major {
    use crate::notation::zoot_allures::Notation;

    pub fn _major() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7]
    }

    pub fn _maj7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7, Notation::Y]
    }

    pub fn _dominant9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z6,
            Notation::X,
            Notation::C,
        ]
    }
    pub fn _dominant11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::X,
            Notation::C,
            Notation::F,
        ]
    }
    pub fn _dominantmin9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::X,
            Notation::B,
        ]
    }
    pub fn _maj7sharp11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z6,
            Notation::Y,
            Notation::G,
        ]
    }

    pub fn _dominant7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7, Notation::X]
    }

    pub fn _dominant7sharp9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::X,
            Notation::D,
        ]
    }

    pub fn _dominant13() -> Vec<Notation> {
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

    pub fn _maj11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Y,
            Notation::C,
            Notation::F,
        ]
    }
    pub fn _harmonic7() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Z9,
            Notation::C,
        ]
    }
    ////////////
    pub fn _maj6() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7, Notation::Z6]
    }
    pub fn _maj69() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Z6,
            Notation::C,
        ]
    }
    pub fn _maj9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Y,
            Notation::C,
        ]
    }
    pub fn _maj13() -> Vec<Notation> {
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
    pub fn _neopolitan() -> Vec<Notation> {
        vec![Notation::Z1, Notation::Z5, Notation::Y]
    }
}

pub mod minor {
    use crate::notation::zoot_allures::Notation;

    pub fn _minor() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z7]
    }
    pub fn _minmaj7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z7, Notation::Y]
    }
    pub fn _min9() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z3,
            Notation::Z7,
            Notation::X,
            Notation::C,
        ]
    }
    pub fn _min7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z7, Notation::X]
    }
    pub fn _min6() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z7, Notation::Z6]
    }
    pub fn _min69() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z3,
            Notation::Z7,
            Notation::Z6,
            Notation::C,
        ]
    }
    pub fn _min13() -> Vec<Notation> {
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

    pub fn _mediant() -> Vec<Notation> {
        vec![Notation::Z4, Notation::Z7, Notation::Y]
    }
    pub fn _min11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z3,
            Notation::Z7,
            Notation::X,
            Notation::C,
            Notation::F,
        ]
    }

    pub fn _dominantparallel() -> Vec<Notation> {
        //dominant relative minor

        vec![Notation::Z4, Notation::Z7, Notation::Y]
    }
}

pub mod augmented {
    use crate::notation::zoot_allures::Notation;

    pub fn _aug() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6]
    }
    pub fn _aug7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6, Notation::X]
    }

    pub fn _aug6_italian() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z9]
    }

    pub fn _aug6_french() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6, Notation::Z9]
    }

    pub fn _aug6_german() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z7, Notation::Z9]
    }
    pub fn _aug11() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::X,
            Notation::C,
            Notation::F,
        ]
    }
    pub fn _lydian() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z7,
            Notation::Y,
            Notation::I,
        ]
    }
    pub fn _augmaj7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6, Notation::Y]
    }
    pub fn _ninthaug5() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z6,
            Notation::X,
            Notation::C,
        ]
    }
    pub fn _ninthdim5() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z4,
            Notation::Z6,
            Notation::Y,
            Notation::C,
        ]
    }
}

pub mod diminished {
    use crate::notation::zoot_allures::Notation;

    pub fn _dim() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z6]
    }

    pub fn _dimmaj7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z6, Notation::Y]
    }

    pub fn _dim7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z6, Notation::Z9]
    }

    pub fn _dominant7flat5() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z4, Notation::Z6, Notation::X]
    }
    pub fn _lead_tone_triad() -> Vec<Notation> {
        vec![Notation::Z2, Notation::Z5, Notation::Y]
    }

    pub fn _halfdim7() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z3, Notation::Z6, Notation::X]
    }
}

pub mod dominant {
    use crate::notation::zoot_allures::Notation;

    pub fn _dominant() -> Vec<Notation> {
        vec![Notation::Z7, Notation::Y, Notation::Z2]
    }
}

pub mod just {
    use crate::notation::zoot_allures::Notation;
    pub fn _magic() -> Vec<Notation> {
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
    pub fn _dream() -> Vec<Notation> {
        vec![Notation::Z0, Notation::Z5, Notation::Z6, Notation::Z7]
    }
}
pub mod bitonal {
    use crate::notation::zoot_allures::Notation;

    pub fn _elektra() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z7,
            Notation::Z6,
            Notation::B,
            Notation::E,
        ] //test
    }
}
pub mod atonal {
    use crate::notation::zoot_allures::Notation;

    pub fn _farben() -> Vec<Notation> {
        vec![
            Notation::Z0,
            Notation::Z6,
            Notation::Y,
            Notation::E,
            Notation::J,
        ]
    }
    pub fn _mystic() -> Vec<Notation> {
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
