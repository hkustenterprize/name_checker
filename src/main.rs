extern crate serde;
extern crate serde_json;

mod deserializer;

use deserializer::{print_index, IndexFile};
use std::path::Path;


fn main() {
    let file_path = Path::new("");
    let json_file = File::open(file_path).expect("file not found");
    let index_file: IndexFile = serde_json::from_reader(json_file).expect("parsing error");
    print_index(&index_file);
}
