use log::{debug, warn};
use rfpp::{pipe, tui};
use std::env;
use std::io::{self, stdin, IsTerminal};

fn main() -> io::Result<()> {
    env_logger::init();

    // TODO: along with others, handle the errors gracefully
    preflight_check()?;

    let candidates = pipe::run()?;
    tui::run(&candidates)?;
    Ok(())
}

fn preflight_check() -> io::Result<()> {
    if stdin().is_terminal() {
        // TODO: this
        panic!("TODO: Gracefully handle this i.e. help message and exit code.");
    }
    #[warn(unused_variables)]
    let editor = env::var("EDITOR").unwrap_or_else(|_| {
        warn!("EDITOR environment variable not set, using vim");
        "vim".to_string()
    });
    debug!("Read $EDITOR: {}", editor);
    Ok(())
}
