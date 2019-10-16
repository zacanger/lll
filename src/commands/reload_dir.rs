use crate::commands::{LllCommand, LllRunnable};
use crate::context::LllContext;
use crate::error::LllError;
use crate::fs::LllDirList;
use crate::window::LllView;

use std::collections::hash_map::Entry;

#[derive(Clone, Debug)]
pub struct ReloadDirList;

impl ReloadDirList {
    pub fn new() -> Self {
        ReloadDirList
    }
    pub const fn command() -> &'static str {
        "reload_dir_list"
    }

    pub fn reload(index: usize, context: &mut LllContext) -> std::io::Result<()> {
        let curr_tab = &mut context.tabs[index];
        let sort_option = &context.config_t.sort_option;
        curr_tab.curr_list.update_contents(sort_option)?;

        if let Some(parent) = curr_tab.curr_list.file_path().parent() {
            match curr_tab.history.entry(parent.to_path_buf().clone()) {
                Entry::Occupied(mut entry) => {
                    let dirlist = entry.get_mut();
                    dirlist.update_contents(sort_option)?;
                }
                Entry::Vacant(entry) => {
                    let s = LllDirList::new(parent.to_path_buf().clone(), sort_option)?;
                    entry.insert(s);
                }
            }
        }
        Ok(())
    }
}

impl LllCommand for ReloadDirList {}

impl std::fmt::Display for ReloadDirList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl LllRunnable for ReloadDirList {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        match Self::reload(context.curr_tab_index, context) {
            Ok(_) => {
                let curr_tab = &mut context.tabs[context.curr_tab_index];
                curr_tab.refresh(view, &context.config_t);
                ncurses::doupdate();
                Ok(())
            }
            Err(e) => Err(LllError::IO(e)),
        }
    }
}
