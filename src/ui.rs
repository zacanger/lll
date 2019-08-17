use std::fs;

use users::mock::{Groups, Users};
use users::UsersCache;

use crate::config::{LllColorTheme, LllConfig};
use crate::context::LllContext;
use crate::fs::{LllDirEntry, LllDirList};
use crate::window;

use crate::THEME_T;

pub const ERR_COLOR: i16 = 240;
pub const EMPTY_COLOR: i16 = 241;

const MIN_WIN_WIDTH: usize = 4;

pub struct DisplayOptions {
    pub detailed: bool,
}

pub const PRIMARY_DISPLAY_OPTION: DisplayOptions = DisplayOptions { detailed: true };

pub fn init_ncurses() {
    ncurses::setlocale(ncurses::LcCategory::all, "");

    ncurses::initscr();
    ncurses::cbreak();

    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::start_color();
    ncurses::use_default_colors();
    ncurses::noecho();
    ncurses::set_escdelay(0);
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    process_theme();

    ncurses::refresh();
}

fn process_theme() {
    for pair in THEME_T.colorpair.iter() {
        ncurses::init_pair(pair.id, pair.fg, pair.bg);
    }

    // error message
    ncurses::init_pair(ERR_COLOR, ncurses::COLOR_RED, -1);
    // empty
    ncurses::init_pair(EMPTY_COLOR, ncurses::COLOR_WHITE, ncurses::COLOR_RED);
}

pub fn end_ncurses() {
    ncurses::endwin();
}

pub fn getmaxyx() -> (i32, i32) {
    let mut term_rows: i32 = 0;
    let mut term_cols: i32 = 0;
    ncurses::getmaxyx(ncurses::stdscr(), &mut term_rows, &mut term_cols);
    (term_rows, term_cols)
}

