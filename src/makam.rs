use crate::notation::zoot_allures::Notation;
use Notation::{Z1, Z3};

use Notation::Z2;
use Notation::ZQ;

/// Is a series of trichords, tetrachords, and/or pentachords which
/// Makam compose scales
#[derive(Default)]
pub struct Makam {
    pub djin: Vec<Notation>,
    pub scale: Vec<Notation>,
}

impl Makam {
    pub fn new() -> Self {
        Self {
            djin: vec![],
            scale: vec![],
        }
    }
    pub fn cargah(&mut self) {
        self.djin = vec![Z2, Z2, Z1, Z2];
    }
    pub fn kurdi(&mut self) {
        self.djin = vec![Z1, Z2, Z3];
    }
    pub fn rast(&mut self) {
        self.djin = vec![Z2, ZQ, ZQ, Z2];
    }
    pub fn ussak(&mut self) {
        self.djin = vec![ZQ, Z2, Z1, Z2];
    }
    pub fn hicaz(&mut self) {
        self.djin = vec![Z2, Z2, Z1, Z2];
    }
}

#[test]
fn get_djin_cargah() {
    let mut cargah = Makam::new();
    cargah.cargah();
    assert_eq!(cargah.djin[1], Z2);
}
