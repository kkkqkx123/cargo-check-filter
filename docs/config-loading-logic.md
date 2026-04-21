# 配置文件加载逻辑分析

## 概述

本文档详细分析 `analyzer` 项目的配置文件加载逻辑，包括配置结构、加载流程、合并策略以及实际使用方式。

## 配置系统架构

### 设计原则

项目采用**三层配置架构**，优先级从低到高：

1. **内置默认值** (Embedded defaults) - 代码中硬编码的默认配置
2. **二进制目录配置** (Binary dir config) - 与可执行文件同目录的配置文件
3. **项目级配置** (Project config) - 项目根目录的配置文件

**重要设计决策**：项目明确**不使用用户目录配置**（如 `~/.analyzer.toml`），以避免配置分散和"在我的机器上能运行"问题。

### 配置文件位置

| 层级 | 路径 | 优先级 | 说明 |
|------|------|--------|------|
| 内置 | 代码中定义 (`src/core/config.rs:embedded_defaults()`) | 最低 | 默认命令映射 |
| 二进制目录 | `<binary_dir>/analyzer.toml` | 低 | 与可执行文件同目录 |
| 项目 | `./.analyzer.toml` | **高** | 隐藏文件，优先查找 |
| 项目 | `./analyzer.toml` | **高** | 普通配置文件 |

## 配置数据结构

### Config (根结构)

```rust
pub struct Config {
    pub version: String,                    // 配置版本
    pub global: GlobalConfig,               // 全局设置
    pub commands: HashMap<String, CommandConfig>,  // 命令定义
    pub tech_stacks: HashMap<String, TechStackConfig>,  // 技术栈配置
}
```

### GlobalConfig (全局配置)

```rust
pub struct GlobalConfig {
    pub default_format: String,      // 默认输出格式: markdown, json, html
    pub filter_warnings: bool,       // 是否默认过滤警告
    pub default_output: Option<String>,  // 默认报告输出路径
}
```

### CommandConfig (命令配置)

```rust
pub struct CommandConfig {
    pub exec: String,                    // 实际执行的命令
    pub description: Option<String>,     // 命令描述
    pub tech_stacks: Vec<String>,        // 适用的技术栈
    pub enabled: bool,                   // 是否启用
}
```

### TechStackConfig (技术栈配置)

```rust
pub struct TechStackConfig {
    pub commands: HashMap<String, CommandConfig>,  // 命令覆盖
    pub scripts: HashMap<String, String>,          // 脚本名称映射
    pub test_framework: Option<String>,            // 测试框架类型
}
```

## 配置加载流程

### 入口点

配置加载在 `src/main.rs:30` 触发：

```rust
let config = Config::load(Path::new(".")).unwrap_or_default();
```

### 加载函数 (`src/core/config.rs:103-120`)

```rust
pub fn load(project_path: &Path) -> Result<Self, ConfigError> {
    let mut config = Self::default();

    // 1. 加载内置默认值
    config.merge(Self::embedded_defaults());

    // 2. 加载二进制目录配置
    if let Some(binary_dir_config) = Self::load_from_binary_dir()? {
        config.merge(binary_dir_config);
    }

    // 3. 加载项目级配置（最高优先级）
    if let Some(project_config) = Self::load_from_project(project_path)? {
        config.merge(project_config);
    }

    Ok(config)
}
```

### 步骤详解

#### 步骤 1: 加载内置默认值

位置: `src/core/config.rs:123-235`

内置默认值包含以下预定义命令：

**Cargo 命令**:
- `check`: `cargo check` - 快速语法和类型检查
- `clippy`: `cargo clippy` - 运行 Clippy linter
- `clippy-all`: `cargo clippy --all-targets --all-features` - 全量 Clippy 检查
- `check-test`: `cargo check --tests` - 检查测试代码
- `test`: `cargo test` - 运行测试

**NPM 命令**:
- `lint`: `npm run lint` - 运行 linter
- `type-check`: `npm run type-check` - TypeScript 类型检查
- `audit`: `npm audit` - 依赖安全审计

