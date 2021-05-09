use std::default::Default;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};

#[derive()]
pub struct MidiManager {
    pub conn_out: Option<MidiOutputConnection>,
}

impl Default for MidiManager {
    fn default() -> Self {
        Self { conn_out: None }
    }
}

impl MidiManager {
    pub fn open_conn_out(&mut self) -> Result<(), Box<dyn Error>> {
        let conn_out = get_conn_out()?;
        self.conn_out = Some(conn_out);
        Ok(())
    }

    pub fn close_conn_out(mut self) {
        if let Some(conn_out) = self.conn_out {
            // let conn_out = self.conn_out.unwrap()
            conn_out.close();
            self.conn_out = None;
            println!("Connection closed");
        }
    }
}

pub fn get_conn_out() -> Result<MidiOutputConnection, Box<dyn Error>> {
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
    Ok(conn_out)
}
