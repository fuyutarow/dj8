use ghakuf::messages::Message;
use ghakuf::reader::Reader;
use ghakuf::writer::Writer;
use std::path::Path;

use cli::BasicHandler;

fn main() {
    let mut read_messages: Vec<Message> = Vec::new();

    {
        let path = Path::new("examples/hotarunohikari.mid");
        let mut handler = BasicHandler {
            messages: &mut read_messages,
        };
        let mut reader = Reader::new(&mut handler, &path).unwrap();
        let _ = reader.read();
    }

    dbg!(&read_messages);

    {
        let path = Path::new("examples/example.mid");
        let mut writer = Writer::new();
        writer.running_status(true);
        for message in &read_messages {
            writer.push(&message);
        }
        let _ = writer.write(&path);
    }
}
