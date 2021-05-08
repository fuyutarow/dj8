use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_while, take_while_m_n},
    character::complete::{digit1, multispace0, multispace1, one_of},
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt},
    error::{context, ParseError, VerboseError},
    multi::{many0, many_m_n, separated_list0},
    number::complete::float,
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

use cli::abc_parser::{parse_basenote, parse_duration, parse_note, parse_pitch, parse_space};
use cli::note::prelude::*;
use cli::note::{Note, Pitch};

fn main() {
    let pitch = Pitch::C4;
    let i = pitch.to_i32();
    dbg!(i);
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
