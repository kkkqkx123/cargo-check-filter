//! Cargo 插件
//! Provide analysis support for Rust/Cargo projects

pub mod parser;
pub mod analyzer;

pub use analyzer::CargoAnalyzer;
