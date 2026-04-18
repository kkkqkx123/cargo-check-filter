//! NPM 插件
//! 提供对 Node.js/npm/pnpm/yarn 项目的分析支持

pub mod parser;
pub mod analyzer;

pub use analyzer::NpmAnalyzer;
