//! Mypy plugin
//! Provides support for analyzing Python/Mypy projects.

pub mod parser;
pub mod analyzer;

pub use analyzer::MypyAnalyzer;
