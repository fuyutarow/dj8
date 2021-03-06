use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while_m_n},
    character::complete::{digit0, digit1, multispace0, multispace1, one_of},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, ParseError, VerboseError},
    multi::{many0, many_m_n},
    number::complete::float,
    number::complete::i64 as parse_i64,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

use super::note::{Note, Pitch};

pub fn parse_space<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

pub fn parse_basenote<'a>(input: &'a str) -> IResult<&'a str, Pitch> {
    let (input, used) = one_of("CDEFGABcdefgab")(input)?;
    Ok((input, Pitch::from_abc(&used.to_string())))
}

pub fn parse_pitch<'a>(input: &'a str) -> IResult<&'a str, Pitch> {
    let (input, (accidental, basenote, octave)) = tuple((
        many_m_n(
            0,
            1,
            alt((tag("^^"), tag("^"), tag("__"), tag("_"), tag("="))),
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
        many_m_n(0, 1, alt((tag(",,"), tag(","), tag("''"), tag("'")))),
    ))(input)?;

    let a = accidental.get(0).unwrap_or(&"");
    let o = octave.get(0).unwrap_or(&"");
    let pitch = format!("{}{}{}", a, basenote, o);
    let p = Pitch::from_abc(&pitch);
    Ok((input, p))
}

pub fn parse_duration<'a>(input: &'a str) -> IResult<&'a str, f64> {
    // let (input, (number, slashes)) = tuple((many_m_n(0, 1, float), many0(tag("/"))))(input)?;

    let parser_integer = map_res(digit0, |s: &str| s.parse::<u64>());
    let (input, (number, slashes)) =
        tuple((many_m_n(0, 1, parser_integer), many0(tag("/"))))(input)?;
    let n = number.get(0).unwrap_or(&1).to_owned() as f64;
    let l = (1 << slashes.len()) as f64;
    let duration = n / l;
    Ok((input, duration as f64))
}

pub fn parse_note<'a>(input: &'a str) -> IResult<&'a str, Note> {
    let (input, (pitch, duration)) = tuple((parse_pitch, parse_duration))(input)?;
    let note = Note { pitch, duration };
    Ok((input, note))
}

pub fn parse_skip_char<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n|\\";
    take_while(move |c| chars.contains(c))(i)
}

pub fn parse_notes<'a>(input: &'a str) -> IResult<&'a str, Vec<Note>> {
    let (input, notes) = many0(preceded(parse_skip_char, parse_note))(input)?;
    Ok((input, notes))
}
