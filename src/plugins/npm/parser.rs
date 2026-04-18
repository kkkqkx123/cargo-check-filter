//! NPM/Node.js 输出解析器
//! 解析 npm/pnpm/yarn lint 和 type-check 的输出

use crate::core::{
    BaseParser, Issue, IssueLevel, Location, OutputParser, ParsedTestOutput, TestCase,
    TestOutputParser, TestStatus, TestSummary,
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

    /// 检查一行是否是文件路径（ESLint 格式中文件路径单独一行）
    fn is_file_path_line(&self, line: &str) -> bool {
        let trimmed = line.trim();
        // 文件路径通常包含 / 或 \，且不以数字开头
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

    /// 查找当前行对应的文件路径
    /// 优先查找最近的文件路径行，如果没有则向上查找
    fn find_eslint_file_path(&self, lines: &[String], current_index: usize) -> String {
        // 首先向上查找文件路径行
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

        // ESLint 格式: "  3:7   warning  message  rule-name"
        // 行首可能有空格，然后是行号:列号
        let parts: Vec<&str> = trimmed.splitn(2, ':').collect();
        if parts.len() < 2 {
            return (None, start_index + 1);
        }

        // 解析行号（处理前导空格）
        let line_num = parts[0].trim().parse::<u32>().ok();
        if line_num.is_none() {
            return (None, start_index + 1);
        }

        let rest = parts[1];
        // 列号后面跟着级别和消息
        // 格式: "7   warning  message  rule-name"
        let rest_parts: Vec<&str> = rest.splitn(2, |c: char| c.is_whitespace()).collect();
        if rest_parts.len() < 2 {
            return (None, start_index + 1);
        }

        let col_num = rest_parts[0].trim().parse::<u32>().ok();
        let after_col = rest_parts[1].trim();

        // 解析级别和消息
        // 格式: "warning  message  rule-name" 或 "error  message  rule-name"
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

        // 提取消息（移除规则名）
        let message = if level_msg_parts.len() > 1 {
            let msg_and_rule = level_msg_parts[1].trim();
            self.base.extract_message(msg_and_rule)
        } else {
            String::new()
        };

        // 使用改进的文件路径查找
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
                    rest.splitn(2, |c: char| c == '-' || c == ':').collect();
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

    /// 解析 NPM Audit 错误格式
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
            // 跳过一些已知的非错误信息行
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

    /// 解析 NPM 依赖缺失错误
    /// 格式: "npm error missing: package@version, required by package@version"
    fn parse_npm_missing_dep(&self, line: &str) -> Option<Issue> {
        let trimmed = line.trim();

        if trimmed.starts_with("npm error missing:") {
            // 提取缺失的包信息
            let rest = trimmed.strip_prefix("npm error missing:").unwrap_or("").trim();
            return Some(Issue::new(
                IssueLevel::Error,
                format!("Missing dependency: {}", rest),
                Location::new("package.json"),
            ));
        }

        None
    }
}

impl Default for NpmParser {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputParser for NpmParser {
    fn parse(&self, output: &str) -> Vec<Issue> {
        let lines: Vec<String> = output.lines().map(|s| s.to_string()).collect();
        let mut issues = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = &lines[i];

            // 优先解析 TypeScript 格式（带括号的格式）
            if let Some(issue) = self.parse_typescript_format(line) {
                issues.push(issue);
                i += 1;
                continue;
            }

            // 解析 ESLint 格式
            let (issue, new_index) = self.parse_eslint_format(&lines, i);
            if let Some(issue) = issue {
                issues.push(issue);
                i = new_index;
                continue;
            }

            // 解析 NPM 错误
            if let Some(issue) = self.parse_npm_error(line) {
                issues.push(issue);
                i += 1;
                continue;
            }

            // 解析 NPM 依赖缺失错误
            if let Some(issue) = self.parse_npm_missing_dep(line) {
                issues.push(issue);
                i += 1;
                continue;
            }

            // 通用错误解析
            if let Some(issue) = self.parse_generic_error(line) {
                issues.push(issue);
            }

            i += 1;
        }

        issues
    }

    fn is_issue_start(&self, line: &str) -> bool {
        let trimmed = line.trim();

        // ESLint 格式：数字开头，格式为 "line:col level message"
        if trimmed.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            if trimmed.contains(':') {
                let parts: Vec<&str> = trimmed.split(':').collect();
                if parts.len() >= 2 {
                    let after_first = parts[1].trim();
                    return after_first
                        .chars()
                        .next()
                        .map(|c| c.is_ascii_digit())
                        .unwrap_or(false);
                }
            }
        }

        // TypeScript 格式：文件路径后跟括号
        if trimmed.contains(".ts(")
            || trimmed.contains(".tsx(")
            || trimmed.contains(".js(")
            || trimmed.contains(".jsx(")
        {
            return true;
        }

        // NPM 错误格式
        if trimmed.starts_with("npm error") {
            return true;
        }

        // 通用错误格式
        trimmed.to_uppercase().starts_with("ERROR")
    }

    fn parse_issue(&self, lines: &[String], start_index: usize) -> (Option<Issue>, usize) {
        if start_index >= lines.len() {
            return (None, start_index);
        }

        let line = &lines[start_index];

        if let Some(issue) = self.parse_typescript_format(line) {
            return (Some(issue), start_index + 1);
        }

        // 解析 NPM 错误
        if let Some(issue) = self.parse_npm_error(line) {
            return (Some(issue), start_index + 1);
        }

        // 解析 NPM 依赖缺失
        if let Some(issue) = self.parse_npm_missing_dep(line) {
            return (Some(issue), start_index + 1);
        }

        self.parse_eslint_format(lines, start_index)
    }
}

impl TestOutputParser for NpmParser {
    fn parse_test_output(&self, output: &str) -> ParsedTestOutput {
        let mut result = ParsedTestOutput::new();

        // 1. 复用现有逻辑解析编译/类型检查问题
        result.compile_issues = self.parse(output);

        // 2. 解析测试执行结果
        let lines: Vec<&str> = output.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            // 解析 Jest 测试用例行: "✓ <name> (<time>)" 或 "✕ <name> (<time>)"
            if let Some(test_case) = self.parse_jest_test_line(line) {
                match test_case.status {
                    TestStatus::Passed => result.passed_tests.push(test_case),
                    TestStatus::Failed => result.failed_tests.push(test_case),
                    TestStatus::Ignored(_) => result.ignored_tests.push(test_case),
                }
                i += 1;
                continue;
            }

            // 解析 Vitest 测试用例行: " ✓ <name> <time>"
            if let Some(test_case) = self.parse_vitest_test_line(line) {
                match test_case.status {
                    TestStatus::Passed => result.passed_tests.push(test_case),
                    TestStatus::Failed => result.failed_tests.push(test_case),
                    TestStatus::Ignored(_) => result.ignored_tests.push(test_case),
                }
                i += 1;
                continue;
            }

            // 解析 Mocha 测试用例行
            if let Some(test_case) = self.parse_mocha_test_line(line) {
                match test_case.status {
                    TestStatus::Passed => result.passed_tests.push(test_case),
                    TestStatus::Failed => result.failed_tests.push(test_case),
                    TestStatus::Ignored(_) => result.ignored_tests.push(test_case),
                }
                i += 1;
                continue;
            }

            // 解析 Jest 测试结果汇总
            if line.starts_with("Tests:") {
                result.test_summary = self.parse_jest_summary(line);
            }

            // 解析 Vitest 测试结果汇总
            if line.contains("Test Files") && line.contains("tests") {
                result.test_summary = self.parse_vitest_summary(&lines, i);
            }

            // 解析 Mocha 测试结果汇总
            if line.starts_with("  ") && line.contains(" passing") && line.contains(" failing") {
                result.test_summary = self.parse_mocha_summary(line);
            }

            i += 1;
        }

        result
    }
}

