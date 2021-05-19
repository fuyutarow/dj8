use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

use dj8::abc_parser::parse_notes;
use dj8::midi_env::get_conn_out;
use dj8::midi_env::MidiManager;
use dj8::note::prelude::*;
use dj8::note::{Note, Pitch};
use dj8::score::Score;

fn main() {
    // std::thread::spawn(|| play_loop(score));

    play_loop();
}

fn play_loop() {
    let mut tempo_p4 = 150.;
    match get_conn_out() {
        Ok(mut conn_out) => {
            std::thread::sleep(std::time::Duration::from_millis(4 * 150));
            loop {
                println!("> ");
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                match input.trim() {
                    s if s.starts_with("Q=") => {
                        tempo_p4 = s
                            .trim_start_matches("Q=")
                            .parse::<f64>()
                            .expect("unsigned interger");
                        println!("tempo={}", tempo_p4 * 4.);
                    }
                    s => {
                        let (_, notes) = parse_notes(&s).unwrap();
                        for note in notes {
                            note.tempo(tempo_p4 * 4.).play(&mut conn_out);
                        }
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(4 * 150));
            conn_out.close();
        }
        Err(err) => println!("Error: {}", err),
    };
}
