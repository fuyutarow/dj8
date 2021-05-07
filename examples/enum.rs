// use std::collections::HashMap;
// use std::str::FromStr;

// use enum_primitive_derive::*;
// use anyhow::{bail, Result};
use enum_primitive_derive::Primitive;
// use ghakuf::messages::MidiEvent::{NoteOff, NoteOn};
// use ghakuf::messages::{Message, MidiEvent};
// use num_traits::{FromPrimitive, ToPrimitive};
// use parse_display::{Display, FromStr};

use cli::note::prelude::*;
use cli::note::Note;

// #[derive(Primitive)]
#[derive(Debug, PartialEq, Primitive)]
enum Foo {
    Bar = 32,
    Dead = 42,
    Beef = 50,
}

fn main() {
    // assert_eq!(Foo::from_i32(32), Some(Foo::Bar));
    assert_eq!(Foo::from_i32(42), Some(Foo::Dead));
    assert_eq!(Foo::from_i64(50), Some(Foo::Beef));

    let bar = Foo::Bar;
    assert_eq!(bar.to_i32(), Some(32));

    let note = Note::C4;
    assert_eq!(note.to_i32(), Some(60));

    assert_eq!(Note::from_i64(60), Some(Note::C4));
}
