# Analyzer 多语言支持架构设计文档

## 1. 概述

### 1.1 目标
将现有的 Cargo 错误分析工具扩展为多语言支持的分析器，支持多种技术栈的错误/警告分析。

### 1.2 支持的构建工具/技术栈

| 技术栈 | 命令 | 状态 |
|--------|------|------|
| Rust (Cargo) | `cargo`, `cargo clippy` | 已实现 |
| Java (Maven) | `mvn` | 规划中 |
| JavaScript/Node.js | `npm`, `pnpm`, `yarn` | 规划中 |
| Python | `mypy` | 规划中 |
| Go | `go build`, `golangci-lint` | 规划中 |

## 2. 架构设计

### 2.1 整体架构

```
analyzer/
├── src/                    # Rust 核心源码
│   ├── main.rs            # 入口点，命令路由
│   ├── cli.rs             # 命令行参数解析
│   ├── core/              # 核心抽象层
│   │   ├── mod.rs
│   │   ├── parser.rs      # 解析器 trait
│   │   ├── analyzer.rs    # 分析器 trait
│   │   ├── reporter.rs    # 报告生成 trait
│   │   └── types.rs       # 通用类型定义
│   ├── plugins/           # 各技术栈实现
│   │   ├── mod.rs
│   │   ├── cargo/         # Rust/Cargo 支持
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs
│   │   │   └── analyzer.rs
│   │   ├── maven/         # Java/Maven 支持
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs
│   │   │   └── analyzer.rs
│   │   ├── npm/           # Node.js/npm 支持
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs
│   │   │   └── analyzer.rs
│   │   ├── mypy/          # Python/mypy 支持
│   │   │   ├── mod.rs
│   │   │   ├── parser.rs
│   │   │   └── analyzer.rs
│   │   └── golang/        # Go 支持
│   │       ├── mod.rs
│   │       ├── parser.rs
│   │       └── analyzer.rs
│   └── report/            # 报告生成
│       ├── mod.rs
│       ├── markdown.rs
│       └── json.rs
├── py-src/                # Python 辅助脚本（可选）
│   └── ...
└── docs/                  # 文档
```

### 2.2 核心抽象层设计

#### 2.2.1 通用类型定义 (core/types.rs)

```rust
/// 问题级别
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IssueLevel {
    Error,
    Warning,
    Info,
    Hint,
}

/// 问题位置
#[derive(Debug, Clone)]
pub struct Location {
    pub file_path: String,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
}

/// 问题信息
#[derive(Debug, Clone)]
pub struct Issue {
    pub level: IssueLevel,
    pub code: Option<String>,       // 错误代码，如 E0308
    pub message: String,
    pub location: Location,
    pub context: Option<String>,    // 上下文信息
}

/// 问题分类统计
#[derive(Debug, Default)]
pub struct AnalysisResult {
    pub total_issues: usize,
    pub issues_by_level: HashMap<IssueLevel, usize>,
    pub issues_by_type: HashMap<String, usize>,
    pub issues_by_file: HashMap<String, Vec<Issue>>,
    pub unique_patterns: HashSet<String>,
}

/// 技术栈类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TechStack {
    Cargo,
    Maven,
    Npm,
    Pnpm,
    Yarn,
    Mypy,
    GoBuild,
    GolangciLint,
}
```

#### 2.2.2 解析器 Trait (core/parser.rs)

```rust
/// 输出解析器 trait
pub trait OutputParser: Send + Sync {
    /// 解析命令输出，提取问题信息
    fn parse(&self, output: &str) -> Vec<Issue>;
    
    /// 检查某行是否为问题起始行
    fn is_issue_start(&self, line: &str) -> bool;
    
    /// 解析单行问题信息
    fn parse_issue_line(&self, line: &str) -> Option<Issue>;
}
```

#### 2.2.3 分析器 Trait (core/analyzer.rs)

```rust
/// 构建工具分析器 trait
#[async_trait]
pub trait BuildAnalyzer: Send + Sync {
    /// 获取技术栈名称
    fn name(&self) -> &str;
    
    /// 获取支持的命令
    fn supported_commands(&self) -> Vec<&str>;
    
    /// 检查当前目录是否适用此分析器
    fn is_applicable(&self, project_path: &Path) -> bool;
    
    /// 运行分析命令
    async fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError>;
    
    /// 获取解析器
    fn parser(&self) -> &dyn OutputParser;
}

/// 分析选项
#[derive(Debug, Default)]
pub struct AnalyzeOptions {
    pub full: bool,              // 完整分析模式
    pub minimal: bool,           // 最小分析模式
    pub filter_warnings: bool,   // 过滤警告
    pub filter_paths: Vec<String>,
    pub additional_args: Vec<String>,
}
```

