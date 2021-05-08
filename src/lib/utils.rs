use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn get_content(fpath: PathBuf) -> String {
    let mut f = File::open(fpath).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents
}
