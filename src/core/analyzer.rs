//! Analyzer trait definition
//! defines the interface to the build tool analyzer

use std::time::Duration;
use super::types::{AnalysisResult, AnalyzeOptions};
use super::parser::OutputParser;
use super::config::Config;

/// Analyzer Error Type
#[derive(Debug)]
pub enum AnalyzerError {
    CommandFailed(String),
    ParseError(String),
    IoError(std::io::Error),
    NotApplicable,
    Timeout(Duration),
}

impl std::fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnalyzerError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            AnalyzerError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AnalyzerError::IoError(e) => write!(f, "IO error: {}", e),
            AnalyzerError::NotApplicable => write!(f, "Analyzer not applicable for this project"),
            AnalyzerError::Timeout(d) => write!(f, "Command timed out after {:?}", d),
        }
    }
}

impl std::error::Error for AnalyzerError {}

impl From<std::io::Error> for AnalyzerError {
    fn from(e: std::io::Error) -> Self {
        AnalyzerError::IoError(e)
    }
}

/// Build tool analyzer trait
/// Implement this trait to support new build tools
pub trait BuildAnalyzer: Send + Sync {
    /// Get the name of the technology stack
    fn name(&self) -> &str;

    /// Get supported command aliases
    fn supported_commands(&self) -> Vec<&str>;

    /// Run Analysis Command
    fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError>;

    /// Get parser
    fn parser(&self) -> &dyn OutputParser;

    /// Setup Configuration (for configuring the drive mode)
    fn set_config(&mut self, config: Config) {
        // Default empty implementation, backward compatible
        let _ = config;
    }

    /// Get configuration (if set)
    fn config(&self) -> Option<&Config> {
        None
    }
}

/// Plugin Registry
pub struct PluginRegistry {
    analyzers: Vec<Box<dyn BuildAnalyzer>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
        }
    }

    /// Registration Analyzer
    pub fn register(&mut self, analyzer: Box<dyn BuildAnalyzer>) {
        self.analyzers.push(analyzer);
    }

    /// Get analyzer by command name
    pub fn get(&self, command: &str) -> Option<&dyn BuildAnalyzer> {
        self.analyzers
            .iter()
            .find(|a| {
                a.name() == command
                    || a.supported_commands().contains(&command)
            })
            .map(|b| b.as_ref())
    }

    /// List all registered analyzers
    pub fn list(&self) -> Vec<&str> {
        self.analyzers.iter().map(|a| a.name()).collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
