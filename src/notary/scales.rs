///ScaleConstructor

//todo

// let major = parse my input. 0123456789xy to construct a vec of frequencies
//char by char input

/// Zetta Notation
/// The point of this is for you to understand alternate music notation
/// 024579Y is a major scale of any tonic, WWHWWWH
/// 023578X is a minor scale; No matter what tonic!
/// Modes are built as such, 
/// 
/// ///   Ex. Tonic[0] == C   &Eb relative minor 
/// C Ionian     024579Y
/// D Dorian     24579Y0 
/// E Phrygian   4579Y02
/// F Lydian     579Y024
/// G Mixolydian 79Y0245
/// A Aeolian    9Y02457 -> Relative minor
/// B Locrian    Y024579
///     for Tonic as T in A..G#          !EXAMPLE::C!
/// *** *** | WWHWWWH T Ionian      0|24579Y|0  => C  D  E  F  G  A  B  C
///         | WHWWWHW T Dorian      0|23579X|0  => C  D  Eb F  G  A  Bb C
///         | HWWWHWW T Phyrgian    0|13578X|0  => C  Db Eb F  G  A  Bb C
///         | WWWHWWH T Lydian      0|24679Y|0  => C  D  E  Gb G  A  B  C
///         | WWHWWHW T Mixolydian  0|24579X|0  => C  D  E  F  G  A  Bb C
///         | WHWWHWW T Aeolian     0|23578X|0  => C  D  Eb F  G  Ab Bb C
///         | HWWHWWW T Locrian     0|13568X|0  => C  Db Eb F  Gb Ab Bb C
/// 
/// There are all your church modes world.
///                 
/// TODO: add blues scale, harmonic scales, all scales,
/// -> Register the result my encoding is accounting for the cyclical nature of the chromatic scale

