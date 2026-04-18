# 配置系统设计文档

## 概述

本文档描述 analyzer 工具的配置系统架构设计，旨在解决当前命令硬编码的问题，支持灵活的命令映射和自定义命令。

## 目标

1. **解耦命令定义**：将命令从代码中分离，支持配置驱动
2. **多层级配置**：支持全局、用户级、项目级配置，优先级递增
3. **自定义命令**：允许用户定义新的命令而不修改代码
4. **技术栈特定配置**：不同技术栈可以有独立的配置

## 配置层级（优先级从低到高）

```
┌─────────────────────────────────────────────────────────────┐
│  层级1: 内置默认值 (Embedded defaults)                       │
│  - 代码中定义的预配置命令                                    │
│  - 最低优先级，可被所有配置覆盖                              │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  层级2: 全局配置 (Global config)                             │
│  - 与二进制文件同目录: analyzer.toml                         │
│  - 或系统配置目录: ~/.config/analyzer/config.toml            │
│  - 适用于所有项目                                            │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  层级3: 用户配置 (User config)                               │
│  - ~/.analyzer.toml                                          │
│  - 用户个人偏好设置                                          │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  层级4: 项目级配置 (Project config)                          │
│  - ./.analyzer.toml (隐藏文件)                               │
│  - 或 ./analyzer.toml                                        │
│  - 最高优先级，项目特定设置                                  │
└─────────────────────────────────────────────────────────────┘
```

## 配置文件格式 (TOML)

### 完整配置示例

```toml
# analyzer.toml
# 配置系统版本
version = "1.0"

[global]
# 默认输出格式: markdown, json, text, html
default_format = "markdown"
# 是否默认过滤警告
filter_warnings = false
# 默认报告输出路径
default_output = "analysis_report.md"

# ============================================
# 预定义命令（可覆盖内置命令）
# ============================================

[commands.check]
# 实际执行的命令
exec = "cargo check"
# 命令描述
description = "Fast syntax and type checking"
# 适用的技术栈
tech_stacks = ["cargo"]
# 是否启用
enabled = true

[commands.lint]
exec = "npm run lint"
description = "Run linter"
tech_stacks = ["npm", "pnpm", "yarn"]

[commands.test]
exec = "cargo test"
description = "Run tests"
tech_stacks = ["cargo"]

[commands.type-check]
exec = "npm run type-check"
description = "Run TypeScript type checker"
tech_stacks = ["npm", "pnpm", "yarn"]

# ============================================
# 自定义命令示例
# ============================================

[commands.clippy-strict]
exec = "cargo clippy --all-targets --all-features -- -D warnings"
description = "Run Clippy with strict warnings"
tech_stacks = ["cargo"]

[commands.test-coverage]
exec = "cargo tarpaulin --out Html"
description = "Generate test coverage report"
tech_stacks = ["cargo"]

# ============================================
# 技术栈特定配置
# ============================================

[tech_stack.cargo]
# 覆盖默认命令
[tech_stack.cargo.commands.check]
exec = "cargo check --all-targets --all-features"
description = "Check all targets and features"

[tech_stack.cargo.commands.test]
exec = "cargo test --lib"
description = "Run library tests only"

[tech_stack.npm]
# 脚本名称映射（用于解析输出）
[tech_stack.npm.scripts]
"test" = "jest"
"lint" = "eslint"
"type-check" = "tsc --noEmit"
"build" = "tsc"

# 测试框架类型（用于输出解析）
test_framework = "jest"  # 可选: jest, vitest, mocha

[tech_stack.pnpm]
# 继承 npm 配置，可覆盖特定设置
[tech_stack.pnpm.scripts]
"test" = "vitest"
"lint" = "eslint . --ext .ts,.tsx"

test_framework = "vitest"
```

### 最小配置示例

```toml
# 仅覆盖特定命令
[commands.type-check]
exec = "npm run typecheck"
tech_stacks = ["npm"]
```

## 配置结构定义

```rust
// 配置根结构
pub struct Config {
    pub version: String,
    pub global: GlobalConfig,
    pub commands: HashMap<String, CommandConfig>,
    pub tech_stacks: HashMap<String, TechStackConfig>,
}

// 全局配置
pub struct GlobalConfig {
    pub default_format: ReportFormat,
    pub filter_warnings: bool,
    pub default_output: Option<String>,
}

// 命令配置
pub struct CommandConfig {
    pub exec: String,
    pub description: Option<String>,
    pub tech_stacks: Vec<String>,
    pub enabled: bool,
}

// 技术栈特定配置
pub struct TechStackConfig {
    pub commands: HashMap<String, CommandConfig>,
    pub scripts: HashMap<String, String>,
    pub test_framework: Option<String>,
}
```

## 配置加载流程

