#[test]
fn main() {
    use musicbox::western_musicbox::WesternMusicBox;
    let mut music = WesternMusicBox::new();

    let a_major_0 = music
        .notes
        .retain(|k, _| k == "A0" && k == "Db0" && k == "E0");
    assert!(a_major_0 != ()) //FAILS!
}
