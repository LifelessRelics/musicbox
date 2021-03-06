use musicbox::notary::Notary;
use musicbox::notation::zoot_allures::Notation;
use musicbox::TONIC;
use std::io::stdin;
use Notation::*;

fn main() {
    let mut io_buffer = String::new();
    println!("Welcome to Musicbox. [0| 123456789XY, A| BCDEFGHIJKL, M| ]");

    println!("Please refer to notation for usage, enter string: \n");
    stdin().read_line(&mut io_buffer);

    // let parse = |buf: String| parse_to_notation(TONIC, buf.to_uppercase().chars().collect());
    let notation = parse_to_notation(io_buffer);

    let scale_construct = notation_to_scales(notation);
    dbg!(&scale_construct);

}



fn parse_to_notation(notation: String) -> Vec<Notation> {
    let mut buffer = vec![];
    for code in notation.to_uppercase().chars() {
        match code {
            '0' => buffer.push(Z0),
            '1' => buffer.push(Z1),
            '2' => buffer.push(Z2),
            '3' => buffer.push(Z3),
            '4' => buffer.push(Z4),
            '5' => buffer.push(Z5),
            '6' => buffer.push(Z6),
            '7' => buffer.push(Z7),
            '8' => buffer.push(Z8),
            '9' => buffer.push(Z9),
            'X' => buffer.push(X),
            'Y' => buffer.push(Y),

            'A' => buffer.push(A), //Octave
            'B' => buffer.push(B),
            'C' => buffer.push(C),
            'D' => buffer.push(D),
            'E' => buffer.push(E),
            'F' => buffer.push(F),
            'G' => buffer.push(G),
            'H' => buffer.push(H),
            'I' => buffer.push(I),
            'J' => buffer.push(J),
            'K' => buffer.push(K),
            'L' => buffer.push(L),
            'M' => buffer.push(M),

            _ => (),
        }
    }
    buffer
}
fn notation_to_scales(extensions: Vec<Notation>) -> Vec<f64> {
    let mut buffer = vec![];
    for music in extensions {
        buffer.push(notation_to_f64_singleton(TONIC, music));
    }
    buffer
}

fn notation_to_f64_singleton(tonic: f64, note: Notation) -> f64 {
    use musicbox::semitone;
    match note {
        Z0 => semitone(tonic, 0),
        Z1 => semitone(tonic, 1),
        Z2 => semitone(tonic, 2),
        Z3 => semitone(tonic, 3),
        Z4 => semitone(tonic, 4),
        Z5 => semitone(tonic, 5),
        Z6 => semitone(tonic, 6),
        Z7 => semitone(tonic, 7),
        Z8 => semitone(tonic, 8),
        Z9 => semitone(tonic, 9),
        X => semitone(tonic, 10),
        Y => semitone(tonic, 11),

        A => semitone(tonic, 12),
        B => semitone(tonic, 13),
        C => semitone(tonic, 14),
        D => semitone(tonic, 15),
        E => semitone(tonic, 16),
        F => semitone(tonic, 17),
        G => semitone(tonic, 18),
        H => semitone(tonic, 19),
        I => semitone(tonic, 20),
        J => semitone(tonic, 21),
        K => semitone(tonic, 22),
        L => semitone(tonic, 23),
        M => semitone(tonic, 24),
        _ => panic!(), // Cover cases, tonight is late.
    }
}
