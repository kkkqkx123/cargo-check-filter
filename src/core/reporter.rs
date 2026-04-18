//! 报告生成器 trait 定义
//! 支持多种输出格式

use std::path::Path;
use super::types::{AnalysisResult, ReportFormat, IssueLevel, TestAnalysisResult, TestStatus};

/// 报告生成错误
#[derive(Debug)]
pub enum ReporterError {
    IoError(std::io::Error),
    FormatError(String),
}

impl std::fmt::Display for ReporterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReporterError::IoError(e) => write!(f, "IO error: {}", e),
            ReporterError::FormatError(msg) => write!(f, "Format error: {}", msg),
        }
    }
}

impl std::error::Error for ReporterError {}

impl From<std::io::Error> for ReporterError {
    fn from(e: std::io::Error) -> Self {
        ReporterError::IoError(e)
    }
}

/// 报告生成器 trait
pub trait Reporter: Send + Sync {
    /// 生成报告内容
    fn generate(&self, result: &AnalysisResult) -> Result<String, ReporterError>;

    /// 生成测试报告内容
    fn generate_test_report(&self, result: &TestAnalysisResult) -> Result<String, ReporterError> {
        // 默认实现：调用普通报告生成
        self.generate(&result.compile_result)
    }

    /// 获取报告格式
    fn format(&self) -> ReportFormat;

