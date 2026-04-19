//! Clang Output Parser
//! Parses Clang compiler output

use crate::core::{Issue, OutputParser};
use crate::plugins::cpp::parser::{CppParser, CompilerType};

pub struct ClangParser {
    inner: CppParser,
}

impl ClangParser {
    pub fn new() -> Self {
        Self {
            inner: CppParser::with_clang(),
        }
    }
}

impl Default for ClangParser {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputParser for ClangParser {
    fn parse(&self, output: &str) -> Vec<Issue> {
        self.inner.parse(output)
    }

    fn is_issue_start(&self, line: &str) -> bool {
        self.inner.is_issue_start(line)
    }

    fn parse_issue(&self, lines: &[String], start_index: usize) -> (Option<Issue>, usize) {
        self.inner.parse_issue(lines, start_index)
    }
}
