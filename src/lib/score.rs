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

impl Score {
    pub fn play(&self, conn_out: &mut midir::MidiOutputConnection) {
        let mut play_note = |note: u8, duration: u64| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            // We're ignoring errors in here
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            sleep(Duration::from_millis(duration * 150));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        sleep(Duration::from_millis(4 * 150));

        for note in self.notes.clone() {
            play_note(note.0, note.1);
        }
    }
}
