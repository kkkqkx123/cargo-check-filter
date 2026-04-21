# 配置模块功能概述

## 模块简介

配置模块（`src/core/config.rs`）是 analyzer 工具的核心基础设施，负责管理整个工具的配置系统。该模块实现了多层级配置加载、合并策略、命令映射等功能，为各个分析器插件提供统一的配置管理能力。

## 核心功能

### 1. 多层级配置架构

配置系统采用三层配置架构，优先级从低到高：

| 层级 | 来源 | 优先级 | 说明 |
|------|------|--------|------|
| 内置默认值 | 代码中定义 | 最低 | 预定义的命令映射和默认设置 |
| 二进制目录配置 | `<binary_dir>/analyzer.toml` | 中 | 与可执行文件同目录的配置 |
| 项目级配置 | `./.analyzer.toml` 或 `./analyzer.toml` | 最高 | 项目特定的配置 |

**设计原则**：不使用用户目录配置（如 `~/.analyzer.toml`），避免配置分散和"在我的机器上能运行"问题。

### 2. 配置数据结构

#### Config（根结构）
```rust
pub struct Config {
    pub version: String,                              // 配置版本
    pub global: GlobalConfig,                         // 全局设置
    pub commands: HashMap<String, CommandConfig>,     // 命令定义
    pub tech_stacks: HashMap<String, TechStackConfig>, // 技术栈配置
}
```

#### GlobalConfig（全局配置）
```rust
pub struct GlobalConfig {
    pub default_format: String,           // 默认输出格式（markdown/json/html）
    pub filter_warnings: bool,            // 是否默认过滤警告
    pub default_output: Option<String>,   // 默认报告输出路径
}
```

#### CommandConfig（命令配置）
```rust
pub struct CommandConfig {
    pub exec: String,                 // 实际执行的命令
    pub description: Option<String>,  // 命令描述
    pub tech_stacks: Vec<String>,     // 适用的技术栈列表
    pub enabled: bool,                // 是否启用
}
```

#### TechStackConfig（技术栈配置）
```rust
pub struct TechStackConfig {
    pub commands: HashMap<String, CommandConfig>,  // 命令覆盖
    pub scripts: HashMap<String, String>,          // 脚本名称映射
    pub test_framework: Option<String>,            // 测试框架类型
}
```

### 3. 配置加载流程

```
程序启动
    ↓
Config::load(project_path)
    ↓
┌─────────────────────────┐
│ 1. 加载内置默认值        │
│   embedded_defaults()   │
└──────────┬──────────────┘
           ↓
┌─────────────────────────┐
│ 2. 加载二进制目录配置    │
│   load_from_binary_dir()│
└──────────┬──────────────┘
           ↓
┌─────────────────────────┐
│ 3. 加载项目级配置        │
│   load_from_project()   │
└──────────┬──────────────┘
           ↓
┌─────────────────────────┐
│ 4. 合并所有配置          │
│   merge()               │
└──────────┬──────────────┘
           ↓
      返回 Config
```

### 4. 核心方法

#### 配置加载
- `Config::load(project_path)` - 主入口，加载并合并所有层级的配置
- `Config::load_from_binary_dir()` - 从二进制目录加载配置
- `Config::load_from_project(project_path)` - 从项目目录加载配置
- `Config::load_from_file(path)` - 从指定文件加载配置

#### 配置合并
- `Config::merge(&mut self, other)` - 合并另一个配置（高优先级覆盖低优先级）

#### 命令查询
- `get_command_exec(tech_stack, command_name)` - 获取指定技术栈的命令执行字符串
- `is_command_enabled(tech_stack, command_name)` - 检查命令是否启用
- `get_available_commands(tech_stack)` - 获取技术栈的所有可用命令

### 5. 内置默认命令

#### Cargo 命令
| 命令名 | 执行命令 | 描述 |
|--------|----------|------|
| check | `cargo check` | 快速语法和类型检查 |
| clippy | `cargo clippy` | 运行 Clippy linter |
| clippy-all | `cargo clippy --all-targets --all-features` | 全量 Clippy 检查 |
| check-test | `cargo check --tests` | 检查测试代码 |
| test | `cargo test` | 运行测试 |

#### NPM/前端命令
| 命令名 | 执行命令 | 适用技术栈 |
|--------|----------|------------|
| lint | `npm run lint` | npm, pnpm, yarn |
| type-check | `npm run type-check` | npm, pnpm, yarn |
| audit | `npm audit` | npm, pnpm, yarn |

#### Python 命令
| 命令名 | 执行命令 | 描述 |
|--------|----------|------|
| mypy | `mypy` | 运行 mypy 类型检查器 |
| mypy-strict | `mypy --strict` | 严格模式类型检查 |

### 6. 配置合并策略

