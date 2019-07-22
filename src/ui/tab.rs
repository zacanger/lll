use crate::error::*;
use std::collections::HashMap;
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Tab {
    pub title: u32,

    // Views
    // TODO: get rid of parent_view, just calculate on navigation
    pub parent_view: PathBuf,
    pub current_view: PathBuf,

    // Selected
    // TODO Currently, based on index, need to change to PathBuf
    pub focused: HashMap<PathBuf, usize>,
}

impl Tab {
    // create tab with name and path
    pub fn from(title: u32, path: &PathBuf) -> Result<Self> {
        // TODO: get rid of this funky error wrapper
        let parent_view: PathBuf = match path.to_str() {
            Some("/") => PathBuf::from("root"),
            Some(e) => PathBuf::from(e)
                .parent()
                .ok_or(ErrorKind::DirNotFound {
                    dirname: format!("Parent for {:?}", e),
                })?
                .to_path_buf(),
            None => PathBuf::new(),
        };
        let current_view: PathBuf = PathBuf::from(path);

        Ok(Self {
            title,
            parent_view,
            current_view,

            focused: HashMap::new(),
            // p_focused: 0,
            // c_focused: None,
        })
    }

    pub fn go_back(&mut self) {
        let temp_path = PathBuf::from(&self.parent_view);
        match temp_path.to_str() {
            Some("/") => {
                self.parent_view = PathBuf::from("root");
                self.current_view = PathBuf::from("/")
            }
            Some("root") => {}
            Some(c) => {
                let path = PathBuf::from(c);
                match path.parent() {
                    Some(d) => {
                        self.parent_view = d.to_path_buf();
                        self.current_view = PathBuf::from(c);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    pub fn go_forward(&mut self, path: PathBuf) {
        self.current_view = PathBuf::from(&path);
        self.parent_view = path.parent().unwrap().to_path_buf();
    }
}
