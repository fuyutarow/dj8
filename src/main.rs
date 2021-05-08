use std::path::{Path, PathBuf};
use structopt::StructOpt;

use ghakuf::messages::Message;
use ghakuf::reader::Reader;
use ghakuf::writer::Writer;

mod lib;
use lib::abc_parser::parse_notes;
use lib::utils;
use lib::BasicHandler;
// use lib::version::Version;
// use lib::Manager;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "from")]
    From {
        /// target config file [possible values: *.mid]
        #[structopt(parse(from_os_str))]
        fpath: PathBuf,

        /// [possible values: midi, json]
        #[structopt(short, long, default_value = "midi")]
        to: String,

        /// [possible values: midi, json]
        #[structopt(short, long)]
        out: Option<String>,
    },
}

fn main() {
    match Opt::from_args() {
        Opt::From { fpath, to, out } => {
            let messages = match fpath.extension() {
                Some(s) if s == "mid" => {
                    let mut read_messages: Vec<Message> = Vec::new();
                    let mut handler = BasicHandler {
                        messages: &mut read_messages,
                    };
                    let mut reader = Reader::new(&mut handler, &fpath).unwrap();
                    let _ = reader.read();
                    read_messages
                }
                Some(s) if s == "abc" => {
                    let content = utils::get_content(fpath);
                    let mut lines = content.lines();
                    for _ in 0..5 {
                        lines.next();
                    }
                    let input = lines.map(String::from).collect::<Vec<String>>().join("\n");
                    dbg!(&input);
                    println!("{}", &input);
                    let (_, notes) = parse_notes(&input).unwrap();
                    dbg!(&notes);
                    let messages = notes
                        .iter()
                        .map(|note| note.tempo(4.))
                        .map(|note| note.to_messages())
                        .flatten()
                        .collect::<Vec<Message>>();
                    messages
                }
                _ => vec![],
            };

            if let Some(out_path) = out {
                let path = Path::new(&out_path);
                let mut writer = Writer::new();
                writer.running_status(true);
                for message in &messages {
                    writer.push(&message);
                }
                writer.write(&path);
            } else {
                dbg!(messages);
            }
        }
    }
}
