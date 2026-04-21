# 未使用代码说明文档

本文档记录了当前代码中未使用但为后续扩展预留的代码项。

## 概述

在重构为多语言支持的架构过程中，为了保持扩展性，我们保留了一些当前未使用但后续扩展必需的代码。这些代码主要用于：

1. 支持新的技术栈（Maven、NPM、Go 等）
2. 提供更丰富的错误/警告级别
3. 实现高级功能（自动检测、流式解析等）

---

## 核心模块 (core/types.rs)

### IssueLevel::Info 和 IssueLevel::Hint

```rust
pub enum IssueLevel {
    Error,
    Warning,
    Info,   // 未使用
    Hint,   // 未使用
}
```

**用途**：某些技术栈（如 TypeScript、ESLint）支持 info 和 hint 级别的问题。

**使用场景**：
- TypeScript 的 `tsc` 可能输出信息性消息
- ESLint 有 "suggestion" 级别

---

### Issue::context 和 with_context()

```rust
pub struct Issue {
    pub level: IssueLevel,
    pub code: Option<String>,
    pub message: String,
    pub location: Location,
    pub context: Option<String>,  // 未使用
}
```

**用途**：存储问题的额外上下文信息，如代码片段、相关变量等。

**使用场景**：
```rust
let issue = Issue::new(level, message, location)
    .with_context("variable `x` was declared here");
```

---

### AnalysisResult::errors() 和 warnings()

```rust
impl AnalysisResult {
    pub fn errors(&self) -> Vec<&Issue>;
    pub fn warnings(&self) -> Vec<&Issue>;
}
```

**用途**：便捷方法，快速获取特定级别的问题。

**使用场景**：
```rust
// 只显示错误
for error in result.errors() {
    println!("Error: {}", error.message);
}
```

---

### TechStack 枚举

```rust
pub enum TechStack {
    Cargo,
    Maven,      // 未使用
    Npm,        // 未使用
    Pnpm,       // 未使用
    Yarn,       // 未使用
    Mypy,       // 未使用
    GoBuild,    // 未使用
    GolangciLint, // 未使用
}
```

**用途**：类型安全的技术栈标识，用于配置和类型检查。

**使用场景**：
```rust
// 配置文件解析
let stack: TechStack = config.stack.parse()?;
match stack {
    TechStack::Cargo => analyze_cargo(),
    TechStack::Maven => analyze_maven(),
    // ...
}
```

---

### SubCommand 预留变体

```rust
pub enum SubCommand {
    // Cargo 子命令
    Check,
    Clippy,
    ClippyAll,
    Test,
    
    // 其他技术栈的预留
    MvnTest,    // Maven 测试 - 未使用
    GoLint,     // Go lint - 未使用
}
```

**用途**：为 Maven、Go 等后续技术栈预留的子命令。

---

### SubCommand::description()

```rust
impl SubCommand {
    pub fn description(&self) -> &'static str;
}
```

**用途**：生成帮助文档时显示子命令说明。

**使用场景**：
```rust
// 自动生成帮助信息
for cmd in available_commands {
    println!("  {} - {}", cmd.as_str(), cmd.description());
}
```

---

## 解析器模块 (core/parser.rs)

### OutputParser Trait 预留方法

```rust
pub trait OutputParser: Send + Sync {
    fn parse(&self, output: &str) -> Vec<Issue>;
    fn is_issue_start(&self, line: &str) -> bool;  // 未使用
    fn parse_issue(&self, lines: &[String], start_index: usize) -> (Option<Issue>, usize);  // 未使用
}
```

**用途**：支持流式解析，逐行处理大文件输出。

**使用场景**：
```rust
// 流式解析大输出
for (i, line) in lines.iter().enumerate() {
    if parser.is_issue_start(line) {
        let (issue, next_i) = parser.parse_issue(&lines, i);
        // 处理 issue...
    }
}
```

---

### BaseParser 辅助结构

```rust
pub struct BaseParser;

impl BaseParser {
    pub fn new() -> Self;
    pub fn parse_location(&self, location_str: &str) -> Option<Location>;
    pub fn detect_level(&self, text: &str) -> Option<IssueLevel>;
    pub fn extract_error_code(&self, text: &str) -> Option<String>;
}
```

