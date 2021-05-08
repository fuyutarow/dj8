use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputPort};

use super::abc_parser::parse_notes;
use super::note::prelude::*;
use super::note::Note;
use super::note::Pitch;

#[derive(Debug, Clone, PartialEq)]
pub struct Score {
    pub notes: Vec<(u8, u64)>,
}

// pub trait MyMidiEvent {
//     fn set(
//         &mut self,
//         major: Option<u64>,
//         minor: Option<u64>,
//         patch: Option<u64>,
//         pre: Option<String>,
//         build: Option<String>,
//     ) -> Self;
//     fn update(&mut self, major: Option<u64>, minor: Option<u64>, patch: Option<u64>) -> Self;
// }
