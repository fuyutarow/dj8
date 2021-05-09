use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time;

use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

use cli::abc_parser::parse_notes;
use cli::midi_env::get_conn_out;
use cli::midi_env::MidiManager;
use cli::note::prelude::*;
use cli::note::{Note, Pitch};
use cli::score::Score;

fn main() {
    let input = r#"
c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
    "#;
    let (input, notes) = parse_notes(input).unwrap();
    let notes = notes
        .iter()
        .map(|note| note.tempo(4.))
        .map(|note| note.to_pair())
        .collect::<Vec<(u8, u64)>>();
    let score = Score { notes };

    thread::spawn(|| play_loop(score));

    let mut time = 0;
    loop {
        println!("hello @ {}", time);
        thread::sleep(time::Duration::from_millis(1000));
        time += 1
    }
}

fn play_loop(score: Score) {
    match get_conn_out() {
        Ok(mut conn_out) => {
            loop {
                score.play(&mut conn_out);
            }
            thread::sleep(time::Duration::from_millis(150));
            conn_out.close();
        }
        Err(err) => println!("Error: {}", err),
    };
}
