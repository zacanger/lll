use crate::commands::{LllCommand, LllRunnable};
use crate::context::LllContext;
use crate::error::LllError;
use crate::window::LllView;

pub fn cursor_move(mut new_index: usize, context: &mut LllContext, view: &LllView) {
    let curr_tab = &mut context.tabs[context.curr_tab_index];

    match curr_tab.curr_list.index {
        None => return,
        Some(_) => {
            let dir_len = curr_tab.curr_list.contents.len();
            if new_index >= dir_len {
                new_index = dir_len - 1;
            }
            curr_tab.curr_list.index = Some(new_index);
        }
    }

    curr_tab.refresh_curr(&view.mid_win, &context.config_t);
    curr_tab.refresh_path_status(&view.top_win);
    curr_tab.refresh_file_status(&view.bot_win);
    ncurses::doupdate();
}

#[derive(Clone, Debug)]
pub struct CursorMoveDown {
    movement: usize,
}

impl CursorMoveDown {
    pub fn new(movement: usize) -> Self {
        CursorMoveDown { movement }
    }
    pub const fn command() -> &'static str {
        "cursor_move_up"
    }
}

impl LllCommand for CursorMoveDown {}

impl std::fmt::Display for CursorMoveDown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl LllRunnable for CursorMoveDown {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        let movement: Option<usize> = {
            let curr_list = &mut context.curr_tab_mut().curr_list;
            curr_list.index.map(|idx| idx + self.movement)
        };
        if let Some(s) = movement {
            cursor_move(s, context, view)
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveUp {
    movement: usize,
}

impl CursorMoveUp {
    pub fn new(movement: usize) -> Self {
        CursorMoveUp { movement }
    }
    pub const fn command() -> &'static str {
        "cursor_move_down"
    }
}

impl LllCommand for CursorMoveUp {}

impl std::fmt::Display for CursorMoveUp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", Self::command(), self.movement)
    }
}

impl LllRunnable for CursorMoveUp {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        let movement: Option<usize> = context.curr_tab_mut().curr_list.index.map(|idx| {
            if idx > self.movement {
                idx - self.movement
            } else {
                0
            }
        });
        if let Some(s) = movement {
            cursor_move(s, context, view);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMovePageUp;

impl CursorMovePageUp {
    pub fn new() -> Self {
        CursorMovePageUp
    }
    pub const fn command() -> &'static str {
        "cursor_move_page_up"
    }
}

impl LllCommand for CursorMovePageUp {}

impl std::fmt::Display for CursorMovePageUp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl LllRunnable for CursorMovePageUp {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        let movement: Option<usize> = {
            let curr_list = &mut context.curr_tab_mut().curr_list;
            let half_page = view.mid_win.cols as usize / 2;
            curr_list
                .index
                .map(|x| if x > half_page { x - half_page } else { 0 })
        };
        if let Some(s) = movement {
            cursor_move(s, context, view);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMovePageDown;

impl CursorMovePageDown {
    pub fn new() -> Self {
        CursorMovePageDown
    }
    pub const fn command() -> &'static str {
        "cursor_move_page_down"
    }
}

impl LllCommand for CursorMovePageDown {}

impl std::fmt::Display for CursorMovePageDown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl LllRunnable for CursorMovePageDown {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        let movement: Option<usize> = {
            let curr_list = &mut context.curr_tab_mut().curr_list;
            let dir_len = curr_list.contents.len();
            let half_page = view.mid_win.cols as usize / 2;
            curr_list.index.map(|x| {
                if x + half_page > dir_len - 1 {
                    dir_len - 1
                } else {
                    x + half_page
                }
            })
        };

        if let Some(s) = movement {
            cursor_move(s, context, view);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveHome;

impl CursorMoveHome {
    pub fn new() -> Self {
        CursorMoveHome
    }
    pub const fn command() -> &'static str {
        "cursor_move_home"
    }
}

impl LllCommand for CursorMoveHome {}

impl std::fmt::Display for CursorMoveHome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl LllRunnable for CursorMoveHome {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        let movement: Option<usize> = {
            let len = context.curr_tab_mut().curr_list.contents.len();
            if len == 0 {
                None
            } else {
                Some(0)
            }
        };

        if let Some(s) = movement {
            cursor_move(s, context, view);
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct CursorMoveEnd;

impl CursorMoveEnd {
    pub fn new() -> Self {
        CursorMoveEnd
    }
    pub const fn command() -> &'static str {
        "cursor_move_end"
    }
}

impl LllCommand for CursorMoveEnd {}

impl std::fmt::Display for CursorMoveEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", Self::command())
    }
}

impl LllRunnable for CursorMoveEnd {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        let movement: Option<usize> = {
            let len = context.curr_tab_mut().curr_list.contents.len();
            if len == 0 {
                None
            } else {
                Some(len - 1)
            }
        };

        if let Some(s) = movement {
            cursor_move(s, context, view);
        }
        Ok(())
    }
}
