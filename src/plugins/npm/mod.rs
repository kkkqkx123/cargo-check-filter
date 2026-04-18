//! NPM Plugin
//! Provide support for analyzing Node.js/npm/pnpm/yarn projects

pub mod parser;
pub mod analyzer;

pub use analyzer::NpmAnalyzer;
