use cli::note::prelude::*;
use cli::note::{AbcPitch, Pitch};

#[test]
fn note() {
    let note = "C4".parse::<Pitch>();
    assert_eq!(note, Ok(Pitch::C4));
    assert_eq!(note.unwrap().to_string(), "C4");

    let note = Pitch::C4;
    assert_eq!(note.to_i32(), Some(60));

    let note = Pitch::from_i32(60);
    assert_eq!(note, Some(Pitch::C4));
}

#[test]
fn abcnote() {
    let note = "C".parse::<AbcPitch>();
    assert_eq!(note, Ok(AbcPitch::C4));
    assert_eq!(note.unwrap().to_string(), "C");

    let note = AbcPitch::C4;
    assert_eq!(note.to_i32(), Some(60));

    let note = AbcPitch::from_i32(60);
    assert_eq!(note, Some(AbcPitch::C4));
}

#[test]
fn note_and_abcnote() {
    let abcnote = "C".parse::<AbcPitch>().unwrap();
    let note = Pitch::from(abcnote);
    assert_eq!("C4", note.to_string());

    let note = Pitch::C4;
    assert_eq!("C", note.to_abc());
    assert_eq!("C4", Pitch::from_abc("C").to_string());
}
