use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

use super::abc_parser::parse_notes;
use super::midi_env::get_conn_out;
use super::note::prelude::*;
use super::note::Note;
use super::note::Pitch;

pub mod MSG {
    pub const NOTE_ON_MSG: u8 = 0x90;
    pub const NOTE_OFF_MSG: u8 = 0x80;
    pub const VELOCITY: u8 = 0x64;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Score {
    pub notes: Vec<Note>,
    pub tempo: f64,
}

impl Score {
    pub fn play(&self, conn_out: &mut midir::MidiOutputConnection) {
        for note in self.notes.clone() {
            note.tempo(self.tempo).play(conn_out);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chord {
    pub notes: Vec<Note>,
    pub duration: f64,
}

impl Chord {
    pub fn from_notes(notes: Vec<Note>) -> Self {
        let (_, duration) = &notes.first().unwrap().to_pair();

        Self {
            notes,
            duration: *duration as f64,
        }
    }

    pub fn tempo(&self, t: f64) -> Self {
        Self {
            notes: self.notes.to_owned(),
            duration: self.duration * t,
        }
    }

    pub fn play(&self, conn_out: &mut MidiOutputConnection) {
        for note in &self.notes {
            let (pitch, duration) = note.to_pair();
            let _ = conn_out.send(&[MSG::NOTE_ON_MSG, pitch, MSG::VELOCITY]);
        }
        // std::thread::sleep(std::time::Duration::from_millis(*duration * 4 * 150));
        std::thread::sleep(std::time::Duration::from_millis(self.duration as u64));
        for note in &self.notes {
            let (pitch, duration) = note.to_pair();
            let _ = conn_out.send(&[MSG::NOTE_OFF_MSG, pitch, MSG::VELOCITY]);
        }
    }
}
