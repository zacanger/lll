use std::{fs, path};

use crate::fs::LllMetadata;

#[derive(Clone)]
pub struct LllDirEntry {
    name: String,
    path: path::PathBuf,
    pub metadata: LllMetadata,
    selected: bool,
    marked: bool,
}

impl LllDirEntry {
    pub fn from(direntry: &fs::DirEntry) -> std::io::Result<Self> {
        let name = match direntry.file_name().into_string() {
            Ok(s) => s,
            Err(_) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Failed converting OsString to String",
                ));
            }
        };

        let path = direntry.path();
        let metadata = LllMetadata::from(&path)?;

        let dir_entry = LllDirEntry {
            name,
            path,
            metadata,
            selected: false,
            marked: false,
        };
        Ok(dir_entry)
    }

    pub fn file_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn file_path(&self) -> &path::PathBuf {
        &self.path
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

impl std::fmt::Debug for LllDirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "LllDirEntry {{\n\tfile_name: {:?}, \n\tpath: {:?} \n}}",
            self.name, self.path
        )
    }
}
