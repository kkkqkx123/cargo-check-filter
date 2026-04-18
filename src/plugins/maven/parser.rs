//! Maven 输出解析器
//! 解析 Maven compile/test 的输出

use crate::core::{BaseParser, Issue, IssueLevel, Location, OutputParser};

pub struct MavenParser {
    base: BaseParser,
}

impl MavenParser {
    pub fn new() -> Self {
        Self {
            base: BaseParser::new(),
        }
    }

    /// 解析 Maven 编译错误/警告行
    /// 格式: [ERROR] /path/to/File.java:[10,5] error: message
    /// 格式: [WARNING] /path/to/File.java:[20,10] warning: message
    fn parse_maven_issue_line(&self, line: &str) -> Option<Issue> {
        let trimmed = line.trim();

        // 检查是否是错误/警告行
        let level = if trimmed.starts_with("[ERROR]") {
            IssueLevel::Error
        } else if trimmed.starts_with("[WARNING]") {
            IssueLevel::Warning
        } else {
            return None;
        };

        // 移除 [ERROR] 或 [WARNING] 前缀
        let content = if trimmed.starts_with("[ERROR]") {
            trimmed[7..].trim()
        } else {
            trimmed[9..].trim()
        };

        // 解析文件路径和位置
        // 格式: /path/to/File.java:[10,5] error: message
        if let Some(location_end) = content.find(']') {
            let location_part = &content[..location_end];
            let rest = &content[location_end + 1..].trim();

            // 解析文件路径和行列号
            if let Some((file_path, line_num, col_num)) = self.parse_java_location(location_part) {
                // 解析消息（移除 "error:" 或 "warning:" 前缀）
                let message = self.extract_message(rest);

                let location = Location::new(file_path)
                    .with_line(line_num)
                    .with_column(col_num);

                return Some(Issue::new(level, message, location));
            }
        }

        // 尝试解析没有行列号的格式
        // 格式: [ERROR] message
        if !content.contains(':') || content.starts_with("Failed to execute goal") {
            let location = Location::new("pom.xml");
            return Some(Issue::new(level, content.to_string(), location));
        }

        None
    }

    /// 解析 Java 文件位置
    /// 格式: /path/to/File.java:[10,5] 或 /path/to/File.java:10
    fn parse_java_location(&self, location_str: &str) -> Option<(String, u32, u32)> {
        // 查找 '[' 的位置（行号列号的开始）
        if let Some(bracket_start) = location_str.rfind('[') {
            let file_path = location_str[..bracket_start].trim();
            // 移除末尾的冒号（如果有）
            let file_path = file_path.trim_end_matches(':');
            let coords = &location_str[bracket_start + 1..]; // 跳过 "["

            // 解析行号和列号，格式: 10,5] 或 10]
            let coords = coords.trim_end_matches(']');
            let parts: Vec<&str> = coords.split(',').collect();
            if parts.len() >= 1 {
                let line_num = parts[0].trim().parse::<u32>().ok()?;
                let col_num = if parts.len() >= 2 {
                    parts[1].trim().parse::<u32>().ok()?
                } else {
                    0
                };
                return Some((file_path.to_string(), line_num, col_num));
            }
        }

        // 尝试简单格式: path:line
        if let Some(colon_pos) = location_str.rfind(':') {
            let file_path = &location_str[..colon_pos];
            let line_str = &location_str[colon_pos + 1..];
            if let Ok(line_num) = line_str.parse::<u32>() {
                return Some((file_path.to_string(), line_num, 0));
            }
        }

        None
    }

    /// 提取消息内容
    fn extract_message(&self, rest: &str) -> String {
        // 移除 "error:" 或 "warning:" 前缀
        let msg = rest
            .trim_start_matches("error:")
            .trim_start_matches("warning:")
            .trim_start_matches("[unchecked]")
            .trim();

        msg.to_string()
    }

    /// 解析多行错误（收集错误详情）
    fn parse_multiline_issue(
        &self,
        lines: &[String],
        start_index: usize,
    ) -> (Option<Issue>, usize) {
        if start_index >= lines.len() {
            return (None, start_index);
        }

        let line = &lines[start_index];

        // 首先尝试解析单行格式
        if let Some(issue) = self.parse_maven_issue_line(line) {
            return (Some(issue), start_index + 1);
        }

        // 检查是否是符号错误的多行格式
        // 符号:   变量 x
        // 位置: 类 com.example.MyClass
        if line.trim().starts_with("符号:") || line.trim().starts_with("位置:") {
            // 向上查找错误行
            for i in (0..start_index).rev() {
                if let Some(issue) = self.parse_maven_issue_line(&lines[i]) {
                    return (Some(issue), start_index + 1);
                }
            }
        }

        (None, start_index + 1)
    }
}

impl Default for MavenParser {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputParser for MavenParser {
    fn parse(&self, output: &str) -> Vec<Issue> {
        let lines: Vec<String> = output.lines().map(|s| s.to_string()).collect();
        let mut issues = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let (issue, new_index) = self.parse_multiline_issue(&lines, i);

            if let Some(issue) = issue {
                issues.push(issue);
                i = new_index;
            } else {
                i += 1;
            }
        }

        issues
    }

    fn is_issue_start(&self, line: &str) -> bool {
        let trimmed = line.trim();
        trimmed.starts_with("[ERROR]") || trimmed.starts_with("[WARNING]")
    }

    fn parse_issue(&self, lines: &[String], start_index: usize) -> (Option<Issue>, usize) {
        self.parse_multiline_issue(lines, start_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_line() {
        let parser = MavenParser::new();
        let line = "[ERROR] /path/to/File.java:[10,5] error: cannot find symbol";

        let issue = parser.parse_maven_issue_line(line).unwrap();

        assert_eq!(issue.level, IssueLevel::Error);
        assert_eq!(issue.location.file_path, "/path/to/File.java");
        assert_eq!(issue.location.line_number, Some(10));
        assert_eq!(issue.location.column_number, Some(5));
        assert!(issue.message.contains("cannot find symbol"));
    }

    #[test]
    fn test_parse_warning_line() {
        let parser = MavenParser::new();
        let line = "[WARNING] /path/to/File.java:[20,10] warning: [unchecked] unchecked conversion";

        let issue = parser.parse_maven_issue_line(line).unwrap();

        assert_eq!(issue.level, IssueLevel::Warning);
        assert_eq!(issue.location.file_path, "/path/to/File.java");
        assert_eq!(issue.location.line_number, Some(20));
        assert_eq!(issue.location.column_number, Some(10));
    }
}
