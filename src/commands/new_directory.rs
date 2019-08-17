use std::path;

use crate::commands::{LllCommand, LllRunnable, ReloadDirList};
use crate::context::LllContext;
use crate::error::LllError;
use crate::window::LllView;

#[derive(Clone, Debug)]
pub struct NewDirectory {
    paths: Vec<path::PathBuf>,
}

impl NewDirectory {
    pub fn new(paths: Vec<path::PathBuf>) -> Self {
        NewDirectory { paths }
    }
    pub const fn command() -> &'static str {
        "mkdir"
    }
}

impl LllCommand for NewDirectory {}

impl std::fmt::Display for NewDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl LllRunnable for NewDirectory {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        for path in &self.paths {
            match std::fs::create_dir_all(path) {
                Ok(_) => {}
                Err(e) => return Err(LllError::IO(e)),
            }
        }
        let res = ReloadDirList::reload(context.curr_tab_index, context);
        match res {
            Ok(_) => {
                let curr_tab = &mut context.tabs[context.curr_tab_index];
                curr_tab.refresh(view, &context.config_t);
                ncurses::doupdate();
            }
            Err(e) => return Err(LllError::IO(e)),
        }
        Ok(())
    }
}