**用途**：为新解析器提供通用的辅助方法。

**使用场景**：
```rust
pub struct MavenParser {
    base: BaseParser,
}

impl OutputParser for MavenParser {
    fn parse(&self, output: &str) -> Vec<Issue> {
        // 使用 base.parse_location() 解析位置
        // 使用 base.detect_level() 检测级别
    }
}
```

---

## 分析器模块 (core/analyzer.rs)

### AnalyzerError 预留变体

```rust
pub enum AnalyzerError {
    CommandFailed(String),
    ParseError(String),     // 未使用
    IoError(std::io::Error),
    NotApplicable,          // 未使用
}
```

**用途**：
- `ParseError`：解析失败时使用
- `NotApplicable`：分析器不适用时使用（如尝试用 Maven 分析 Go 项目）

---

### BuildAnalyzer::parser()

```rust
pub trait BuildAnalyzer: Send + Sync {
    // ...
    fn parser(&self) -> &dyn OutputParser;
}
```

**用途**：获取分析器使用的解析器，用于测试或自定义解析。

**使用场景**：
```rust
// 测试解析器
let analyzer = CargoAnalyzer::new();
let parser = analyzer.parser();
let issues = parser.parse(test_output);
```

---

### PluginRegistry::detect()

```rust
impl PluginRegistry {
    pub fn detect(&self, path: &Path) -> Vec<&dyn BuildAnalyzer>;
}
```

**用途**：自动检测项目适用的分析器。

**使用场景**：
```rust
// 自动检测项目类型
let registry = create_registry_with_config(None);
let applicable = registry.detect(Path::new("."));
for analyzer in applicable {
    println!("Detected: {}", analyzer.name());
}
```

---

## 报告模块 (core/reporter.rs)

### ReporterError::FormatError

```rust
pub enum ReporterError {
    IoError(std::io::Error),
    FormatError(String),  // 未使用
}
```

**用途**：报告格式错误时使用。

**使用场景**：
```rust
fn generate(&self, result: &AnalysisResult) -> Result<String, ReporterError> {
    if result.too_large() {
        return Err(ReporterError::FormatError("Result too large".into()));
    }
    // ...
}
```

---

### Reporter::format()

```rust
pub trait Reporter: Send + Sync {
    fn format(&self) -> ReportFormat;
    // ...
}
```

**用途**：获取报告生成器的格式类型。

**使用场景**：
```rust
let reporter = ReporterFactory::create(format);
match reporter.format() {
    ReportFormat::Markdown => println!("Generating Markdown..."),
    ReportFormat::Json => println!("Generating JSON..."),
    // ...
}
```

---

## 插件模块 (plugins/mod.rs)

### detect_project()

```rust
pub fn detect_project(path: &Path) -> Vec<String>;
```

**用途**：便捷的自动检测函数。

**使用场景**：
```rust
// CLI 自动检测
if args.detect {
    let detected = detect_project(Path::new("."));
    println!("Detected project types: {:?}", detected);
}
```

---

## 已删除的多余导入

以下导入已被删除，因为它们未被使用：

1. **`plugins/cargo/mod.rs`**：`pub use parser::CargoParser`
   - CargoParser 只被内部使用，不需要公开导出

2. **`main.rs`**：`use core::PluginRegistry`
   - 通过 `plugins::create_registry_with_config()` 间接使用

3. **`plugins/mod.rs`**：`use crate::core::BuildAnalyzer`
   - 只在类型签名中使用，实际代码未直接使用

---

## 总结

| 类别 | 数量 | 说明 |
|------|------|------|
| 核心扩展预留 | 10+ | 支持多语言、丰富功能 |
| 报告生成 | 2 | 多种格式支持 |
| 错误处理 | 2 | 完善的错误类型 |
| 辅助工具 | 4+ | BaseParser、便捷方法等 |
| 已删除 | 3 | 多余导入 |

这些未使用的代码是架构设计的一部分，为后续扩展提供了基础。在实际添加新功能时，这些代码会被自然使用。

