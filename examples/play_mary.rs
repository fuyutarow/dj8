use crossbeam_channel::{bounded, select};
use crossbeam_utils::thread;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use priority_queue::PriorityQueue;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use dj8::abc_parser::parse_note;
use dj8::abc_parser::parse_notes;
use dj8::midi_env::get_conn_out;
use dj8::midi_env::MidiManager;
use dj8::note::prelude::*;
use dj8::note::{Note, Pitch};
use dj8::score::MSG;
use dj8::score::{Block, Event, Sequence, Stem};

fn main() {
    let c_major = Stem::join_from_abc("C,4E,4G,4");
    let f_major = Stem::join_from_abc("F,4A,4C4");

    let block = Block {
        tempo: 1000.,
        stem: Stem::Cat(vec![
            Stem::Join(vec![Stem::cat_from_abc("AGFG"), f_major.clone()]),
            Stem::Join(vec![Stem::cat_from_abc("AAA2"), f_major.clone()]),
            Stem::Join(vec![Stem::cat_from_abc("GGG2"), c_major.clone()]),
            Stem::Join(vec![Stem::cat_from_abc("Acc2"), f_major.clone()]),
            Stem::Join(vec![Stem::cat_from_abc("AGFG"), f_major.clone()]),
            Stem::Join(vec![Stem::cat_from_abc("AAAA"), f_major.clone()]),
            Stem::Join(vec![Stem::cat_from_abc("GGAG"), c_major.clone()]),
            Stem::Join(vec![Stem::cat_from_abc("F4"), f_major.clone()]),
        ]),
    };
    let mut score = Sequence {
        events: HashMap::<u64, Vec<Event>>::new(),
        times: BinaryHeap::<u64>::new(),
        // tempo: 4 * 150,
    };

    let mut time = 0.;

    score.play(block.stem, &mut time);

    let mut events_block = score.events.into_iter().collect::<Vec<_>>();
    events_block.sort_by(|x, y| x.0.cmp(&y.0));
    play_block(events_block);
}

fn play_block(events_block: Vec<(u64, Vec<Event>)>) {
    match get_conn_out() {
        Ok(mut conn_out) => {
            let mut time = 0;
            let mut s = 0;

            for (t, events) in events_block {
                let duration = t - s;

                for event in events {
                    match event {
                        Event::On(pitch) => {
                            let p = pitch.to_u8().unwrap();
                            let _ = conn_out.send(&[MSG::NOTE_ON, p, MSG::VELOCITY]);
                        }
                        Event::Off(pitch) => {
                            let p = pitch.to_u8().unwrap();
                            let _ = conn_out.send(&[MSG::NOTE_OFF, p, MSG::VELOCITY]);
                        }
                    }
                }
                let d = duration * 150 * 4;
                std::thread::sleep(std::time::Duration::from_millis(d));

                s = t;
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}
