extern crate serde;
use serde::Deserialize;

fn default_spell() -> String {
    "".to_string()
}

#[derive(Debug)]
enum NameType {
    Func,
    Var,
    Macro,
    Type,
}

#[derive(Deserialize)]
pub struct Entry {
    #[serde(default = "default_spell")]
    pub spell: String,
    pub detailed_name: String,
    pub short_name_offset: usize,
    pub short_name_size: usize,
    pub kind: i16,
}

#[derive(Deserialize)]
pub struct IndexFile {
    pub usr2func: Vec<Entry>,
    pub usr2type: Vec<Entry>,
    pub usr2var: Vec<Entry>,
}

fn print_entry(entry: &Entry, name_type: NameType) -> () {
    if entry.spell != "" {
        let name: String = entry
            .detailed_name
            .chars()
            .skip(entry.short_name_offset)
            .take(entry.short_name_size)
            .collect();
        if !name.starts_with("anon") {
            print!("{:?}: {}\n", name_type, name);
        }
    }
}

pub fn print_index(index: &IndexFile) -> () {
    for entry in index.usr2func.iter() {
        print_entry(&entry, NameType::Func);
    }
    for entry in index.usr2type.iter() {
        print_entry(&entry, NameType::Type);
    }
    for entry in index.usr2var.iter() {
        print_entry(
            &entry,
            if entry.kind == 255 {
                NameType::Macro
            } else {
                NameType::Var
            },
        );
    }
}
