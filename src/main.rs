use ghakuf::messages::Message;
use ghakuf::reader::Reader;
use ghakuf::writer::Writer;
use std::path::PathBuf;
use structopt::StructOpt;

mod lib;
use lib::BasicHandler;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(parse(from_os_str))]
    fpath: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    match opt.fpath {
        Some(fpath) => {
            let mut read_messages: Vec<Message> = Vec::new();
            {
                let mut handler = BasicHandler {
                    messages: &mut read_messages,
                };
                let mut reader = Reader::new(&mut handler, &fpath).unwrap();
                let _ = reader.read();
            }

            dbg!(&read_messages);
        }
        None => {}
    };
}
