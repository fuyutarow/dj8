use crossbeam_channel::{bounded, select};
use crossbeam_utils::thread;

use dj8::abc_parser::parse_notes;
use dj8::midi_env::get_conn_out;

fn main() {
    let (sender, receiver) = bounded(1); // Make room for one unmatched send.

    thread::scope(|scope| {
        let (sender, receiver) = (sender.clone(), receiver.clone());
        // scope.spawn(move |_| seek(name, s, r));
        scope.spawn(move |_| match get_conn_out() {
            Ok(mut conn_out) => loop {
                let input = receiver.recv().unwrap();
                let (input, notes) = parse_notes(input).unwrap();
                for note in notes {
                    note.play(&mut conn_out);
                }
            },
            Err(err) => println!("Error: {}", err),
        });
    })
    .unwrap();

    loop {
        println!("> ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        // let s = input.clone();

        let mut s = String::from("c");
        // sender.send(&s).unwrap();
        // println!("{}", input);
    }

    // // Check if there is a pending send operation.
    // if let Ok(name) = r.try_recv() {
    //     println!("No one received {}’s message.", name);
    // }
}
