// use musicbox::{lambda, phi, twlv_root_two};
use musicbox::{quartertone, three4tone};

fn main() {
    let iota = quartertone(440.0, 3);
    let kappa = three4tone(440.0, 1);

    dbg!(iota);
    dbg!(kappa);

    // let tonic = lambda(440.0, 0);
    // let fifth = lambda(tonic, 7);
    // let twlv_root_two = twlv_root_two();
    // let phi = phi();

    // dbg!(fifth);
    // dbg!(twlv_root_two);
    // dbg!(phi);
}
