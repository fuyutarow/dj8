use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use ghakuf::messages::Message;
use ghakuf::writer::Writer;
use midir::{MidiOutput, MidiOutputPort};

use cli::abc_parser::parse_notes;
// use cli::midi_env::setup_midi_conn_out;
use cli::note::prelude::*;
use cli::note::{Note, Pitch};
use cli::score::Score;

fn main() {
    let input = r#"
c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
    "#;
    let (input, notes) = parse_notes(input).unwrap();
    let messages = notes
        .iter()
        .map(|note| note.tempo(4.))
        .map(|note| note.to_messages())
        .flatten()
        .collect::<Vec<Message>>();

    dbg!(&messages);

    {
        let path = Path::new("samples/example.mid");
        let mut writer = Writer::new();
        writer.running_status(true);
        for message in &messages {
            writer.push(&message);
        }
        let _ = writer.write(&path);
    }
}
