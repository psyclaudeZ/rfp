use log::{debug, warn};
use rfpp::{pipe, tui};
use std::env;
use std::io::{self, stdin, IsTerminal};
use std::process::Command;

struct Config {
    editor: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: "vim".to_string(),
        }
    }
}

fn main() -> io::Result<()> {
    env_logger::init();

    // TODO: along with others, handle the errors gracefully
    let config = preflight_check()?;

    let candidates = pipe::run()?;
    let paths = tui::run(candidates)?;
    if paths.is_empty() {
        print!("No paths found!");
    } else {
        Command::new(config.editor).args(paths).status()?;
    }
    Ok(())
}

fn preflight_check() -> io::Result<Config> {
    if stdin().is_terminal() {
        // TODO: this
        panic!("TODO: Gracefully handle this i.e. help mejjjssage and exit code.");
    }
    let mut config = Config::default();
    config.editor = env::var("EDITOR").unwrap_or_else(|_| {
        warn!("EDITOR environment variable not set, using vim");
        config.editor
    });
    debug!("Read $EDITOR: {}", config.editor);
    Ok(config)
}
