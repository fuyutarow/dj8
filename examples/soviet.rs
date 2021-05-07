use ghakuf::messages::MidiEvent::{NoteOff, NoteOn};
use ghakuf::messages::{Message, MidiEvent};

fn main() {
    let abc = r##"
c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
"##;
    for note in abc.split_whitespace() {
        dbg!(note);
    }
}
