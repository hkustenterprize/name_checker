extern crate serde;
extern crate serde_json;

mod deserializer;

use deserializer::{IndexFile};
use std::path::Path;
use std::fs::File;
use std::io::Write;


fn main() {
    let mut path = String::new();
    std::io::stdout().write_all("Please input file path\n".as_bytes()).unwrap();
    let r = std::io::stdin().read_line(&mut path);
    if r.is_err() {
        print!("unexpected error in stdin");
        return;
    }
    let p = path.trim();
    let file_path = Path::new(p);
    let json_file = File::open(file_path).expect("file not found");
    let index_file: IndexFile = serde_json::from_reader(json_file).expect("parsing error");
    for entry in index_file.get_iter() {
        print!("{}: {:?}\n", entry.name, entry.name_type);
    }
}