pub fn display_menu(win: &window::LllPanel, vals: &[String]) {
    ncurses::werase(win.win);
    ncurses::mvwhline(win.win, 0, 0, 0, win.cols);

    for (i, val) in vals.iter().enumerate() {
        ncurses::wmove(win.win, (i + 1) as i32, 0);
        ncurses::waddstr(win.win, val.as_str());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_msg(win: &window::LllPanel, msg: &str) {
    ncurses::werase(win.win);
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_err(win: &window::LllPanel, msg: &str) {
    let attr = ncurses::A_BOLD() | ncurses::COLOR_PAIR(ERR_COLOR);

    ncurses::werase(win.win);
    ncurses::wattron(win.win, attr);

    ncurses::mvwaddstr(win.win, 0, 0, msg);

    ncurses::wattroff(win.win, attr);
    ncurses::wnoutrefresh(win.win);
}

pub fn wprint_empty(win: &window::LllPanel, msg: &str) {
    ncurses::werase(win.win);
    ncurses::wattron(win.win, ncurses::COLOR_PAIR(EMPTY_COLOR));
    ncurses::mvwaddstr(win.win, 0, 0, msg);
    ncurses::wattroff(win.win, ncurses::COLOR_PAIR(EMPTY_COLOR));
    ncurses::wnoutrefresh(win.win);
}

fn wprint_file_name(
    win: &window::LllPanel,
    file_name: &str,
    coord: (i32, i32),
    mut space_avail: usize,
) {
    let name_visual_space = unicode_width::UnicodeWidthStr::width(file_name);
    if name_visual_space < space_avail {
        ncurses::mvwaddstr(win.win, coord.0, coord.1, &file_name);
        return;
    }
    if let Some(ext) = file_name.rfind('.') {
        let extension: &str = &file_name[ext..];
        let ext_len = unicode_width::UnicodeWidthStr::width(extension);
        if space_avail > ext_len {
            space_avail -= ext_len;
            ncurses::mvwaddstr(win.win, coord.0, space_avail as i32, &extension);
        }
    }
    if space_avail < 2 {
        return;
    } else {
        space_avail -= 2;
    }

    ncurses::wmove(win.win, coord.0, coord.1);

    let mut trim_index: usize = file_name.len();

    let mut total: usize = 0;
    for (index, ch) in file_name.char_indices() {
        if total >= space_avail {
            trim_index = index;
            break;
        }
        total += unicode_width::UnicodeWidthChar::width(ch).unwrap_or(2);
    }
    ncurses::waddstr(win.win, &file_name[..trim_index]);
    ncurses::waddstr(win.win, "…");
}

fn wprint_entry(
    win: &window::LllPanel,
    file: &LllDirEntry,
    prefix: (usize, &str),
    coord: (i32, i32),
) {
    if win.cols <= prefix.0 as i32 {
        return;
    }
    ncurses::waddstr(win.win, prefix.1);
    let space_avail = win.cols as usize - prefix.0;

    wprint_file_name(
        &win,
        file.file_name(),
        (coord.0, coord.1 + prefix.0 as i32),
        space_avail,
    );
}

fn wprint_entry_detailed(
    win: &window::LllPanel,
    file: &LllDirEntry,
    prefix: (usize, &str),
    coord: (i32, i32),
) {
    if win.cols <= prefix.0 as i32 {
        return;
    }
    ncurses::waddstr(win.win, prefix.1);
    let space_avail = win.cols as usize - prefix.0;

    let coord = (coord.0, coord.1 + prefix.0 as i32);

    wprint_file_name(win, file.file_name(), coord, space_avail);
}

pub fn display_contents(
    win: &window::LllPanel,
    dirlist: &mut LllDirList,
    config_t: &LllConfig,
    options: &DisplayOptions,
) {
    if win.cols < MIN_WIN_WIDTH as i32 {
        return;
    }
    let dir_len = dirlist.contents.len();
    if dir_len == 0 {
        wprint_empty(win, "empty");
        return;
    }
    ncurses::werase(win.win);
    ncurses::wmove(win.win, 0, 0);

    let draw_func = if options.detailed {
        wprint_entry_detailed
    } else {
        wprint_entry
    };

    let curr_index = dirlist.index.unwrap();
    dirlist
        .pagestate
        .update_page_state(curr_index, win.rows, dir_len, config_t.scroll_offset);

    let (start, end) = (dirlist.pagestate.start, dirlist.pagestate.end);
    let dir_contents = &dirlist.contents[start..end];

    ncurses::werase(win.win);
    ncurses::wmove(win.win, 0, 0);

    for (i, entry) in dir_contents.iter().enumerate() {
        let coord: (i32, i32) = (i as i32, 0);

        ncurses::wmove(win.win, coord.0, coord.1);

        let attr = if i + start == curr_index {
            ncurses::A_STANDOUT()
        } else {
            0
        };
        let attrs = get_theme_attr(attr, entry);

        draw_func(win, entry, attrs.0, coord);

        ncurses::mvwchgat(win.win, coord.0, coord.1, -1, attrs.1, attrs.2);
    }
    win.queue_for_refresh();
}

pub fn wprint_file_status(win: &window::LllPanel, entry: &LllDirEntry) {
    ncurses::waddch(win.win, ' ' as ncurses::chtype);

    let usercache: UsersCache = UsersCache::new();
    match usercache.get_user_by_uid(entry.metadata.uid) {
        Some(s) => match s.name().to_str() {
            Some(name) => ncurses::waddstr(win.win, name),
            None => ncurses::waddstr(win.win, "OsStr error"),
        },
        None => ncurses::waddstr(win.win, "unknown user"),
    };
    ncurses::waddch(win.win, ' ' as ncurses::chtype);
    match usercache.get_group_by_gid(entry.metadata.gid) {
        Some(s) => match s.name().to_str() {
            Some(name) => ncurses::waddstr(win.win, name),
            None => ncurses::waddstr(win.win, "OsStr error"),
        },
        None => ncurses::waddstr(win.win, "unknown user"),
    };

    ncurses::waddstr(win.win, "  ");
    wprint_file_info(win.win, entry);
    win.queue_for_refresh();
}

pub fn wprint_file_info(win: ncurses::WINDOW, file: &LllDirEntry) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mode = file.metadata.permissions.mode();

        ncurses::waddch(win, ' ' as ncurses::chtype);

        if file.file_path().is_dir() {
            let is_link: u32 = libc::S_IFLNK as u32;
            if mode >> 9 & is_link >> 9 == mode >> 9 {
                if let Ok(path) = fs::read_link(&file.file_path()) {
                    ncurses::waddstr(win, " -> ");
                    ncurses::waddstr(win, path.to_str().unwrap());
                }
            }
        }
    }
}

pub fn redraw_tab_view(win: &window::LllPanel, context: &LllContext) {
    let tab_len = context.tabs.len();
    ncurses::werase(win.win);
    if tab_len == 1 {
    } else if tab_len >= 6 {
        ncurses::wmove(win.win, 0, 0);
        ncurses::wattron(win.win, ncurses::A_BOLD() | ncurses::A_STANDOUT());
        ncurses::waddstr(win.win, &format!("{}", context.curr_tab_index + 1));
        ncurses::wattroff(win.win, ncurses::A_STANDOUT());
        ncurses::waddstr(win.win, &format!(" {}", tab_len));
        ncurses::wattroff(win.win, ncurses::A_BOLD());
    } else {
        ncurses::wattron(win.win, ncurses::A_BOLD());
        for i in 0..tab_len {
            if i == context.curr_tab_index {
                ncurses::wattron(win.win, ncurses::A_STANDOUT());
                ncurses::waddstr(win.win, &format!("{} ", i + 1));
                ncurses::wattroff(win.win, ncurses::A_STANDOUT());
            } else {
                ncurses::waddstr(win.win, &format!("{} ", i + 1));
            }
        }
        ncurses::wattroff(win.win, ncurses::A_BOLD());
    }
    ncurses::wnoutrefresh(win.win);
}

pub fn show_fs_operation_progress(win: &window::LllPanel, process_info: &fs_extra::TransitProcess) {
    let percentage: f64 = process_info.copied_bytes as f64 / process_info.total_bytes as f64;

    let cols: i32 = (f64::from(win.cols) * percentage) as i32;
    ncurses::mvwchgat(
        win.win,
        0,
        0,
        cols,
        ncurses::A_STANDOUT(),
        THEME_T.selection.colorpair,
    );
    win.queue_for_refresh();
}

pub fn get_theme_attr(
    mut attr: ncurses::attr_t,
    entry: &LllDirEntry,
) -> ((usize, &str), ncurses::attr_t, i16) {
    use std::os::unix::fs::FileTypeExt;

    let theme: &LllColorTheme;
    let colorpair: i16;

    let file_type = &entry.metadata.file_type;
    if entry.is_selected() {
        theme = &THEME_T.selection;
        colorpair = THEME_T.selection.colorpair;
    } else if file_type.is_dir() {
        theme = &THEME_T.directory;
        colorpair = THEME_T.directory.colorpair;
    } else if file_type.is_symlink() {
        theme = &THEME_T.link;
        colorpair = THEME_T.link.colorpair;
    } else if file_type.is_block_device()
        || file_type.is_char_device()
        || file_type.is_fifo()
        || file_type.is_socket()
    {
        theme = &THEME_T.socket;
        colorpair = THEME_T.link.colorpair;
    } else {
        if let Some(ext) = entry.file_name().rfind('.') {
            let extension: &str = &entry.file_name()[ext + 1..];
            if let Some(s) = THEME_T.ext.get(extension) {
                theme = &s;
                colorpair = theme.colorpair;
            } else {
                theme = &THEME_T.regular;
                colorpair = theme.colorpair;
            }
        } else {
            theme = &THEME_T.regular;
            colorpair = theme.colorpair;
        }
    }

    if theme.bold {
        attr |= ncurses::A_BOLD();
    }
    if theme.underline {
        attr |= ncurses::A_UNDERLINE();
    }

    let prefix = match theme.prefix.as_ref() {
        Some(p) => (p.size(), p.prefix()),
        None => (1, " "),
    };

    (prefix, attr, colorpair)
}
