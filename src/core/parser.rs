//! 解析器 trait 定义
//! 定义了解析命令输出的接口

use super::types::{Issue, IssueLevel, Location};

/// 输出解析器 trait
/// 实现此 trait 以支持新的技术栈输出格式
pub trait OutputParser: Send + Sync {
    /// 解析命令输出，提取所有问题信息
    fn parse(&self, output: &str) -> Vec<Issue>;

    /// 检查某行是否为问题起始行
    fn is_issue_start(&self, line: &str) -> bool;

    /// 解析单行问题信息
    /// 返回解析出的 Issue 和消耗的文本行数
    fn parse_issue(&self, lines: &[String], start_index: usize) -> (Option<Issue>, usize);
}

/// 基础解析器实现，提供通用的辅助方法
pub struct BaseParser;

impl BaseParser {
    pub fn new() -> Self {
        Self
    }

    /// 从路径字符串中提取文件路径、行号和列号
    /// 支持格式：path/to/file.rs:10:5
    pub fn parse_location(&self, location_str: &str) -> Option<Location> {
        let parts: Vec<&str> = location_str.rsplitn(3, ':').collect();

        if parts.len() == 3 {
            let col = parts[0].parse::<u32>().ok()?;
            let line = parts[1].parse::<u32>().ok()?;
            let file = parts[2];

            Some(
                Location::new(file.to_string())
                    .with_line(line)
                    .with_column(col),
            )
        } else {
            Some(Location::new(location_str.to_string()))
        }
    }

    /// 检测问题级别
    pub fn detect_level(&self, text: &str) -> Option<IssueLevel> {
        let lower = text.to_lowercase();
        if lower.contains("error") {
            Some(IssueLevel::Error)
        } else if lower.contains("warning") || lower.contains("warn") {
            Some(IssueLevel::Warning)
        } else if lower.contains("info") {
            Some(IssueLevel::Info)
        } else if lower.contains("hint") {
            Some(IssueLevel::Hint)
        } else if lower.contains("note") {
            Some(IssueLevel::Info)
        } else {
            None
        }
    }

    /// 提取错误代码（如 E0308 或 TS1234）
    pub fn extract_error_code(&self, text: &str) -> Option<String> {
        if let Some(start) = text.find('[') {
            if let Some(end) = text.find(']') {
                if start < end {
                    let code = &text[start..=end];
                    if code.starts_with('[') && code.ends_with(']') {
                        let inner = &code[1..code.len() - 1];
                        if inner.chars().all(|c| c.is_ascii_alphanumeric()) {
                            return Some(code.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    /// 解析标准格式：file:line:col: level: message
    pub fn parse_standard_format(&self, line: &str) -> Option<Issue> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return None;
        }

        let parts: Vec<&str> = trimmed.splitn(5, ':').collect();
        if parts.len() < 4 {
            return None;
        }

        let file_path = parts[0].trim();
        let line_num = parts[1].trim().parse::<u32>().ok()?;

        let (col_num, level_str, message) = if parts.len() >= 5 {
            let col = parts[2].trim().parse::<u32>().ok()?;
            (Some(col), parts[3].trim(), parts[4].trim())
        } else {
            (None, parts[2].trim(), parts[3].trim())
        };

        let level = self.detect_level(level_str)?;

        let location = if let Some(col) = col_num {
            Location::new(file_path.to_string())
                .with_line(line_num)
                .with_column(col)
        } else {
            Location::new(file_path.to_string())
                .with_line(line_num)
        };

        let mut issue = Issue::new(level, message.to_string(), location);

        if let Some(code) = self.extract_error_code(message) {
            issue = issue.with_code(code);
        }

        Some(issue)
    }

    /// 解析带括号的格式：file(line,col): level: message
    pub fn parse_parentheses_format(&self, line: &str) -> Option<Issue> {
        let trimmed = line.trim();
        
        if let Some(open_paren) = trimmed.find('(') {
            if let Some(close_paren) = trimmed.find(')') {
                let file_path = &trimmed[..open_paren].trim();
                let location_part = &trimmed[open_paren + 1..close_paren];
                let after_paren = &trimmed[close_paren + 1..].trim();

                let loc_parts: Vec<&str> = location_part.split(',').collect();
                if loc_parts.len() == 2 {
                    let line_num = loc_parts[0].trim().parse::<u32>().ok()?;
                    let col_num = loc_parts[1].trim().parse::<u32>().ok()?;

                    if after_paren.starts_with(':') {
                        let rest = &after_paren[1..].trim();
                        let level = self.detect_level(rest)?;

                        let (code, message) = if let Some(colon_pos) = rest.find(':') {
                            let before_colon = rest[..colon_pos].trim();
                            let msg_part = rest[colon_pos + 1..].trim();
                            
                            let parts: Vec<&str> = before_colon.split_whitespace().collect();
                            let code_part = parts.last().unwrap_or(&before_colon);
                            
                            let formatted_code = if code_part.starts_with('[') && code_part.ends_with(']') {
                                Some(code_part.to_string())
                            } else if code_part.chars().all(|c| c.is_alphanumeric()) && code_part.len() > 1 {
                                Some(format!("[{}]", code_part))
                            } else {
                                None
                            };
                            
                            (formatted_code, msg_part.to_string())
                        } else {
                            (None, rest.to_string())
                        };

                        let location = Location::new(file_path.to_string())
                            .with_line(line_num)
                            .with_column(col_num);

                        let mut issue = Issue::new(level, message, location);

                        if let Some(c) = code {
                            issue = issue.with_code(c);
                        }

                        return Some(issue);
                    }
                }
            }
        }

        None
    }

    /// 从文本中提取消息（移除规则名等后缀）
    pub fn extract_message(&self, text: &str) -> String {
        let parts: Vec<&str> = text.split_whitespace().collect();
        if parts.len() > 1 {
            if let Some(last) = parts.last() {
                if last.contains('/') || last.contains('-') {
                    return parts[..parts.len() - 1].join(" ");
                }
            }
        }
        text.to_string()
    }

    /// 查找文件路径（向上查找非空行）
    pub fn find_file_path(&self, lines: &[String], current_index: usize) -> String {
        for i in (0..current_index).rev() {
            let prev_line = &lines[i];
            let prev_trimmed = prev_line.trim();
            
            if !prev_trimmed.is_empty()
                && !prev_trimmed.chars().next().unwrap_or(' ').is_ascii_digit()
                && !prev_trimmed.starts_with('✖')
                && !prev_trimmed.to_lowercase().starts_with("error")
                && !prev_trimmed.to_lowercase().starts_with("warning")
            {
                return prev_trimmed.to_string();
            }
        }
        String::from("unknown")
    }
}

impl Default for BaseParser {
    fn default() -> Self {
        Self::new()
    }
}
