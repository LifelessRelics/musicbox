use musicbox::{phi, lambda, twlv_root_two};

fn main() {
    let tonic = lambda(440, 0);
    let twlv_root_two = twlv_root_two();
    let phi = phi();
    dbg!(tonic);
    dbg!(twlv_root_two);
    dbg!(phi);
}

