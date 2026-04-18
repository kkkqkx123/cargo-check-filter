//! Go Plugin
//! Provide analysis support for Go projects

pub mod parser;
pub mod analyzer;

pub use analyzer::GoAnalyzer;
