//! Markdown 报告生成器

use super::{Reporter, ReporterError};
use crate::core::types::{AnalysisResult, IssueLevel, ReportFormat, TestAnalysisResult, TestStatus};

/// Markdown 报告生成器
pub struct MarkdownReporter;

impl MarkdownReporter {
    pub fn new() -> Self {
        Self
    }

    /// 检测报告类型并返回合适的标题
    fn detect_report_type(&self, result: &AnalysisResult) -> (String, String) {
        // 收集所有 issue 的消息用于判断类型
        let all_messages: Vec<String> = result
            .issues_by_file
            .values()
            .flatten()
            .map(|i| i.message.to_lowercase())
            .collect();

        // 判断是否为安全审计报告
        let is_security_audit = all_messages.iter().any(|m| {
            m.contains("security vulnerability")
                || m.contains("severity: high")
                || m.contains("severity: critical")
                || m.contains("npm audit")
        });

        if is_security_audit {
            return (
                "Security Audit Report".to_string(),
                "Vulnerability Summary".to_string(),
            );
        }

        // 判断是否为类型检查报告
        let is_type_check = all_messages.iter().any(|m| {
            m.contains("type")
                || m.contains("typescript")
                || m.contains("type mismatch")
                || m.contains("expected")
                || m.contains("mypy")
        });

        if is_type_check {
            return (
                "Type Check Report".to_string(),
                "Type Issues Summary".to_string(),
            );
        }

        // 判断是否为 Lint 报告
        let is_lint = all_messages.iter().any(|m| {
            m.contains("eslint") || m.contains("clippy") || m.contains("lint") || m.contains("style")
        });

        if is_lint {
            return ("Lint Report".to_string(), "Lint Issues Summary".to_string());
        }

        // 默认为通用分析报告
        (
            "Analysis Report".to_string(),
            "Issues Summary".to_string(),
        )
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

        // 检测报告类型并设置合适的标题
        let (title, summary_title) = self.detect_report_type(result);
        report.push_str(&format!("# {}\n\n", title));

        // 摘要
        report.push_str(&format!("## {}\n\n", summary_title));
        
        if result.total_issues == 0 {
            report.push_str("✅ No issues found.\n\n");
            return Ok(report);
        }
        
        report.push_str(&format!("- **Total**: {}\n", result.total_issues));

        // 按级别统计，按严重程度排序
        let level_order = [IssueLevel::Error, IssueLevel::Warning, IssueLevel::Info, IssueLevel::Hint];
        for level in &level_order {
            if let Some(count) = result.issues_by_level.get(level) {
                let icon = match level {
                    IssueLevel::Error => "❌",
                    IssueLevel::Warning => "⚠️",
                    IssueLevel::Info => "ℹ️",
                    IssueLevel::Hint => "💡",
                };
                report.push_str(&format!("- **{}** {}: {}\n", icon, level, count));
            }
        }

        report.push_str(&format!("- **Categories**: {}\n", result.unique_patterns.len()));
        report.push_str(&format!("- **Files Affected**: {}\n\n", result.issues_by_file.len()));

        // 按类型统计
        if !result.issues_by_type.is_empty() {
            report.push_str("## Breakdown by Category\n\n");
            let mut types: Vec<_> = result.issues_by_type.iter().collect();
            types.sort_by(|a, b| b.1.cmp(a.1));

            for (issue_type, count) in types.iter().take(20) {
                report.push_str(&format!("- **{}**: {} occurrence(s)\n", issue_type, count));
            }
            report.push('\n');
        }

        // 按文件统计
        if !result.issues_by_file.is_empty() {
            report.push_str("## Details by File\n\n");
            let mut files: Vec<_> = result.issues_by_file.iter().collect();
            files.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

            for (file_path, issues) in files.iter().take(20) {
                report.push_str(&format!("### `{}` ({} item(s))\n\n", file_path, issues.len()));

                for issue in issues.iter().take(10) {
                    let location = match (issue.location.line_number, issue.location.column_number) {
                        (Some(line), Some(col)) => format!("{}:{}", line, col),
                        (Some(line), None) => format!("{}", line),
                        _ => "-".to_string(),
                    };

                    let code = issue.code.as_ref().map(|c| format!(" `[{}]`", c)).unwrap_or_default();
                    let level_icon = match issue.level {
                        IssueLevel::Error => "❌",
                        IssueLevel::Warning => "⚠️",
                        IssueLevel::Info => "ℹ️",
                        IssueLevel::Hint => "💡",
                    };

                    report.push_str(&format!(
                        "- {} **{}**{} at line {}: {}\n",
                        level_icon, issue.level, code, location, issue.message
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

        // 根据测试结果选择标题
        let all_passed = result.failed_tests.is_empty() && result.compile_result.total_issues == 0;
        if all_passed {
            report.push_str("# ✅ Test Report - All Passed\n\n");
        } else {
            report.push_str("# ❌ Test Report - Issues Found\n\n");
        }

        // 测试摘要
        if let Some(ref summary) = result.test_summary {
            report.push_str("## Summary\n\n");
            
            // 计算通过率
            let pass_rate = if summary.total > 0 {
                (summary.passed as f64 / summary.total as f64) * 100.0
            } else {
                0.0
            };
            
            report.push_str(&format!("- **Total**: {} test(s)\n", summary.total));
            report.push_str(&format!("- **Passed**: ✅ {} ({:.1}%)\n", summary.passed, pass_rate));
            if summary.failed > 0 {
                report.push_str(&format!("- **Failed**: ❌ {}\n", summary.failed));
            }
            if summary.ignored > 0 {
                report.push_str(&format!("- **Ignored**: 🔕 {}\n", summary.ignored));
            }
            if summary.measured > 0 {
                report.push_str(&format!("- **Measured**: {}\n", summary.measured));
            }
            if summary.filtered > 0 {
                report.push_str(&format!("- **Filtered out**: {}\n", summary.filtered));
            }
            if let Some(time) = summary.execution_time {
                report.push_str(&format!("- **Duration**: {:.2}s\n", time));
            }
            report.push('\n');
        }

        // 失败的测试详情
        if !result.failed_tests.is_empty() {
            report.push_str(&format!("## Failed Tests ({} item(s))\n\n", result.failed_tests.len()));
            for (idx, test) in result.failed_tests.iter().enumerate() {
                report.push_str(&format!("### {}. `{}`\n\n", idx + 1, test.name));

                if let Some(ref location) = test.location {
                    report.push_str(&format!(
                        "📍 **Location**: `{}:{}`\n\n",
                        location.file_path,
                        location
                            .line_number
                            .map(|n| n.to_string())
                            .unwrap_or_else(|| "-".to_string())
                    ));
                }

                if let Some(ref details) = test.failure_details {
                    report.push_str("🔍 **Failure Details**:\n");
                    report.push_str("```\n");
                    report.push_str(details);
                    report.push_str("\n```\n\n");
                }
            }
        }

        // 被忽略的测试
        if !result.ignored_tests.is_empty() {
            report.push_str(&format!("## Ignored Tests ({} item(s))\n\n", result.ignored_tests.len()));
            for test in &result.ignored_tests {
                let reason = match &test.status {
                    TestStatus::Ignored(Some(r)) => format!(" - *Reason: {}*", r),
                    _ => String::new(),
                };
                report.push_str(&format!("- `{}`{}\n", test.name, reason));
            }
            report.push('\n');
        }

        // 编译问题（如果有）
        if result.compile_result.total_issues > 0 {
            report.push_str("## Build Issues\n\n");
            report.push_str("The following issues were found during compilation:\n\n");
            report.push_str(&self.generate(&result.compile_result)?);
        }

        Ok(report)
    }

    fn format(&self) -> ReportFormat {
        ReportFormat::Markdown
    }
}
