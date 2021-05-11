use std::collections::BinaryHeap;
use std::collections::HashMap;

use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

use super::abc_parser::parse_notes;
use super::note::{Note, Pitch};

pub mod MSG {
    pub const NOTE_ON: u8 = 0x90;
    pub const NOTE_OFF: u8 = 0x80;
    pub const VELOCITY: u8 = 0x64;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Score {
    pub notes: Vec<Note>,
    pub tempo: f64,
}

impl Score {
    pub fn play(&self, conn_out: &mut midir::MidiOutputConnection) {
        for note in self.notes.clone() {
            note.tempo(self.tempo).play(conn_out);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub notes: Vec<Note>,
    pub duration: f64,
}

impl Chord {
    pub fn from_notes(notes: Vec<Note>) -> Self {
        let (_, duration) = &notes.first().unwrap().to_pair();

        Self {
            notes,
            duration: *duration as f64,
        }
    }

    pub fn tempo(&self, t: f64) -> Self {
        Self {
            notes: self.notes.to_owned(),
            duration: self.duration * t,
        }
    }

    pub fn play(&self, conn_out: &mut MidiOutputConnection) {
        for note in &self.notes {
            let (pitch, _duration) = note.to_pair();
            let _ = conn_out.send(&[MSG::NOTE_ON, pitch, MSG::VELOCITY]);
        }
        // std::thread::sleep(std::time::Duration::from_millis(*duration * 4 * 150));
        std::thread::sleep(std::time::Duration::from_millis(self.duration as u64));
        for note in &self.notes {
            let (pitch, _duration) = note.to_pair();
            let _ = conn_out.send(&[MSG::NOTE_OFF, pitch, MSG::VELOCITY]);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Stem {
    Cat(Vec<Stem>),  // tuplet
    Join(Vec<Stem>), // chord
    Note(Note),
}

impl Stem {
    pub fn cat_from_abc(input: &str) -> Self {
        let (input, notes) = parse_notes(input).unwrap();
        let stems = notes
            .into_iter()
            .map(|note| Stem::Note(note))
            .collect::<Vec<_>>();
        Self::Cat(stems)
    }

    pub fn join_from_abc(input: &str) -> Self {
        let (input, notes) = parse_notes(input).unwrap();
        let stems = notes
            .into_iter()
            .map(|note| Stem::Note(note))
            .collect::<Vec<_>>();
        Self::Join(stems)
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stem: Stem,
    pub tempo: f64,
}

#[derive(Debug, Clone)]
pub enum Event {
    On(Pitch),
    Off(Pitch),
}

#[derive(Debug, Clone)]
pub struct Sequence {
    pub events: HashMap<u64, Vec<Event>>,
    pub times: BinaryHeap<u64>,
}

impl Sequence {
    pub fn play<'a>(&'a mut self, stem: Stem, time: &'a mut f64) -> &'a mut f64 {
        match stem {
            Stem::Note(note) => {
                let t = (*time).floor().abs() as u64;
                self.events
                    .entry(t)
                    .or_insert(vec![])
                    .push(Event::On(note.pitch));
                self.times.push(t);

                *time += note.duration;
                let t = (*time).floor().abs() as u64;
                self.events
                    .entry(t)
                    .or_insert(vec![])
                    .push(Event::Off(note.pitch));
                self.times.push(t);
                time
            }
            Stem::Cat(stems) => {
                for stem in &stems {
                    *time = *self.play(stem.clone(), time);
                }
                time
            }
            Stem::Join(stems) => {
                let current_time = time.clone();

                let mut v = Vec::<u64>::new();
                for stem in &stems {
                    let ti = *self.play(stem.clone(), &mut current_time.clone());
                    let t = ti.floor().abs() as u64;
                    v.push(t);
                }

                *time = (*v.iter().max().unwrap()) as f64;
                time
            }
        }
    }
}
