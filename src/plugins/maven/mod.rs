//! Maven 插件
//! 提供对 Java/Maven 项目的分析支持

pub mod parser;
pub mod analyzer;

pub use analyzer::MavenAnalyzer;
