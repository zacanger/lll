mod commands;
mod config;
mod context;
mod error;
mod fs;
mod history;
mod run;
mod sort;
mod tab;
mod textfield;
mod ui;
mod unix;
mod window;

use lazy_static::lazy_static;
use std::path::PathBuf;
use structopt::StructOpt;

use config::{
    ConfigStructure, LllCommandMapping, LllConfig, LllKeyMapping, LllMimetype, LllPreview, LllTheme,
};
use run::run;

const PROGRAM_NAME: &str = "lll";
const CONFIG_FILE: &str = "lll.toml";
const MIMETYPE_FILE: &str = "mimetype.toml";
const KEYMAP_FILE: &str = "keymap.toml";
const THEME_FILE: &str = "theme.toml";
const PREVIEW_FILE: &str = "preview.toml";

lazy_static! {
    // dynamically builds the config hierarchy
    static ref CONFIG_HIERARCHY: Vec<PathBuf> = {
        let mut temp = vec![];
        // TODO: get rid of this, zero config files are the goal
        match xdg::BaseDirectories::with_prefix(PROGRAM_NAME) {
            Ok(dirs) => temp.push(dirs.get_config_home()),
            Err(e) => eprintln!("{}", e),
        };
        // adds the default config files to the config hierarchy if running through cargo
        if cfg!(debug_assertions) {
            temp.push(PathBuf::from("./config"));
        }
        temp
    };
    static ref THEME_T: LllTheme = LllTheme::get_config();
    static ref MIMETYPE_T: LllMimetype = LllMimetype::get_config();
    static ref PREVIEW_T: LllPreview = LllPreview::get_config();
    static ref KEYMAP_T: LllKeyMapping = LllKeyMapping::get_config();

    static ref HOME_DIR: Option<PathBuf> = dirs::home_dir();
}

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

fn main() {
    let args = Args::from_args();

    let config = LllConfig::get_config();
    let keymap = LllCommandMapping::get_config();

    if args.debug {
        eprintln!("config: {:#?}", config);
        eprintln!("theme config: {:#?}", *THEME_T);
        eprintln!("mimetype config: {:#?}", *MIMETYPE_T);
    }

    run(config, keymap);
}
