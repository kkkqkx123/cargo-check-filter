//! plug-in module
//! Analyzer implementations containing various technology stacks

pub mod cargo;
pub mod maven;
pub mod mypy;
pub mod npm;

use std::path::Path;
use crate::core::PluginRegistry;

/// Create and configure the plug-in registry
pub fn create_registry() -> PluginRegistry {
    let mut registry = PluginRegistry::new();

    // Register Cargo Analyzer
    registry.register(Box::new(cargo::CargoAnalyzer::new()));

    // Register Mypy Analyzer
    registry.register(Box::new(mypy::MypyAnalyzer::new()));

    // Register NPM Analyzer
    registry.register(Box::new(npm::NpmAnalyzer::npm()));

    // Register PNPM Analyzer
    registry.register(Box::new(npm::NpmAnalyzer::pnpm()));

    // Registering the Yarn Analyzer
    registry.register(Box::new(npm::NpmAnalyzer::yarn()));

    // Registering the Maven Analyzer
    registry.register(Box::new(maven::MavenAnalyzer::new()));

    registry
}

/// Automated testing of project types
pub fn detect_project(path: &Path) -> Vec<String> {
    let registry = create_registry();
    registry
        .detect(path)
        .into_iter()
        .map(|a| a.name().to_string())
        .collect()
}
