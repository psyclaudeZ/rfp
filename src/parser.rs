use regex::Regex;
// use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
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
                    line_number,
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{FilePathParser, MatchResult};

    #[test]
    fn can_match_standard_path_no_line() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("/abc/def/g.e").unwrap(),
            MatchResult {
                path: String::from("/abc/def/g.e"),
                line_number: None,
            }
        );
        assert_eq!(
            parser.match_line("abcdefg.e").unwrap(),
            MatchResult {
                path: String::from("abcdefg.e"),
                line_number: None,
            }
        );
    }

    #[test]
    fn can_match_standard_path_with_line_number() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("/abc/def/g.e:123").unwrap(),
            MatchResult {
                path: String::from("/abc/def/g.e"),
                line_number: Some(123),
            }
        );
    }

    #[test]
    fn cannot_match_line_without_extension() {
        let parser = FilePathParser::new();
        assert_eq!(parser.match_line("/abc/def:123"), None);
    }
}
