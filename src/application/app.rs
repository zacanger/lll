use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use cursive::align::*;
use cursive::event::{Event, EventResult, Key};
use cursive::traits::{Boxable, Identifiable};
use cursive::views::*;
use cursive::Cursive;

use alphanumeric_sort::compare_os_str;
use walkdir;
use walkdir::{DirEntry, WalkDir};

use crate::ui::MultiSelectView;
use crate::ui::Tab;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn path_info<P: AsRef<Path> + Display>(path: P) -> String {
    format!("{}", path)
}

pub fn init() -> Result<App, failure::Error> {
    let path = env::current_dir()?;
    let mut app = App::new()?;
    app.add_tab(1, path)?;
    app.load_bindings();
    Ok(app)
}

/// The data structure holding various elements related to `App`.
pub struct App {
    pub siv: Cursive,
    pub vec_tabs: Rc<RefCell<HashMap<u32, Tab>>>,
    focused_tab: usize,
    focused_entry: usize,
}

impl App {
    pub fn new() -> Result<Self, failure::Error> {
        let mut siv = Cursive::default();
        siv.load_toml(include_str!("theme.toml")).unwrap();

        // Create empty views
        let c_widget = MultiSelectView::<PathBuf>::new();
        let c_widget = OnEventView::new(c_widget).with_id("current");
        let top_widget = LinearLayout::horizontal().child(
            TextView::new(path_info("/"))
                .h_align(HAlign::Left)
                .with_id("topbar/left")
                .full_width(),
        );

        // Horizontal panes
        let mut panes = LinearLayout::horizontal();

        panes.add_child(
            Panel::new(c_widget)
                .full_width()
                .max_width(50)
                .full_height(),
        );
        let h_panes = LinearLayout::vertical()
            .child(top_widget.full_width())
            .child(panes);

        siv.add_layer(h_panes);
        siv.add_global_callback('q', |s| s.quit());
        let vec_tabs = Rc::new(RefCell::new(HashMap::<u32, Tab>::new()));

        Ok(Self {
            siv,
            vec_tabs,
            focused_entry: 0,
            focused_tab: 0,
        })
    }

    /// Funtion to load key-bindings.
    pub fn load_bindings(&mut self) {
        let v_clone = self.vec_tabs.clone();
        self.siv.add_global_callback('h', move |s: &mut Cursive| {
            // Get current_view selection index
            let mut current_selection = None;
            s.call_on_id(
                "current",
                |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                    let view = event_view.get_inner();
                    current_selection = view.selected_id();
                },
            );
            if let Some(mut tab) = v_clone.borrow_mut().get_mut(&1) {
                tab.focused.insert(
                    PathBuf::from(&tab.current_view),
                    current_selection.unwrap_or(0),
                );
                tab.go_back();
                // tab.c_focused = current_selection;
                App::update_tab(s, &mut tab);
                s.call_on_id("topbar/left", |view: &mut TextView| {
                    let mut text: TextContent = view.get_shared_content();
                    text.set_content(format!(" {}", tab.current_view.to_str().unwrap()));
                });
            };
        });

