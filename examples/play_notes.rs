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
    Note::from_abc("C").play(150, &mut conn_out);
    Note::from_abc("D").play(150, &mut conn_out);
    Note::from_abc("E").play(150, &mut conn_out);
    Note::from_abc("F").play(150, &mut conn_out);
    Note::from_abc("G").play(150, &mut conn_out);
    Ok(())
}
