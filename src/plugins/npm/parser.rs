//! NPM/Node.js Output Parser
//! Parsing the output of npm/pnpm/yarn lint and type-check

use crate::core::{
    BaseParser, Issue, IssueLevel, Location, OutputParser, ParsedTestOutput,
    TestCase, TestOutputParser, TestStatus, TestSummary,
};

pub struct NpmParser {
    base: BaseParser,
}

impl NpmParser {
    pub fn new() -> Self {
        Self {
            base: BaseParser::new(),
        }
    }

    /// Check if a line is a file path (file paths are on a separate line in ESLint format)
    fn is_file_path_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        // File paths usually contain / or \ and do not begin with a number
        !trimmed.is_empty()
            && (trimmed.contains('/') || trimmed.contains('\\'))
            && !trimmed.chars().next().unwrap_or(' ').is_ascii_digit()
            && !trimmed.starts_with('✖')
            && !trimmed.starts_with('│')
            && !trimmed.starts_with('├')
            && !trimmed.starts_with('└')
            && !trimmed.starts_with("npm error")
            && !trimmed.starts_with("error")
    }

    /// Find the file path corresponding to the current line
    /// Prioritizes the nearest file path line, and looks up if there is none.
    fn find_eslint_file_path(&self, lines: &[String], current_index: usize) -> String {
        // First look up the file path line
        for i in (0..current_index).rev() {
            let line = &lines[i];
            if self.is_file_path_line(line) {
                return line.trim().to_string();
            }
        }
        String::from("unknown")
    }

    fn parse_eslint_format(
        &self,
        lines: &[String],
        start_index: usize,
    ) -> (Option<Issue>, usize) {
        if start_index >= lines.len() {
            return (None, start_index);
        }

        let line = &lines[start_index];
        let trimmed = line.trim();

        // ESLint 格式: " 3:7 warning message rule-name"
        // There may be a space at the beginning of the line, followed by the line number:column number
        let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
        if parts.len() < 2 {
            return (None, start_index + 1);
        }

        // Parsing line numbers (handling leading spaces)
        let line_num = parts[0].trim().parse::<u32>().ok();
        if line_num.is_none() {
            return (None, start_index + 1);
        }

        let rest = parts[1];
        // The column number is followed by the level and message
        // 格式: "7 warning message rule-name"
        let rest_parts: Vec<&str> = rest.splitn(2, |c: char| c.is_whitespace()).collect();
        if rest_parts.len() < 2 {
            return (None, start_index + 1);
        }

        let col_num = rest_parts[0].trim().parse::<u32>().ok();
        let after_col = rest_parts[1].trim();

        // Resolution Levels and Messages
        // 格式: "warning message rule-name" 或 "error message rule-name"
        let level_msg_parts: Vec<&str> = after_col.splitn(2, |c: char| c.is_whitespace()).collect();
        if level_msg_parts.is_empty() {
            return (None, start_index + 1);
        }

        let level_str = level_msg_parts[0].trim();
        let level = match level_str.to_lowercase().as_str() {
            "error" => IssueLevel::Error,
            "warning" | "warn" => IssueLevel::Warning,
            "info" => IssueLevel::Info,
            _ => return (None, start_index + 1),
        };

        // Extract message (remove rule name)
        let message = if level_msg_parts.len() > 1 {
            let msg_and_rule = level_msg_parts[1].trim();
            self.base.extract_message(msg_and_rule)
        } else {
            String::new()
        };

        // Using Improved File Path Finding
        let file_path = self.find_eslint_file_path(lines, start_index);

        let location = if let Some(col) = col_num {
            Location::new(file_path)
                .with_line(line_num.unwrap())
                .with_column(col)
        } else {
            Location::new(file_path).with_line(line_num.unwrap())
        };

        (Some(Issue::new(level, message, location)), start_index + 1)
    }

    fn parse_typescript_format(&self, line: &str) -> Option<Issue> {
        self.base.parse_parentheses_format(line).or_else(|| {
            let parts: Vec<&str> = line.splitn(4, ':').collect();
            if parts.len() >= 3 {
                let file_path = parts[0].trim();
                let line_num = parts[1].trim().parse::<u32>().ok()?;
                let rest = parts[2..].join(":");

                let rest_parts: Vec<&str> =
                    rest.splitn(2, ['-', ':']).collect();
                if rest_parts.len() >= 2 {
                    let col_num = rest_parts[0].trim().parse::<u32>().ok()?;
                    let level_msg = rest_parts[1].trim();

                    let level = self.base.detect_level(level_msg)?;

                    let message = if let Some(colon_pos) = level_msg.find(':') {
                        level_msg[colon_pos + 1..].trim().to_string()
                    } else {
                        level_msg.to_string()
                    };

                    let location = Location::new(file_path.to_string())
                        .with_line(line_num)
                        .with_column(col_num);

                    return Some(Issue::new(level, message, location));
                }
            }
            None
        })
    }

    fn parse_generic_error(&self, line: &str) -> Option<Issue> {
        let trimmed = line.trim();

        if trimmed.to_uppercase().starts_with("ERROR") {
            let message = if let Some(space) = trimmed.find(|c: char| c.is_whitespace()) {
                trimmed[space..].trim().to_string()
            } else {
                trimmed.to_string()
            };

            return Some(Issue::new(
                IssueLevel::Error,
                message,
                Location::new("unknown"),
            ));
        }

        None
    }

    /// Parsing NPM Audit Error Formats
    /// 格式: "npm error code CODE" 或 "npm error message"
    fn parse_npm_error(&self, line: &str) -> Option<Issue> {
        let trimmed = line.trim();

        // 匹配 "npm error code XXX"
        if trimmed.starts_with("npm error code") {
            let code = trimmed.strip_prefix("npm error code").unwrap_or("").trim();
            return Some(Issue::new(
                IssueLevel::Error,
                format!("NPM error code: {}", code),
                Location::new("package.json"),
            ));
        }

        // 匹配 "npm error XXX"（不包括 code 行）
        if trimmed.starts_with("npm error") && !trimmed.starts_with("npm error code") {
            let message = trimmed.strip_prefix("npm error").unwrap_or("").trim();
            // Skip some known non-error message lines
            if message.starts_with("A complete log")
                || message.starts_with("audit")
                || message.is_empty()
            {
                return None;
            }
            return Some(Issue::new(
                IssueLevel::Error,
                message.to_string(),
                Location::new("package.json"),
            ));
        }

        None
    }

    /// Resolving NPM Dependency Missing Errors
    /// 格式: "npm error missing: package@version, required by package@version"
    fn parse_npm_missing_dep(&self, line: &str) -> Option<Issue> {
        let trimmed = line.trim();

        if trimmed.starts_with("npm error missing:") {
            // Extracting missing package information
            let rest = trimmed.strip_prefix("npm error missing:").unwrap_or("").trim();
            return Some(Issue::new(
                IssueLevel::Error,
                format!("Missing dependency: {}", rest),
                Location::new("package.json"),
            ));
        }

        None
    }

    /// Analyzing npm audit security vulnerability reports
    /// Format.
    ///   package  version_range
    ///   Severity: level
    ///   description - https://...
    fn parse_npm_audit_vulnerability(&self, lines: &[String], start_index: usize) -> (Option<Issue>, usize) {
        if start_index >= lines.len() {
            return (None, start_index);
        }

        let line = &lines[start_index];
        let trimmed = line.trim();

        // Check if it is a package name line (format: "package version_range")
        // This is usually the package name and version range before the severity line in the audit report.
        if trimmed.starts_with("Severity:") || trimmed.is_empty() {
            return (None, start_index + 1);
        }

        // Look ahead one row to check if it is a Severity row
        if start_index + 1 >= lines.len() {
            return (None, start_index + 1);
        }

        let next_line = &lines[start_index + 1];
        if !next_line.trim().starts_with("Severity:") {
            return (None, start_index + 1);
        }

        // Extract package name and version range
        let package_info = trimmed.to_string();

        // 解析 severity
        let severity_line = next_line.trim();
        let severity = severity_line
            .strip_prefix("Severity:")
            .unwrap_or("")
            .trim()
            .to_lowercase();

        let level = match severity.as_str() {
            "critical" => IssueLevel::Error,
            "high" => IssueLevel::Error,
            "moderate" => IssueLevel::Warning,
            "low" => IssueLevel::Info,
            _ => IssueLevel::Warning,
        };

        // Collect descriptive information (may be on multiple lines)
        let mut descriptions = Vec::new();
        let mut i = start_index + 2;

        while i < lines.len() {
            let desc_line = &lines[i];
            let desc_trimmed = desc_line.trim();

            // Stop conditions: empty line, next set of vulnerabilities, dependency tree or fix suggestion
            if desc_trimmed.is_empty()
                || desc_trimmed.starts_with("fix available")
                || desc_trimmed.starts_with("node_modules/")
                || desc_trimmed.starts_with("Severity:")
                || (desc_trimmed.contains(" vulnerabilities") && desc_trimmed.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
            {
                break;
            }

            // Collection description (excluding dependency tree rows)
            if !desc_trimmed.starts_with("Depends on vulnerable")
                && !desc_trimmed.starts_with("node_modules/")
            {
                descriptions.push(desc_trimmed.to_string());
            }

            i += 1;
        }

        let message = if descriptions.is_empty() {
            format!("NPM error: Security vulnerability in {}", package_info)
        } else {
            format!(
                "NPM error: Security vulnerability in {} - {}",
                package_info,
                descriptions.join(" ")
            )
        };

        (
            Some(Issue::new(level, message, Location::new("package.json"))),
            i,
        )
    }
}