impl NpmParser {
    /// 解析 Jest 测试用例行
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

    /// 解析 Vitest 测试用例行
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

    /// 解析 Mocha 测试用例行
    /// 格式: "    ✓ test name" 或 "    1) test name"
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

    /// 从测试名称中提取时间
    /// 格式: "test name (5 ms)" -> ("test name", Some(0.005))
    fn extract_time_from_name(&self, name: &str) -> (String, Option<f64>) {
        if let Some(start) = name.rfind("(") {
            if let Some(end) = name[start..].find(")") {
                let time_str = &name[start + 1..start + end];
                // 解析 "5 ms" 或 "0.5 s"
                let time = if time_str.contains("ms") {
                    time_str
                        .trim()
                        .trim_end_matches("ms")
                        .trim()
                        .parse::<f64>()
                        .map(|t| t / 1000.0)
                        .ok()
                } else if time_str.contains('s') {
                    time_str
                        .trim()
                        .trim_end_matches('s')
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

    /// 从 Vitest 行中提取时间
    /// 格式: "test name 5ms" -> ("test name", Some(0.005))
    fn extract_vitest_time(&self, rest: &str) -> (String, Option<f64>) {
        // 从末尾查找时间
        let parts: Vec<&str> = rest.split_whitespace().collect();
        if parts.len() >= 2 {
            let last = parts.last().unwrap();
            if last.ends_with("ms") {
                if let Ok(time) = last[..last.len() - 2].parse::<f64>() {
                    let name = parts[..parts.len() - 1].join(" ");
                    return (name, Some(time / 1000.0));
                }
            }
        }
        (rest.to_string(), None)
    }

    /// 解析 Jest 测试结果汇总
    /// 格式: "Tests: 5 passed, 1 failed, 2 skipped, 10 total"
    fn parse_jest_summary(&self, line: &str) -> Option<TestSummary> {
        let re = regex::Regex::new(
            r"Tests:\s+(\d+)\s+passed,?\s*(?:(\d+)\s+failed,?)?\s*(?:(\d+)\s+skipped,?)?\s*(?:(\d+)\s+total)?",
        )
        .ok()?;

        let caps = re.captures(line)?;

        let passed: usize = caps.get(1)?.as_str().parse().ok()?;
        let failed: usize = caps.get(2).map(|m| m.as_str().parse().ok()).flatten().unwrap_or(0);
        let ignored: usize = caps.get(3).map(|m| m.as_str().parse().ok()).flatten().unwrap_or(0);

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

    /// 解析 Vitest 测试结果汇总
    /// 格式多行:
    /// " Test Files  1 passed (1)"
    /// "      Tests  5 passed (5)"
    fn parse_vitest_summary(&self, lines: &[&str], start_index: usize) -> Option<TestSummary> {
        let mut passed = 0;
        let mut failed = 0;
        let mut ignored = 0;

        // 从当前行开始向后查找几行
        for i in start_index..(start_index + 5).min(lines.len()) {
            let line = lines[i];

            // 匹配 "Tests  5 passed (5)"
            if let Some(caps) = regex::Regex::new(r"Tests\s+(\d+)\s+passed")
                .ok()?
                .captures(line)
            {
                passed = caps.get(1)?.as_str().parse().ok()?;
            }
            // 匹配失败数
            if let Some(caps) = regex::Regex::new(r"(\d+)\s+failed").ok()?.captures(line) {
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

    /// 解析 Mocha 测试结果汇总
    /// 格式: "  5 passing (10ms)" 或 "  5 passing (10ms)\n  1 failing"
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