**Mypy 命令**:
- `mypy`: `mypy` - 运行 mypy 类型检查器
- `mypy-strict`: `mypy --strict` - 严格模式

#### 步骤 2: 加载二进制目录配置

位置: `src/core/config.rs:238-248`

```rust
fn load_from_binary_dir() -> Result<Option<Self>, ConfigError> {
    let exe_path = std::env::current_exe()?;
    let binary_dir = exe_path.parent()?;
    let config_path = binary_dir.join("analyzer.toml");
    Self::load_from_file(&config_path)
}
```

此配置适用于所有使用该二进制文件的项目，可用于分发时附带默认配置。

#### 步骤 3: 加载项目级配置

位置: `src/core/config.rs:251-261`

```rust
fn load_from_project(project_path: &Path) -> Result<Option<Self>, ConfigError> {
    // 优先查找隐藏文件 .analyzer.toml
    let hidden_config = project_path.join(".analyzer.toml");
    if hidden_config.exists() {
        return Self::load_from_file(&hidden_config);
    }

    // 然后查找 analyzer.toml
    let config_path = project_path.join("analyzer.toml");
    Self::load_from_file(&config_path)
}
```

**优先级**: `.analyzer.toml` > `analyzer.toml`

#### 步骤 4: 文件解析

位置: `src/core/config.rs:264-276`

```rust
fn load_from_file(path: &Path) -> Result<Option<Self>, ConfigError> {
    if !path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(Some(config))
}
```

使用 TOML 格式解析配置文件。

## 配置合并策略

### 合并函数 (`src/core/config.rs:279-314`)

```rust
fn merge(&mut self, other: Self) {
    // 1. 合并全局配置
    if other.global.default_format != default_format() {
        self.global.default_format = other.global.default_format;
    }
    self.global.filter_warnings = other.global.filter_warnings;
    if other.global.default_output.is_some() {
        self.global.default_output = other.global.default_output;
    }

    // 2. 合并命令（高优先级覆盖低优先级）
    for (name, cmd_config) in other.commands {
        self.commands.insert(name, cmd_config);
    }

    // 3. 合并技术栈配置
    for (stack_name, stack_config) in other.tech_stacks {
        self.tech_stacks
            .entry(stack_name)
            .and_modify(|existing| {
                // 合并命令
                for (cmd_name, cmd_config) in &stack_config.commands {
                    existing.commands.insert(cmd_name.clone(), cmd_config.clone());
                }
                // 合并脚本
                for (script_name, script_value) in &stack_config.scripts {
                    existing.scripts.insert(script_name.clone(), script_value.clone());
                }
                // 覆盖测试框架
                if stack_config.test_framework.is_some() {
                    existing.test_framework = stack_config.test_framework.clone();
                }
            })
            .or_insert(stack_config);
    }
}
```

### 合并规则

1. **全局配置**: 高优先级的非默认值覆盖低优先级
2. **命令配置**: 完全覆盖，相同命令名的配置会被替换
3. **技术栈配置**: 增量合并，命令和脚本会添加到现有配置中

## 配置使用

### 在 main.rs 中的使用

位置: `src/main.rs:149-151`

```rust
// 应用配置默认值（如果 CLI 未覆盖）
if !options.filter_warnings && config.global.filter_warnings {
    options.filter_warnings = true;
}
```

**注意**: 当前代码中，配置主要用于全局设置的默认值，命令映射尚未完全集成到插件系统中。

### 当前限制

1. **命令映射未完全使用**: 虽然 `Config` 定义了命令映射，但各插件（如 `CargoAnalyzer`, `NpmAnalyzer`）仍然使用硬编码的命令构建逻辑。

   例如 `src/plugins/cargo/analyzer.rs:23-48`:
   ```rust
   fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
       let mut builder = CommandBuilder::new("cargo");
       match options.subcommand {
           Some(SubCommand::Check) => {
               builder = builder.arg("check");
           }
           // ... 硬编码的命令构建
       }
   }
   ```

