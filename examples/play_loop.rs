use dj8::abc_parser::parse_notes;
use dj8::midi_env::get_conn_out;

use dj8::score::Score;

fn main() {
    let input = r#"
c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
    "#;
    let (_input, notes) = parse_notes(input).unwrap();
    let score = Score {
        notes,
        tempo: 4. * 150.,
    };

    std::thread::spawn(|| play_loop(score));

    timer()
}

fn timer() {
    let mut time = 0;
    loop {
        println!("hello @ {}", time);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        time += 1
    }
}

fn play_loop(score: Score) {
    match get_conn_out() {
        Ok(mut conn_out) => {
            std::thread::sleep(std::time::Duration::from_millis(4 * 150));
            loop {
                score.play(&mut conn_out);
            }
            std::thread::sleep(std::time::Duration::from_millis(4 * 150));
            conn_out.close();
        }
        Err(err) => println!("Error: {}", err),
    };
}
