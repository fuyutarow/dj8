use dj8::note::prelude::*;
use dj8::note::{AbcPitch, Pitch};

#[test]
fn parse_pitch() {
    let note = "C4".parse::<Pitch>();
    assert_eq!(note, Ok(Pitch::C4));
    assert_eq!(note.unwrap().to_string(), "C4");
}

#[test]
fn pitch_midi_number() {
    let note = Pitch::C4;
    assert_eq!(note.to_i32(), Some(60));

    let note = Pitch::from_i32(60);
    assert_eq!(note, Some(Pitch::C4));
}

#[test]
fn pitch_freq() {
    assert_eq!(880., (Pitch::A5.to_freq() * 1000.).round() / 1000.);
    assert_eq!(440., (Pitch::A4.to_freq() * 1000.).round() / 1000.);
    assert_eq!(261.626, (Pitch::C4.to_freq() * 1000.).round() / 1000.);
    assert_eq!(220., (Pitch::A3.to_freq() * 1000.).round() / 1000.);
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
