//! Mypy 插件
//! 提供对 Python/Mypy 项目的分析支持

pub mod parser;
pub mod analyzer;

pub use analyzer::MypyAnalyzer;