#### 2.2.4 报告生成器 Trait (core/reporter.rs)

```rust
/// 报告格式
#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    Markdown,
    Json,
    Html,
}

/// 报告生成器 trait
pub trait Reporter: Send + Sync {
    /// 生成报告
    fn generate(&self, result: &AnalysisResult, format: ReportFormat) -> Result<String, ReporterError>;
    
    /// 写入报告到文件
    fn write_to_file(&self, content: &str, path: &Path) -> Result<(), ReporterError>;
}
```

### 2.3 插件注册机制

```rust
// plugins/mod.rs

use std::collections::HashMap;
use crate::core::BuildAnalyzer;

pub struct PluginRegistry {
    analyzers: HashMap<String, Box<dyn BuildAnalyzer>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            analyzers: HashMap::new(),
        };
        registry.register_defaults();
        registry
    }
    
    fn register_defaults(&mut self) {
        self.register(Box::new(cargo::CargoAnalyzer::new()));
        self.register(Box::new(maven::MavenAnalyzer::new()));
        self.register(Box::new(npm::NpmAnalyzer::new()));
        self.register(Box::new(mypy::MypyAnalyzer::new()));
        self.register(Box::new(golang::GoAnalyzer::new()));
    }
    
    pub fn register(&mut self, analyzer: Box<dyn BuildAnalyzer>) {
        for cmd in analyzer.supported_commands() {
            self.analyzers.insert(cmd.to_string(), analyzer);
        }
    }
    
    pub fn get(&self, command: &str) -> Option<&dyn BuildAnalyzer> {
        self.analyzers.get(command).map(|b| b.as_ref())
    }
    
    pub fn detect_project(&self, path: &Path) -> Vec<&dyn BuildAnalyzer> {
        self.analyzers
            .values()
            .map(|b| b.as_ref())
            .filter(|a| a.is_applicable(path))
            .collect()
    }
}
```

## 3. 命令行接口设计

### 3.1 命令格式

```bash
# 基本格式
analyzer <tech-stack> [options]

# 示例
analyzer cargo --full
analyzer cargo --minimal --filter-warnings
analyzer maven --filter-paths src/main/java
analyzer npm --full
analyzer mypy --filter-warnings
analyzer go --full
```

### 3.2 全局选项

```
Global Options:
  -h, --help              显示帮助信息
  -v, --version           显示版本信息
  -o, --output <FILE>     指定输出文件路径
  -f, --format <FORMAT>   报告格式: markdown, json, html [默认: markdown]
  --detect                自动检测项目类型
```

### 3.3 分析选项

```
Analyze Options:
  --full                  完整分析模式（运行所有检查）
  --minimal               最小分析模式（仅基本检查）
  --filter-warnings       过滤警告，仅显示错误
  --filter-paths <PATHS>  按路径过滤（逗号分隔）
  --no-cache              禁用缓存
```

## 4. 各技术栈实现细节

### 4.1 Cargo (Rust)

```rust
// plugins/cargo/mod.rs

pub struct CargoAnalyzer {
    parser: CargoParser,
}

impl CargoAnalyzer {
    pub fn new() -> Self {
        Self {
            parser: CargoParser::new(),
        }
    }
}

#[async_trait]
impl BuildAnalyzer for CargoAnalyzer {
    fn name(&self) -> &str {
        "cargo"
    }
    
    fn supported_commands(&self) -> Vec<&str> {
        vec!["cargo", "rust"]
    }
    
    fn is_applicable(&self, project_path: &Path) -> bool {
        project_path.join("Cargo.toml").exists()
    }
    
    async fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let cmd = if options.full {
            vec!["cargo", "clippy", "--all-targets", "--all-features", "--message-format=short"]
        } else if options.minimal {
            vec!["cargo", "check", "--message-format=short"]
        } else {
            vec!["cargo", "test", "--lib", "--message-format=short"]
        };
        
        let output = run_command(&cmd).await?;
        let issues = self.parser.parse(&output);
        
        Ok(AnalysisResult::from_issues(issues))
    }
    
    fn parser(&self) -> &dyn OutputParser {
        &self.parser
    }
}
```

