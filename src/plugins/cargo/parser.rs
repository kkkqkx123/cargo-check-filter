//! Cargo 输出解析器
//! 解析 cargo check/clippy/test 的输出

use crate::core::{BaseParser, Issue, IssueLevel, Location, OutputParser};

pub struct CargoParser {
    base: BaseParser,
}

impl CargoParser {
    pub fn new() -> Self {
        Self {
            base: BaseParser::new(),
        }
    }

    fn parse_single_line(&self, line: &str) -> Option<Issue> {
        let parts: Vec<&str> = line.splitn(5, ':').collect();
        if parts.len() < 5 {
            return None;
        }

        let file_path = parts[0].trim();
        let line_num = parts[1].trim().parse::<u32>().ok()?;
        let col_num = parts[2].trim().parse::<u32>().ok()?;
        let error_type = parts[3].trim();
        let description = parts[4].trim();

        let level = if error_type.starts_with("error") {
            IssueLevel::Error
        } else if error_type.starts_with("warning") {
            IssueLevel::Warning
        } else {
            return None;
        };

        let location = Location::new(file_path.to_string())
            .with_line(line_num)
            .with_column(col_num);

        let mut issue = Issue::new(level, description.to_string(), location);
        if let Some(code) = self.base.extract_error_code(error_type) {
            issue = issue.with_code(code);
        }

        Some(issue)
    }

    fn parse_multiline_error(
        &self,
        lines: &[String],
        start_index: usize,
    ) -> (Option<Issue>, usize) {
        if start_index >= lines.len() {
            return (None, start_index);
        }

        let line = &lines[start_index];

        let (level, desc) = if let Some(rest) = line.strip_prefix("error:") {
            (IssueLevel::Error, rest.trim())
        } else if let Some(rest) = line.strip_prefix("warning:") {
            (IssueLevel::Warning, rest.trim())
        } else {
            return (None, start_index + 1);
        };

        if start_index + 1 < lines.len() {
            let next_line = &lines[start_index + 1];
            let trimmed = next_line.trim();

            if trimmed.starts_with("-->") {
                let location_part = trimmed[3..].trim();

                if let Some(location) = self.parse_cargo_location(location_part) {
                    let mut issue = Issue::new(level, desc.to_string(), location);

                    if let Some(code) = self.base.extract_error_code(desc) {
                        issue = issue.with_code(code);
                    }

                    return (Some(issue), start_index + 2);
                }
            }
        }

        (None, start_index + 1)
    }

    fn parse_cargo_location(&self, location_str: &str) -> Option<Location> {
        let mut colon_positions = Vec::new();
        for (i, c) in location_str.char_indices() {
            if c == ':' {
                colon_positions.push(i);
            }
        }

        if colon_positions.len() >= 2 {
            let last_colon = colon_positions[colon_positions.len() - 1];
            let second_last_colon = colon_positions[colon_positions.len() - 2];

            let col_part = &location_str[last_colon + 1..];
            let line_part = &location_str[second_last_colon + 1..last_colon];
            let file_part = &location_str[..second_last_colon];

            let line_num = line_part.parse::<u32>().ok()?;
            let col_num = col_part.parse::<u32>().ok()?;

            Some(
                Location::new(file_part.trim().to_string())
                    .with_line(line_num)
                    .with_column(col_num),
            )
        } else {
            None
        }
    }
}

impl Default for CargoParser {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputParser for CargoParser {
    fn parse(&self, output: &str) -> Vec<Issue> {
        let lines: Vec<String> = output.lines().map(|s| s.to_string()).collect();
        let mut issues = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let (issue, new_index) = self.parse_multiline_error(&lines, i);

            if let Some(issue) = issue {
                issues.push(issue);
                i = new_index;
            } else {
                if let Some(issue) = self.parse_single_line(&lines[i]) {
                    issues.push(issue);
                }
                i += 1;
            }
        }

        issues
    }

    fn is_issue_start(&self, line: &str) -> bool {
        line.starts_with("error:")
            || line.starts_with("warning:")
            || (line.contains(": error") && line.contains(":"))
            || (line.contains(": warning") && line.contains(":"))
    }

    fn parse_issue(&self, lines: &[String], start_index: usize) -> (Option<Issue>, usize) {
        self.parse_multiline_error(lines, start_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_line() {
        let parser = CargoParser::new();
        let line = "src/main.rs:10:5: error[E0308]: mismatched types";
        let issue = parser.parse_single_line(line).unwrap();

        assert_eq!(issue.location.file_path, "src/main.rs");
        assert_eq!(issue.location.line_number, Some(10));
        assert_eq!(issue.location.column_number, Some(5));
        assert_eq!(issue.code, Some("[E0308]".to_string()));
        assert_eq!(issue.message, "mismatched types");
        assert!(matches!(issue.level, IssueLevel::Error));
    }

    #[test]
    fn test_parse_multiline_error() {
        let parser = CargoParser::new();
        let lines = vec![
            "error: mismatched types".to_string(),
            " --> src/main.rs:10:5".to_string(),
        ];

        let (issue, next_index) = parser.parse_multiline_error(&lines, 0);
        let issue = issue.unwrap();

        assert_eq!(issue.location.file_path, "src/main.rs");
        assert_eq!(issue.location.line_number, Some(10));
        assert_eq!(issue.location.column_number, Some(5));
        assert_eq!(issue.message, "mismatched types");
        assert_eq!(next_index, 2);
    }

    #[test]
    fn test_parse_windows_path() {
        let parser = CargoParser::new();
        let lines = vec![
            "error: some error".to_string(),
            " --> D:\\project\\src\\main.rs:10:5".to_string(),
        ];

        let (issue, _) = parser.parse_multiline_error(&lines, 0);
        let issue = issue.unwrap();

        assert_eq!(issue.location.file_path, "D:\\project\\src\\main.rs");
        assert_eq!(issue.location.line_number, Some(10));
        assert_eq!(issue.location.column_number, Some(5));
    }
}
