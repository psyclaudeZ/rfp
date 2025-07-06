use log::{debug, warn};
use rfpp::parser::FilePathParser;
use std::env;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    env_logger::init();
    #[warn(unused_variables)]
    let editor = env::var("EDITOR").unwrap_or_else(|_| {
        warn!("EDITOR environment variable not set, using vim");
        "vim".to_string()
    });
    debug!("Read $EDITOR: {}", editor);

    let lines = process_input();
    let parser = FilePathParser::new();

    for line in lines? {
        if let Some(match_result) = parser.match_line(&line) {
            println!(
                "Matched: {} on line {:?}",
                match_result.path, match_result.line_number
            );
        }
    }
    Ok(())
}

fn process_input() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    // TODO: come back to double check the perf for large input
    let reader = BufReader::new(stdin.lock());
    return reader.lines().collect();
}
