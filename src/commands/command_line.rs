use crate::commands::{self, LllCommand, LllRunnable};
use crate::context::LllContext;
use crate::error::LllError;
use crate::textfield::LllTextField;
use crate::ui;
use crate::window::LllView;

#[derive(Clone, Debug)]
pub struct CommandLine {
    pub prefix: String,
    pub suffix: String,
}

impl CommandLine {
    pub fn new(prefix: String, suffix: String) -> Self {
        CommandLine { prefix, suffix }
    }
    pub const fn command() -> &'static str {
        "console"
    }

    pub fn readline(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        const PROMPT: &str = ":";
        let (term_rows, term_cols) = ui::getmaxyx();
        let user_input: Option<String> = {
            let textfield = LllTextField::new(
                1,
                term_cols,
                (term_rows as usize - 1, 0),
                PROMPT,
                &self.prefix,
                &self.suffix,
            );
            textfield.readline()
        };

        if let Some(s) = user_input {
            let trimmed = s.trim_start();
            match trimmed.find(' ') {
                Some(ind) => {
                    let (command, xs) = trimmed.split_at(ind);
                    let xs = xs.trim_start();
                    let wexp = wordexp::wordexp(xs, wordexp::Wordexp::new(0), 0);
                    let args: Vec<&str> = match wexp.as_ref() {
                        Ok(wexp) => wexp.iter().collect(),
                        Err(_) => Vec::new(),
                    };
                    match commands::from_args(command, &args) {
                        Ok(s) => s.execute(context, view),
                        Err(e) => Err(LllError::Keymap(e)),
                    }
                }
                None => match commands::from_args(trimmed, &Vec::new()) {
                    Ok(s) => s.execute(context, view),
                    Err(e) => Err(LllError::Keymap(e)),
                },
            }
        } else {
            Ok(())
        }
    }
}

impl LllCommand for CommandLine {}

impl std::fmt::Display for CommandLine {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {} {}", Self::command(), self.prefix, self.suffix)
    }
}

impl LllRunnable for CommandLine {
    fn execute(&self, context: &mut LllContext, view: &LllView) -> Result<(), LllError> {
        let res = self.readline(context, view);
        ncurses::doupdate();
        res
    }
}
