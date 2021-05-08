use anyhow::bail;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while_m_n},
    character::complete::{digit1, multispace0, multispace1, one_of},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, ParseError, VerboseError},
    multi::{many0, many_m_n},
    number::complete::float,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

use super::note::Note;

pub fn parse_space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

pub fn parse_basenote<'a>(input: &'a str) -> IResult<&'a str, Note> {
    let (input, used) = one_of("CDEFGABcdefgab")(input)?;
    Ok((input, Note::from_abc(&used.to_string())))
}

pub fn parse_tune<'a>(input: &'a str) -> IResult<&'a str, Note> {
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

pub fn parse_duration<'a>(input: &'a str) -> IResult<&'a str, f32> {
    let (input, (number, slashes)) = tuple((many_m_n(0, 1, float), many0(tag("/"))))(input)?;
    let n = number.get(0).unwrap_or(&1.);
    let duration = 2. * n / slashes.len() as f32;
    Ok((input, duration))
}
