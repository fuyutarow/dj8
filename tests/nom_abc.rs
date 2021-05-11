use cli::{
    abc_parser::{parse_basenote, parse_duration, parse_note, parse_notes, parse_pitch},
    note::Pitch::*,
    note::{Note, Pitch},
};

pub enum BasePitch {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
    c,
    d,
    e,
    f,
    g,
    a,
    b,
}

#[test]
fn test_basenote() {
    let input = "c G A B E";
    let (_input, pitch) = parse_basenote(input).unwrap();
    assert_eq!(Pitch::from_abc("c"), pitch);
}

#[test]
fn test_pitch() {
    let input = "A";
    let (_input, pitch) = parse_pitch(input).unwrap();
    assert_eq!(Pitch::from_abc("A"), pitch);

    let input = "^C,";
    let (_input, pitch) = parse_pitch(input).unwrap();
    assert_eq!(Pitch::from_abc("^C,"), pitch);
}

#[test]
fn test_duration() {
    let input = "2";
    let (_input, duration) = parse_duration(input).unwrap();
    assert_eq!(2., duration);

    let input = "/";
    let (_input, duration) = parse_duration(input).unwrap();
    assert_eq!(0.5, duration);

    let input = "3/";
    let (_input, duration) = parse_duration(input).unwrap();
    assert_eq!(1.5, duration);

    let input = "15//";
    let (_input, duration) = parse_duration(input).unwrap();
    assert_eq!(15. / 4., duration);
}

#[test]
fn test_note() {
    let input = "C,//";
    let (_input, note) = parse_note(input).unwrap();
    assert_eq!(
        Note {
            pitch: Pitch::C3,
            duration: 1. / 4.,
        },
        note
    );
}

#[test]
fn test_notes() {
    let input = "  C,//A3   D";
    let (_input, notes) = parse_notes(input).unwrap();
    assert_eq!(
        vec![
            Note {
                pitch: Pitch::C3,
                duration: 0.25,
            },
            Note {
                pitch: Pitch::A4,
                duration: 3.0,
            },
            Note {
                pitch: Pitch::D4,
                duration: 1.0,
            },
        ],
        notes,
    );

    let input = r##"
c G3// A// B E/ E/ |\
A G3// F// G C/ C/ |]
"##;
    let (_input, notes) = parse_notes(input).unwrap();
    assert_eq!(
        vec![
            Note {
                pitch: C5,
                duration: 1.0
            },
            Note {
                pitch: G4,
                duration: 0.75
            },
            Note {
                pitch: A4,
                duration: 0.25
            },
            Note {
                pitch: B4,
                duration: 1.0
            },
            Note {
                pitch: E4,
                duration: 0.5
            },
            Note {
                pitch: E4,
                duration: 0.5
            },
            Note {
                pitch: A4,
                duration: 1.0
            },
            Note {
                pitch: G4,
                duration: 0.75
            },
            Note {
                pitch: F4,
                duration: 0.25
            },
            Note {
                pitch: G4,
                duration: 1.0
            },
            Note {
                pitch: C4,
                duration: 0.5
            },
            Note {
                pitch: C4,
                duration: 0.5
            }
        ],
        notes
    );

    let input = "D4C";
    let (_input, notes) = parse_notes(input).unwrap();
    assert_eq!(
        vec![
            Note {
                pitch: Pitch::D4,
                duration: 4.,
            },
            Note {
                pitch: Pitch::C4,
                duration: 1.,
            },
        ],
        notes,
    );
}

#[test]
fn test_parse_as_an_integer_not_a_float() {
    let input = "C4e2";
    let (_input, notes) = parse_notes(input).unwrap();
    assert_eq!(
        vec![
            Note {
                pitch: Pitch::C4,
                duration: 4.,
            },
            Note {
                pitch: Pitch::E5,
                duration: 2.,
            },
        ],
        notes,
    );
}