impl Default for NpmParser {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputParser for NpmParser {
    // Custom parse implementation for NPM output
    fn parse(&self, output: &str) -> Vec<Issue> {
        let lines: Vec<String> = output.lines().map(|s| s.to_string()).collect();
        let mut issues = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = &lines[i];

            // Prioritize parsing of TypeScript formats (formats with parentheses)
            if let Some(issue) = self.parse_typescript_format(line) {
                issues.push(issue);
                i += 1;
                continue;
            }

            // Parsing the ESLint Format
            let (issue, new_index) = self.parse_eslint_format(&lines, i);
            if let Some(issue) = issue {
                issues.push(issue);
                i = new_index;
                continue;
            }

            // Parsing NPM Errors
            if let Some(issue) = self.parse_npm_error(line) {
                issues.push(issue);
                i += 1;
                continue;
            }

            // Resolving NPM Dependency Missing Errors
            if let Some(issue) = self.parse_npm_missing_dep(line) {
                issues.push(issue);
                i += 1;
                continue;
            }

            // Analyzing npm audit security vulnerabilities
            let (audit_issue, new_index) = self.parse_npm_audit_vulnerability(&lines, i);
            if let Some(issue) = audit_issue {
                issues.push(issue);
                i = new_index;
                continue;
            }

            // Generic error analysis
            if let Some(issue) = self.parse_generic_error(line) {
                issues.push(issue);
            }

            i += 1;
        }

