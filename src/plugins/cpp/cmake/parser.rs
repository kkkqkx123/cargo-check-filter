//! CMake Output Parser
//! Parses CMake configuration and build output

use regex::Regex;
use crate::core::{Issue, IssueLevel, Location, OutputParser};
use crate::plugins::cpp::parser::{CppParser, CompilerType};

pub struct CMakeParser {
    cmake_error_regex: Regex,
    cmake_warning_regex: Regex,
}

impl CMakeParser {
    pub fn new() -> Self {
        let cmake_error_regex = Regex::new(
            r"CMake Error at\s+(.*?):(\d+)\s*\((.*?)\):\s*(.*)"
        ).unwrap();

        let cmake_warning_regex = Regex::new(
            r"CMake Warning at\s+(.*?):(\d+)\s*\((.*?)\):\s*(.*)"
        ).unwrap();

        Self {
            cmake_error_regex,
            cmake_warning_regex,
        }
    }

    fn parse_cmake_errors(&self, output: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        for line in output.lines() {
            // Parse CMake errors
            if let Some(caps) = self.cmake_error_regex.captures(line) {
                let file_path = caps[1].to_string();
                let line_num = caps[2].parse::<u32>().ok();
                let _command = &caps[3];
                let message = caps[4].to_string();

                let mut location = Location::new(file_path);
                if let Some(ln) = line_num {
                    location = location.with_line(ln);
                }

                let issue = Issue::new(IssueLevel::Error, message, location)
                    .with_code("CMake Error");
                issues.push(issue);
            }

            // Parse CMake warnings
            if let Some(caps) = self.cmake_warning_regex.captures(line) {
                let file_path = caps[1].to_string();
                let line_num = caps[2].parse::<u32>().ok();
                let _command = &caps[3];
                let message = caps[4].to_string();

                let mut location = Location::new(file_path);
                if let Some(ln) = line_num {
                    location = location.with_line(ln);
                }

                let issue = Issue::new(IssueLevel::Warning, message, location)
                    .with_code("CMake Warning");
                issues.push(issue);
            }
        }

        issues
    }

    fn detect_compiler_type(&self, output: &str) -> CompilerType {
        CppParser::detect_compiler_type(output)
    }
}

impl Default for CMakeParser {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputParser for CMakeParser {
    fn parse(&self, output: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Parse CMake configuration errors
        issues.extend(self.parse_cmake_errors(output));

        // Parse compiler errors from build output
        let compiler_type = self.detect_compiler_type(output);
        let cpp_parser = CppParser::new(compiler_type);
        issues.extend(cpp_parser.parse(output));

        issues
    }

    fn is_issue_start(&self, line: &str) -> bool {
        // CMake errors
        if line.starts_with("CMake Error") || line.starts_with("CMake Warning") {
            return true;
        }

        // Compiler errors - try generic detection
        line.contains(": error:")
            || line.contains(": warning:")
            || line.contains("(error")
            || line.contains("(warning")
            || line.contains("(fatal error")
    }

    fn parse_issue(&self, lines: &[String], start_index: usize) -> (Option<Issue>, usize) {
        if start_index >= lines.len() {
            return (None, start_index);
        }

        let line = &lines[start_index];
        let issues = self.parse(line);

        if let Some(issue) = issues.into_iter().next() {
            (Some(issue), start_index + 1)
        } else {
            (None, start_index + 1)
        }
    }
}
