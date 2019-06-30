extern crate cursive;

use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{LinearLayout, OnEventView, SelectView, TextView};
use cursive::Cursive;
use open::*;

// use std::fs::{self, DirEntry, File};
use std::fs::{self, DirEntry};
// use std::io::Read;
use std::path::Path;

fn file_picker<D>(directory: D) -> SelectView<DirEntry>
where
    D: AsRef<Path>,
{
    let mut view = SelectView::new();
    for entry in fs::read_dir(directory).expect("can't read directory") {
        // TODO: sort:
        // directories
        // dotfiles
        // regular files
        // capitalized comes first

        if let Ok(e) = entry {
            let file_name = e.file_name().into_string().unwrap();
            view.add_item(file_name, e);
        }
    }

    /*
    // TODO: make this work
    // also add l for submit or navigate down
    // also add h for navigate up
    let view = OnEventView::new(view)
        .on_pre_event_inner('k', |s, _| {
            s.select_up(1);
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner('j', |s, _| {
            s.select_down(1);
            Some(EventResult::Consumed(None))
        });
        */

    view.on_submit(open_file)
}

fn open_file(_siv: &mut Cursive, entry: &DirEntry) {
    open::that(entry.path());
}

fn main() {
    let mut siv = Cursive::default();
    siv.load_toml(include_str!("theme.toml")).unwrap();
    siv.add_global_callback('q', Cursive::quit);
    let mut panes = LinearLayout::horizontal();
    let picker = file_picker(".");
    panes.add_child(picker.fixed_size((100, 100)));
    let mut layout = LinearLayout::vertical();
    layout.add_child(panes);
    siv.add_layer(layout);
    siv.run();
}
