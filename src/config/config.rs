use serde_derive::Deserialize;

use super::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::sort;

use crate::CONFIG_FILE;

const fn default_true() -> bool {
    true
}
const fn default_scroll_offset() -> usize {
    6
}
const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}
const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

#[derive(Clone, Debug, Deserialize)]
struct SortRawOption {
    #[serde(default)]
    show_hidden: bool,
    #[serde(default = "default_true")]
    directories_first: bool,
    #[serde(default)]
    case_sensitive: bool,
    #[serde(default)]
    reverse: bool,
}

impl SortRawOption {
    pub fn into_sort_option(self, sort_method: sort::SortType) -> sort::SortOption {
        sort::SortOption {
            show_hidden: self.show_hidden,
            directories_first: self.directories_first,
            case_sensitive: self.case_sensitive,
            reverse: self.reverse,
            sort_method,
        }
    }
}

impl std::default::Default for SortRawOption {
    fn default() -> Self {
        SortRawOption {
            show_hidden: bool::default(),
            directories_first: default_true(),
            case_sensitive: bool::default(),
            reverse: bool::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LllRawConfig {
    #[serde(default = "default_scroll_offset")]
    scroll_offset: usize,
    #[serde(default = "default_max_preview_size")]
    max_preview_size: u64,
    column_ratio: Option<[usize; 3]>,
    sort_method: Option<String>,
    #[serde(default)]
    sort_option: SortRawOption,
}

impl Flattenable<LllConfig> for LllRawConfig {
    fn flatten(self) -> LllConfig {
        let column_ratio = match self.column_ratio {
            Some(s) => (s[0], s[1], s[2]),
            _ => default_column_ratio(),
        };

        let sort_method = match self.sort_method {
            Some(s) => match sort::SortType::parse(s.as_str()) {
                Some(s) => s,
                None => sort::SortType::Natural,
            },
            None => sort::SortType::Natural,
        };
        let sort_option = self.sort_option.into_sort_option(sort_method);

        LllConfig {
            scroll_offset: self.scroll_offset,
            max_preview_size: self.max_preview_size,
            column_ratio,
            sort_option,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LllConfig {
    pub scroll_offset: usize,
    pub max_preview_size: u64,
    pub sort_option: sort::SortOption,
    pub column_ratio: (usize, usize, usize),
}

impl ConfigStructure for LllConfig {
    fn get_config() -> Self {
        parse_to_config_file::<LllRawConfig, LllConfig>(CONFIG_FILE)
            .unwrap_or_else(LllConfig::default)
    }
}

impl std::default::Default for LllConfig {
    fn default() -> Self {
        let sort_option = sort::SortOption::default();

        LllConfig {
            scroll_offset: default_scroll_offset(),
            max_preview_size: default_max_preview_size(),
            sort_option,
            column_ratio: default_column_ratio(),
        }
    }
}
