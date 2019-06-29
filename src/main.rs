extern crate cursive;

use cursive::traits::*;
use cursive::views::{Dialog, DummyView, LinearLayout, SelectView, TextView};
use cursive::Cursive;

use std::fs::{self, DirEntry, File};
use std::io::Read;
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
    view.on_select(update_status).on_submit(load_contents)
}

fn update_status(siv: &mut Cursive, entry: &DirEntry) {
    let mut status_bar = siv.find_id::<TextView>("status").unwrap();
    let file_name = entry.file_name().into_string().unwrap();
    let content = format!("{}", file_name);
    status_bar.set_content(content);
}

fn load_contents(siv: &mut Cursive, entry: &DirEntry) {
    let mut text_view = siv.find_id::<TextView>("contents").unwrap();
    let content = if entry.metadata().unwrap().is_dir() {
        "<DIR>".to_string()
    } else {
        let mut buf = String::new();
        let _ = File::open(entry.file_name())
            .and_then(|mut f| f.read_to_string(&mut buf))
            .map_err(|e| buf = format!("Error: {}", e));
        buf
    };
    text_view.set_content(content);
}

fn main() {
    let mut siv = Cursive::new();
    let mut panes = LinearLayout::horizontal();
    let picker = file_picker(".");
    panes.add_child(picker.fixed_size((30, 25)));
    panes.add_child(DummyView);
    panes.add_child(
        TextView::new("file contents")
            .with_id("contents")
            .fixed_size((50, 25)),
    );
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
