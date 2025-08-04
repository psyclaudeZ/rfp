use crate::parser::FilePathParser;
use log::debug;
use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
// Bruh I don't like it that rust-analyzer isn't smart enough for conditionally compiled test logic.
use std::path::Path;

pub fn run() -> io::Result<Vec<String>> {
    let lines = process_pipe_input();
    run_with_input(lines?)
}

fn run_with_input(lines: Vec<String>) -> io::Result<Vec<String>> {
    let parser = FilePathParser::new();
    let mut matches = vec![];
    let mut seen: HashSet<String> = HashSet::new();

    for line in lines {
        if let Some(match_result) = parser.match_line(&line) {
            debug!(
                "Matched: {} on line {:?}",
                match_result.path, match_result.line_number
            );
            if file_exists(&match_result.path) && !seen.contains(&match_result.path) {
                seen.insert(match_result.path.clone());
                matches.push(match_result.path);
            }
        }
    }
    Ok(matches)
}

#[cfg(not(test))]
fn file_exists(path: &str) -> bool {
    Path::new(path).try_exists().unwrap_or(false)
}

#[cfg(test)]
fn file_exists(_path: &str) -> bool {
    true
}

fn process_pipe_input() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    // TODO: come back to double check the perf for large input
    let reader = BufReader::new(stdin.lock());
    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use crate::pipe::run_with_input;

    fn assert_helper(input: Vec<&str>, expected: Vec<&str>) {
        let res = run_with_input(input.iter().map(|s| s.to_string()).collect());
        let expected_strings: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
        assert_eq!(res.unwrap(), expected_strings);
    }

    #[test]
    fn produces_no_duplicates() {
        assert_helper(vec!["abc/d/e.rs", "abc/d/e.rs"], vec!["abc/d/e.rs"]);
    }

    #[test]
    fn order_is_retained() {
        assert_helper(vec!["c.rs", "b.rs", "a.rs"], vec!["c.rs", "b.rs", "a.rs"]);
    }
}
