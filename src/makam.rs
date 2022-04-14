use crate::interval::Interval;
use Interval::{Semitone, Three4tone, Wholetone};

/// Is a series of trichords, tetrachords, and/or pentachords which
/// Makam compose scales
pub struct Makam {
    pub djin: Vec<Interval>,
    pub scale: Vec<Interval>,
}

impl Makam {
    pub fn new() -> Self {
        Self {
            djin: vec![],
            scale: vec![],
        }
    }
    pub fn cargah(&mut self) {
        self.djin = vec![Wholetone, Wholetone, Semitone, Wholetone];
    }
    pub fn kurdi(&mut self) {
        self.djin = vec![Semitone, Wholetone, Wholetone];
    }
    pub fn rast(&mut self) {
        self.djin = vec![Wholetone, Three4tone, Three4tone, Wholetone];
    }
    pub fn ussak(&mut self) {
        self.djin = vec![Three4tone, Wholetone, Semitone, Wholetone];
    }
    pub fn hicaz(&mut self) {
        self.djin = vec![Wholetone, Wholetone, Semitone, Wholetone];
    }
}

#[test]
fn get_djin_cargah() {
    let mut cargah = Makam::new();
    cargah.cargah();
    assert_eq!(cargah.djin[1], Wholetone);
}
