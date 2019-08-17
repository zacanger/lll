use std::path::PathBuf;

use crate::fs::LllDirList;
use crate::history::{DirectoryHistory, LllHistory};
use crate::sort;
use crate::ui;
use crate::window::{LllPanel, LllView};
use crate::LllConfig;

use crate::THEME_T;

pub struct LllTab {
    pub history: LllHistory,
    pub curr_path: PathBuf,
    pub curr_list: LllDirList,
}

impl LllTab {
    pub fn new(curr_path: PathBuf, sort_option: &sort::SortOption) -> std::io::Result<Self> {
        let mut history = LllHistory::new();
        history.populate_to_root(&curr_path, sort_option)?;

        let curr_list = history.pop_or_create(&curr_path, sort_option)?;

        let tab = LllTab {
            curr_path,
            history,
            curr_list,
        };
        Ok(tab)
    }

    pub fn refresh(&mut self, views: &LllView, config_t: &LllConfig) {
        self.refresh_curr(&views.mid_win, config_t);
        self.refresh_path_status(&views.top_win);
        self.refresh_file_status(&views.bot_win);
    }

    pub fn refresh_curr(&mut self, win: &LllPanel, config_t: &LllConfig) {
        ui::display_contents(
            win,
            &mut self.curr_list,
            config_t,
            &ui::PRIMARY_DISPLAY_OPTION,
        );
    }

    pub fn refresh_file_status(&self, win: &LllPanel) {
        ncurses::werase(win.win);
        ncurses::wmove(win.win, 0, 0);
        if let Some(index) = self.curr_list.index {
            let entry = &self.curr_list.contents[index];
            ui::wprint_file_status(win, entry);
        }
    }

    pub fn refresh_path_status(&self, win: &LllPanel) {
        let path_str: &str = self.curr_path.to_str().unwrap();

        ncurses::werase(win.win);

        ncurses::wattron(win.win, ncurses::COLOR_PAIR(THEME_T.directory.colorpair));
        ncurses::waddstr(win.win, path_str);
        ncurses::waddstr(win.win, "/");
        ncurses::wattroff(win.win, ncurses::COLOR_PAIR(THEME_T.directory.colorpair));

        win.queue_for_refresh();
    }
}