        let v_clone2 = self.vec_tabs.clone();
        self.siv.add_global_callback('l', move |s: &mut Cursive| {
            s.call_on_id(
                "current",
                |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                    let event = event_view.get_inner_mut();
                    if let Some(path) = event.selection() {
                        if path.is_dir() {
                            if let Some(tab) = v_clone2.borrow_mut().get_mut(&1) {
                                tab.go_forward(path.to_path_buf());
                            };
                        } // if
                    };
                },
            );
            if let Some(mut tab) = v_clone2.borrow_mut().get_mut(&1) {
                App::update_tab(s, &mut tab);
                s.call_on_id("topbar/left", |view: &mut TextView| {
                    let mut text = view.get_shared_content();
                    text.set_content(format!(" {}", tab.current_view.to_str().unwrap()));
                });
            }
        });

        self.siv.call_on_id(
            "current",
            |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                event_view.set_on_pre_event_inner('k', |s| {
                    let cb = s.select_up(1);
                    Some(EventResult::Consumed(Some(cb)))
                });
                event_view.set_on_pre_event_inner('j', |s| {
                    let cb = s.select_down(1);
                    Some(EventResult::Consumed(Some(cb)))
                });
            },
        );

        self.siv.add_global_callback('/', |_c: &mut Cursive| {
            // filter based on entered text
            unimplemented!()
        });

        // cancel current action
        self.siv
            .add_global_callback(Event::Key(Key::Esc), |s: &mut Cursive| {
                let mut exists: bool = false;
                {
                    let stack_view = s.screen_mut();
                    if let Some(_data) = stack_view.get(LayerPosition::FromBack(1)) {
                        exists = true;
                    }
                }
                if exists {
                    s.pop_layer();
                }
            });
    }

    /// Add new tab to main view.
    pub fn add_tab(&mut self, name: u32, path: PathBuf) -> Result<(), failure::Error> {
        let mut tab = Tab::from(name, &path)?;
        self.siv.call_on_id("topbar/center", |view: &mut TextView| {
            let mut current_text: TextContent = view.get_shared_content();
            current_text.set_content(format!(" {}", path.to_str().unwrap()));
        });
        self.siv.call_on_id(
            "current",
            |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                let view = event_view.get_inner_mut();
                view.clear();
                for entry in App::get_path_iter(&tab.current_view)
                    .filter_entry(|e| e.path().is_dir() && !is_hidden(e))
                {
                    let entry = entry.unwrap();
                    match entry.file_name().to_str() {
                        Some(c) => view.add_item(format!(r"{}/", c), PathBuf::from(entry.path())),
                        None => {}
                    }
                }
                for entry in App::get_path_iter(&tab.current_view)
                    .filter_entry(|e| e.path().is_file() && !is_hidden(e))
                {
                    let entry = entry.unwrap();
                    match entry.file_name().to_str() {
                        Some(c) => view.add_item(format!(r"{}", c), PathBuf::from(entry.path())),
                        None => {}
                    };
                }
                view.set_selection(0);
            },
        );

        let mut i: usize = 0;
        self.siv
            .call_on_id("parent", |view: &mut MultiSelectView<PathBuf>| {
                view.clear();
                match tab.parent_view.to_str() {
                    Some("root") => {
                        view.add_item("/", PathBuf::from("/"));
                        view.set_enabled(false);
                        view.set_selection(0);
                    }
                    Some(_) | None => {
                        for (index, entry) in App::get_path_iter(&tab.parent_view)
                            .filter_entry(|e| e.path().is_dir() && !is_hidden(e))
                            .enumerate()
                        {
                            let entry = entry.unwrap();
                            if entry.path() == &tab.current_view {
                                i = index;
                            }
                            match entry.file_name().to_str() {
                                Some(c) => {
                                    view.add_item(format!("{}/", c), PathBuf::from(entry.path()))
                                }
                                None => {}
                            };
                        }
                        for entry in App::get_path_iter(&tab.parent_view)
                            .filter_entry(|e| e.path().is_file() && !is_hidden(e))
                        {
                            let entry = entry.unwrap();
                            match entry.file_name().to_str() {
                                Some(c) => {
                                    view.add_item(format!("{}", c), PathBuf::from(entry.path()))
                                }
                                None => {}
                            };
                        }
                        view.set_selection(i);
                        view.set_enabled(false);
                    } // None
                };
            });
        tab.focused.insert(PathBuf::from(&tab.parent_view), i);
        // tab.p_focused = i;
        self.vec_tabs.borrow_mut().insert(1, tab);
        self.focused_entry = i;
        Ok(())
    }

    /// Update contents of tab when navigating up and down
    fn update_tab(siv: &mut Cursive, tab: &mut Tab) {
        siv.call_on_id(
            "current",
            |event_view: &mut OnEventView<MultiSelectView<PathBuf>>| {
                let c_focused = tab.focused.get(&tab.current_view).unwrap_or(&0usize);
                let view = event_view.get_inner_mut();
                view.clear();
                for entry in App::get_path_iter(&tab.current_view)
                    .filter_entry(|e| e.path().is_dir() && !is_hidden(e))
                {
                    let entry = entry.unwrap();
                    match entry.file_name().to_str() {
                        Some(c) => view.add_item(format!(r"{}/", c), PathBuf::from(entry.path())),
                        None => {}
                    }
                }
                for entry in App::get_path_iter(&tab.current_view)
                    .filter_entry(|e| e.path().is_file() && !is_hidden(e))
                {
                    let entry = entry.unwrap();
                    match entry.file_name().to_str() {
                        Some(c) => view.add_item(format!(r"{}", c), PathBuf::from(entry.path())),
                        None => {}
                    };
                }
                // TODO keep last selection
                view.set_selection(*c_focused);
                //view.set_selection(focused);
            },
        );

        let mut i: usize = 0;
        siv.call_on_id("parent", |view: &mut MultiSelectView<PathBuf>| {
            view.clear();
            match tab.parent_view.to_str() {
                Some("root") => {
                    view.add_item("/", PathBuf::from("/"));
                    view.set_selection(0);
                }
                Some(_) | None => {
                    for (index, entry) in App::get_path_iter(&tab.parent_view)
                        .filter_entry(|e| e.path().is_dir() && !is_hidden(e))
                        .enumerate()
                    {
                        let entry = entry.unwrap();
                        if entry.path() == &tab.current_view {
                            i = index;
                        }
                        match entry.file_name().to_str() {
                            Some(c) => {
                                view.add_item(format!("{}/", c), PathBuf::from(entry.path()))
                            }
                            None => {}
                        };
                    }
                    for entry in App::get_path_iter(&tab.parent_view)
                        .filter_entry(|e| e.path().is_file() && !is_hidden(e))
                    {
                        let entry = entry.unwrap();
                        match entry.file_name().to_str() {
                            Some(c) => view.add_item(format!("{}", c), PathBuf::from(entry.path())),
                            None => {}
                        };
                    }
                    view.set_selection(i);
                }
            }
        });
        // tab.p_focused = i;
        tab.focused.insert(PathBuf::from(&tab.parent_view), i);
    }

    fn get_path_iter(path: &PathBuf) -> walkdir::IntoIter {
        WalkDir::new(path)
            .max_depth(1)
            .min_depth(1)
            .sort_by(|a, b| compare_os_str(&a.file_name(), &b.file_name()))
            .into_iter()
    }

    pub fn run(&mut self) {
        self.siv.run();
    }
}
