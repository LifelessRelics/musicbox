pub mod fills_notary;
use crate::semitone;
use crate::TONIC;
use fills_notary::GetIntervals;

#[derive(Default, Debug)]
pub struct Notary {
    pub tonic: f64,
    pub octave: i64,
    pub notary: Vec<f64>,
}

impl Notary {
    pub fn new() -> Self {
        Self {
            tonic: TONIC,
            octave: 0,
            notary: vec![],
        }
    }

    pub fn from(tonic: f64, octave: i64) -> Self {
        Self {
            tonic,
            octave,
            notary: vec![]
        }
    }
    pub fn z0(&mut self) {
        self._0();
    }
    pub fn z1(&mut self){
        self._1();
    }
    pub fn z2(&mut self){
        self._2();
    }
    pub fn z3(&mut self){
        self._3();
    }
    pub fn z4(&mut self){
        self._4();
    }
    pub fn z5(&mut self){
        self._5();
    }
    pub fn z6(&mut self){
        self._6();
    }
    pub fn z7(&mut self){
        self._7();
    }
    pub fn z8(&mut self){
        self._8();
    }
    pub fn z9(&mut self){
        self._9();
    }
    pub fn x(&mut self){
        self._x();
    }
    pub fn y(&mut self){
        self._y();
    }
    pub fn a(&mut self){
        self._a();
    }
    pub fn b(&mut self){
        self._b();
    }
    pub fn c(&mut self){
        self._c();
    }
    pub fn d(&mut self){
        self._d();
    }
    pub fn e(&mut self){
        self._e();
    }
    pub fn f(&mut self){
        self._f();
    }
    pub fn g(&mut self){
        self._g();
    }
    pub fn h(&mut self){
        self._h();
    }
    pub fn i(&mut self){
        self._i();
    }
    pub fn j(&mut self){
        self._j();
    }
    pub fn k(&mut self){
        self._k();
    }
    pub fn l(&mut self){
        self._l();
    }
    pub fn m(&mut self){
        self._m();
    }
}

impl GetIntervals for Notary {
    ///[0]Tonic
    fn _0(&mut self) {
        self.notary.push(semitone(self.tonic , 0));
    }
    ///[1]Semitone
    fn _1(&mut self) {
        self.notary.push(semitone(self.tonic, 1));
    }
    ///[2]Wholetone
    fn _2(&mut self) {
        self.notary.push(semitone(self.tonic, 2));
    }
    ///[3]Minor3
    fn _3(&mut self) {
        self.notary.push(semitone(self.tonic, 3));
    }
    ///[4]Major3
    fn _4(&mut self) {
        self.notary.push(semitone(self.tonic, 4));
    }
    ///[5]Fourth(prime)inv[7]
    fn _5(&mut self) {
        self.notary.push(semitone(self.tonic, 5));
    }
    ///[6]Tritone***
    fn _6(&mut self) {
        self.notary.push(semitone(self.tonic, 6));
    }
    ///[7] Fifth(prime)inv[5],
    fn _7(&mut self) {
        self.notary.push(semitone(self.tonic, 7));
    }
    ///[8]Inverse[4]&&AugFifth
    fn _8(&mut self) {
        self.notary.push(semitone(self.tonic, 8));
    }
    ///[9]Inverse[3]Major6
    fn _9(&mut self) {
        self.notary.push(semitone(self.tonic, 9));
    }
    ///[x]Inverse[2]Dominant7
    fn _x(&mut self) {
        self.notary.push(semitone(self.tonic, 10));
    }
    ///[y]Inverse[1]Major7&&LeadTone
    fn _y(&mut self) {
        self.notary.push(semitone(self.tonic, 11));
    }
    ///[a] Octave(prime)
    fn _a(&mut self) {
        self.notary.push(semitone(self.tonic, 12));
    }
    ///[b] Minor9(invert[9])
    fn _b(&mut self) {
        self.notary.push(semitone(self.tonic, 13));
    }
    ///[c] Major9(invert[x])
    fn _c(&mut self) {
        self.notary.push(semitone(self.tonic, 14));
    }
    ///[d] Sharp9(invert[9])
    fn _d(&mut self) {
        self.notary.push(semitone(self.tonic, 15));
    }
    ///[e] Tenth(invert[8])
    fn _e(&mut self) {
        self.notary.push(semitone(self.tonic, 16));
    }
    ///[f] Eleventh(invert[7])
    fn _f(&mut self) {
        self.notary.push(semitone(self.tonic, 17));
    }
    ///[g] Sharp11(invert[6])
    fn _g(&mut self) {
        self.notary.push(semitone(self.tonic, 18));
    }
    ///[h] Twelfth(inv[5])
    fn _h(&mut self) {
        self.notary.push(semitone(self.tonic, 19));
    }
    ///[i] Flat13(invert[4])
    fn _i(&mut self) {
        self.notary.push(semitone(self.tonic, 20));
    }
    ///[j] Thirteen(inv[3])
    fn _j(&mut self) {
        self.notary.push(semitone(self.tonic, 21));
    }
    ///[k] Sharp13(inv[2])
    fn _k(&mut self) {
        self.notary.push(semitone(self.tonic, 22));
    }
    ///[l] Flat15 (invert[1])
    fn _l(&mut self) {
        self.notary.push(semitone(self.tonic, 23));
    }
    ///[m] Fifteen && DoubleOctave
    fn _m(&mut self) {
        self.notary.push(semitone(self.tonic, 24));
    }
}

#[test]
fn get_amaj0() {

    // Create chords using a generic trait method that is implemented on different tonics.
}
