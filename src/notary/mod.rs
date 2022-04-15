use crate::core::semitone;
use crate::TONIC;




pub struct Notary {
    pub tonic: f64,
    pub octave: i64,
    pub chord: Vec<f64>,
}



// struct Chord {
//      //hash map of various length, we will store the rounded value as the key, and  
// }
//TODO: impl chord match     fn chord_match(&mut self) -> Result<Chord ,E>{}
impl Notary {
    pub fn new() -> Self {
        Self {
            tonic: TONIC,
            octave: 0,
            chord: vec![],
        }
    }
    ///[0]Tonic
    pub fn _0 (&mut self) {
        self.chord.push(semitone(self.tonic, 0));
    }
    ///[1]Semitone
    pub fn _1 (&mut self) {
        self.chord.push(semitone(self.tonic, 1));

    }
    ///[2]Wholetone
    pub fn _2(&mut self) {
        self.chord.push(semitone(self.tonic, 2));
    }
    ///[3]Minor3
    pub fn _3(&mut self) {
        self.chord.push(semitone(self.tonic, 3));

    }
    ///[4]Major3
    pub fn _4(&mut self) {
        self.chord.push(semitone(self.tonic, 4));

    }
    ///[5]Fourth(prime)inv[7]
    pub fn _5(&mut self) {
        self.chord.push(semitone(self.tonic, 5));

    }
    ///[6]Tritone***
    pub fn _6(&mut self) {
        self.chord.push(semitone(self.tonic, 6));

    }
    ///[7] Fifth(prime)inv[5],
    pub fn _7(&mut self) {
        self.chord.push(semitone(self.tonic, 7));

    }
    ///[8]Inverse[4]&&AugFifth
    pub fn _8(&mut self) {
        self.chord.push(semitone(self.tonic, 8));

    }
    ///[9]Inverse[3]Major6
    pub fn _9(&mut self) {
        self.chord.push(semitone(self.tonic, 9));

    }
    ///[x]Inverse[2]Dominant7
    pub fn _x(&mut self) {
        self.chord.push(semitone(self.tonic, 10));

    }
    ///[y]Inverse[1]Major7&&LeadTone
    pub fn _y(&mut self) {
        self.chord.push(semitone(self.tonic, 11));

    }
    ///[a] Octave(prime)
    pub fn _a(&mut self) {
        self.chord.push(semitone(self.tonic, 12));

    }
    ///[b] Minor9(invert[9])
    pub fn _b(&mut self) {
        self.chord.push(semitone(self.tonic, 13));

    }
    ///[c] Major9(invert[x])
    pub fn _c(&mut self) {
        self.chord.push(semitone(self.tonic, 14));

    }
    ///[d] Sharp9(invert[9])
    pub fn _d(&mut self) {
        self.chord.push(semitone(self.tonic, 15));

    }
    ///[e] Tenth(invert[8])
    pub fn _e(&mut self) {
        self.chord.push(semitone(self.tonic, 16));

    }
    ///[f] Eleventh(invert[7])
    pub fn _f(&mut self) {
        self.chord.push(semitone(self.tonic, 17));

    }
    ///[g] Sharp11(invert[6])
    pub fn _g(&mut self) {
        self.chord.push(semitone(self.tonic, 18));

    }
    ///[h] Twelfth(inv[5])
    pub fn _h(&mut self) {
        self.chord.push(semitone(self.tonic, 19));

    }
    ///[i] Flat13(invert[4])
    pub fn _i(&mut self) {
        self.chord.push(semitone(self.tonic, 20));

    }
    ///[j] Thirteen(inv[3])
    pub fn _j(&mut self) {
        self.chord.push(semitone(self.tonic, 21));

    }
    ///[k] Sharp13(inv[2])
    pub fn _k(&mut self) {
        self.chord.push(semitone(self.tonic, 22));

    }
    ///[l] Flat15 (invert[1])
    pub fn _l(&mut self) {
        self.chord.push(semitone(self.tonic, 23));

    }
    ///[m] Fifteen && DoubleOctave
    pub fn _m(&mut self) {
        self.chord.push(semitone(self.tonic, 24));

    }
    
}



#[test]
fn get_amaj0 () {

    // Create chords using a generic trait method that is implemented on different tonics.



}

