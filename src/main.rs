extern crate cursive;
extern crate failure;
extern crate lll;

use lll::application;

fn main() {
    if let Err(err) = try_main() {
        let backtrace = err.backtrace().to_string();
        if !backtrace.trim().is_empty() {
            eprintln!("{}", backtrace);
        }
        ::std::process::exit(1);
    }
}

fn try_main() -> Result<(), failure::Error> {
    let mut app = application::app::init().unwrap();
    app.run();
    Ok(())
}
