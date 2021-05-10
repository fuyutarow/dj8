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

pub enum Stem {
    Tuplet(Vec<Note>),
    Chord(Vec<Note>),
}

pub struct Block {
    pub stem: Stem,
    pub tempo: f64,
}

pub fn play_tuplet(notes: Vec<Note>, conn_out: &mut midir::MidiOutputConnection) {
    for note in &notes {
        note.tempo(4. * 150.).play(conn_out);
    }
}

fn play_chord(notes: Vec<Note>, conn_out: &mut MidiOutputConnection) {
    let (_, duration) = &notes.first().unwrap().to_pair();
    for note in &notes {
        let (pitch, duration) = note.to_pair();
        let _ = conn_out.send(&[MSG::NOTE_ON_MSG, pitch, MSG::VELOCITY]);
    }
    std::thread::sleep(std::time::Duration::from_millis(*duration * 4 * 150));
    for note in &notes {
        let (pitch, duration) = note.to_pair();
        let _ = conn_out.send(&[MSG::NOTE_OFF_MSG, pitch, MSG::VELOCITY]);
    }
}

impl Block {
    fn play(&self, conn_out: &mut midir::MidiOutputConnection) {
        match &self.stem {
            Stem::Tuplet(notes) => play_tuplet(notes.to_owned(), conn_out),
            Stem::Chord(notes) => play_chord(notes.to_owned(), conn_out),
        }
    }
}

fn main() {
    let tempo_p4 = 102.;
    let input = r#"
AGFGAAA2GGG2Acc2 AGFGAAAAGGAGF4
"#;
    let (input, notes) = parse_notes(input).unwrap();

    let block = Block {
        stem: Stem::Tuplet(notes),
        tempo: 4. * tempo_p4,
    };
    play(block);

    // play2(vec![score1, score2]);

    // let mut time = 0;
    // loop {
    //     println!("hello @ {}", time);
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     time += 1
    // }
}

fn play(block: Block) {
    match get_conn_out() {
        Ok(mut conn_out) => {
            block.play(&mut conn_out);
        }
        Err(err) => println!("Error: {}", err),
    };
}
