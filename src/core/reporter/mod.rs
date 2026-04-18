//! Report Generator Module
//! Support for multiple output formats (Markdown, JSON, HTML)

use std::path::Path;
use super::types::{AnalysisResult, ReportFormat, TestAnalysisResult};

mod markdown;
mod json;
mod html;

pub use markdown::MarkdownReporter;
pub use json::JsonReporter;
pub use html::HtmlReporter;

/// Report Generation Error
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

/// Report generator trait
pub trait Reporter: Send + Sync {
    /// Generate report content
    fn generate(&self, result: &AnalysisResult) -> Result<String, ReporterError>;

    /// Generate test report content
    fn generate_test_report(&self, result: &TestAnalysisResult) -> Result<String, ReporterError> {
        // Default implementation: call General Report Generation
        self.generate(&result.compile_result)
    }

    /// Access to report formats
    fn format(&self) -> ReportFormat;

    /// Write report to file
    fn write_to_file(&self, content: &str, path: &Path) -> Result<(), ReporterError> {
        std::fs::write(path, content)?;
        Ok(())
    }
}

/// Report Generator Factory
pub struct ReporterFactory;

impl ReporterFactory {
    /// Create a report generator based on the format
    pub fn create(format: ReportFormat) -> Box<dyn Reporter> {
        match format {
            ReportFormat::Markdown => Box::new(MarkdownReporter::new()),
            ReportFormat::Json => Box::new(JsonReporter::new()),
            ReportFormat::Html => Box::new(HtmlReporter::new()),
        }
    }
}
