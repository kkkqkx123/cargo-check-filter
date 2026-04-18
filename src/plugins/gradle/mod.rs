//! Gradle Plugin
//! Provides support for analyzing Java/Gradle projects

pub mod parser;
pub mod analyzer;

pub use analyzer::GradleAnalyzer;
