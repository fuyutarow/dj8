use cli::note::prelude::*;
use cli::note::{AbcNote, Note};

#[test]
fn note_and_str() {
    let note = "c4".parse::<Note>();
    assert_eq!(note, Ok(Note::C4));
    assert_eq!(note.unwrap().to_string(), "c4");

    let note = Note::C4;
    assert_eq!(note.to_i32(), Some(60));
}

// #[test]
// fn abcnote_and_str() {
//     let note = "C".parse::<AbcNote>();
//     assert_eq!(note, Ok(AbcNote::C4));
// }

// #[test]
// fn notec4() {
//     let note = Note::C4;
//     assert_eq!(note.to_i32(), Some(60));
// }
