//! Java Analyzer Module
//! Provides analysis support for Java build tools (maven, gradle)

pub mod maven;
pub mod gradle;

pub use maven::MavenAnalyzer;
pub use gradle::GradleAnalyzer;