```
┌─────────────────┐
│   启动程序      │
└────────┬────────┘
         ↓
┌─────────────────┐
│ 加载内置默认值  │
│ (embedded.rs)   │
└────────┬────────┘
         ↓
┌─────────────────┐     ┌─────────────────┐
│ 查找全局配置    │────→│ 存在则合并      │
│ (二进制目录)    │     │ (低优先级)      │
└─────────────────┘     └─────────────────┘
         ↓
┌─────────────────┐     ┌─────────────────┐
│ 查找用户配置    │────→│ 存在则合并      │
│ (~/.analyzer.toml)    │ (中优先级)      │
└─────────────────┘     └─────────────────┘
         ↓
┌─────────────────┐     ┌─────────────────┐
│ 查找项目配置    │────→│ 存在则合并      │
│ (./.analyzer.toml)    │ (高优先级)      │
└─────────────────┘     └─────────────────┘
         ↓
┌─────────────────┐
│ 配置验证        │
│ - 检查命令格式  │
│ - 检查技术栈    │
└────────┬────────┘
         ↓
┌─────────────────┐
│ 使用合并后的配置 │
└─────────────────┘
```

## 命令解析流程

```
用户输入: analyzer cargo check
                    │    │
                    │    └─→ 命令名: "check"
                    └──────→ 技术栈: "cargo"
                              ↓
                    ┌─────────────────┐
                    │ 查找配置        │
                    │ 1. 先查 tech_stack.cargo.commands.check
                    │ 2. 再查 commands.check
                    │ 3. 使用内置默认值
                    └────────┬────────┘
                              ↓
                    ┌─────────────────┐
                    │ 获取 exec 命令  │
                    │ "cargo check"   │
                    └────────┬────────┘
                              ↓
                    ┌─────────────────┐
                    │ 执行命令        │
                    │ 解析输出        │
                    └─────────────────┘
```

## 实现阶段

### 阶段1: 基础配置系统

**目标**: 支持项目级配置，允许覆盖现有命令

**任务**:
1. 创建 `core/config.rs` 模块
2. 定义配置数据结构
3. 实现 TOML 解析
4. 实现配置加载和合并
5. 修改插件系统使用配置

**文件变更**:
- 新增: `src/core/config.rs`
- 修改: `src/core/mod.rs` (导出 config)
- 修改: `src/core/types.rs` (SubCommand 支持动态命令)
- 修改: `src/plugins/cargo/analyzer.rs` (使用配置)
- 修改: `src/plugins/npm/analyzer.rs` (使用配置)

### 阶段2: 自定义命令

**目标**: 支持通过配置添加新命令

**任务**:
1. 改造 `SubCommand` 从 enum 到动态结构
2. 实现命令注册表
3. 支持运行时命令发现
4. 更新 CLI 参数解析

**文件变更**:
- 修改: `src/core/types.rs` (重构 SubCommand)
- 修改: `src/main.rs` (动态命令帮助信息)

### 阶段3: 全局配置和高级功能

**目标**: 支持多层级配置和高级功能

**任务**:
1. 实现全局配置加载
2. 实现用户配置加载
3. 实现配置合并策略
4. 添加配置验证
5. 支持命令别名
6. 支持条件命令

**文件变更**:
- 修改: `src/core/config.rs` (多层级加载)
- 新增: 配置验证模块

## 向后兼容性

1. **默认行为不变**: 无配置文件时，使用内置默认值
2. **现有命令保留**: 预定义命令仍然可用
3. **渐进式采用**: 用户可逐步迁移到配置系统

## 迁移指南

### 从硬编码到配置

**之前 (代码中)**:
```rust
fn build_npm_command(&self, options: &AnalyzeOptions) -> CommandBuilder {
    let mut builder = CommandBuilder::new("npm");
    match options.subcommand {
        Some(SubCommand::TypeCheck) => {
            builder = builder.arg("run").arg("type-check");
        }
        // ...
    }
    builder
}
```

**之后 (配置中)**:
```toml
[commands.type-check]
exec = "npm run type-check"
description = "Run TypeScript type checker"
tech_stacks = ["npm", "pnpm", "yarn"]
```

## 附录

### 配置文件搜索路径

| 层级 | 路径 | 优先级 |
|------|------|--------|
| 全局 | `<binary_dir>/analyzer.toml` | 低 |
| 全局 | `~/.config/analyzer/config.toml` | 低 |
| 用户 | `~/.analyzer.toml` | 中 |
| 项目 | `./.analyzer.toml` | 高 |
| 项目 | `./analyzer.toml` | 高 |

### 内置命令列表

| 命令 | 描述 | 适用技术栈 |
|------|------|-----------|
| check | 语法和类型检查 | cargo |
| clippy | 运行 Clippy | cargo |
| test | 运行测试 | cargo, npm, pnpm, yarn |
| lint | 运行 Linter | npm, pnpm, yarn |
| type-check | 类型检查 | npm, pnpm, yarn |
| audit | 安全审计 | npm, pnpm, yarn |

---

*文档版本: 1.0*
*最后更新: 2026-04-18*
