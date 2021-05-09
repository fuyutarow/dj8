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

fn main() -> Result<(), Box<dyn Error>> {
    let mut conn_out = get_conn_out()?;
    let input = "c G3// A// B E/ E/";
    let ss = input.clone().split_whitespace();

    for s in ss {
        let note = Note::from_abc(s).tempo(4. * 150.);
        note.play(&mut conn_out);
    }

    Ok(())
}
