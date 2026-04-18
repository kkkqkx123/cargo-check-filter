# Analyzer - 多语言构建工具错误分析器

一款支持多种技术栈的多语言构建工具错误分析器，采用插件化架构，支持 Cargo、NPM、Maven、Gradle、Mypy、Go 和 Pytest 等工具。

## 功能特性

- **多语言支持**：分析来自各种构建工具的错误
  - Rust/Cargo: `cargo check`、`cargo clippy`、`cargo test`
  - Python/Mypy: `mypy`、`mypy --strict`
  - Node.js/NPM: `npm lint`、`npm type-check`、`npm audit`
  - Java/Maven: `mvn compile`、`mvn test`
  - Java/Gradle: `gradle compileJava`、`gradle test`
  - Go: `go build`、`go test`、`go vet`
  - Python/Pytest: `pytest`
- **插件化架构**：可轻松扩展支持新工具
- **多种报告格式**：Markdown、JSON、HTML
- **灵活过滤**：按警告或特定文件路径进行过滤
- **配置支持**：通过 `.analyzer.toml` 自定义配置

## 安装

### 从源码构建

```bash
cargo build --release
```

编译后的二进制文件位于 `target/release/analyzer`。

### 预构建版本

为 Windows 用户提供预编译的发行版包。

## 使用方法

```bash
# 基本用法
analyzer <tech-stack> <subcommand> [options]

# 分析 Rust 项目
analyzer cargo check
analyzer cargo clippy
analyzer cargo test

# 分析 Python/Mypy 项目
analyzer mypy check
analyzer mypy --strict

# 分析 Node.js 项目
analyzer npm lint
analyzer npm type-check
analyzer npm audit

# 分析 Java/Maven 项目
analyzer maven compile
analyzer maven test

# 分析 Java/Gradle 项目
analyzer gradle compile
analyzer gradle test

# 分析 Go 项目
analyzer go build
analyzer go test
analyzer go vet

# 分析 Python/Pytest
analyzer pytest
```

### 选项

- `--filter-warnings`：过滤所有警告，仅显示错误
- `--filter-paths <paths>`：按文件路径过滤错误（逗号分隔）
- `--output <file>`：指定输出文件路径（默认：analysis_report.md）

## 配置

在项目根目录创建 `.analyzer.toml` 来自定义行为：

```toml
version = "1.0"

[global]
default_format = "markdown"
filter_warnings = false

[commands.type-check]
exec = "npm run typecheck"
description = "运行 TypeScript 类型检查器"
tech_stacks = ["npm", "pnpm", "yarn"]

[tech_stack.npm]
test_framework = "jest"
```

## 报告输出

该工具支持多种格式的综合报告：

- **Markdown**：人类可读的统计和分类报告
- **JSON**：机器可读格式，适合 CI/CD 集成
- **HTML**：用于网页查看的样式化 HTML 报告

报告包含：

- 摘要统计
- 错误和警告类型分解
- 问题文件排名
- 详细分类和示例
- 每个错误的行号和描述

## 架构设计

```
CLI 入口 → 核心模块 → 插件模块
```

### 核心模块 (core/)

| 组件               | 描述                                            |
| ------------------ | ----------------------------------------------- |
| `types.rs`         | 通用数据类型（Issue、Location、AnalysisResult） |
| `parser.rs`        | 输出解析接口                                    |
| `analyzer.rs`      | 统一分析器接口                                  |
| `reporter/*`       | 报告生成（Markdown/JSON/HTML）                  |
| `command.rs`       | 命令构建和执行                                  |
| `base_analyzer.rs` | 通用分析器实现                                  |

### 插件模块 (plugins/)

| 插件   | 支持的命令                    |
| ------ | ----------------------------- |
| Cargo  | `check`、`clippy`、`test`     |
| Mypy   | `mypy`、`mypy --strict`       |
| NPM    | `lint`、`type-check`、`audit` |
| Maven  | `compile`、`test`             |
| Gradle | `compileJava`、`test`         |
| Go     | `build`、`test`、`vet`        |
| Pytest | `pytest`                      |

## 使用场景

- **代码质量评估**：识别代码库中的重复错误模式
- **重构规划**：重点关注错误/警告最多的文件
- **CI/CD 集成**：构建管道中的自动错误报告
- **团队培训**：与团队成员分享常见错误模式

## 许可证

该项目采用 MIT 许可证。
