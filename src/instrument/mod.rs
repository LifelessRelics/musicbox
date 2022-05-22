pub mod guitar;

pub enum Classification {
    Percussion,
    Wind,
    Stringed,
    Electronic,
}

pub struct Instrument {
    class: Classification,
}