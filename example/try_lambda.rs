use musicbox::{lambda, phi, twlv_root_two};

///dbgs useful operators for dealing with string harmonics
fn main() {
    let tonic = lambda(440.0, 0);
    let fifth = lambda(tonic, 7);
    let twlv_root_two = twlv_root_two();
    let phi = phi();
    dbg!(fifth);
    dbg!(twlv_root_two);
    dbg!(phi);
}
