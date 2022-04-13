

pub mod major {
    use crate::chords::Interval;

    pub fn _major() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
        ]
    }

    pub fn _maj7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::LeadTone,
        ]
    }
   

    pub fn _dominant9() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Aug5,
            Interval::Dom7,
            Interval::Maj9,
        ]
    }
    pub fn _dominant11() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Dom7,
            Interval::Maj9,
            Interval::Eleventh
        ]
    }
    pub fn _dominantmin9() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Dom7,
            Interval::Min9,
        ]
    }
    pub fn _maj7sharp11() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Aug5,
            Interval::LeadTone,
            Interval::Aug11,
        ]
    }

    pub fn _dominant7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Dom7,
        ]
    }

    pub fn _dominant7sharp9() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Dom7,
            Interval::Aug9,
        ]
    }

    pub fn _dominant13() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Dom7,
            Interval::Maj9,
            Interval::Eleventh,
            Interval::Thirteenth,
        ]
    }

    pub fn _maj11() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::LeadTone,
            Interval::Maj9,
            Interval::Eleventh,
        ]
    }
    pub fn _harmonic7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Maj6,
            Interval::Maj9,
        ]
    }
    ////////////
    pub fn _maj6() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Aug5,
        ]
    }
    pub fn _maj69() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Aug5,
            Interval::Maj9,
        ]
    }
    pub fn _maj9()-> Vec<Interval>  {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::LeadTone,
            Interval::Maj9,
        ]
    }
    pub fn _maj13() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::LeadTone,
            Interval::Maj9,
            Interval::Eleventh,
            Interval::Thirteenth,
        ]
    }
    pub fn _neopolitan() -> Vec<Interval> {
        vec![
            Interval::Semitone,
            Interval::Subdominant,
            Interval::LeadTone,
        ]
    }
    
}

pub mod minor {
    use crate::chords::Interval;

    pub fn _minor() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dominant,
        ]
    }
    pub fn _minmaj7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dominant,
            Interval::LeadTone,
        ]
    }
    pub fn _min9() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dominant,
            Interval::Dom7,
            Interval::Maj9,
        ]
    }
    pub fn _min7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dominant,
            Interval::Dom7,
        ]
    }
    pub fn _min6() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dominant,
            Interval::Aug5,
        ]
    }
    pub fn _min69() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dominant,
            Interval::Aug5,
            Interval::Maj9,
        ]
    }
    pub fn _min13() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dominant,
            Interval::Dom7,
            Interval::Maj9,
            Interval::Eleventh,
            Interval::Thirteenth
        ]
    }
    
    pub fn _mediant() -> Vec<Interval> {
        vec![
            Interval::Maj3,
            Interval::Dominant,
            Interval::LeadTone,
        ]
    }
    pub fn _min11() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dominant,
            Interval::Dom7,
            Interval::Maj9,
            Interval::Eleventh,
        ]
    }

    pub fn _dominantparallel() -> Vec<Interval> {
        //dominant relative minor

        vec![Interval::Maj3, Interval::Dominant, Interval::LeadTone]
    }

}

pub mod augmented {
    use crate::chords::Interval;

    pub fn _aug() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Aug5,
        ]
    }
    pub fn _aug7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Aug5,
            Interval::Dom7,
        ]
    }

    pub fn _aug6_italian() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Maj6,
        ]
    } 

    pub fn _aug6_french() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dim5,
            Interval::Maj6,
        ]
    } 

    pub fn _aug6_german() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Maj6,
        ]
    } 
    pub fn _aug11() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::Dom7,
            Interval::Maj9,
            Interval::Eleventh,
        ]
    } 
    pub fn _lydian() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dominant,
            Interval::LeadTone,
            Interval::Aug12,
        ]
    }
    pub fn _augmaj7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Aug5,
            Interval::LeadTone,
        ]
    }
    pub fn _ninthaug5() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Aug5,
            Interval::Dom7,
            Interval::Maj9, 
        ]
    }
    pub fn _ninthdim5()-> Vec<Interval>  {
       vec![ Interval::Tonic,
        Interval::Maj3,
        Interval::Dim5,
        Interval::LeadTone,
        Interval::Maj9,]
    }

}

pub mod diminished {
    use crate::chords::Interval;

    pub fn _dim() -> Vec<Interval> {
        vec![Interval::Tonic, Interval::Min3, Interval::Dim5]
    }

    pub fn _dimmaj7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dim5,
            Interval::LeadTone,
        ]
    }

    pub fn _dim7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dim5,
            Interval::Maj6,
        ]
    }

    pub fn _dominant7flat5() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Maj3,
            Interval::Dim5,
            Interval::Dom7,
        ]
    }
    pub fn _lead_tone_triad() -> Vec<Interval> {
        vec![
            Interval::Wholetone,
            Interval::Subdominant,
            Interval::LeadTone
        ]
    }
    
    pub fn _halfdim7() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Min3,
            Interval::Dim5,
            Interval::Dom7,
        ]
    }
}

pub mod dominant {
    use crate::chords::Interval;

    pub fn _dominant() -> Vec<Interval> {
        vec![Interval::Dominant, Interval::LeadTone, Interval::Wholetone]
    }

  
}

pub mod just {
    use crate::chords::Interval;
    pub fn _magic() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Semitone,
            Interval::Subdominant,
            Interval::Dim5,
            Interval::LeadTone,
            Interval::Maj9,
            Interval::Tenth,
            Interval::Twelfth,
            Interval::Thirteenth,
        ]
    }
    pub fn _dream() -> Vec<Interval> {
        vec![Interval::Tonic,Interval::Subdominant, Interval::Dim5, Interval::Dominant, ]
    }
}
pub mod bitonal {
    use crate::chords::Interval;

    pub fn _elektra() -> Vec<Interval> {
        vec![Interval::Tonic, Interval::Dominant, Interval::Aug5, Interval::Min9, Interval::Tenth] //test
    }
}
pub mod atonal {
    use crate::chords::Interval;

    pub fn _farben() -> Vec<Interval> {
        vec![Interval::Tonic, Interval::Aug5, Interval::LeadTone, Interval::Tenth, Interval::Thirteenth]
    }
    pub fn _mystic() -> Vec<Interval> {
        vec![
            Interval::Tonic,
            Interval::Dim5,
            Interval::Dom7,
            Interval::Tenth,
            Interval::Thirteenth,
            Interval::Mystic,
        ]
    }
    pub fn _northern_lights() -> Vec<Interval> {
       vec![ Interval::Semitone,
        Interval::Wholetone,
        Interval::Aug5,
        Interval::Octave,
        Interval::Aug9,
        Interval::Aug11,
        Interval::Twelfth,
        Interval::Aug13,
        Interval::Fifteenth,
        Interval::OctaveMaj10,
        Interval::OctaveTwelfth,]
    }

    //TODO: finish implementing starting at "Ode-toNapoleon hexachord": https://en.wikipedia.org/wiki/List_of_chords 
}