#### 全局配置合并
- `default_format`: 高优先级的非默认值覆盖低优先级
- `filter_warnings`: 直接覆盖
- `default_output`: 高优先级的 Some 值覆盖低优先级

#### 命令配置合并
- 完全覆盖策略：相同命令名的配置会被替换
- 新命令会添加到命令映射中

#### 技术栈配置合并
- 增量合并策略：命令和脚本会添加到现有配置中
- 测试框架配置会被覆盖

### 7. 错误处理

```rust
pub enum ConfigError {
    IoError(String),      // 文件读写错误
    ParseError(String),   // TOML 解析错误
}
```

错误处理策略：
- 配置加载失败时使用 `unwrap_or_default()` 回退到默认配置
- 配置错误不会阻止程序运行，但可能导致功能受限

## 使用示例

### 基本配置文件示例

```toml
# .analyzer.toml
version = "1.0"

[global]
default_format = "markdown"
filter_warnings = false
default_output = "analysis_report.md"

# 覆盖默认命令
[commands.type-check]
exec = "npm run typecheck"
description = "Run TypeScript type checker (custom alias)"
tech_stacks = ["npm", "pnpm", "yarn"]

# 自定义命令
[commands.lint-fix]
exec = "npm run lint -- --fix"
description = "Run linter with auto-fix"
tech_stacks = ["npm", "pnpm", "yarn"]

# 技术栈特定配置
[tech_stack.npm]
test_framework = "jest"

[tech_stack.npm.scripts]
"test" = "jest"
"lint" = "eslint . --ext .ts,.tsx"
```

### 代码中使用

```rust
// 加载配置
let config = Config::load(Path::new(".")).unwrap_or_default();

// 获取命令执行字符串
let cmd = config.get_command_exec("cargo", "check");
// 返回: Some("cargo check")

// 检查命令是否启用
if config.is_command_enabled("npm", "lint") {
    // 执行 lint 命令
}

// 获取所有可用命令
let commands = config.get_available_commands("cargo");
// 返回: ["check", "clippy", "clippy-all", "check-test", "test"]
```

## 测试覆盖

配置模块包含完整的单元测试和集成测试：

### 单元测试（`src/core/config.rs`）
- `test_embedded_defaults` - 测试内置默认值加载
- `test_merge` - 测试配置合并
- `test_get_command_exec` - 测试命令获取
- `test_is_command_enabled` - 测试命令启用检查
- `test_get_available_commands` - 测试可用命令列表
- `test_tech_stack_override` - 测试技术栈覆盖

### 集成测试（`tests/config_integration_tests.rs`）
- `test_config_loading` - 测试配置加载
- `test_registry_with_config` - 测试带配置的插件注册
- `test_registry_without_config` - 测试无配置的插件注册
- `test_config_get_command_exec` - 测试命令执行获取
- `test_config_is_command_enabled` - 测试命令启用状态
- `test_config_get_available_commands` - 测试可用命令
- `test_config_merge` - 测试配置合并
- `test_tech_stack_override` - 测试技术栈覆盖

## 与其他模块的集成

### 与主程序集成（`src/main.rs`）
```rust
// 加载配置
let config = Config::load(Path::new(".")).unwrap_or_default();

// 创建带配置的插件注册表
let registry = plugins::create_registry_with_config(Some(config.clone()));

// 应用配置默认值
if !options.filter_warnings && config.global.filter_warnings {
    options.filter_warnings = true;
}
```

### 与插件系统集成
配置通过 `create_registry_with_config()` 传递给插件注册表，各分析器插件可以：
- 获取自定义命令配置
- 检查命令是否启用
- 使用技术栈特定配置

## 设计优势

1. **多层级配置** - 支持灵活的配置覆盖，满足不同场景需求
2. **版本控制友好** - 所有配置在项目内，便于团队协作
3. **容错设计** - 配置加载失败时回退到默认配置
4. **类型安全** - 使用 Rust 结构体和 serde 保证配置类型安全
5. **可扩展性** - 支持自定义命令和技术栈特定配置
6. **无用户目录污染** - 避免配置分散和环境污染

## 待改进项

1. **命令映射集成** - 将命令映射完全集成到插件系统，替代硬编码逻辑
2. **配置验证** - 添加配置验证（命令格式、技术栈名称等）
3. **配置热重载** - 支持运行时配置重载
4. **配置调试** - 添加配置调试模式，显示最终合并后的配置
5. **配置文档生成** - 自动生成配置文件模板和文档

## 相关文档

- [配置加载逻辑分析](./config-loading-logic.md) - 详细的加载流程和实现分析
- [配置系统设计](./configuration-system-design.md) - 系统架构设计和实现计划

---

*文档版本: 1.0*  
*创建日期: 2026-04-21*  
*基于代码版本: analyzer 0.2.0*