        issues
    }
}

impl TestOutputParser for NpmParser {
    fn parse_test_output(&self, output: &str) -> ParsedTestOutput {
        let mut result = ParsedTestOutput::new();

        // 1. Reuse of existing logic to resolve compilation/type-checking issues
        result.compile_issues = <Self as OutputParser>::parse(self, output);

        // 2. Parsing test execution results
        let lines: Vec<&str> = output.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            // Parse Jest test case line: "✓ <name> (<time>)" or "✕ <name> (<time>)"
            if let Some(test_case) = self.parse_jest_test_line(line) {
                match test_case.status {
                    TestStatus::Passed => result.passed_tests.push(test_case),
                    TestStatus::Failed => result.failed_tests.push(test_case),
                    TestStatus::Ignored(_) => result.ignored_tests.push(test_case),
                }
                i += 1;
                continue;
            }

            // Parsing Vitest test case line: " ✓ <name> <time>"
            if let Some(test_case) = self.parse_vitest_test_line(line) {
                match test_case.status {
                    TestStatus::Passed => result.passed_tests.push(test_case),
                    TestStatus::Failed => result.failed_tests.push(test_case),
                    TestStatus::Ignored(_) => result.ignored_tests.push(test_case),
                }
                i += 1;
                continue;
            }

            // Parsing Mocha Test Case Lines
            if let Some(test_case) = self.parse_mocha_test_line(line) {
                match test_case.status {
                    TestStatus::Passed => result.passed_tests.push(test_case),
                    TestStatus::Failed => result.failed_tests.push(test_case),
                    TestStatus::Ignored(_) => result.ignored_tests.push(test_case),
                }
                i += 1;
                continue;
            }

            // Parsing Jest Test Results Summary
            if line.starts_with("Tests:") {
                result.test_summary = self.parse_jest_summary(line);
            }

            // Analyzing Vitest Test Results Summary
            if line.contains("Test Files") && line.contains("tests") {
                result.test_summary = self.parse_vitest_summary(&lines, i);
            }

            // Analyzing Mocha Test Results Summary
            if line.starts_with("  ") && line.contains(" passing") && line.contains(" failing") {
                result.test_summary = self.parse_mocha_summary(line);
            }

            i += 1;
        }

        result
    }
}

