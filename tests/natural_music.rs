
#[test]
///Ensure the indexing is correct and an entry exists
/// TODO: Confirm this matches the physical situation
fn test_natural() {
    use musicbox::natural_music_box::NaturalMusicBox;

    let music = NaturalMusicBox::new();
    let query_natural_harmonics_4over3 = match music.harmonics.get("3/4") {
        Some(_v) => true,
        None => false,
    };
    let query_minor_third = match music.harmonics.get("6/7") {
        Some(_v) => true,
        None => false,
    };
    let query_large_fractions = match music.harmonics.get("100/101") {
        //Almost the tonic consistently by this ratio, but this is reason for analysis
        Some(_v) => true,
        None => false,
    };
    let query_3quarter_tones = match music.harmonics.get("11/12") {
        //Three-Quarter Tones above tonic
        Some(_v) => true,
        None => false,
    };
    {
        assert_eq!(query_natural_harmonics_4over3, true);
    }
    {
        assert_eq!(query_minor_third, true);
    }
    {
        assert_eq!(query_large_fractions, true);
    }
    {
        assert_eq!(query_3quarter_tones, true);
    }
}
#[test]
fn test_western() {
    use musicbox::western_music_box::WesternMusicBox;

    let music = WesternMusicBox::new();
    let query_a0 = match music.harmonics.get("A0") {
        Some(_v) => true,
        None => false,
    };
    let query_a_octave_lower = match music.harmonics.get("A-1") {
        // A three octaves lower
        Some(_v) => true,
        None => false,
    };
    let query_low_b = match music.harmonics.get("B-1") {
        // The B one Octave below tonic
        Some(_v) => true,
        None => false,
    };

    let query_two_octaves_below = match music.harmonics.get("C-2") {
        // 'C' two octaves below A0
        Some(_v) => true,
        None => false,
    };
    {
        assert_eq!(query_low_b, true);
    }
    {
        assert_eq!(query_a0, true);
    }
    {
        assert_eq!(query_a_octave_lower, true);
    }
    {
        assert_eq!(query_two_octaves_below, true);
    }
}