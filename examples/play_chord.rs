use midir::MidiOutputConnection;

use dj8::abc_parser::parse_notes;
use dj8::midi_env::get_conn_out;

use dj8::score::Chord;

fn main() {
    match get_conn_out() {
        Ok(mut conn_out) => play(&mut conn_out),
        Err(err) => println!("{}", err),
    }
}

fn play(conn_out: &mut MidiOutputConnection) {
    let input = "CEG";
    let (_input, notes) = parse_notes(input).unwrap();

    let mut chord = Chord::from_notes(notes);
    chord = chord.tempo(4. * 150.);
    chord.play(conn_out);
}
