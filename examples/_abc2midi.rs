use std::io::Write;
use std::path::Path;

use ghakuf::messages::Message;
use ghakuf::writer::Writer;

use cli::abc_parser::parse_notes;
// use cli::midi_env::setup_midi_conn_out;

fn main() {
    let input = r#"
c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
    "#;
    let (_input, notes) = parse_notes(input).unwrap();
    let messages = notes
        .iter()
        .map(|note| note.tempo(4.))
        .map(|note| note.to_messages())
        .flatten()
        .collect::<Vec<Message>>();

    dbg!(&messages);

    {
        let path = Path::new("data/example.mid");
        let mut writer = Writer::new();
        writer.running_status(true);
        for message in &messages {
            writer.push(&message);
        }
        let _ = writer.write(&path);
    }
}
