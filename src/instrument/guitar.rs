use crate::western_musicbox::WesternMusicBox;
use crate::semitone;

/// Tuning, is a vector of string tonics <f64>
/// Tones wrapping vector is a single string and starting at index 0 gives the tonic and successive semitones[frets]
#[derive(Debug)]
pub struct Guitar{
 
    tuning: Vec<f64>,
    
    tones: Vec<Vec<f64>>,

    }


 impl Guitar {
     ///Uses western music box to get notes, so adjust crate::TONIC for different
     pub fn standard_tuning() -> Self {
          let music = WesternMusicBox::new();
          let e = music.notes.get("E1").unwrap(); //Use of closures here in .map() do anything for me?
          let a = music.notes.get("A2").unwrap();
          let d = music.notes.get("D2").unwrap();
          let g = music.notes.get("G2").unwrap();
          let b = music.notes.get("B3").unwrap();
          let high_e = music.notes.get("E3").unwrap();
          let tuning = vec![*e, *a, *d, *g, *b, *high_e];
          let mut tones = vec![];
          for i in 0..tuning.len(){
               let mut string = vec![];

          for fret in 0..=24 {
      

              string.push(semitone(tuning[i], fret as i64));
                
          }
          tones.push(string);

     }
     Self {
          tuning,
          tones,
     }
     }

     }
 


