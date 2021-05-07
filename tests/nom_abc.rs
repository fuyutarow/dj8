use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while_m_n},
    character::complete::{alpha1, char, digit1, multispace0, multispace1, one_of},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, ParseError, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
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

fn parse_space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

#[test]
fn test_basenote() {
    fn parse_basenote<'a>(input: &'a str) -> IResult<&'a str, Note> {
        let (input, used) = one_of("CDEFGABcdefgab")(input)?;
        Ok((input, Note::from_abc(&used.to_string())))
    }
    let input = "c G A B E";
    let (input, note) = parse_basenote(input).unwrap();
    assert_eq!(Note::from_abc("c"), note);
}

#[test]
fn test_basenotes() {
    fn parse_basenote<'a>(input: &'a str) -> IResult<&'a str, Note> {
        let (input, used) = preceded(
            parse_space,
            alt((
                tag("C"),
                tag("D"),
                tag("E"),
                tag("F"),
                tag("G"),
                tag("A"),
                tag("B"),
                tag("c"),
                tag("d"),
                tag("e"),
                tag("f"),
                tag("g"),
                tag("a"),
                tag("b"),
            )),
        )(input)?;
        Ok((input, Note::from_abc(&used.to_string())))
    }
    let input = "c G  A B E";
    let (input, note) = parse_basenote(input).unwrap();
    assert_eq!(Note::from_abc("c"), note);

    let (input, note) = parse_basenote(input).unwrap();
    assert_eq!(Note::from_abc("G"), note);

    let (input, note) = parse_basenote(input).unwrap();
    assert_eq!(Note::from_abc("A"), note);
}
