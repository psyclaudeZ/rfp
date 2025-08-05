use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct MatchResult {
    pub path: String,
    pub line_number: Option<u32>,
}

pub struct FilePathParser {
    regex_configs: Vec<RegexConfig>,
}

impl Default for FilePathParser {
    fn default() -> Self {
        Self::new()
    }
}

struct RegexConfig {
    regex: Regex,
    line_number_idx: usize,
}

impl FilePathParser {
    pub fn new() -> Self {
        // TODO(bz): single files, files/paths with spaces
        let regex_configs = vec![
            // Homedir paths. ~/a/b/c.ext:123
            RegexConfig {
                regex: Regex::new(
                    r"(~/([a-zA-Z0-9._-]+/)*[a-zA-Z0-9._-]+(\.[a-zA-Z0-9]{1,42})?)[:-]?(\d+)?",
                )
                .unwrap(),
                line_number_idx: 4,
            },
            // Standard paths, w/ or w/o extension. a/b/c.ext:123
            RegexConfig {
                regex: Regex::new(
                    r"(/?([a-zA-Z0-9._-]+/)+[a-zA-Z0-9._-]+(\.[a-zA-Z0-9]{1,42})?)[:-]?(\d+)?",
                )
                .unwrap(),
                line_number_idx: 4,
            },
            // Single file with extension
            RegexConfig {
                regex: Regex::new(r"(/?[a-zA-Z0-9._-]+\.[a-zA-Z0-9]{1,42})[:-]?(\d+)?").unwrap(),
                line_number_idx: 2,
            },
        ];
        Self { regex_configs }
    }

    pub fn match_line(&self, line: &str) -> Option<MatchResult> {
        // -> Option<MatchResult> {
        for regex_config in &self.regex_configs {
            let RegexConfig {
                regex,
                line_number_idx,
            } = regex_config;

            if let Some(captures) = regex.captures(line) {
                let path = self.post_processing(captures.get(1)?.as_str());
                let line_number = captures
                    .get(*line_number_idx)
                    .and_then(|m| m.as_str().parse().ok());

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
    fn can_match_standard_path_no_line_number() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("/abc/def/g.e").unwrap(),
            MatchResult {
                path: String::from("/abc/def/g.e"),
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
    fn can_match_standard_path_without_extension() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("/abc/def/g").unwrap(),
            MatchResult {
                path: String::from("/abc/def/g"),
                line_number: None,
            }
        );
    }

    #[test]
    fn can_match_homedir_default() {
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
    fn can_match_homedir_single_file_with_extension() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("~/file.rs:42").unwrap(),
            MatchResult {
                path: String::from("~/file.rs"),
                line_number: Some(42),
            }
        );
    }

    #[test]
    fn can_match_homedir_single_file_without_extension() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("~/file").unwrap(),
            MatchResult {
                path: String::from("~/file"),
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
    fn can_match_a_single_file_at_root_with_extension() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser.match_line("/file.rs").unwrap(),
            MatchResult {
                path: String::from("/file.rs"),
                line_number: None,
            }
        );
    }

    #[test]
    fn can_match_a_file_with_long_extension() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser
                .match_line("f.l.i.l.e.asomehowsuperduperlongextension")
                .unwrap(),
            MatchResult {
                path: String::from("f.l.i.l.e.asomehowsuperduperlongextension"),
                line_number: None,
            }
        );
    }

    #[test]
    fn can_match_in_an_error_message() {
        let parser = FilePathParser::new();
        assert_eq!(
            parser
                .match_line("error: file not found in project/file.txt")
                .unwrap(),
            MatchResult {
                path: String::from("project/file.txt"),
                line_number: None,
            }
        );
    }
}
