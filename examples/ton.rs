use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputPort};

use cli::note::prelude::*;
use cli::note::Note;

use abc_parser::abc;
use abc_parser::datatypes::*;

fn main() {
    let parsed = abc::tune_book(
        r#"X:1
T:Example
K:D
^F F3// ^D// ^C3/ B/ ^A ^G ^F
"#,
    )
    .unwrap();
    // assert_eq!(
    //     parsed,
    //     TuneBook::new(
    //         None,
    //         vec![Tune::new(
    //             TuneHeader::new(vec![
    //                 InfoField::new('X', "1".to_string()),
    //                 InfoField::new('T', "Example".to_string()),
    //                 InfoField::new('K', "D".to_string())
    //             ]),
    //             None
    //         )]
    //     )
    // )
    dbg!(parsed);
}

// struct Score {
//     notes: Vec<(u8, u64)>,
// }

// fn main() {
//     let abc = r#"
//     ^F F3// ^D// ^C3/ B/ ^A ^G ^F
//      "#;

//     let s = abc
//         .split_whitespace()
//         .collect::<Vec<&str>>()
//         .iter()
//         .map(|nodul| {
//             nodul
//                 .split(&['/', '1', '2', '3'][..])
//                 .collect::<Vec<&str>>()
//         })
//         .collect::<Vec<_>>();
//     // let s = abc.split();
//     dbg!(s);
// }
