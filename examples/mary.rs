use std::sync::{mpsc, Arc, Mutex};

use crossbeam_channel::{bounded, select};
use crossbeam_utils::thread;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

use cli::abc_parser::parse_notes;
use cli::midi_env::get_conn_out;
use cli::midi_env::MidiManager;
use cli::note::prelude::*;
use cli::note::{Note, Pitch};
use cli::score::{Score, MSG};

#[derive(Debug, Clone)]
pub enum Stem {
    Cat(Vec<Stem>),  // tuplet
    Join(Vec<Stem>), // chord
    Note(Note),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stem: Stem,
    pub tempo: f64,
}

// pub fn play_tuplet(notes: Vec<Note>, conn_out: &mut midir::MidiOutputConnection) {
//     for note in &notes {
//         note.tempo(4. * 150.).play(conn_out);
//     }
// }

// fn play_chord(notes: Vec<Note>, conn_out: &mut MidiOutputConnection) {
//     let (_, duration) = &notes.first().unwrap().to_pair();
//     for note in &notes {
//         let (pitch, duration) = note.to_pair();
//         let _ = conn_out.send(&[MSG::NOTE_ON_MSG, pitch, MSG::VELOCITY]);
//     }
//     std::thread::sleep(std::time::Duration::from_millis(*duration * 4 * 150));
//     for note in &notes {
//         let (pitch, duration) = note.to_pair();
//         let _ = conn_out.send(&[MSG::NOTE_OFF_MSG, pitch, MSG::VELOCITY]);
//     }
// }

fn play_on(stem: Stem, conn_out: &mut MidiOutputConnection) {
    match stem {
        Stem::Note(note) => {
            note.on(conn_out);
        }
        Stem::Cat(stems) => {
            for stem in &stems {
                play_on(stem);
            }
        }
        Stem::Join(stems) => {
            for stem in &stems {
                play_on(stem);
            }
            let duration = 4 * 150;
            std::thread::sleep(std::time::Duration::from_millis(duration));
            for stem in &stems {
                play_off(stem);
            }
        }
    }
}

fn play(stem: Stem, conn_out: &mut MidiOutputConnection) {
    match stem {
        Stem::Note(note) => {
            note.on(conn_out);
            let duration = 4 * 150;
            std::thread::sleep(std::time::Duration::from_millis(duration));
            note.off(conn_out);
        }
        Stem::Cat(stems) => {
            for stem in &stems {
                play(stem);
                // play_on(stem);
                // std::thread::sleep(std::time::Duration::from_millis(duration));
                // play_off(stem);
            }
        }
        Stem::Join(stems) => {
            for stem in &stems {
                play_on(stem);
            }
            let duration = 4 * 150;
            std::thread::sleep(std::time::Duration::from_millis(duration));
            for stem in &stems {
                play_off(stem);
            }
        }
    }
}

fn play_on(stem: Stem, conn_out: &mut MidiOutputConnection) {
    match stem {
        Stem::Note(note) => {
            note.on(conn_out);
            let duration = 4 * 150;
            std::thread::sleep(std::time::Duration::from_millis(duration));
            note.off(conn_out);
        }
        Stem::Cat(stems) => {
            for stem in &stems {
                play_on(stem);
                play_off(stem);
            }
        }
        Stem::Join(stems) => {
            for stem in &stems {
                play_on(stem);
            }
            let duration = 4 * 150;
            std::thread::sleep(std::time::Duration::from_millis(duration));
            for stem in &stems {
                play_off(stem);
            }
        }
    }
}

impl Block {
    fn play(&self, conn_out: &mut midir::MidiOutputConnection) {
        // match &self.stem {
        //     Stem::Tuplet(stems) => {
        //         play_tuplet(notes.to_owned(), conn_out);
        //     }
        //     Stem::Chord(stems) => {
        //         play_tuplet(notes.to_owned(), conn_out);
        //     }
        //     Stem::Note(note) => play_chord(vec, conn_out: &mut MidiOutputConnection),
        // }
    }
}

fn main() {
    let tempo_p4 = 102.;
    let melody = {
        // AGFGAAA2GGG2Acc2 AGFGAAAAGGAGF4
        let input = r#"AGFGAAA2GGG2"#;
        let (input, notes) = parse_notes(input).unwrap();
        Stem::Tuplet(
            notes
                .into_iter()
                .map(|note| Stem::Note(note))
                .collect::<Vec<Stem>>(),
        )
    };

    let f_major = {
        let input = r#"FAc"#;
        let (input, notes) = parse_notes(input).unwrap();
        Stem::Chord(
            notes
                .into_iter()
                .map(|note| Stem::Note(note))
                .collect::<Vec<Stem>>(),
        )
    };

    let c_major = {
        let input = r#"CEG"#;
        let (input, notes) = parse_notes(input).unwrap();
        Stem::Chord(
            notes
                .into_iter()
                .map(|note| Stem::Note(note))
                .collect::<Vec<Stem>>(),
        )
    };

    let sub = Stem::Tuplet(vec![f_major.clone(), f_major.clone(), c_major]);

    dbg!(sub);

    let block = Block {
        stem: melody,
        tempo: 4. * tempo_p4,
    };
    dbg!(block);

    // let block = Block {
    //     stem: Stem::Chord(notes),
    //     tempo: 4. * tempo_p4,
    // };
    // play(block);
}

fn play(block: Block) {
    match get_conn_out() {
        Ok(mut conn_out) => {
            block.play(&mut conn_out);
        }
        Err(err) => println!("Error: {}", err),
    };
}
