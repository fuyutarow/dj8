use crossbeam_channel::{bounded, select};
use crossbeam_utils::thread;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use priority_queue::PriorityQueue;

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
    pub events: PriorityQueue<Event, u64>,
    // pub time: u64,
}

impl Score {
    fn play<'a>(&'a mut self, stem: Stem, time: &'a mut u64) -> &'a mut u64 {
        match stem {
            Stem::Note(note) => {
                // self.events.push(Event::On(note.pitch), time.clone());
                self.events.push(Event::On(note.pitch), *time);
                dbg!(&time, Event::On(note.pitch));
                *time += note.duration;
                // self.events.push(Event::Off(note.pitch), time.clone());
                self.events.push(Event::Off(note.pitch), *time);
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

                for stem in &stems {
                    let mut time = *self.play(stem.clone(), &mut current_time.clone());
                }

                time
            }
        }
    }

    // fn _stem_on<'a>(&'a mut self, stem: Stem, time: &'a mut u64) -> &'a mut u64 {
    //     match stem {
    //         Stem::Note(note) => {
    //             self.events.push(Event::On(note.pitch), time.clone());
    //             dbg!(&time, Event::On(note.pitch));
    //             *time += note.duration;
    //             time
    //         }
    //         Stem::Cat(stems) => {
    //             for stem in &stems {
    //                 *time = *self.stem_on(stem.clone(), time);
    //                 *time = *self.stem_off(stem.to_owned(), time);
    //             }
    //             time
    //         }
    //         Stem::Join(stems) => {
    //             for stem in &stems {
    //                 let mut time = *self.stem_on(stem.clone(), &mut time.clone());
    //                 let _ = *self.stem_off(stem.clone(), &mut time);
    //             }

    //             // let mut v = vec![];
    //             // for stem in &stems {
    //             //     let t = *self.stem_off(stem.to_owned(), time);
    //             //     v.push(t);
    //             // }
    //             // *time += v.iter().max().unwrap();
    //             time
    //         }
    //     }
    // }

    // fn stem_off<'a>(&'a mut self, stem: Stem, time: &'a mut u64) -> &'a mut u64 {
    //     match stem {
    //         Stem::Note(note) => {
    //             self.events.push(Event::Off(note.pitch), time.clone());
    //             dbg!(&time, Event::Off(note.pitch));
    //             time
    //         }
    //         Stem::Cat(stems) => {
    //             for stem in &stems {
    //                 *time += *self.stem_off(stem.clone(), time);
    //             }
    //             time
    //         }
    //         Stem::Join(stems) => {
    //             let mut v = vec![];
    //             for stem in &stems {
    //                 let t = *self.stem_off(stem.clone(), time);
    //                 v.push(t);
    //             }
    //             *time += v.iter().max().unwrap();
    //             time
    //         }
    //     }
    // }
}

fn main() {
    let block = Block {
        tempo: 1000,
        stem: Stem::Join(vec![
            Stem::Cat(vec![
                Stem::Note(Note {
                    pitch: Pitch::A4,
                    duration: 5,
                }),
                Stem::Note(Note {
                    pitch: Pitch::G4,
                    duration: 5,
                }),
            ]),
            Stem::Note(Note {
                pitch: Pitch::E4,
                duration: 10,
            }),
        ]),
    };

    let mut score = Score {
        events: PriorityQueue::<Event, u64>::new(),
    };

    let mut time = 0;

    score.play(block.stem, &mut time);

    dbg!(score);
}
