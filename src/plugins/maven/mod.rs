//! Maven 插件
//! Provides support for analyzing Java/Maven projects

pub mod parser;
pub mod analyzer;

pub use analyzer::MavenAnalyzer;
