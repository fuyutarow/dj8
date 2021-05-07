use cli::note::prelude::*;
use cli::note::{AbcNote, Note};

#[test]
fn note() {
    let note = "C4".parse::<Note>();
    assert_eq!(note, Ok(Note::C4));
    assert_eq!(note.unwrap().to_string(), "C4");

    let note = Note::C4;
    assert_eq!(note.to_i32(), Some(60));

    let note = Note::from_i32(60);
    assert_eq!(note, Some(Note::C4));
}

#[test]
fn abcnote() {
    let note = "C".parse::<AbcNote>();
    assert_eq!(note, Ok(AbcNote::C4));
    assert_eq!(note.unwrap().to_string(), "C");

    let note = AbcNote::C4;
    assert_eq!(note.to_i32(), Some(60));

    let note = AbcNote::from_i32(60);
    assert_eq!(note, Some(AbcNote::C4));
}

#[test]
fn note_and_abcnote() {
    let abcnote = "C".parse::<AbcNote>().unwrap();
    let note = Note::from(abcnote);
    assert_eq!("C4", note.to_string());

    let note = Note::C4;
    assert_eq!("C", note.to_abc());
    assert_eq!("C4", Note::from_abc("C").to_string());
}
