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
    pub notes: Vec<Note>,
    pub tempo: u64,
}

impl Score {
    pub fn play(&self, conn_out: &mut midir::MidiOutputConnection) {
        for note in self.notes.clone() {
            note.play(self.tempo, conn_out);
        }
    }
}
