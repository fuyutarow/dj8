use std::sync::{Arc, Mutex};

use dj8::abc_parser::parse_notes;
use dj8::midi_env::get_conn_out;

use dj8::score::Score;

fn main() {
    let score1 = {
        let input = r#"
G3/E C E G c2 e3/d c E ^F G2 GG e3/ d c B2 A B c c G E C
"#;
        let (_input, notes) = parse_notes(input).unwrap();
        Score {
            notes,
            tempo: 4. * 150.,
        }
    };

    let score2 = {
        let input = r#"
c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
"#;
        let (_input, notes) = parse_notes(input).unwrap();
        Score {
            notes,
            tempo: 4. * 150.,
        }
    };

    play2(vec![score1, score2]);

    let mut time = 0;
    loop {
        println!("hello @ {}", time);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        time += 1
    }
}

fn play2(scores: Vec<Score>) {
    match get_conn_out() {
        Ok(conn_out) => {
            let conn = Arc::new(Mutex::new(conn_out));
            std::thread::sleep(std::time::Duration::from_millis(4 * 150));

            for score in scores {
                let conn = Arc::clone(&conn);
                std::thread::spawn(move || loop {
                    println!("i={:?}", score);
                    let mut conn_out = conn.lock().unwrap();
                    score.play(&mut conn_out)
                });
            }
        }
        Err(err) => println!("Error: {}", err),
    };
}
