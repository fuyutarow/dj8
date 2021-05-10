use midir::MidiOutputConnection;

use cli::abc_parser::parse_notes;
use cli::chord::Chord;
use cli::midi_env::get_conn_out;
use cli::note::Note;
use cli::note::Pitch;

fn main() {
    match get_conn_out() {
        Ok(mut conn_out) => play(&mut conn_out),
        Err(err) => println!("{}", err),
    }
}

fn play(conn_out: &mut MidiOutputConnection) {
    let input = "CEG";
    let (input, notes) = parse_notes(input).unwrap();

    let chord = Chord { notes };
    chord.play(conn_out);
}
