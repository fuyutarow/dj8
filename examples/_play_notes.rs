use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputPort};

use dj8::note::prelude::*;
use dj8::note::Pitch;

struct Score {
    notes: Vec<(u8, u64)>,
}

fn main() {
    let notes = vec![
        ("^F", 4),
        ("F", 3),
        ("^D", 1),
        ("^C", 6),
        ("B,", 2),
        ("^A,", 4),
        ("^G,", 4),
        ("^F,", 4),
    ]
    .iter()
    .map(move |(abc, d)| (Pitch::from_abc(abc).to_u8().unwrap(), *d as u64))
    .collect::<Vec<(u8, u64)>>();

    let score = Score { notes };

    match run(score) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run(score: Score) -> Result<(), Box<dyn Error>> {
    let midi_out = MidiOutput::new("My Test Output")?;
    // Get an output port (read from console if multiple are available)
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
    {
        // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
        let mut play_note = |note: u8, duration: u64| {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;
            // We're ignoring errors in here
            let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
            sleep(Duration::from_millis(duration * 150));
            let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
        };

        sleep(Duration::from_millis(4 * 150));

        for note in score.notes {
            play_note(note.0, note.1);
        }
    }
    sleep(Duration::from_millis(150));
    println!();
    println!("Closing connection");
    // This is optional, the connection would automatically be closed as soon as it goes out of scope
    conn_out.close();
    println!("Connection closed");
    Ok(())
}
