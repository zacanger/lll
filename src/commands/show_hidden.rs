use crate::commands::{LllCommand, LllRunnable, ReloadDirList};
use crate::context::LllContext;
use crate::error::LllError;
use crate::history::DirectoryHistory;
use crate::window::LllView;

#[derive(Clone, Debug)]
pub struct ToggleHiddenFiles;

impl ToggleHiddenFiles {
    pub fn new() -> Self {
        ToggleHiddenFiles
    }
    pub const fn command() -> &'static str {
        "toggle_hidden"
    }
    pub fn toggle_hidden(context: &mut LllContext) {
        let opposite = !context.config_t.sort_option.show_hidden;
        context.config_t.sort_option.show_hidden = opposite;

        for tab in &mut context.tabs {
            tab.history.depreciate_all_entries();
            tab.curr_list.depreciate();
        }
    }
}

impl LllCommand for ToggleHiddenFiles {}

impl std::fmt::Display for ToggleHiddenFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl LllRunnable for ToggleHiddenFiles {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        Self::toggle_hidden(context);
        ReloadDirList::new().execute(context, view)
    }
}
