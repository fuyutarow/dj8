use cli::{
    abc_parser::{parse_basenote, parse_duration, parse_space, parse_tune},
    note::Note,
};

pub enum BaseNote {
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
    let (input, note) = parse_basenote(input).unwrap();
    assert_eq!(Note::from_abc("c"), note);
}

#[test]
fn test_tune() {
    let input = "A";
    let (input, note) = parse_tune(input).unwrap();
    assert_eq!(Note::from_abc("A"), note);

    let input = "^C,";
    let (input, note) = parse_tune(input).unwrap();
    assert_eq!(Note::from_abc("^C,"), note);
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

// #[test]
// fn test_pitch() {
//     let input = "D,//";
//     let (input, duration) = parse_duration(input).unwrap();
//     assert_eq!(2., duration);

//     let input = "3/";
//     let (input, duration) = parse_duration(input).unwrap();
//     assert_eq!(6., duration);

//     let input = "15//";
//     let (input, duration) = parse_duration(input).unwrap();
//     assert_eq!(15., duration);
// }
