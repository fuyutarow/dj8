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
"#,
        // ^F F3//
        // ^F F3// ^D// ^C3/ B/ ^A ^G ^F
    )
    .unwrap();
    dbg!(parsed);
}
