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

use cli::{
    abc_parser::{parse_basenote, parse_duration, parse_pitch, parse_space, parse_tune},
    note::Note,
};

pub fn parse_notes<'a>(input: &'a str) -> IResult<&'a str, ()> {
    let (input, notes) = many0(parse_basenote)(input)?;
    dbg!(notes);
    Ok((input, ()))
}

fn main() {
    // let input = "C,D//_E3";
    let input = "C,//";
    let r = parse_pitch(input);
    dbg!(r);

    let input = "CDEFGABcdefgab";
    let r = parse_notes(input);
    dbg!(r);
}

// fn abc_notation(input: &str) -> IResult<&str, Color> {
//     let (input, _) = parse_space(input)?;
//     let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

//     Ok((input, Color { red, green, blue }))
// }

// fn _main() {
//     assert_eq!(
//         abc_notation(
//             // c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/
//             r#"
// c G A B E
// "#
//         ),
//         Ok((
//             "",
//             Color {
//                 red: 47,
//                 green: 20,
//                 blue: 223,
//             }
//         ))
//     );
// }
