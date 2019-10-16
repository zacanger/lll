use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt;

use super::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::MIMETYPE_FILE;

const fn default_false() -> bool {
    false
}

#[derive(Debug, Deserialize)]
pub struct LllMimetypeEntry {
    pub id: usize,
    pub program: String,
    pub args: Option<Vec<String>>,
    #[serde(default = "default_false")]
    pub fork: bool,
    #[serde(default = "default_false")]
    pub silent: bool,
}

impl std::fmt::Display for LllMimetypeEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.program.as_str()).unwrap();
        if let Some(s) = self.args.as_ref() {
            s.iter().for_each(|arg| write!(f, " {}", arg).unwrap());
        }
        f.write_str("\t[").unwrap();
        if self.fork {
            f.write_str("fork,").unwrap();
        }
        if self.silent {
            f.write_str("silent").unwrap();
        }
        f.write_str("]")
    }
}

#[derive(Debug, Deserialize)]
pub struct LllRawMimetype {
    #[serde(default)]
    entry: Vec<LllMimetypeEntry>,
    #[serde(default)]
    extension: HashMap<String, Vec<usize>>,
    #[serde(default)]
    mimetype: HashMap<String, Vec<usize>>,
}

impl Flattenable<LllMimetype> for LllRawMimetype {
    fn flatten(self) -> LllMimetype {
        let mut entries = HashMap::with_capacity(self.entry.len());
        for entry in self.entry {
            entries.insert(entry.id, entry);
        }
        LllMimetype {
            entries,
            extension: self.extension,
            mimetype: self.mimetype,
        }
    }
}

#[derive(Debug)]
pub struct LllMimetype {
    pub entries: HashMap<usize, LllMimetypeEntry>,
    pub extension: HashMap<String, Vec<usize>>,
    pub mimetype: HashMap<String, Vec<usize>>,
}

impl LllMimetype {
    pub fn get_entries_for_ext(&self, extension: &str) -> Vec<&LllMimetypeEntry> {
        Self::get_entries(&self.extension, &self.entries, extension)
    }
    fn get_entries<'a>(
        map: &HashMap<String, Vec<usize>>,
        entry_map: &'a HashMap<usize, LllMimetypeEntry>,
        key: &str,
    ) -> Vec<&'a LllMimetypeEntry> {
        match map.get(key) {
            Some(entry_ids) => entry_ids
                .iter()
                .filter_map(|id| entry_map.get(id))
                .collect(),
            None => Vec::new(),
        }
    }
}

impl ConfigStructure for LllMimetype {
    fn get_config() -> Self {
        parse_to_config_file::<LllRawMimetype, LllMimetype>(MIMETYPE_FILE)
            .unwrap_or_else(LllMimetype::default)
    }
}

impl std::default::Default for LllMimetype {
    fn default() -> Self {
        LllMimetype {
            entries: HashMap::new(),
            mimetype: HashMap::new(),
            extension: HashMap::new(),
        }
    }
}
