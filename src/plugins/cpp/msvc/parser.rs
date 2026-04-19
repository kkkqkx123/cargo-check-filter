//! MSVC Output Parser
//! Parses Microsoft Visual C++ compiler output

use crate::core::{Issue, OutputParser, StreamingOutputParser};
use crate::plugins::cpp::parser::CppParser;

pub struct MsvcParser {
    inner: CppParser,
}

impl MsvcParser {
    pub fn new() -> Self {
        Self {
            inner: CppParser::with_msvc(),
        }
    }
}

impl Default for MsvcParser {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputParser for MsvcParser {
    fn parse(&self, output: &str) -> Vec<Issue> {
        // Use OutputParser::parse explicitly to avoid ambiguity
        <CppParser as OutputParser>::parse(&self.inner, output)
    }
}

impl StreamingOutputParser for MsvcParser {
    fn is_issue_start(&self, line: &str) -> bool {
        self.inner.is_issue_start(line)
    }

    fn parse_issue(&self, lines: &[String], start_index: usize) -> (Option<Issue>, usize) {
        self.inner.parse_issue(lines, start_index)
    }
}
