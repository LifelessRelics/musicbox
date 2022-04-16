///FOR INTERVAL NOTATION USE INTERVAL!

///Notation:[[
/// [0,1,2,3,4,5,6,7,8,9,10,11,  12,13,14,15,16,17,18,19,20,21,22,23,24] ===
/// [0,1,2,3,4,5,6,7,8,9, x, y,  a, b, c, d, e, f, g, h, i, j, k, l, m]
/// ==
/// [0]Tonic,                [1]Semitone,         [2]Wholetone,            [3]Minor3,
/// [4]Major3,               [5]Fourth(prime)inv[7],    ,           [7] Fifth(prime)inv[5],
/// [8]Inverse[4]&&AugFifth, [9]Inverse[3]Major6, [x]Inverse[2]Dominant7,  [y]Inverse[1]Major7&&LeadTone,
///                                                         ***(6/12 == 1/2 in base12)
/// Start extensions a-k
/// [a] Octave(prime),      [b] Minor9(invert[9]),   [c] Major9(invert[x]),  [d] Sharp9(invert[9]),
/// [e] Tenth(invert[8]),   [f] Eleventh(invert[7]), [g] Sharp11(invert[6]), [h] Twelfth(inv[5]),
/// [i] Flat13(invert[4]),  [j] Thirteen(inv[3]),    [k] Sharp13(inv[2]),    [l] Flat15 (invert[1])
///
/// For completeness sake of two octaves
///         [m] Fifteen && DoubleOctave ([a] * 2)
/// ]]
///
/// [6]Tritone***
/// idx 0 may store octave information. if vec.0 == 2, tonic is made 2 octaves higher.
/// ==> [0|||| f64[0], .. f64[11]] == [440.0      |||| semitone(440.0), .. y(440.0) ]
/// ==> [3|||| f64[0], .. f64[22]] == [440.0 *3.0 |||| semitone(440.0 * 3.0), .. k(440.0 * 3.0)]
///
/// Im aware as you expand this patterns of primes you get some interesting results. ask me about it @github issues.
/// Donate ADA @ addr1q9lgm2alght76ng6lcgjkgq026mpjc0jws3t4ursr5t67ggazamuteg3cnawm0f6jdpl6lmw956s3vug27v6tl9fgd5sesvyv6
/// Ive been using this notation for years.
#[derive(Debug, PartialEq)]
pub enum Notation {
    Z0, //[0] Tonic
    Z1, //[1] Semitone
    Z2, //[2] Wholetone
    Z3, //[3] Minor3
    Z4, //[4] Major3
    Z5, //[5] Fourth(prime)
    Z6, //[6] Tritone***
    Z7, //[7] Fifth(prime)
    Z8, //[8] Minor6
    Z9, //[9] Major6
    X,  //[x] Dominant7
    Y,  //[y] Major7&&LeadTone
    A,  //[a] Octave(prime)
    B,  //[b] Minor9(invert[9])
    C,  //[c] Major9(invert[x])
    D,  //[d] Sharp9(invert[9])
    E,  //[e] Tenth(invert[8])
    F,  //[f] Eleventh(invert[7])
    G,  //[g] Sharp11(invert[6])
    H,  //[h] Twelfth(inv[5])
    I,  //[i] Flat13(invert[4])
    J,  //[j] Thirteen(inv[3])
    K,  //[k] Sharp13(inv[2])
    L,  //[l] Flat15 (invert[1])
    M,  //[m] Fifteen && DoubleOctave

    T,   // [t] TwoOctaves + Fifth
    ZM,  //TwoOctaves+Z2 Mystic <Singleton>
    Z64, //[0.0158125] SixtyFourth Tone
    ZT,  //[0.031625] ThirtySecond Tone

    ZS, //[0.06275*]Sixteenth Tone
    ZE, //[0.125*] Eigth Tone
    Q,  //[0.25*] Quartertone
    //Semitone
    ZQ, //[0.750*] Three Quarter Tone
        //Wholetone
}
