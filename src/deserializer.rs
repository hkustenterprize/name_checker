extern crate serde;
use serde::Deserialize;

fn default_spell() -> String {
    "".to_string()
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NameType {
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

pub struct EntryName<'a> {
    pub name: &'a str,
    pub name_type: NameType,
}

pub struct IndexFileIterator<'a> {
    index_file: &'a IndexFile,
    current_type: NameType,
    index: usize,
}

impl IndexFile {
    pub fn get_iter(&self) -> IndexFileIterator {
        IndexFileIterator {index_file: self, current_type: NameType::Func, index: 0}
    }
}

fn entry_valid(entry: &Entry) -> bool {
    entry.spell != "" && !entry.detailed_name.starts_with("anon")
}

impl<'a> Iterator for IndexFileIterator<'a> {
    type Item = EntryName<'a>;
    fn next(&mut self) -> Option<EntryName<'a>> {
        loop {
            let entries: &Vec<Entry> = match &self.current_type {
                NameType::Func  => self.index_file.usr2func.as_ref(),
                NameType::Type  => self.index_file.usr2type.as_ref(),
                NameType::Var   => self.index_file.usr2var.as_ref(),
                NameType::Macro => self.index_file.usr2var.as_ref()
            };
            while entries.len() > self.index {
                let entry = &entries[self.index];
                self.index += 1;
                if entry_valid(entry) {
                    let name: &str = &entry
                        .detailed_name
                        .as_str()[entry.short_name_offset..entry.short_name_offset + entry.short_name_size];
                    let mut t = self.current_type;
                    if t == NameType::Var && entry.kind == 255 {
                        t = NameType::Macro;
                    }
                    return Some(EntryName {name: name, name_type: t});
                }
            }
            self.current_type = match &self.current_type {
                NameType::Func  => NameType::Type,
                NameType::Type  => NameType::Var,
                NameType::Var   => return None,
                NameType::Macro => return None
            };
            self.index = 0;
        }
    }
}

