//! plug-in module
//! Analyzer implementations containing various technology stacks

pub mod cargo;
pub mod cpp;
pub mod go;
pub mod java;
pub mod npm;
pub mod python;

use crate::core::{Config, PluginRegistry};

/// Create and configure the plug-in registry with optional config
pub fn create_registry_with_config(config: Option<Config>) -> PluginRegistry {
    let mut registry = PluginRegistry::new();

    // Register Cargo Analyzer
    if let Some(ref cfg) = config {
        registry.register(Box::new(cargo::CargoAnalyzer::new().with_config(cfg.clone())));
    } else {
        registry.register(Box::new(cargo::CargoAnalyzer::new()));
    }

    // Register NPM Analyzer
    if let Some(ref cfg) = config {
        registry.register(Box::new(npm::NpmAnalyzer::npm().with_config(cfg.clone())));
    } else {
        registry.register(Box::new(npm::NpmAnalyzer::npm()));
    }

    // Register PNPM Analyzer
    if let Some(ref cfg) = config {
        registry.register(Box::new(npm::NpmAnalyzer::pnpm().with_config(cfg.clone())));
    } else {
        registry.register(Box::new(npm::NpmAnalyzer::pnpm()));
    }

    // Registering the Yarn Analyzer
    if let Some(ref cfg) = config {
        registry.register(Box::new(npm::NpmAnalyzer::yarn().with_config(cfg.clone())));
    } else {
        registry.register(Box::new(npm::NpmAnalyzer::yarn()));
    }

    // Registering the Python Analyzers
    if let Some(ref cfg) = config {
        registry.register(Box::new(python::MypyAnalyzer::new().with_config(cfg.clone())));
        registry.register(Box::new(python::PytestAnalyzer::new().with_config(cfg.clone())));
    } else {
        registry.register(Box::new(python::MypyAnalyzer::new()));
        registry.register(Box::new(python::PytestAnalyzer::new()));
    }

    // Registering the Java Analyzers
    if let Some(ref cfg) = config {
        registry.register(Box::new(java::MavenAnalyzer::new().with_config(cfg.clone())));
        registry.register(Box::new(java::GradleAnalyzer::new().with_config(cfg.clone())));
    } else {
        registry.register(Box::new(java::MavenAnalyzer::new()));
        registry.register(Box::new(java::GradleAnalyzer::new()));
    }

    // Registering the Go Analyzer
    if let Some(ref cfg) = config {
        registry.register(Box::new(go::GoAnalyzer::new().with_config(cfg.clone())));
    } else {
        registry.register(Box::new(go::GoAnalyzer::new()));
    }

    // Registering the C++ Analyzers
    if let Some(ref cfg) = config {
        registry.register(Box::new(cpp::CMakeAnalyzer::new().with_config(cfg.clone())));
        registry.register(Box::new(cpp::GccAnalyzer::new().with_config(cfg.clone())));
        registry.register(Box::new(cpp::ClangAnalyzer::new().with_config(cfg.clone())));
        registry.register(Box::new(cpp::MsvcAnalyzer::new().with_config(cfg.clone())));
    } else {
        registry.register(Box::new(cpp::CMakeAnalyzer::new()));
        registry.register(Box::new(cpp::GccAnalyzer::new()));
        registry.register(Box::new(cpp::ClangAnalyzer::new()));
        registry.register(Box::new(cpp::MsvcAnalyzer::new()));
    }

    registry
}
