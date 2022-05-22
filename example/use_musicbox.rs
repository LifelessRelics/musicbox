use musicbox::MusicBox;

// Chord formation
fn main() {
    let music = MusicBox::new();

    let a0 = music
        .notary
        .get("A0").unwrap();
    let c0 = music.notary.get("C0").unwrap();
    let e0 = music.notary.get("E0").unwrap();
    let a_minor_0 = (a0, c0, e0);
    dbg!(a_minor_0);
}
