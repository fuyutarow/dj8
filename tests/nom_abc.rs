use cli::{
    abc_parser::{parse_basenote, parse_duration, parse_note, parse_pitch, parse_space},
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
    let (input, pitch) = parse_basenote(input).unwrap();
    assert_eq!(Pitch::from_abc("c"), pitch);
}

#[test]
fn test_pitch() {
    let input = "A";
    let (input, pitch) = parse_pitch(input).unwrap();
    assert_eq!(Pitch::from_abc("A"), pitch);

    let input = "^C,";
    let (input, pitch) = parse_pitch(input).unwrap();
    assert_eq!(Pitch::from_abc("^C,"), pitch);
}

#[test]
fn test_duration() {
    let input = "2";
    let (input, duration) = parse_duration(input).unwrap();
    assert_eq!(2., duration);

    let input = "/";
    let (input, duration) = parse_duration(input).unwrap();
    assert_eq!(0.5, duration);

    let input = "3/";
    let (input, duration) = parse_duration(input).unwrap();
    assert_eq!(1.5, duration);

    let input = "15//";
    let (input, duration) = parse_duration(input).unwrap();
    assert_eq!(15. / 4., duration);
}

#[test]
fn test_note() {
    let input = "C,//";
    let (input, note) = parse_note(input).unwrap();
    assert_eq!(
        Note {
            pitch: Pitch::C3,
            duration: 1. / 4.,
        },
        note
    );
}
