//! Cargo 插件
//! 提供对 Rust/Cargo 项目的分析支持

pub mod parser;
pub mod analyzer;

pub use analyzer::CargoAnalyzer;
