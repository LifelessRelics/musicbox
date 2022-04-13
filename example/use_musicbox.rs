use musicbox::western_music_box::WesternMusicBox;

// Chord formation
fn main() {
    let mut music = WesternMusicBox::new();

    let a_major_0 = music
        .harmonics
        .retain(|k, _| k == "A0" || k == "Db0" || k == "E0");
    dbg!(a_major_0);
}
