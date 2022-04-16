use musicbox::notary::Notary;

fn main() {
    let mut maj7 = Notary::new();
maj7.z0();
maj7.z4();
maj7.z7();
maj7.y();
dbg!(maj7);
}
