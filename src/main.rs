use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
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

use regex::Regex;
// use std::path::Path;

#[derive(Debug)]
pub struct MatchResult {
    pub path: String,
    pub line_number: Option<u32>,
}

pub struct FilePathParser {
    patterns: Vec<Regex>,
}

impl FilePathParser {
    pub fn new() -> Self {
        let patterns = vec![
            // Standard path. a/b/c.ext:123
            Regex::new(r"(/?([a-zA-Z0-9._-]+/)*[a-zA-Z0-9._-]+\.[a-zA-Z0-9]{1,10})[:-]?(\d+)?")
                .unwrap(),
        ];
        Self { patterns }
    }

    pub fn match_line(&self, line: &str) -> Option<MatchResult> {
        // -> Option<MatchResult> {
        for pattern in &self.patterns {
            if let Some(captures) = pattern.captures(line) {
                let path = captures.get(1)?.as_str();
                let line_number = captures.get(3).and_then(|m| m.as_str().parse().ok());

                return Some(MatchResult {
                    path: path.to_string(),
                    line_number: line_number,
                });
            }
        }
        None
    }
}
