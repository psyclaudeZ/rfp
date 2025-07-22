use crate::parser::FilePathParser;
use log::debug;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn run() -> io::Result<Vec<String>> {
    let lines = process_pipe_input();
    let parser = FilePathParser::new();
    let mut matches = vec![];
    let mut seen: HashSet<String> = HashSet::new();

    for line in lines? {
        if let Some(match_result) = parser.match_line(&line) {
            debug!(
                "Matched: {} on line {:?}",
                match_result.path, match_result.line_number
            );
            if Path::new(&match_result.path).try_exists().unwrap_or(false)
                && seen.insert(match_result.path.clone())
            {
                matches.push(match_result.path);
            }
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
