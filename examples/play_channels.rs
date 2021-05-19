use std::sync::mpsc;

use dj8::abc_parser::parse_notes;
use dj8::midi_env::get_conn_out;

fn main() {
    let (sender_to_sub, receiver_from_main) = mpsc::channel();

    std::thread::spawn(move || {
        match get_conn_out() {
            Ok(mut conn_out) => loop {
                loop {
                    let input = receiver_from_main.recv().unwrap();
                    let (input, notes) = parse_notes(input).unwrap();
                    for note in notes {
                        note.play(&mut conn_out);
                    }
                }
            },
            Err(err) => println!("Error: {}", err),
        };
    });

    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        // let s = input.clone();

        let mut s = String::from("c");
        sender_to_sub.send(s.clone()).unwrap();
        // println!("{}", input);
    }
}
