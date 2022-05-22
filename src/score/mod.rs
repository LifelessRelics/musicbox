mod rhythm;
pub mod sound;
mod time_signature;
use rhythm::Rhythm;
use crate::notation::zoot_allures::Notation;
use crate::instrument::Instrument;

pub struct MusicScore{
    bars: Bar,
}


struct Bar {
    rhythm: Rhythm, 
    instruments: Instrument,
}



struct TimeSign {
    division: Notation,
}

