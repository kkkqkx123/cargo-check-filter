//! 报告生成器模块
//! 支持多种输出格式（Markdown、JSON、HTML）

use std::path::Path;
use super::types::{AnalysisResult, ReportFormat, TestAnalysisResult};

mod markdown;
mod json;
mod html;

pub use markdown::MarkdownReporter;
pub use json::JsonReporter;
pub use html::HtmlReporter;

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
