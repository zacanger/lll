use crate::commands::FileOperationThread;
use crate::config;
use crate::tab::LllTab;

pub struct LllContext {
    pub threads: Vec<FileOperationThread<u64, fs_extra::TransitProcess>>,
    pub curr_tab_index: usize,
    pub tabs: Vec<LllTab>,
    pub exit: bool,

    pub config_t: config::LllConfig,
}

impl LllContext {
    pub fn new(config_t: config::LllConfig) -> Self {
        LllContext {
            threads: Vec::new(),
            curr_tab_index: 0,
            tabs: Vec::new(),
            exit: false,
            config_t,
        }
    }
    pub fn curr_tab_ref(&self) -> &LllTab {
        &self.tabs[self.curr_tab_index]
    }
    pub fn curr_tab_mut(&mut self) -> &mut LllTab {
        &mut self.tabs[self.curr_tab_index]
    }
}