2. **配置验证缺失**: 加载配置后没有验证命令格式、技术栈名称等。

3. **测试覆盖不足**: 测试文件中没有配置加载的集成测试。

## 配置文件示例

### 项目根目录示例 (`.analyzer.toml`)

```toml
version = "1.0"

[global]
default_format = "markdown"
filter_warnings = false

# 覆盖默认的 type-check 命令
[commands.type-check]
exec = "npm run typecheck"
description = "Run TypeScript type checker (custom alias)"
tech_stacks = ["npm", "pnpm", "yarn"]

# 自定义命令
[commands.lint-fix]
exec = "npm run lint -- --fix"
description = "Run linter with auto-fix"
tech_stacks = ["npm", "pnpm", "yarn"]

# NPM 技术栈特定配置
[tech_stack.npm]
[tech_stack.npm.scripts]
"test" = "jest"
"lint" = "eslint . --ext .ts,.tsx"
"typecheck" = "tsc --noEmit"

test_framework = "jest"
```

## 错误处理

### ConfigError 枚举

位置: `src/core/config.rs:319-332`

```rust
pub enum ConfigError {
    IoError(String),      // 文件读写错误
    ParseError(String),   // TOML 解析错误
}
```

### 错误处理策略

- `Config::load()` 返回 `Result<Config, ConfigError>`
- 在 `main.rs:30` 使用 `unwrap_or_default()`，配置加载失败时使用默认配置
- 这意味着配置错误不会阻止程序运行，但可能导致功能受限

## 测试

### 单元测试

位置: `src/core/config.rs:336-368`

```rust
#[test]
fn test_embedded_defaults() {
    let config = Config::embedded_defaults();
    assert!(!config.commands.is_empty());
    assert!(config.commands.contains_key("check"));
    assert!(config.commands.contains_key("test"));
}

#[test]
fn test_merge() {
    let mut base = Config::embedded_defaults();
    let mut override_config = Config::default();
    override_config.commands.insert(
        "check".to_string(),
        CommandConfig {
            exec: "cargo check --all-targets".to_string(),
            description: Some("Custom check".to_string()),
            tech_stacks: vec!["cargo".to_string()],
            enabled: true,
        },
    );
    base.merge(override_config);
    let check_cmd = base.commands.get("check");
    assert_eq!(check_cmd.unwrap().exec, "cargo check --all-targets");
}
```

## 总结

### 配置加载流程图

```
程序启动
    ↓
Config::load(Path::new("."))
    ↓
┌─────────────────────────┐
│ 1. 加载内置默认值        │
│   (embedded_defaults)   │
└──────────┬──────────────┘
           ↓
┌─────────────────────────┐
│ 2. 加载二进制目录配置    │
│   (<binary>/analyzer.toml)│
└──────────┬──────────────┘
           ↓
┌─────────────────────────┐
│ 3. 加载项目级配置        │
│   (./.analyzer.toml     │
│    或 ./analyzer.toml)  │
└──────────┬──────────────┘
           ↓
┌─────────────────────────┐
│ 4. 合并所有配置          │
│   (高优先级覆盖低优先级) │
└──────────┬──────────────┘
           ↓
      返回 Config
```

### 关键特点

1. **三层架构**: 内置 → 二进制目录 → 项目
2. **优先级明确**: 项目配置 > 二进制目录配置 > 内置默认
3. **增量合并**: 技术栈配置支持增量合并
4. **容错设计**: 配置加载失败时回退到默认配置
5. **版本控制友好**: 所有配置在项目内，便于团队协作

### 待改进项

1. 将命令映射集成到插件系统，替代硬编码逻辑
2. 添加配置验证（命令格式、技术栈名称等）
3. 添加配置加载的集成测试
4. 支持配置热重载
5. 添加配置调试模式（显示最终合并后的配置）

---

*文档版本: 1.0*
*分析日期: 2026-04-21*
*基于代码版本: analyzer 0.2.0*
