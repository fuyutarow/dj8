use ghakuf::messages::*;
use ghakuf::reader::*;
use std::path;

fn main() {
    let fpath = "examples/hotarunohikari.mid";
    let path = path::Path::new(fpath);
    let mut handler = HogeHandler {};
    let mut reader = Reader::new(&mut handler, &path).unwrap();
    dbg!(&handler);
    let read = reader.read();
    dbg!(&read);

    #[derive(Debug)]
    struct HogeHandler {}
    impl Handler for HogeHandler {
        fn header(&mut self, format: u16, track: u16, time_base: u16) {
            // Something
        }
        fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
            // you
        }
        fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
            // want
        }
        fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
            // to
        }
        fn track_change(&mut self) {
            // do
        }
    }
}
