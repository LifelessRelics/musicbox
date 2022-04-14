use musicbox::chords::chord_library::minor::_minor;
use musicbox::chords::*;
use musicbox::western_musicbox::WesternMusicBox;
#[test]
fn get_frequencies_of_a_chord() {
    let music = WesternMusicBox::new();
    let middle_c = match music.notes.get("C3") {
        Some(v) => v,
        None => panic!(),
    };
    let mut c_minor_chord = Chord {
        tonic: *middle_c,
        notes: _minor(), //Main functions of chord_lib return a vector of intervals to match and populate the vector below
        frequencies: vec![], //Match statement is in src/chords/mod.rs, freq calculation dont in src/lib.rs by fn lambda. Tonic * sqrt(2)^(x/12)
    };
    c_minor_chord.get_freq();
    assert!(&c_minor_chord.frequencies[0].round() != &c_minor_chord.frequencies[2].round());
    //Results in a single value filling the chord.frequencies vector...
}