    /// 写入报告到文件
    fn write_to_file(&self, content: &str, path: &Path) -> Result<(), ReporterError> {
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// 报告生成器工厂
pub struct ReporterFactory;

impl ReporterFactory {
    /// 根据格式创建对应的报告生成器
    pub fn create(format: ReportFormat) -> Box<dyn Reporter> {
        match format {
            ReportFormat::Markdown => Box::new(MarkdownReporter::new()),
            ReportFormat::Json => Box::new(JsonReporter::new()),
            ReportFormat::Html => Box::new(HtmlReporter::new()),
        }
    }
}

/// Markdown 报告生成器
pub struct MarkdownReporter;

impl MarkdownReporter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MarkdownReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for MarkdownReporter {
    fn generate(&self, result: &AnalysisResult) -> Result<String, ReporterError> {
        let mut report = String::new();

        // 标题
        report.push_str("# Analysis Report\n\n");

        // 摘要
        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total Issues**: {}\n", result.total_issues));

        for (level, count) in &result.issues_by_level {
            report.push_str(&format!("- **{}s**: {}\n", level, count));
        }

        report.push_str(&format!("- **Unique Patterns**: {}\n", result.unique_patterns.len()));
        report.push_str(&format!("- **Files with Issues**: {}\n\n", result.issues_by_file.len()));

        // 按类型统计
        if !result.issues_by_type.is_empty() {
            report.push_str("## Issues by Type\n\n");
            let mut types: Vec<_> = result.issues_by_type.iter().collect();
            types.sort_by(|a, b| b.1.cmp(a.1));

            for (issue_type, count) in types.iter().take(20) {
                report.push_str(&format!("- **{}**: {}\n", issue_type, count));
            }
            report.push('\n');
        }

        // 按文件统计
        if !result.issues_by_file.is_empty() {
            report.push_str("## Issues by File\n\n");
            let mut files: Vec<_> = result.issues_by_file.iter().collect();
            files.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

            for (file_path, issues) in files.iter().take(20) {
                report.push_str(&format!("### `{}` ({} issues)\n\n", file_path, issues.len()));

                for issue in issues.iter().take(10) {
                    let location = match (issue.location.line_number, issue.location.column_number) {
                        (Some(line), Some(col)) => format!("{}:{}", line, col),
                        (Some(line), None) => format!("{}", line),
                        _ => "unknown".to_string(),
                    };

                    let code = issue.code.as_ref().map(|c| format!(" [{}]", c)).unwrap_or_default();

                    report.push_str(&format!(
                        "- **{}**{} at {}: {}\n",
                        issue.level, code, location, issue.message
                    ));
                }

                if issues.len() > 10 {
                    report.push_str(&format!("- ... and {} more\n", issues.len() - 10));
                }

                report.push('\n');
            }
        }

        Ok(report)
    }

    fn generate_test_report(&self, result: &TestAnalysisResult) -> Result<String, ReporterError> {
        let mut report = String::new();

        // 标题
        report.push_str("# Test Analysis Report\n\n");

        // 测试摘要
        if let Some(ref summary) = result.test_summary {
            report.push_str("## Test Summary\n\n");
            report.push_str(&format!("- **Total Tests**: {}\n", summary.total));
            report.push_str(&format!("- **Passed**: ✅ {}\n", summary.passed));
            report.push_str(&format!("- **Failed**: ❌ {}\n", summary.failed));
            report.push_str(&format!("- **Ignored**: 🔕 {}\n", summary.ignored));
            if summary.measured > 0 {
                report.push_str(&format!("- **Measured**: {}\n", summary.measured));
            }
            if summary.filtered > 0 {
                report.push_str(&format!("- **Filtered**: {}\n", summary.filtered));
            }
            if let Some(time) = summary.execution_time {
                report.push_str(&format!("- **Execution Time**: {:.2}s\n", time));
            }
            report.push('\n');
        }

        // 失败的测试详情
        if !result.failed_tests.is_empty() {
            report.push_str("## Failed Tests\n\n");
            for test in &result.failed_tests {
                report.push_str(&format!("### `{}`\n\n", test.name));

                if let Some(ref location) = test.location {
                    report.push_str(&format!(
                        "**Location**: `{}:{}`\n\n",
                        location.file_path,
                        location
                            .line_number
                            .map(|n| n.to_string())
                            .unwrap_or_default()
                    ));
                }

                if let Some(ref details) = test.failure_details {
                    report.push_str("**Failure Details**:\n");
                    report.push_str("```\n");
                    report.push_str(details);
                    report.push_str("\n```\n\n");
                }
            }
        }

        // 被忽略的测试
        if !result.ignored_tests.is_empty() {
            report.push_str("## Ignored Tests\n\n");
            for test in &result.ignored_tests {
                let reason = match &test.status {
                    TestStatus::Ignored(Some(r)) => format!(" ({})",                                                                                                                                                                                                                                                 r),
                    _ => String::new(),
                };
                report.push_str(&format!("- `{}`{}\n", test.name, reason));
            }
            report.push('\n');
        }

        // 编译问题（如果有）
        if result.compile_result.total_issues > 0 {
            report.push_str("## Compile Issues\n\n");
            report.push_str(&self.generate(&result.compile_result)?);
        }

        Ok(report)
    }

    fn format(&self) -> ReportFormat {
        ReportFormat::Markdown
    }
}

/// JSON 报告生成器
pub struct JsonReporter;

impl JsonReporter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for JsonReporter {
    fn generate(&self, result: &AnalysisResult) -> Result<String, ReporterError> {
        // 简化的 JSON 生成
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str(&format!("  \"total_issues\": {},\n", result.total_issues));
        json.push_str(&format!("  \"unique_patterns\": {},\n", result.unique_patterns.len()));
        json.push_str(&format!("  \"files_with_issues\": {},\n", result.issues_by_file.len()));

        // issues_by_level
        json.push_str("  \"issues_by_level\": {\n");
        for (i, (level, count)) in result.issues_by_level.iter().enumerate() {
            let comma = if i < result.issues_by_level.len() - 1 { "," } else { "" };
            json.push_str(&format!("    \"{}\": {}{}\n", level, count, comma));
        }
        json.push_str("  },\n");

        // issues
        json.push_str("  \"issues\": [\n");
        let all_issues: Vec<_> = result.issues_by_file.values().flatten().collect();
        for (i, issue) in all_issues.iter().enumerate() {
            let comma = if i < all_issues.len() - 1 { "," } else { "" };
            json.push_str("    {\n");
            json.push_str(&format!("      \"level\": \"{}\",\n", issue.level));
            if let Some(code) = &issue.code {
                json.push_str(&format!("      \"code\": \"{}\",\n", code));
            }
            json.push_str(&format!("      \"message\": \"{}\",\n", issue.message.replace('"', "\\\"")));
            json.push_str(&format!("      \"file\": \"{}\"", issue.location.file_path));
            if let Some(line) = issue.location.line_number {
                json.push_str(&format!(",\n      \"line\": {}", line));
            }
            json.push_str(&format!("\n    }}{}\n", comma));
        }
        json.push_str("  ]\n");

        json.push('}');
        Ok(json)
    }

    fn format(&self) -> ReportFormat {
        ReportFormat::Json
    }
}

/// HTML 报告生成器（基础实现）
pub struct HtmlReporter;

impl HtmlReporter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HtmlReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for HtmlReporter {
    fn generate(&self, result: &AnalysisResult) -> Result<String, ReporterError> {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str("<title>Analysis Report</title>\n");
        html.push_str("<style>\n");
        html.push_str("body { font-family: Arial, sans-serif; margin: 40px; }\n");
        html.push_str("h1 { color: #333; }\n");
        html.push_str(".error { color: #d32f2f; }\n");
        html.push_str(".warning { color: #f57c00; }\n");
        html.push_str(".info { color: #1976d2; }\n");
        html.push_str("table { border-collapse: collapse; width: 100%; margin: 20px 0; }\n");
        html.push_str("th, td { border: 1px solid #ddd; padding: 12px; text-align: left; }\n");
        html.push_str("th { background-color: #f5f5f5; }\n");
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str("<h1>Analysis Report</h1>\n");

        // 摘要
        html.push_str("<h2>Summary</h2>\n");
        html.push_str("<ul>\n");
        html.push_str(&format!("<li>Total Issues: {}</li>\n", result.total_issues));
        for (level, count) in &result.issues_by_level {
            let class = match level {
                IssueLevel::Error => "error",
                IssueLevel::Warning => "warning",
                _ => "info",
            };
            html.push_str(&format!(
                "<li class=\"{}\">{}s: {}</li>\n",
                class, level, count
            ));
        }
        html.push_str("</ul>\n");

        // 详细表格
        html.push_str("<h2>Issues</h2>\n");
        html.push_str("<table>\n");
        html.push_str("<tr><th>Level</th><th>File</th><th>Location</th><th>Message</th></tr>\n");

        for issues in result.issues_by_file.values() {
            for issue in issues {
                let level_class = match issue.level {
                    IssueLevel::Error => "error",
                    IssueLevel::Warning => "warning",
                    _ => "info",
                };

                let location = match (issue.location.line_number, issue.location.column_number) {
                    (Some(line), Some(col)) => format!("{}:{}", line, col),
                    (Some(line), None) => format!("{}", line),
                    _ => "-".to_string(),
                };

                html.push_str(&format!(
                    "<tr><td class=\"{}\">{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                    level_class,
                    issue.level,
                    issue.location.file_path,
                    location,
                    issue.message
                ));
            }
        }

        html.push_str("</table>\n");
        html.push_str("</body>\n</html>");

        Ok(html)
    }

    fn format(&self) -> ReportFormat {
        ReportFormat::Html
    }
}
