use midir::MidiOutputConnection;

use super::abc_parser::parse_notes;
use super::midi_env::get_conn_out;
use super::note::Note;
use super::note::Pitch;

mod MSG {
    pub const NOTE_ON_MSG: u8 = 0x90;
    pub const NOTE_OFF_MSG: u8 = 0x80;
    pub const VELOCITY: u8 = 0x64;
}

pub struct Chord {
    pub notes: Vec<Note>,
}

impl Chord {
    pub fn play(&self, conn_out: &mut MidiOutputConnection) {
        let (_, duration) = &self.notes.first().unwrap().to_pair();

        for note in &self.notes {
            let (pitch, duration) = note.to_pair();
            let _ = conn_out.send(&[MSG::NOTE_ON_MSG, pitch, MSG::VELOCITY]);
        }
        std::thread::sleep(std::time::Duration::from_millis(*duration * 4 * 150));
        for note in &self.notes {
            let (pitch, duration) = note.to_pair();
            let _ = conn_out.send(&[MSG::NOTE_OFF_MSG, pitch, MSG::VELOCITY]);
        }
    }
}
