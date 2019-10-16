use serde_derive::Deserialize;
use std::collections::{hash_map::Entry, HashMap};
use std::process::exit;

use super::{parse_to_config_file, ConfigStructure, Flattenable};
use crate::commands::{self, CommandKeybind, LllCommand};
use crate::KEYMAP_FILE;

pub const ESCAPE: i32 = 0x1B;

const fn default_up() -> i32 {
    ncurses::KEY_UP
}

const fn default_down() -> i32 {
    ncurses::KEY_DOWN
}

const fn default_left() -> i32 {
    ncurses::KEY_LEFT
}

const fn default_right() -> i32 {
    ncurses::KEY_RIGHT
}

const fn default_home() -> i32 {
    ncurses::KEY_HOME
}

const fn default_end() -> i32 {
    ncurses::KEY_END
}

const fn default_backspace() -> i32 {
    ncurses::KEY_BACKSPACE
}

const fn default_delete() -> i32 {
    ncurses::KEY_DC
}

const fn default_enter() -> i32 {
    '\n' as i32
}

const fn default_escape() -> i32 {
    ESCAPE
}

const fn default_tab() -> i32 {
    '\t' as i32
}

#[derive(Debug, Deserialize)]
struct LllRawKeymapping {
    #[serde(default)]
    keymaps: LllKeyMapping,
    #[serde(skip)]
    mapcommand: Vec<LllMapCommand>,
}

#[derive(Debug, Deserialize)]
pub struct LllKeyMapping {
    #[serde(default = "default_up")]
    pub up: i32,
    #[serde(default = "default_down")]
    pub down: i32,
    #[serde(default = "default_left")]
    pub left: i32,
    #[serde(default = "default_right")]
    pub right: i32,
    #[serde(default = "default_home")]
    pub home: i32,
    #[serde(default = "default_end")]
    pub end: i32,
    #[serde(default = "default_backspace")]
    pub backspace: i32,
    #[serde(default = "default_delete")]
    pub delete: i32,
    #[serde(default = "default_enter")]
    pub enter: i32,
    #[serde(default = "default_escape")]
    pub escape: i32,
    #[serde(default = "default_tab")]
    pub tab: i32,
}

impl std::default::Default for LllKeyMapping {
    fn default() -> Self {
        LllKeyMapping {
            up: default_up(),
            down: default_down(),
            left: default_left(),
            right: default_right(),
            home: default_home(),
            end: default_end(),
            backspace: default_backspace(),
            delete: default_delete(),
            enter: default_enter(),
            escape: default_escape(),
            tab: default_tab(),
        }
    }
}

impl Flattenable<LllKeyMapping> for LllRawKeymapping {
    fn flatten(self) -> LllKeyMapping {
        self.keymaps
    }
}

impl ConfigStructure for LllKeyMapping {
    fn get_config() -> Self {
        parse_to_config_file::<LllRawKeymapping, LllKeyMapping>(KEYMAP_FILE)
            .unwrap_or_else(LllKeyMapping::default)
    }
}

#[derive(Debug, Deserialize)]
struct LllMapCommand {
    pub keys: Vec<i32>,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct LllRawCommandMapping {
    #[serde(skip)]
    keymaps: LllKeyMapping,
    #[serde(default)]
    mapcommand: Vec<LllMapCommand>,
}

impl Flattenable<LllCommandMapping> for LllRawCommandMapping {
    fn flatten(self) -> LllCommandMapping {
        let mut keymaps = LllCommandMapping::new();
        self.mapcommand.iter().for_each(|m| {
            let args: Vec<&str> = m.args.iter().map(String::as_str).collect();
            match commands::from_args(m.command.as_str(), &args) {
                Ok(command) => insert_keycommand(&mut keymaps, command, &m.keys[..]),
                Err(e) => eprintln!("{}", e),
            }
        });
        keymaps
    }
}

pub type LllCommandMapping = HashMap<i32, CommandKeybind>;

impl ConfigStructure for LllCommandMapping {
    fn get_config() -> Self {
        parse_to_config_file::<LllRawCommandMapping, LllCommandMapping>(KEYMAP_FILE)
            .unwrap_or_else(LllCommandMapping::default)
    }
}

fn insert_keycommand(map: &mut LllCommandMapping, keycommand: Box<dyn LllCommand>, keys: &[i32]) {
    match keys.len() {
        0 => {}
        1 => match map.entry(keys[0]) {
            Entry::Occupied(_) => {
                eprintln!("Error: Keybindings ambiguous");
                exit(1);
            }
            Entry::Vacant(entry) => {
                entry.insert(CommandKeybind::SimpleKeybind(keycommand));
            }
        },
        _ => match map.entry(keys[0]) {
            Entry::Occupied(mut entry) => match entry.get_mut() {
                CommandKeybind::CompositeKeybind(ref mut m) => {
                    insert_keycommand(m, keycommand, &keys[1..])
                }
                _ => {
                    eprintln!("Error: Keybindings ambiguous");
                    exit(1);
                }
            },
            Entry::Vacant(entry) => {
                let mut new_map = LllCommandMapping::new();
                insert_keycommand(&mut new_map, keycommand, &keys[1..]);
                let composite_command = CommandKeybind::CompositeKeybind(new_map);
                entry.insert(composite_command);
            }
        },
    }
}
