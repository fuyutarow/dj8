use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use ghakuf::messages::Message;
use ghakuf::writer::Writer;
use midir::{MidiOutput, MidiOutputPort};

use dj8::abc_parser::parse_notes;
use dj8::note::prelude::*;
use dj8::note::{Note, Pitch};
use dj8::score::Score;

fn main() {
    let input = (0..10).map(|_| "c/").collect::<String>();
    //     let input = r#"
    // c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
    // e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
    //     "#;
    let (input, notes) = parse_notes(&input).unwrap();
    let messages = notes
        .iter()
        .map(|note| note.tempo(4.))
        .map(|note| note.to_messages())
        .flatten()
        .collect::<Vec<Message>>();
    // let score = Score { notes };
    {
        let out_path = "data/example.mid";
        let path = Path::new(&out_path);
        let mut writer = Writer::new();
        writer.running_status(true);
        for message in &messages {
            writer.push(&message);
        }
        writer.write(&path);
    }
}
