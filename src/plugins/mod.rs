//! 插件模块
//! 包含各种技术栈的分析器实现

pub mod cargo;
pub mod mypy;
pub mod npm;

use std::path::Path;
use crate::core::PluginRegistry;

/// 创建并配置插件注册表
pub fn create_registry() -> PluginRegistry {
    let mut registry = PluginRegistry::new();

    // 注册 Cargo 分析器
    registry.register(Box::new(cargo::CargoAnalyzer::new()));

    // 注册 Mypy 分析器
    registry.register(Box::new(mypy::MypyAnalyzer::new()));

    // 注册 NPM 分析器
    registry.register(Box::new(npm::NpmAnalyzer::npm()));

    // 注册 PNPM 分析器
    registry.register(Box::new(npm::NpmAnalyzer::pnpm()));

    // 注册 Yarn 分析器
    registry.register(Box::new(npm::NpmAnalyzer::yarn()));

    registry
}

/// 自动检测项目类型
pub fn detect_project(path: &Path) -> Vec<String> {
    let registry = create_registry();
    registry
        .detect(path)
        .into_iter()
        .map(|a| a.name().to_string())
        .collect()
}
