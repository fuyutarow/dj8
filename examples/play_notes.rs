use std::error::Error;

use cli::midi_env::get_conn_out;

use cli::note::Note;

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
