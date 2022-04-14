#[derive(Debug, PartialEq)]
pub enum Interval {
    Tonic,
    Sextone,
    Octone,
    Quartertone,
    Semitone,
    Three4tone,
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
    Tenth,      //maj3 + octave
    Eleventh,   //fourth + octave
    Aug11,      //tritone + octave
    Twelfth,    //Fifth + octave
    Aug12,      //Aug5 + octave
    Thirteenth, //Maj6 + octave
    Aug13,      //Dom7 + octave
    Maj14,      //leadtone two octaves over
    Fifteenth,
    Mystic,        //two octaves + wholetone
    OctaveMaj10,   //octave + Tenth
    OctaveTwelfth, //Octave over twelfth
}
