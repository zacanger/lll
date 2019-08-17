use crate::commands::{LllCommand, LllRunnable};
use crate::context::LllContext;
use crate::error::LllError;
use crate::window::LllView;

#[derive(Clone, Debug)]
pub struct Quit;

impl Quit {
    pub fn new() -> Self {
        Self::default()
    }
    pub const fn command() -> &'static str {
        "quit"
    }

    pub fn quit(context: &mut LllContext) -> Result<(), LllError> {
        if !context.threads.is_empty() {
            let err = std::io::Error::new(
                std::io::ErrorKind::Other,
                "operations running in background, use force_quit to quit",
            );
            Err(LllError::IO(err))
        } else {
            context.exit = true;
            Ok(())
        }
    }
}

impl LllCommand for Quit {}

impl std::fmt::Display for Quit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl LllRunnable for Quit {
    fn execute(&self, context: &mut LllContext, _: &LllView) -> Result<(), LllError> {
        Self::quit(context)
    }
}

impl std::default::Default for Quit {
    fn default() -> Self {
        Quit
    }
}

#[derive(Clone, Debug)]
pub struct ForceQuit;

impl ForceQuit {
    pub fn new() -> Self {
        ForceQuit
    }
    pub const fn command() -> &'static str {
        "force_quit"
    }

    pub fn force_quit(context: &mut LllContext) {
        context.exit = true;
    }
}

impl LllCommand for ForceQuit {}

impl std::fmt::Display for ForceQuit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl LllRunnable for ForceQuit {
    fn execute(&self, context: &mut LllContext, _: &LllView) -> Result<(), LllError> {
        Self::force_quit(context);
        Ok(())
    }
}
