extern crate cursive;

use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
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
        if let Ok(e) = entry {
            let file_name = e.file_name().into_string().unwrap();
            view.add_item(file_name, e);
        }
    }
    view.on_select(update_status).on_submit(open_file)
}

fn update_status(siv: &mut Cursive, entry: &DirEntry) {
    let mut status_bar = siv.find_id::<TextView>("status").unwrap();
    let file_name = entry.file_name().into_string().unwrap();
    let content = format!("{}", file_name);
    status_bar.set_content(content);
}

fn open_file(_siv: &mut Cursive, entry: &DirEntry) {
    open::that(entry.path());
}

fn main() {
    let mut siv = Cursive::new();
    let mut panes = LinearLayout::horizontal();
    let picker = file_picker(".");
    panes.add_child(picker.fixed_size((100, 100)));
    panes.add_child(DummyView);
    let mut layout = LinearLayout::vertical();
    layout.add_child(panes);
    layout.add_child(
        TextView::new("status")
            .scrollable(false)
            .with_id("status")
            .fixed_size((80, 1)),
    );
    siv.add_layer(Dialog::around(layout).button("Quit", |a| a.quit()));
    siv.run();
}
