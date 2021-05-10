use std::sync::{mpsc, Arc, Mutex};

use crossbeam_channel::{bounded, select};
use crossbeam_utils::thread;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

use cli::abc_parser::parse_notes;
use cli::midi_env::get_conn_out;
use cli::midi_env::MidiManager;
use cli::note::prelude::*;
use cli::note::{Note, Pitch};
use cli::score::Score;

fn main() {
    let score1 = {
        let input = r#"
G3/E C E G c2 e3/d c E ^F G2 GG e3/ d c B2 A B c c G E C
"#;
        let (input, notes) = parse_notes(input).unwrap();
        Score {
            notes,
            tempo: 4. * 150.,
        }
    };

    let score2 = {
        let input = r#"
c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
"#;
        let (input, notes) = parse_notes(input).unwrap();
        Score {
            notes,
            tempo: 4. * 150.,
        }
    };

    play2(vec![score1, score2]);

    let mut time = 0;
    loop {
        println!("hello @ {}", time);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        time += 1
    }
}

fn play2(scores: Vec<Score>) {
    match get_conn_out() {
        Ok(mut conn_out) => {
            thread::scope(|scope| {
                for score in scores {
                    std::thread::spawn(move || match get_conn_out() {
                        Ok(mut conn_out) => loop {
                            score.play(&mut conn_out);
                        },
                        Err(err) => println!("Error: {}", err),
                    });
                }
            })
            .unwrap();
        }
        Err(err) => println!("Error: {}", err),
    };
}
