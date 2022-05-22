use crate::notary::Notary;
use crate::notation::zoot_allures::Notation;
use Notation::*;
use crate::TONIC;
use std::io::stdin;
use Notation::*;
use crate::Chord;


pub fn fetch_input() -> Vec<Notation> {
    let mut io_buffer = String::new();
    println!("Enter Notation: \n");
    stdin().read_line(&mut io_buffer);
    let notation = parse_to_notation(format!("{}",io_buffer));
    notation
}


pub fn parse_to_notation(notation: String) -> Vec<Notation> {
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

