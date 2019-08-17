use serde_derive::Deserialize;
use std::collections::HashMap;

use super::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::PREVIEW_FILE;

#[derive(Debug, Deserialize)]
pub struct LllPreviewEntry {
    pub program: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct LllRawPreview {
    pub extension: Option<HashMap<String, LllPreviewEntry>>,
    pub mimetype: Option<HashMap<String, LllPreviewEntry>>,
}

impl std::default::Default for LllRawPreview {
    fn default() -> Self {
        LllRawPreview {
            extension: None,
            mimetype: None,
        }
    }
}

impl Flattenable<LllPreview> for LllRawPreview {
    fn flatten(self) -> LllPreview {
        let extension = self.extension.unwrap_or_default();
        let mimetype = self.mimetype.unwrap_or_default();

        LllPreview {
            extension,
            mimetype,
        }
    }
}

#[derive(Debug)]
pub struct LllPreview {
    pub extension: HashMap<String, LllPreviewEntry>,
    pub mimetype: HashMap<String, LllPreviewEntry>,
}

impl ConfigStructure for LllPreview {
    fn get_config() -> Self {
        parse_to_config_file::<LllRawPreview, LllPreview>(PREVIEW_FILE)
            .unwrap_or_else(LllPreview::default)
    }
}

impl std::default::Default for LllPreview {
    fn default() -> Self {
        LllPreview {
            extension: HashMap::new(),
            mimetype: HashMap::new(),
        }
    }
}
