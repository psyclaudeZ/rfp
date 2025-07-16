use crate::parser::FilePathParser;
use log::debug;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<Vec<String>> {
    let lines = process_pipe_input();
    let parser = FilePathParser::new();
    let mut matches = vec![];

    for line in lines? {
        if let Some(match_result) = parser.match_line(&line) {
            debug!(
                "Matched: {} on line {:?}",
                match_result.path, match_result.line_number
            );
            matches.push(match_result.path);
        }
    }
    Ok(matches)
}

fn process_pipe_input() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    // TODO: come back to double check the perf for large input
    let reader = BufReader::new(stdin.lock());
    return reader.lines().collect();
}
