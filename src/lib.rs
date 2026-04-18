//! Analyzer - Multilingual Build Tool Error Analyzer
//!
//! Library interface for integration testing and external calls

pub mod core;
pub mod plugins;

// Re-export common types
pub use core::{
    AnalyzeOptions, AnalyzerError, BaseParser, BuildAnalyzer,
    CommandBuilder, Issue, IssueLevel, Location, OutputParser, ReportFormat, SubCommand,
};
