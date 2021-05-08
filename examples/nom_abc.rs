use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

use cli::note::{prelude::*, Note};

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

// #[test]
// fn tune1() {
//     let op_kind_parser = map(alt((char('*'), char('/'))), |op_char| match op_char {
//         '*' => OpKind::Mul,
//         '/' => OpKind::Div,
//         _ => panic!("error!"),
//     });
// }

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn abc_notation(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
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
