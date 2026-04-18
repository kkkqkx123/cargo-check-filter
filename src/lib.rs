//! Analyzer - 多语言构建工具错误分析器
//!
//! 库接口，用于集成测试和外部调用

pub mod core;
pub mod plugins;

// 重新导出常用类型
pub use core::{
    AnalyzeOptions, AnalyzerError, BaseParser, BuildAnalyzer,
    CommandBuilder, Issue, IssueLevel, Location, OutputParser, ReportFormat, SubCommand,
};
