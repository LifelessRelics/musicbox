///Fill harmonics instantiates harmonics above the defined crate::TONIC
///The inverse hashmaps the frequencies below crate::TONIC
pub trait FillHarmonics {
    fn fill_harmonics(&mut self);
    fn fill_inverse_harmonics(&mut self);
}
