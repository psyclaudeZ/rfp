use log::{debug, warn};
use rfpp::tui::TUILoopEvent;
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
    let (paths, action) = tui::run(candidates)?;
    if paths.is_empty() {
        if action == TUILoopEvent::Submit {
            println!("No paths selected.");
        } else if action == TUILoopEvent::EarlyReturn {
            println!("No paths found.");
        } else if action == TUILoopEvent::Interrupted {
            println!("Interrupted.");
        }
    } else {
        Command::new(config.editor).args(paths).status()?;
    }
    Ok(())
}

fn preflight_check() -> io::Result<Config> {
    if stdin().is_terminal() {
        eprintln!("Error: No input provided. Please pipe data to thie command.");
        std::process::exit(2);
    }
    let mut config = Config::default();
    config.editor = env::var("EDITOR").unwrap_or_else(|_| {
        warn!("EDITOR environment variable not set, using vim");
        config.editor
    });
    debug!("Read $EDITOR: {}", config.editor);
    Ok(config)
}
