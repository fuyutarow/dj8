use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputPort};

use cli::abc_parser::parse_notes;
use cli::note::prelude::*;
use cli::note::{Note, Pitch};
use cli::score::Score;

fn main() {
    let input = r#"
c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
    "#;
    let (input, notes) = parse_notes(input).unwrap();
    let notes = notes
        .iter()
        .map(|note| note.tempo(4.))
        .map(|note| note.to_pair())
        .collect::<Vec<(u8, u64)>>();
    let score = Score { notes };

    match run(score) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run(score: Score) -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!();
            println!("Available output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };
    println!();
    println!("Opening connection");
    let mut conn_out = midi_out.connect(out_port, "midir-test")?;
    println!("Connection open. Listen!");

    score.play(&mut conn_out);

    sleep(Duration::from_millis(150));
    println!();
    println!("Closing connection");
    conn_out.close();
    println!("Connection closed");
    Ok(())
}
