use musicbox::MusicBox;

// Chord formation
fn main() {
    let mut music = MusicBox::new();

    let a_major_0 = music
        .notary
        .retain(|k, _| k == "A0" && k == "Db0" && k == "E0");
    dbg!(a_major_0); //FAILS!
}

