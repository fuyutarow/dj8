use std::thread;

use anyhow::{bail, Result};
pub use enum_primitive_derive::Primitive;
use ghakuf::messages::Message;
use ghakuf::messages::Message::MidiEvent;
use ghakuf::messages::MidiEvent::{ControlChange, NoteOff, NoteOn};
pub use num_traits::{FromPrimitive, ToPrimitive};
use parse_display::{Display, FromStr};

use super::abc_parser::parse_note;
use super::score::MSG;

pub mod prelude {
    pub use num_traits::{FromPrimitive, ToPrimitive};
}

#[derive(Debug, Clone, Copy, PartialEq, Display, FromStr, Primitive, Eq, Hash)]
pub enum Pitch {
    // C10 = 0,
    // Cs10 = 1,
    // D10 = 2,
    // Ds10 = 3,
    // E10 = 4,
    // F10 = 5,
    // Fs10 = 6,
    // G10 = 7,
    // Gs10 = 8,
    // A10 = 9,
    // As10 = 10,
    // B10 = 11,
    // C0 = 12,
    // Cs0 = 13,
    // D0 = 14,
    // Ds0 = 15,
    // E0 = 16,
    // F0 = 17,
    // Fs0 = 18,
    // G0 = 19,
    // Gs0 = 20,
    // A0 = 21,
    // As0 = 22,
    // B0 = 23,
    // C1 = 24,
    // Cs1 = 25,
    // D1 = 26,
    // Ds1 = 27,
    // E1 = 28,
    // F1 = 29,
    // Fs1 = 30,
    // G1 = 31,
    // Gs1 = 32,
    // A1 = 33,
    // As1 = 34,
    // B1 = 35,
    C2 = 36,
    Cs2 = 37,
    D2 = 38,
    Ds2 = 39,
    E2 = 40,
    F2 = 41,
    Fs2 = 42,
    G2 = 43,
    Gs2 = 44,
    A2 = 45,
    As2 = 46,
    B2 = 47,
    C3 = 48,
    Cs3 = 49,
    D3 = 50,
    Ds3 = 51,
    E3 = 52,
    F3 = 53,
    Fs3 = 54,
    G3 = 55,
    Gs3 = 56,
    A3 = 57,
    As3 = 58,
    B3 = 59,
    C4 = 60,
    Cs4 = 61,
    D4 = 62,
    Ds4 = 63,
    E4 = 64,
    F4 = 65,
    Fs4 = 66,
    G4 = 67,
    Gs4 = 68,
    A4 = 69,
    As4 = 70,
    B4 = 71,
    C5 = 72,
    Cs5 = 73,
    D5 = 74,
    Ds5 = 75,
    E5 = 76,
    F5 = 77,
    Fs5 = 78,
    G5 = 79,
    Gs5 = 80,
    A5 = 81,
    As5 = 82,
    B5 = 83,
    C6 = 84,
    Cs6 = 85,
    D6 = 86,
    Ds6 = 87,
    E6 = 88,
    F6 = 89,
    Fs6 = 90,
    G6 = 91,
    Gs6 = 92,
    A6 = 93,
    As6 = 94,
    B6 = 95,
    // C7 = 96,
    // Cs7 = 97,
    // D7 = 98,
    // Ds7 = 99,
    // E7 = 100,
    // F7 = 101,
    // Fs7 = 102,
    // G7 = 103,
    // Gs7 = 104,
    // A7 = 105,
    // As7 = 106,
    // B7 = 107,
    // C8 = 108,
    // Cs8 = 109,
    // D8 = 110,
    // Ds8 = 111,
    // E8 = 112,
    // F8 = 113,
    // Fs8 = 114,
    // G8 = 115,
    // Gs8 = 116,
    // A8 = 117,
    // As8 = 118,
    // B8 = 119,
    // C9 = 120,
    // Cs9 = 121,
    // D9 = 122,
    // Ds9 = 123,
    // E9 = 124,
    // F9 = 125,
    // Fs9 = 126,
    // G9 = 127,
}

