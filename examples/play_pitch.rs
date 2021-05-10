use std::sync::{mpsc, Arc, Mutex};

use crossbeam_channel::{bounded, select};
use crossbeam_utils::thread;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

use cli::abc_parser::parse_notes;
use cli::chord::Chord;
use cli::midi_env::get_conn_out;
use cli::midi_env::MidiManager;
use cli::note::prelude::*;
use cli::note::{Note, Pitch};
use cli::score::Score;

fn main() {
    let tempo_p4 = 102.;
    let score1 = {
        let input = r#"
        C,, ^C,, D,, ^D,, E,, F,, ^F,, G,, ^G,, A,, ^A,, B,,
        C, ^C, D, ^D, E, F, ^F, G, ^G, A, ^A, B,
        C ^C D ^D E F ^F G ^G A ^A B
        c' ^c' d' ^d' e' f' ^f' g' ^g' a' ^a' b'
        "#;
        // let input = r#"
        // AGFGAAA2GGG2Acc2 AGFGAAAAGGAGF4
        // "#;
        let (input, notes) = parse_notes(input).unwrap();

        dbg!(&notes);

        Score {
            notes,
            tempo: 4. * tempo_p4,
        }
    };

    match get_conn_out() {
        Ok(mut conn_out) => score1.play(&mut conn_out),
        Err(err) => println!("Error: {}", err),
    };

    // let input = "CEG";
    // let (input, notes) = parse_notes(input).unwrap();

    // let mut chord = Chord::from_notes(notes);
    // chord = chord.tempo(4. * 150.);
    // chord.play(conn_out);

    // let mut time = 0;
    // loop {
    //     println!("hello @ {}", time);
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     time += 1
    // }
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
