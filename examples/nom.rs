use anyhow::bail;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while_m_n},
    character::complete::{multispace0, multispace1, one_of},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, ParseError, VerboseError},
    multi::{many0, many_m_n},
    number::complete::float,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

use cli::note::Note;

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

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn _main() {
    assert_eq!(
        hex_color("#2F14DF"),
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
fn parse_space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}
fn parse_tune<'a>(input: &'a str) -> IResult<&'a str, Note> {
    let (input, (accidental, basenote, octave)) = tuple((
        many_m_n(
            0,
            1,
            alt((tag("^"), tag("^^"), tag("_"), tag("__"), tag("="))),
        ),
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
        many_m_n(0, 1, alt((tag(","), tag("'")))),
    ))(input)?;

    let a = accidental.get(0).unwrap_or(&"");
    let o = octave.get(0).unwrap_or(&"");
    let tune = format!("{}{}{}", a, basenote, o);
    let note = Note::from_abc(&tune);
    Ok((input, note))
}

fn parse_duration<'a>(input: &'a str) -> IResult<&'a str, f32> {
    let (input, (number, slashes)) = tuple((many_m_n(0, 1, float), many0(tag("/"))))(input)?;
    let n = number.get(0).unwrap_or(&1.);
    let duration = 2. * n / slashes.len() as f32;
    Ok((input, duration))
}

fn main() {
    // fn parse_tune<'a>(input: &'a str) -> IResult<&'a str, (Vec<&str>, &str, &str)> {
    fn parse_duration<'a>(input: &'a str) -> IResult<&'a str, ()> {
        let (input, (number, slashes)) = tuple((many_m_n(0, 1, float), many0(tag("/"))))(input)?;

        let n = number.get(0).unwrap_or(&1.);
        let duration = 2. * n * slashes.len() as f32;
        dbg!(duration);

        // let a = accidental.get(0).unwrap_or(&"");
        // let o = octave.get(0).unwrap_or(&"");
        // let tune = format!("{}{}{}", a, basenote, o);
        // let note = Note::from_abc(&tune);
        Ok((input, ()))
    }

    let input = "3///";
    let (input, note) = parse_duration(input).unwrap();
    dbg!(note);

    let input = "/";
    let (input, note) = parse_duration(input).unwrap();
    dbg!(note);

    let input = "3/";
    let (input, note) = parse_duration(input).unwrap();
    dbg!(note);
    // dbg!(used);
    // assert_eq!(Note::from_abc("c"), note);
}
