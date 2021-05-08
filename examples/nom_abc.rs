use cli::{
    abc_parser::{parse_basenote, parse_duration, parse_space, parse_tune},
    note::{prelude::*, Note},
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
fn digit1_test() {
    let s = "63abc";
    let result: IResult<&str, &str> = digit1(&s);
    let (no_used, used) = result.unwrap();
    assert_eq!("63", used);
    assert_eq!("abc", no_used);
}

#[test]
fn test_basenotes() {
    let mut notes = vec![];
    fn parse_basenotes(input: &str) -> IResult<&str, Color> {
        while input.len() > 0 {
            let (input, BaseNote): IResult<&str, &str> = one_of("CDEFGABcdefgab")(&input)?;
            let note = Note::from_abc(used);
            notes.push(note);
        }
    }
    assert_eq!("abc", notes);
}

#[test]
fn test_basenote() {
    let s = "c G A B E";
    let result: IResult<&str, &str> = one_of("CDEFGABcdefgab")(&s);
    let (s, used) = result.unwrap();
    assert_eq!("c", used);
    let result: IResult<&str, &str> = one_of("CDEFGABcdefgab")(&s);
    let (no_used, used) = result.unwrap();
    assert_eq!("G", used);
}

fn main() {
    assert_eq!(
        abc_notation(
            // c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/
            r#"
c G A B E
"#
        ),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}
