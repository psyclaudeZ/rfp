use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct MatchResult {
    pub path: String,
    pub line_number: Option<u32>,
}

pub struct FilePathParser {
    patterns: Vec<Regex>,
}

impl Default for FilePathParser {
    fn default() -> Self {
        Self::new()
    }
}

impl FilePathParser {
    pub fn new() -> Self {
        // TODO(bz): single files, files with spaces
        let patterns = vec![
            // Standard homedir path. ~/a/b/c.ext:123
            Regex::new(r"(~/([a-zA-Z0-9._-]+/)*[a-zA-Z0-9._-]+\.[a-zA-Z0-9]{1,10})[:-]?(\d+)?")
                .unwrap(),
            // Standard path with extension. a/b/c.ext:123
            Regex::new(r"(/?([a-zA-Z0-9._-]+/)*[a-zA-Z0-9._-]+\.[a-zA-Z0-9]{1,10})[:-]?(\d+)?")
                .unwrap(),
        ];
        Self { patterns }
    }

    pub fn match_line(&self, line: &str) -> Option<MatchResult> {
        // -> Option<MatchResult> {
        for pattern in &self.patterns {
            if let Some(captures) = pattern.captures(line) {
                let path = self.post_processing(captures.get(1)?.as_str());
                let line_number = captures.get(3).and_then(|m| m.as_str().parse().ok());

                return Some(MatchResult {
                    path: path.to_string(),
                    line_number,
                });
            }
        }
        None
    }

    fn post_processing<'a>(&self, line: &'a str) -> &'a str {
        // git diff
        if line.starts_with("a/") || line.starts_with("b/") {
            return &line[2..];
        }
        line
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
    fn can_match_homedir() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("~/a/b/c.rs").unwrap(),
            MatchResult {
                path: String::from("~/a/b/c.rs"),
                line_number: None,
            }
        );
    }

    #[test]
    fn can_match_a_single_file_with_extension() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("file.rs").unwrap(),
            MatchResult {
                path: String::from("file.rs"),
                line_number: None,
            }
        );
    }

    #[test]
    fn can_match_git_diff_path() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("a/abc/d/e.rs:123").unwrap(),
            MatchResult {
                path: String::from("abc/d/e.rs"),
                line_number: Some(123),
            }
        );
    }
}
