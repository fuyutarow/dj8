use std::sync::mpsc;
use std::thread;

fn main() {
    // チャンネルの作成. sender がメッセージの送信機で receiver が受信機
    // メインスレッド -> サブスレッドのチャンネル
    let (sender_to_sub, receiver_from_main) = mpsc::channel();
    // メインスレッド <- サブスレッドのチャンネル
    let (sender_to_main, receiver_from_sub) = mpsc::channel();

    // サブスレッドの開始
    thread::spawn(move || {
        // let (input, notes) = parse_notes(input).unwrap();
        // let score = Score {
        //     notes,
        //     tempo: 4. * 150.,
        // };

        // match get_conn_out() {
        //     Ok(mut conn_out) => loop {
        //         score.play(&mut conn_out);
        //     },
        //     Err(err) => println!("Error: {}", err),
        // };
        // メインスレッドからメッセージの受信. 受信するまで処理を待つ

        // メインスレッドへメッセージの送信
        sender_to_main.send("hi".to_string()).unwrap();

        loop {
            let val = receiver_from_main.recv().unwrap();
            println!("send from main thread. {}", val);
        }
    });

    // サブスレッドへメッセージの送信
    let input = r#"
c G3// A// B E/ E/ | A G3// F// G C/ C/ | D D/ E/ F F/ G/ | A B/ c/ d3/ G/ |\
e d3// c// d B/ G/ | c B3// A// B E/ E/ | A G/ F/ G C/ C/ | c B3// A// G2 |]
    "#;
    sender_to_sub.send(input.to_string()).unwrap();
    sender_to_sub.send(input.to_string()).unwrap();

    // サブスレッドからメッセージの受信
    let val = receiver_from_sub.recv().unwrap();
    println!("send from sub thread. {}", val);

    timer();
}

fn timer() {
    let mut time = 0;
    loop {
        println!("hello @ {}", time);
        std::thread::sleep(std::time::Duration::from_millis(1000));
        time += 1
    }
}