impl NpmParser {
    /// Parsing Jest Test Case Lines
    /// 格式: "✓ test name (5 ms)" 或 "✕ test name (5 ms)"
    fn parse_jest_test_line(&self, line: &str) -> Option<TestCase> {
        let trimmed = line.trim();

        // 通过: "✓ test name"
        if let Some(name) = trimmed.strip_prefix('✓') {
            let name = name.trim();
            // 提取时间: "test name (5 ms)"
            let (name, time) = self.extract_time_from_name(name);
            return Some(TestCase {
                name,
                status: TestStatus::Passed,
                location: None,
                failure_details: None,
                execution_time: time,
            });
        }

        // 失败: "✕ test name"
        if let Some(name) = trimmed.strip_prefix('✕') {
            let name = name.trim();
            let (name, time) = self.extract_time_from_name(name);
            return Some(TestCase {
                name,
                status: TestStatus::Failed,
                location: None,
                failure_details: None,
                execution_time: time,
            });
        }

        // 跳过: "○ test name"
        if let Some(name) = trimmed.strip_prefix('○') {
            let name = name.trim();
            return Some(TestCase {
                name: name.to_string(),
                status: TestStatus::Ignored(None),
                location: None,
                failure_details: None,
                execution_time: None,
            });
        }

        None
    }

    /// Parsing Vitest Test Case Lines
    /// 格式: " ✓ test name 5ms" 或 " ✗ test name 5ms"
    fn parse_vitest_test_line(&self, line: &str) -> Option<TestCase> {
        let trimmed = line.trim();

        // 通过: "✓ test name 5ms"
        if trimmed.starts_with("✓ ") {
            let rest = &trimmed[2..];
            let (name, time) = self.extract_vitest_time(rest);
            return Some(TestCase {
                name,
                status: TestStatus::Passed,
                location: None,
                failure_details: None,
                execution_time: time,
            });
        }

        // 失败: "✗ test name 5ms"
        if trimmed.starts_with("✗ ") {
            let rest = &trimmed[2..];
            let (name, time) = self.extract_vitest_time(rest);
            return Some(TestCase {
                name,
                status: TestStatus::Failed,
                location: None,
                failure_details: None,
                execution_time: time,
            });
        }

        // 跳过: "⏭ test name"
        if trimmed.starts_with("⏭ ") {
            let name = trimmed[2..].trim().to_string();
            return Some(TestCase {
                name,
                status: TestStatus::Ignored(None),
                location: None,
                failure_details: None,
                execution_time: None,
            });
        }

        None
    }

    /// Parsing Mocha Test Case Lines
    /// 格式: " ✓ test name" 或 " 1) test name"
    fn parse_mocha_test_line(&self, line: &str) -> Option<TestCase> {
        let trimmed = line.trim();

        // 通过: "✓ test name"
        if trimmed.starts_with("✓ ") {
            let name = trimmed[2..].trim().to_string();
            return Some(TestCase {
                name,
                status: TestStatus::Passed,
                location: None,
                failure_details: None,
                execution_time: None,
            });
        }

        // 失败: "1) test name"
        if let Some(caps) = regex::Regex::new(r"^\d+\)\s+(.+)$").ok()?.captures(trimmed) {
            let name = caps.get(1)?.as_str().to_string();
            return Some(TestCase {
                name,
                status: TestStatus::Failed,
                location: None,
                failure_details: None,
                execution_time: None,
            });
        }

        None
    }

    /// Extract time from test name
    /// 格式: "test name (5 ms)" -> ("test name", Some(0.005))
    fn extract_time_from_name(&self, name: &str) -> (String, Option<f64>) {
        if let Some(start) = name.rfind("(") {
            if let Some(end) = name[start..].find(")") {
                let time_str = &name[start + 1..start + end];
                // Parsing "5 ms" or "0.5 s"
                let time = if time_str.contains("ms") {
                    time_str
                        .trim()
                        .strip_suffix("ms")
                        .unwrap_or("")
                        .trim()
                        .parse::<f64>()
                        .map(|t| t / 1000.0)
                        .ok()
                } else if time_str.contains('s') {
                    time_str
                        .trim()
                        .strip_suffix('s')
                        .unwrap_or("")
                        .trim()
                        .parse::<f64>()
                        .ok()
                } else {
                    None
                };
                return (name[..start].trim().to_string(), time);
            }
        }
        (name.to_string(), None)
    }