### 4.2 Maven (Java)

```rust
// plugins/maven/mod.rs

pub struct MavenAnalyzer {
    parser: MavenParser,
}

#[async_trait]
impl BuildAnalyzer for MavenAnalyzer {
    fn name(&self) -> &str {
        "maven"
    }
    
    fn supported_commands(&self) -> Vec<&str> {
        vec!["maven", "mvn", "java"]
    }
    
    fn is_applicable(&self, project_path: &Path) -> bool {
        project_path.join("pom.xml").exists()
    }
    
    async fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let cmd = if options.full {
            vec!["mvn", "clean", "compile", "-Dmaven.compiler.showWarnings=true"]
        } else {
            vec!["mvn", "compile", "-Dmaven.compiler.showWarnings=true"]
        };
        
        let output = run_command(&cmd).await?;
        let issues = self.parser.parse(&output);
        
        Ok(AnalysisResult::from_issues(issues))
    }
    
    fn parser(&self) -> &dyn OutputParser {
        &self.parser
    }
}
```

### 4.3 NPM/PNPM/Yarn (Node.js)

```rust
// plugins/npm/mod.rs

pub struct NpmAnalyzer {
    parser: NpmParser,
    package_manager: PackageManager,
}

#[derive(Debug, Clone)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
}

#[async_trait]
impl BuildAnalyzer for NpmAnalyzer {
    fn name(&self) -> &str {
        match self.package_manager {
            PackageManager::Npm => "npm",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Yarn => "yarn",
        }
    }
    
    fn supported_commands(&self) -> Vec<&str> {
        vec!["npm", "pnpm", "yarn", "node"]
    }
    
    fn is_applicable(&self, project_path: &Path) -> bool {
        project_path.join("package.json").exists()
    }
    
    async fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let cmd = match self.package_manager {
            PackageManager::Npm => {
                if options.full {
                    vec!["npm", "run", "lint", "--", "--max-warnings=0"]
                } else {
                    vec!["npm", "run", "type-check"]
                }
            }
            PackageManager::Pnpm => {
                if options.full {
                    vec!["pnpm", "lint", "--max-warnings=0"]
                } else {
                    vec!["pnpm", "type-check"]
                }
            }
            PackageManager::Yarn => {
                if options.full {
                    vec!["yarn", "lint", "--max-warnings=0"]
                } else {
                    vec!["yarn", "type-check"]
                }
            }
        };
        
        let output = run_command(&cmd).await?;
        let issues = self.parser.parse(&output);
        
        Ok(AnalysisResult::from_issues(issues))
    }
    
    fn parser(&self) -> &dyn OutputParser {
        &self.parser
    }
}
```

### 4.4 Mypy (Python)

```rust
// plugins/mypy/mod.rs

pub struct MypyAnalyzer {
    parser: MypyParser,
}

#[async_trait]
impl BuildAnalyzer for MypyAnalyzer {
    fn name(&self) -> &str {
        "mypy"
    }
    
    fn supported_commands(&self) -> Vec<&str> {
        vec!["mypy", "python"]
    }
    
    fn is_applicable(&self, project_path: &Path) -> bool {
        project_path.join("requirements.txt").exists()
            || project_path.join("pyproject.toml").exists()
            || project_path.join("setup.py").exists()
    }
    
    async fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let cmd = if options.full {
            vec!["mypy", "--strict", "--show-column-numbers", "."]
        } else {
            vec!["mypy", "--show-column-numbers", "."]
        };
        
        let output = run_command(&cmd).await?;
        let issues = self.parser.parse(&output);
        
        Ok(AnalysisResult::from_issues(issues))
    }
    
    fn parser(&self) -> &dyn OutputParser {
        &self.parser
    }
}
```

### 4.5 Go

