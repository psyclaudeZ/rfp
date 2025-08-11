use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub trait Matcher {
    fn match_line(&self, line: &str) -> Option<MatchResult>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct MatchResult {
    pub path: String,
    pub line_number: Option<u32>,
}

struct RegexConfig {
    regex: Regex,
    line_number_idx: usize,
}

pub struct RegexMatcher {
    regex_configs: Vec<RegexConfig>,
}

impl Default for RegexMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Matcher for RegexMatcher {
    fn match_line(&self, line: &str) -> Option<MatchResult> {
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
}

impl RegexMatcher {
    pub fn new() -> Self {
        // TODO(bz): files/paths with spaces
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

    fn post_processing<'a>(&self, line: &'a str) -> &'a str {
        // git diff
        if line.starts_with("a/") || line.starts_with("b/") {
            return &line[2..];
        }
        line
    }
}

lazy_static! {
    static ref SINGLE_FILE_REGEX: Regex = Regex::new(r"\b([a-zA-Z0-9_-]+)[:-]?(\d+)?\b").unwrap();
}

/// This is a matcher specialized for matching single files (as opposed to paths), particularly
/// extension-less single files since ones with extension have already been captured by the
/// RegexMatcher.
///
/// Unlike how fpp handles the case, this matcher uses a heuristic where if a single file is
/// supposed to show up in the selection view, it's very likely that the file is located exactly at
/// the cwd. Therefore, it simply caches all the files in cwd and checks agsint each line.
pub struct SingleFileMatcher {
    cached_single_files: HashSet<String>,
}

impl Default for SingleFileMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl SingleFileMatcher {
    pub fn new() -> Self {
        let cwd = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(_) => {
                eprintln!(
                    "Warning: Could not get current directory, unable to match single files without extension"
                );
                return Self {
                    cached_single_files: HashSet::new(),
                };
            }
        };
        let single_files = std::fs::read_dir(cwd)
            .map(|read_dir| {
                read_dir
                    .filter_map(|res| res.ok())
                    .filter(|e| e.path().is_file())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .filter(|p| !p.contains("."))
                    .collect::<HashSet<String>>()
            })
            .unwrap_or_else(|_| HashSet::new());

        Self {
            cached_single_files: single_files,
        }
    }
}

impl Matcher for SingleFileMatcher {
    fn match_line(&self, line: &str) -> Option<MatchResult> {
        for capture in SINGLE_FILE_REGEX.captures_iter(line) {
            let word = capture.get(1)?.as_str();
            if self.cached_single_files.contains(word) {
                let line_number = capture.get(2).and_then(|m| m.as_str().parse().ok());
                return Some(MatchResult {
                    path: word.to_string(),
                    line_number,
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{MatchResult, Matcher, RegexMatcher, SingleFileMatcher};

    #[test]
    fn can_match_standard_path_no_line_number() {
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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
        let parser = RegexMatcher::new();
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

    /// Assuming cwd is at the root of the project, which seems to be an invariant.
    #[test]
    fn can_match_single_extensionless_file_in_the_directory() {
        let parser = SingleFileMatcher::new();
        assert_eq!(
            parser
                .match_line("you might want to read the LICENSE")
                .unwrap(),
            MatchResult {
                path: String::from("LICENSE"),
                line_number: None,
            }
        );
    }
}