#[derive(Debug, PartialEq, Display, FromStr, Primitive)]
pub enum AbcPitch {
    // C10 = 0,
    // Cs10 = 1,
    // D10 = 2,
    // Ds10 = 3,
    // E10 = 4,
    // F10 = 5,
    // Fs10 = 6,
    // G10 = 7,
    // Gs10=8,
    // A10 = 9,
    // As10 = 10,
    // B10 =11,
    // C0 = 12,
    // Cs0 = 13,
    // D0 = 14,
    // Ds0 = 15,
    // E0 = 16,
    // F0 = 17,
    // Fs0 = 18,
    // G0 = 19,
    // Gs0=20,
    // A0 = 21,
    // As0 = 22,
    // B0 =23,
    // C1 = 24,
    // Cs1 = 25,
    // D1 = 26,
    // Ds1 = 27,
    // E1 = 28,
    // F1 = 29,
    // Fs1 = 30,
    // G1 = 31,
    // Gs1=32,
    // A1 = 33,
    // As1 = 34,
    // B1 =35,
    #[display("C,,")]
    C2 = 36,
    #[display("^C,,")]
    Cs2 = 37,
    #[display("D,,")]
    D2 = 38,
    #[display("^D,,")]
    Ds2 = 39,
    #[display("E,,")]
    E2 = 40,
    #[display("F,,")]
    F2 = 41,
    #[display("^F,,")]
    Fs2 = 42,
    #[display("G,,")]
    G2 = 43,
    #[display("^G,,")]
    Gs2 = 44,
    #[display("A,,")]
    A2 = 45,
    #[display("^A,,")]
    As2 = 46,
    #[display("B,,")]
    B2 = 47,
    #[display("C,")]
    C3 = 48,
    #[display("^C,")]
    Cs3 = 49,
    #[display("D,")]
    D3 = 50,
    #[display("^D,")]
    Ds3 = 51,
    #[display("E,")]
    E3 = 52,
    #[display("F,")]
    F3 = 53,
    #[display("^F,")]
    Fs3 = 54,
    #[display("G,")]
    G3 = 55,
    #[display("^G,")]
    Gs3 = 56,
    #[display("A,")]
    A3 = 57,
    #[display("^A,")]
    As3 = 58,
    #[display("B,")]
    B3 = 59,
    #[display("C")]
    C4 = 60,
    #[display("^C")]
    Cs4 = 61,
    #[display("D")]
    D4 = 62,
    #[display("^D")]
    Ds4 = 63,
    #[display("E")]
    E4 = 64,
    #[display("F")]
    F4 = 65,
    #[display("^F")]
    Fs4 = 66,
    #[display("G")]
    G4 = 67,
    #[display("^G")]
    Gs4 = 68,
    #[display("A")]
    A4 = 69,
    #[display("^A")]
    As4 = 70,
    #[display("B")]
    B4 = 71,
    #[display("c")]
    C5 = 72,
    #[display("^c")]
    Cs5 = 73,
    #[display("d")]
    D5 = 74,
    #[display("^d")]
    Ds5 = 75,
    #[display("e")]
    E5 = 76,
    #[display("f")]
    F5 = 77,
    #[display("^f")]
    Fs5 = 78,
    #[display("g")]
    G5 = 79,
    #[display("^g")]
    Gs5 = 80,
    #[display("a")]
    A5 = 81,
    #[display("^a")]
    As5 = 82,
    #[display("b")]
    B5 = 83,
    #[display("c'")]
    C6 = 84,
    #[display("^c'")]
    Cs6 = 85,
    #[display("d'")]
    D6 = 86,
    #[display("^d'")]
    Ds6 = 87,
    #[display("e'")]
    E6 = 88,
    #[display("f'")]
    F6 = 89,
    #[display("^f'")]
    Fs6 = 90,
    #[display("g'")]
    G6 = 91,
    #[display("^g'")]
    Gs6 = 92,
    #[display("a'")]
    A6 = 93,
    #[display("^a'")]
    As6 = 94,
    #[display("b'")]
    B6 = 95,
    // C7 = 96,
    // Cs7 = 97,
    // D7 = 98,
    // Ds7 = 99,
    // E7 = 100,
    // F7 = 101,
    // Fs7 = 102,
    // G7 = 103,
    // Gs7=104,
    // A7 = 105,
    // As7 = 106,
    // B7 =95,
    // C8 = 108,
    // Cs8 = 109,
    // D8 = 110,
    // Ds8 = 111,
    // E8 = 112,
    // F8 = 113,
    // Fs8 = 114,
    // G8 = 115,
    // Gs8=116,
    // A8 = 117,
    // As8 = 118,
    // B8 =95,
    // C9 = 120,
    // Cs9 = 121,
    // D9 = 122,
    // Ds9 = 123,
    // E9 = 124,
    // F9 = 125,
    // Fs9 = 126,
    // G9 = 127,
}

impl From<AbcPitch> for Pitch {
    fn from(v: AbcPitch) -> Self {
        let i = v.to_i32().unwrap();
        Self::from_i32(i).unwrap()
    }
}

impl From<Pitch> for AbcPitch {
    fn from(v: Pitch) -> Self {
        let i = v.to_i32().unwrap();
        Self::from_i32(i).unwrap()
    }
}

impl Pitch {
    pub fn from_abc(s: &str) -> Self {
        let abcpitch = s.parse::<AbcPitch>().unwrap();
        Pitch::from(abcpitch)
    }

    pub fn to_abc(&self) -> String {
        AbcPitch::from(self.clone()).to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: f64,
}

impl Note {
    pub fn from_abc(s: &str) -> Self {
        let (_, note) = parse_note(s).unwrap();
        note
    }

    pub fn tempo(&self, t: f64) -> Self {
        Self {
            pitch: self.pitch,
            duration: self.duration * t,
        }
    }

    pub fn to_pair(&self) -> (u8, u64) {
        (self.pitch as u8, self.duration as u64)
    }

    pub fn to_messages(&self) -> Vec<Message> {
        let delta_time = self.duration as u32 * 240 - 1;
        vec![
            MidiEvent {
                delta_time: 1,
                event: NoteOn {
                    ch: 0,
                    note: self.pitch.to_i32().unwrap() as u8,
                    velocity: 80,
                },
            },
            MidiEvent {
                delta_time,
                event: NoteOff {
                    ch: 0,
                    note: self.pitch.to_i32().unwrap() as u8,
                    velocity: 0,
                },
            },
        ]
    }

    // pub fn on(&self, conn_out: &mut midir::MidiOutputConnection) {
    //     let _ = conn_out.send(&[MSG::NOTE_ON, pitch, MSG::VELOCITY]);
    // }

    // pub fn off(&self, conn_out: &mut midir::MidiOutputConnection) {
    //     let _ = conn_out.send(&[MSG::NOTE_OFF, pitch, MSG::VELOCITY]);
    // }

    pub fn play(&self, conn_out: &mut midir::MidiOutputConnection) {
        let mut play_note = |note: u8, duration: u64| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            thread::sleep(std::time::Duration::from_millis(duration));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        play_note(self.pitch as u8, self.duration as u64);
    }
}
