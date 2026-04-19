//! GCC Output Parser
//! Parses GCC compiler output

use crate::core::{Issue, OutputParser};
use crate::plugins::cpp::parser::{CppParser, CompilerType};

pub struct GccParser {
    inner: CppParser,
}

impl GccParser {
    pub fn new() -> Self {
        Self {
            inner: CppParser::with_gcc(),
        }
    }
}

impl Default for GccParser {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputParser for GccParser {
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
