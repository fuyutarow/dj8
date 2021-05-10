use crossbeam_channel::{bounded, select};
use crossbeam_utils::thread;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use priority_queue::PriorityQueue;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use cli::abc_parser::parse_notes;
use cli::midi_env::get_conn_out;
use cli::midi_env::MidiManager;
use cli::note::prelude::*;
use cli::note::Pitch;
// use cli::note::{Note, Pitch};
// use cli::score::{Score, MSG};

#[derive(Debug, Clone)]
pub struct Note {
    pitch: Pitch,
    duration: u64,
}

#[derive(Debug, Clone)]
pub enum Stem {
    Cat(Vec<Stem>),  // tuplet
    Join(Vec<Stem>), // chord
    Note(Note),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stem: Stem,
    pub tempo: u64,
}

#[derive(Debug, Clone, Hash)]
pub enum Event {
    On(Pitch),
    Off(Pitch),
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Event::On(pitch1), Event::On(pitch2)) => pitch1 == pitch2,
            (Event::Off(pitch1), Event::Off(pitch2)) => pitch1 == pitch2,
            _ => false,
        }
    }
}
impl Eq for Event {}

#[derive(Debug, Clone)]
struct Score {
    pub events: HashMap<u64, Vec<Event>>,
    pub times: BinaryHeap<u64>,
}

impl Score {
    fn play<'a>(&'a mut self, stem: Stem, time: &'a mut u64) -> &'a mut u64 {
        match stem {
            Stem::Note(note) => {
                self.events
                    .entry(*time)
                    .or_insert(vec![])
                    .push(Event::On(note.pitch));
                self.times.push(*time);
                dbg!(&time, Event::On(note.pitch));
                *time += note.duration;
                self.events
                    .entry(*time)
                    .or_insert(vec![])
                    .push(Event::Off(note.pitch));
                self.times.push(*time);
                dbg!(&time, Event::Off(note.pitch));
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

                let mut v = vec![];
                for stem in &stems {
                    let t = *self.play(stem.clone(), &mut current_time.clone());
                    v.push(t);
                }

                *time = *v.iter().max().unwrap();
                time
            }
        }
    }
}

fn main() {
    let f_major = Stem::Join(vec![
        Stem::Note(Note {
            pitch: Pitch::F3,
            duration: 20,
        }),
        Stem::Note(Note {
            pitch: Pitch::A3,
            duration: 20,
        }),
        Stem::Note(Note {
            pitch: Pitch::C4,
            duration: 20,
        }),
    ]);

    let c_major = Stem::Join(vec![
        Stem::Note(Note {
            pitch: Pitch::C3,
            duration: 20,
        }),
        Stem::Note(Note {
            pitch: Pitch::F3,
            duration: 20,
        }),
        Stem::Note(Note {
            pitch: Pitch::G3,
            duration: 20,
        }),
    ]);

    let block = Block {
        tempo: 1000,
        stem: Stem::Cat(vec![
            Stem::Join(vec![
                Stem::Cat(vec![
                    Stem::Note(Note {
                        pitch: Pitch::A4,
                        duration: 5,
                    }),
                    Stem::Note(Note {
                        pitch: Pitch::G4,
                        duration: 5,
                    }),
                    Stem::Note(Note {
                        pitch: Pitch::F4,
                        duration: 5,
                    }),
                    Stem::Note(Note {
                        pitch: Pitch::G4,
                        duration: 5,
                    }),
                ]),
                f_major.clone(),
            ]),
            Stem::Join(vec![
                Stem::Cat(vec![
                    Stem::Note(Note {
                        pitch: Pitch::A4,
                        duration: 5,
                    }),
                    Stem::Note(Note {
                        pitch: Pitch::A4,
                        duration: 5,
                    }),
                    Stem::Note(Note {
                        pitch: Pitch::A4,
                        duration: 10,
                    }),
                ]),
                f_major.clone(),
            ]),
            Stem::Join(vec![
                Stem::Cat(vec![
                    Stem::Note(Note {
                        pitch: Pitch::G4,
                        duration: 5,
                    }),
                    Stem::Note(Note {
                        pitch: Pitch::G4,
                        duration: 5,
                    }),
                    Stem::Note(Note {
                        pitch: Pitch::G4,
                        duration: 10,
                    }),
                ]),
                c_major.clone(),
            ]),
        ]),
    };

    let mut score = Score {
        events: HashMap::<u64, Vec<Event>>::new(),
        times: BinaryHeap::<u64>::new(),
    };

    let mut time = 0;

    score.play(block.stem, &mut time);

    let mut map = score.events;
    let mut v: Vec<_> = map.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));
    for (t, events) in v {
        dbg!(t, events);
    }
}
