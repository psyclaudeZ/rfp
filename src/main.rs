use log::{debug, warn};
use rfpp::tui::TUILoopEvent;
use rfpp::{pipe, tui};
use std::env;
use std::io::{self, IsTerminal, stdin};
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

type ExitCode = i32;

const EXIT_ERROR: ExitCode = 1;
const EXIT_INTERRUPTED: ExitCode = 130;

fn main() -> io::Result<()> {
    env_logger::init();

    let config = preflight_check().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(EXIT_ERROR);
    });

    let candidates = pipe::run().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(EXIT_ERROR);
    });
    let (paths, action) = tui::run(candidates).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(EXIT_ERROR);
    });

    if paths.is_empty() {
        if action == TUILoopEvent::Submit {
            println!("No paths selected.");
        } else if action == TUILoopEvent::EarlyReturn {
            println!("No paths found.");
        } else if action == TUILoopEvent::Interrupted {
            println!("Interrupted.");
            std::process::exit(EXIT_INTERRUPTED);
        }
    } else {
        Command::new(config.editor).args(paths).status()?;
    }
    Ok(())
}

fn preflight_check() -> Result<Config, Box<dyn std::error::Error>> {
    if stdin().is_terminal() {
        return Err("No input provided. Please pipe data to thie command.".into());
    }
    let mut config = Config::default();
    config.editor = env::var("EDITOR").unwrap_or_else(|_| {
        warn!("EDITOR environment variable not set, using vim");
        config.editor
    });
    debug!("Read $EDITOR: {}", config.editor);
    Ok(config)
}