    /// Extracting time from Vitest rows
    /// 格式: "test name 5ms" -> ("test name", Some(0.005))
    fn extract_vitest_time(&self, rest: &str) -> (String, Option<f64>) {
        // lookup time from the end
        let parts: Vec<&str> = rest.split_whitespace().collect();
        if parts.len() >= 2 {
            let last = parts.last().unwrap();
            if last.ends_with("ms") {
                if let Ok(time) = last.strip_suffix("ms").unwrap_or("").parse::<f64>() {
                    let name = parts[..parts.len() - 1].join(" ");
                    return (name, Some(time / 1000.0));
                }
            }
        }
        (rest.to_string(), None)
    }

    /// Parsing Jest Test Results Summary
    /// 格式: "Tests: 5 passed, 1 failed, 2 skipped, 10 total"
    fn parse_jest_summary(&self, line: &str) -> Option<TestSummary> {
        let re = regex::Regex::new(
            r"Tests:\s+(\d+)\s+passed,?\s*(?:(\d+)\s+failed,?)?\s*(?:(\d+)\s+skipped,?)?\s*(?:(\d+)\s+total)?",
        )
        .ok()?;

        let caps = re.captures(line)?;

        let passed: usize = caps.get(1)?.as_str().parse().ok()?;
        let failed: usize = caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
        let ignored: usize = caps.get(3).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);

        Some(TestSummary {
            total: passed + failed + ignored,
            passed,
            failed,
            ignored,
            measured: 0,
            filtered: 0,
            execution_time: None,
        })
    }

    /// Analyzing Vitest Test Results Summary
    /// Format Multiline.
    /// " Test Files  1 passed (1)"
    /// "      Tests  5 passed (5)"
    fn parse_vitest_summary(&self, lines: &[&str], start_index: usize) -> Option<TestSummary> {
        let mut passed = 0;
        let mut failed = 0;
        let ignored = 0;

        let passed_regex = regex::Regex::new(r"Tests\s+(\d+)\s+passed").ok()?;
        let failed_regex = regex::Regex::new(r"(\d+)\s+failed").ok()?;

        // Look back a few rows from the current row
        for line in lines.iter().skip(start_index).take(5) {
            // 匹配 "Tests 5 passed (5)"
            if let Some(caps) = passed_regex.captures(line) {
                passed = caps.get(1)?.as_str().parse().ok()?;
            }
            // Number of failed matches
            if let Some(caps) = failed_regex.captures(line) {
                failed = caps.get(1)?.as_str().parse().ok()?;
            }
        }

        Some(TestSummary {
            total: passed + failed + ignored,
            passed,
            failed,
            ignored,
            measured: 0,
            filtered: 0,
            execution_time: None,
        })
    }

    /// Analyzing Mocha Test Results Summary
    /// 格式: " 5 passing (10ms)" 或 " 5 passing (10ms)\n 1 failing"
    fn parse_mocha_summary(&self, line: &str) -> Option<TestSummary> {
        let re = regex::Regex::new(r"(\d+)\s+passing").ok()?;
        let caps = re.captures(line)?;
        let passed: usize = caps.get(1)?.as_str().parse().ok()?;

        let failed_re = regex::Regex::new(r"(\d+)\s+failing").ok()?;
        let failed = failed_re
            .captures(line)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);

        Some(TestSummary {
            total: passed + failed,
            passed,
            failed,
            ignored: 0,
            measured: 0,
            filtered: 0,
            execution_time: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_typescript_parentheses() {
        let parser = NpmParser::new();
        let line = "src/file.ts(10,5): error TS1234: Message";
        let issue = parser.parse_typescript_format(line).expect("Failed to parse");

        assert_eq!(issue.location.file_path, "src/file.ts");
        assert_eq!(issue.location.line_number, Some(10));
        assert_eq!(issue.location.column_number, Some(5));
        assert_eq!(issue.message, "Message");
        assert!(matches!(issue.level, IssueLevel::Error));
        assert!(issue.code.is_some());
    }

    #[test]
    fn test_parse_eslint_format() {
        let parser = NpmParser::new();
        let lines = vec![
            "/path/to/file.js".to_string(),
            "  10:5  error  Message  rule-name".to_string(),
        ];

        let (issue, next_index) = parser.parse_eslint_format(&lines, 1);
        let issue = issue.unwrap();

        assert_eq!(issue.location.file_path, "/path/to/file.js");
        assert_eq!(issue.location.line_number, Some(10));
        assert_eq!(issue.location.column_number, Some(5));
        assert_eq!(issue.message, "Message");
        assert_eq!(next_index, 2);
    }
}