```rust
// plugins/golang/mod.rs

pub struct GoAnalyzer {
    parser: GoParser,
    linter: GoLinter,
}

#[derive(Debug, Clone)]
pub enum GoLinter {
    Build,        // go build
    GolangciLint, // golangci-lint
}

#[async_trait]
impl BuildAnalyzer for GoAnalyzer {
    fn name(&self) -> &str {
        match self.linter {
            GoLinter::Build => "go",
            GoLinter::GolangciLint => "golangci-lint",
        }
    }
    
    fn supported_commands(&self) -> Vec<&str> {
        vec!["go", "golang", "golangci-lint"]
    }
    
    fn is_applicable(&self, project_path: &Path) -> bool {
        project_path.join("go.mod").exists()
    }
    
    async fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let cmd = match self.linter {
            GoLinter::Build => {
                if options.full {
                    vec!["go", "build", "-v", "./..."]
                } else {
                    vec!["go", "build", "./..."]
                }
            }
            GoLinter::GolangciLint => {
                if options.full {
                    vec!["golangci-lint", "run", "--enable-all", "./..."]
                } else {
                    vec!["golangci-lint", "run", "./..."]
                }
            }
        };
        
        let output = run_command(&cmd).await?;
        let issues = self.parser.parse(&output);
        
        Ok(AnalysisResult::from_issues(issues))
    }
    
    fn parser(&self) -> &dyn OutputParser {
        &self.parser
    }
}
```

## 5. 迁移计划

### 5.1 阶段一：重构现有代码

1. **创建新的目录结构**
   ```
   mkdir -p src/core src/plugins/cargo src/report
   ```

2. **提取通用类型**
   - 将 `lib.rs` 中的 `ErrorInfo` 和 `ErrorStats` 迁移到 `core/types.rs`
   - 标准化字段命名

3. **实现 Cargo 插件**
   - 将 `analyze_cargo.rs` 中的解析逻辑迁移到 `plugins/cargo/parser.rs`
   - 实现 `CargoAnalyzer` 结构体

### 5.2 阶段二：添加新插件

1. 按优先级逐个实现新插件
2. 每个插件包含完整的单元测试
3. 添加集成测试

### 5.3 阶段三：完善功能

1. 实现多种报告格式（JSON、HTML）
2. 添加缓存机制
3. 支持配置文件

## 6. 配置支持

### 6.1 配置文件格式 (.analyzer.toml)

```toml
[global]
format = "markdown"
output = "analysis_report.md"
cache = true

[cargo]
enabled = true
default_mode = "check"  # check, test, clippy

[maven]
enabled = true
goals = ["compile", "test"]

[npm]
enabled = true
package_manager = "npm"  # npm, pnpm, yarn

[mypy]
enabled = true
strict = false

[go]
enabled = true
linter = "golangci-lint"  # build, golangci-lint
```

## 7. 扩展指南

### 7.1 添加新的技术栈支持

1. 在 `src/plugins/` 下创建新目录
2. 实现 `OutputParser` trait
3. 实现 `BuildAnalyzer` trait
4. 在 `PluginRegistry` 中注册

### 7.2 示例：添加 Gradle 支持

```rust
// src/plugins/gradle/mod.rs

use crate::core::{BuildAnalyzer, OutputParser, Issue, Location, IssueLevel};

pub struct GradleAnalyzer;

#[async_trait]
impl BuildAnalyzer for GradleAnalyzer {
    fn name(&self) -> &str {
        "gradle"
    }
    
    fn supported_commands(&self) -> Vec<&str> {
        vec!["gradle"]
    }
    
    fn is_applicable(&self, path: &Path) -> bool {
        path.join("build.gradle").exists() || 
        path.join("build.gradle.kts").exists()
    }
    
    async fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        // 实现 Gradle 分析逻辑
        todo!()
    }
    
    fn parser(&self) -> &dyn OutputParser {
        &GradleParser
    }
}
```

## 8. 性能考虑

1. **并行分析**：支持同时运行多个分析器
2. **缓存机制**：缓存解析结果，避免重复分析
3. **增量分析**：只分析变更的文件
4. **流式解析**：大项目支持流式解析，减少内存占用

## 9. 测试策略

1. **单元测试**：每个解析器、分析器独立测试
2. **集成测试**：完整分析流程测试
3. ** fixtures**：使用真实项目输出作为测试数据
4. **性能测试**：大项目分析性能基准

## 10. 总结

本架构设计遵循以下原则：

1. **开闭原则**：对扩展开放，对修改关闭
2. **单一职责**：每个插件只负责一种技术栈
3. **依赖倒置**：依赖抽象而非具体实现
4. **插件化**：易于添加新的技术栈支持

通过这种设计，可以灵活地支持多种构建工具和技术栈，同时保持代码的可维护性和可扩展性。
