use std::sync::{mpsc, Arc, Mutex};

use crossbeam_channel::{bounded, select};
use crossbeam_utils::thread;
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

use dj8::abc_parser::parse_notes;
use dj8::midi_env::get_conn_out;
use dj8::midi_env::MidiManager;
use dj8::note::prelude::*;
use dj8::note::{Note, Pitch};
use dj8::score::Score;

fn _main() -> anyhow::Result<()> {
    let (sender, receiver) = bounded(1);

    let mut conn_out = get_conn_out()?;

    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        thread::scope(|scope| {
            let input = input.clone();
            let (sender, receiver) = (sender.clone(), receiver.clone());
            std::thread::spawn(move || {
                let (input, notes) = parse_notes(input).unwrap();
                for note in notes {
                    note.play(&mut conn_out)
                }
            });
        })
        .unwrap();

        // let mut s = String::from("c");
        // sender.send(&s).unwrap();
        // // println!("{}", input);
    }

    let mut time = 0;
    loop {
        println!("hello @ {}", time);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        time += 1
    }

    Ok(())
}
