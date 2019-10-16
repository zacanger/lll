use serde_derive::Deserialize;
use std::collections::HashMap;

use super::{parse_config_file, ConfigStructure};

const fn default_zero() -> i16 {
    0
}
const fn default_false() -> bool {
    false
}
const fn default_prefix() -> Option<LllPrefix> {
    None
}

#[derive(Clone, Debug, Deserialize)]
pub struct LllColorPair {
    pub id: i16,
    #[serde(default = "default_zero")]
    pub fg: i16,
    #[serde(default = "default_zero")]
    pub bg: i16,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LllPrefix {
    prefix: String,
    size: usize,
}

impl LllPrefix {
    pub fn prefix(&self) -> &str {
        self.prefix.as_str()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LllColorTheme {
    pub colorpair: i16,
    #[serde(default = "default_false")]
    pub bold: bool,
    #[serde(default = "default_false")]
    pub underline: bool,
    #[serde(default = "default_prefix")]
    pub prefix: Option<LllPrefix>,
}

impl std::default::Default for LllColorTheme {
    fn default() -> Self {
        LllColorTheme {
            colorpair: default_zero(),
            bold: default_false(),
            underline: default_false(),
            prefix: default_prefix(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LllTheme {
    #[serde(default)]
    pub colorpair: Vec<LllColorPair>,
    #[serde(default)]
    pub regular: LllColorTheme,
    #[serde(default)]
    pub selection: LllColorTheme,
    #[serde(default)]
    pub directory: LllColorTheme,
    #[serde(default)]
    pub executable: LllColorTheme,
    #[serde(default)]
    pub link: LllColorTheme,
    #[serde(default)]
    pub socket: LllColorTheme,
    #[serde(default)]
    pub ext: HashMap<String, LllColorTheme>,
}

impl ConfigStructure for LllTheme {
    fn get_config() -> Self {
        parse_config_file::<LllTheme>(crate::THEME_FILE).unwrap_or_else(LllTheme::default)
    }
}

impl std::default::Default for LllTheme {
    fn default() -> Self {
        let colorpair: Vec<LllColorPair> = Vec::new();
        let selection = LllColorTheme::default();
        let executable = LllColorTheme::default();
        let regular = LllColorTheme::default();
        let directory = LllColorTheme::default();
        let link = LllColorTheme::default();
        let socket = LllColorTheme::default();
        let ext = HashMap::new();

        LllTheme {
            colorpair,
            selection,
            executable,
            regular,
            directory,
            link,
            socket,
            ext,
        }
    }
}
