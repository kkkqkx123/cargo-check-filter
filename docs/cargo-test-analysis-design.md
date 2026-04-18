# Cargo Test 分析功能扩展设计文档

## 1. 概述

### 1.1 目标

扩展现有的 Cargo 插件，使其支持 `cargo test` 命令的分析，能够：

1. 复用现有的编译检查功能（解析编译错误和警告）
2. 额外处理测试输出，识别失败的测试用例
3. 提取失败测试的详细信息（测试名称、位置、失败原因等）

### 1.2 架构决策

经过分析，我们推荐采用**方案三：核心层扩展 TestAnalyzer trait + Cargo 插件实现**的架构。这种设计能够：

- 保持测试分析与其他技术栈的一致性
- 避免插件过度拆分导致的维护复杂度
- 为未来其他技术栈（npm、maven等）的测试分析提供统一接口

### 1.3 支持的命令变体

| 命令                       | 说明                       |
| -------------------------- | -------------------------- |
| `cargo test`               | 运行所有测试               |
| `cargo test --lib`         | 仅运行库测试               |
| `cargo test --bin <name>`  | 仅运行指定二进制文件的测试 |
| `cargo test --test <name>` | 仅运行指定集成测试         |
| `cargo test --doc`         | 仅运行文档测试             |
| `cargo test <filter>`      | 运行匹配过滤器的测试       |

## 2. 架构设计方案对比

### 2.1 方案一：在 Cargo 插件中直接扩展（原方案）

**实现方式**：在现有的 `CargoAnalyzer` 和 `CargoParser` 中直接添加 test 支持。

```
plugins/cargo/
├── mod.rs
├── analyzer.rs      # 扩展 SubCommand 匹配
└── parser.rs        # 添加 parse_test_output() 方法
```

**优点**：

- 实现简单，改动范围小
- 可以复用现有的编译错误解析逻辑

**缺点**：

- 测试分析与编译分析耦合在一起
- 其他技术栈（npm、maven）需要重复实现测试分析逻辑
- 随着支持的技术栈增多，代码难以维护

### 2.2 方案二：独立的 cargo-test 插件

**实现方式**：创建独立的 `cargo-test` 插件目录，复用 cargo 插件的编译解析代码。

```
plugins/
├── cargo/
│   ├── mod.rs
│   ├── analyzer.rs
│   └── parser.rs
└── cargo-test/      # 新插件
    ├── mod.rs
    ├── analyzer.rs  # 复用 cargo::parser
    └── parser.rs    # 专门解析 test 输出
```

**优点**：

- 职责分离清晰
- 可以独立发布和版本管理

**缺点**：

- 需要处理插件间的代码复用（编译错误解析）
- 用户需要知道使用 `cargo` 还是 `cargo-test` 命令
- 增加了架构复杂度
- 对于 `cargo test` 来说，编译和测试是紧密关联的，分离反而不自然

### 2.3 方案三：核心层扩展 TestAnalyzer trait（推荐）

**实现方式**：在核心层定义 `TestAnalyzer` trait，各技术栈插件按需实现。

```
core/
├── mod.rs
├── analyzer.rs      # BuildAnalyzer trait
├── test_analyzer.rs # 新增：TestAnalyzer trait
├── types.rs         # 扩展 TestResult, TestCase 等类型
└── ...

plugins/
├── cargo/
│   ├── mod.rs
│   ├── analyzer.rs  # 实现 BuildAnalyzer + TestAnalyzer
│   └── parser.rs    # 复用编译解析 + 新增 test 解析
└── npm/
    ├── mod.rs
    ├── analyzer.rs  # 未来可实现 TestAnalyzer
    └── parser.rs
```

**优点**：

- 统一的测试分析接口，各技术栈实现一致
- 编译分析和测试分析在核心层解耦，但在插件层可以灵活组合
- 为未来 npm test、mvn test 等提供标准接口
- 保持现有架构的简洁性

**缺点**：

- 需要修改核心层，影响范围较大
- 需要仔细设计 trait 边界

### 2.4 方案对比总结

| 维度             | 方案一：直接扩展 | 方案二：独立插件 | 方案三：TestAnalyzer trait |
| ---------------- | ---------------- | ---------------- | -------------------------- |
| 实现复杂度       | 低               | 中               | 中                         |
| 可维护性         | 低               | 中               | 高                         |
| 扩展性           | 差               | 中               | 好                         |
| 多技术栈一致性   | 差               | 中               | 好                         |
| 与现有架构契合度 | 好               | 差               | 好                         |

**推荐选择方案三**，因为它在保持架构一致性的同时，提供了最好的扩展性和可维护性。

## 3. 方案三详细设计

### 3.1 核心层扩展

#### 3.1.1 新增 test_analyzer.rs

```rust
//! 测试分析器 trait 定义
//! 定义测试执行的统一接口

use std::path::Path;
use super::types::{TestResult, TestSummary};

/// 测试分析器错误
#[derive(Debug)]
pub enum TestAnalyzerError {
    CommandFailed(String),
    ParseError(String),
    NotSupported,
}

impl std::fmt::Display for TestAnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestAnalyzerError::CommandFailed(msg) => write!(f, "Test command failed: {}", msg),
            TestAnalyzerError::ParseError(msg) => write!(f, "Test parse error: {}", msg),
            TestAnalyzerError::NotSupported => write!(f, "Test analysis not supported for this analyzer"),
        }
    }
}

impl std::error::Error for TestAnalyzerError {}

/// 测试分析器 trait
/// 实现此 trait 以支持测试执行和分析
pub trait TestAnalyzer: Send + Sync {
    /// 是否支持测试分析
    fn supports_test(&self) -> bool;

    /// 运行测试并返回结果
    fn run_tests(&self, options: &TestOptions) -> Result<TestResult, TestAnalyzerError>;

    /// 获取测试解析器
    fn test_parser(&self) -> Option<&dyn TestOutputParser> {
        None
    }
}

/// 测试输出解析器 trait
pub trait TestOutputParser: Send + Sync {
    /// 解析测试输出
    fn parse_test_output(&self, output: &str) -> ParsedTestOutput;
}

/// 测试选项
#[derive(Debug, Default, Clone)]
pub struct TestOptions {
    /// 测试过滤器（如 test name pattern）
    pub filter: Option<String>,
    /// 仅运行库测试
    pub lib_only: bool,
    /// 仅运行指定二进制文件的测试
    pub bin: Option<String>,
    /// 仅运行集成测试
    pub test: Option<String>,
    /// 仅运行文档测试
    pub doc_only: bool,
    /// 其他参数
    pub extra_args: Vec<String>,
}

/// 解析后的测试输出
#[derive(Debug, Default)]
pub struct ParsedTestOutput {
    /// 编译阶段的问题
    pub compile_issues: Vec<Issue>,
    /// 测试摘要
    pub test_summary: Option<TestSummary>,
    /// 失败的测试用例
    pub failed_tests: Vec<TestCase>,
    /// 通过的测试用例
    pub passed_tests: Vec<TestCase>,
    /// 被忽略的测试用例
    pub ignored_tests: Vec<TestCase>,
}
```

#### 3.1.2 扩展 types.rs

在现有基础上添加测试相关类型：

```rust
/// 测试结果类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestStatus {
    Passed,
    Failed,
    Ignored(Option<String>), // 忽略原因
}

/// 测试用例信息
#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub status: TestStatus,
    pub location: Option<Location>,
    pub failure_details: Option<String>,
    pub execution_time: Option<f64>,
}

/// 测试摘要
#[derive(Debug, Clone)]
pub struct TestSummary {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub ignored: usize,
    pub measured: usize,
    pub filtered: usize,
    pub execution_time: Option<f64>,
}

/// 扩展 AnalysisResult
#[derive(Debug, Default)]
pub struct AnalysisResult {
    // ... 现有字段

    /// 测试相关统计（新增）
    pub test_summary: Option<TestSummary>,
    /// 失败的测试用例（新增）
    pub failed_tests: Vec<TestCase>,
    /// 通过的测试用例（新增）
    pub passed_tests: Vec<TestCase>,
    /// 被忽略的测试用例（新增）
    pub ignored_tests: Vec<TestCase>,
    /// 是否有测试输出（新增）
    pub has_test_output: bool,
}
```

### 3.2 Cargo 插件实现

#### 3.2.1 CargoAnalyzer 实现 TestAnalyzer

```rust
use crate::core::{TestAnalyzer, TestOptions, TestAnalyzerError, TestOutputParser};

impl CargoAnalyzer {
    /// 创建测试命令
    fn create_test_command(&self, options: &TestOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("cargo").arg("test");

        if options.lib_only {
            builder = builder.arg("--lib");
        }

        if let Some(ref bin) = options.bin {
            builder = builder.arg("--bin").arg(bin);
        }

        if let Some(ref test) = options.test {
            builder = builder.arg("--test").arg(test);
        }

        if options.doc_only {
            builder = builder.arg("--doc");
        }

        if let Some(ref filter) = options.filter {
            builder = builder.arg(filter);
        }

        // 添加 --nocapture 以获取完整输出
        builder = builder.arg("--").arg("--nocapture");

        builder
    }
}

impl TestAnalyzer for CargoAnalyzer {
    fn supports_test(&self) -> bool {
        true
    }

    fn run_tests(&self, options: &TestOptions) -> Result<TestResult, TestAnalyzerError> {
        let builder = self.create_test_command(options);
        let output = builder.execute()
            .map_err(|e| TestAnalyzerError::CommandFailed(e.to_string()))?;

        // 使用 TestOutputParser 解析输出
        let parsed = self.test_parser()
            .ok_or(TestAnalyzerError::NotSupported)?
            .parse_test_output(&output);

        Ok(TestResult::ParsedOutput(parsed))
    }

    fn test_parser(&self) -> Option<&dyn TestOutputParser> {
        Some(&self.parser)
    }
}
```

#### 3.2.2 CargoParser 实现 TestOutputParser

```rust
use crate::core::{TestOutputParser, ParsedTestOutput, TestCase, TestStatus, TestSummary};

impl TestOutputParser for CargoParser {
    fn parse_test_output(&self, output: &str) -> ParsedTestOutput {
        let mut result = ParsedTestOutput::new();

        // 1. 复用现有逻辑解析编译问题
        result.compile_issues = self.parse(output);

        // 2. 解析测试执行结果
        let lines: Vec<&str> = output.lines().collect();
        let mut i = 0;
        let mut in_failures_section = false;
        let mut current_failure: Option<(String, Vec<String>)> = None;

        while i < lines.len() {
            let line = lines[i];

            // 解析测试用例行: "test <name> ... <result>"
            if let Some(test_case) = self.parse_test_case_line(line) {
                match test_case.status {
                    TestStatus::Passed => result.passed_tests.push(test_case),
                    TestStatus::Failed => {
                        // 稍后填充失败详情
                        result.failed_tests.push(test_case);
                    }
                    TestStatus::Ignored(_) => result.ignored_tests.push(test_case),
                }
                i += 1;
                continue;
            }

            // 识别 failures 区块开始
            if line == "failures:" {
                in_failures_section = true;
                i += 1;
                continue;
            }

            // 在 failures 区块中解析失败详情
            if in_failures_section {
                if line.starts_with("---- ") && line.contains("stdout ----") {
                    // 开始新的失败详情
                    let test_name = line[5..line.find(" stdout ----").unwrap_or(line.len())].to_string();
                    current_failure = Some((test_name, Vec::new()));
                } else if line.trim().is_empty() && current_failure.is_some() {
                    // 空行表示当前失败详情结束
                    if let Some((name, details)) = current_failure.take() {
                        // 找到对应的测试用例并填充详情
                        if let Some(test) = result.failed_tests.iter_mut().find(|t| t.name == name) {
                            test.failure_details = Some(details.join("\n"));
                            // 尝试从详情中解析位置
                            test.location = self.parse_panic_location(&details.join("\n"));
                        }
                    }
                } else if let Some((_, ref mut details)) = current_failure {
                    details.push(line.to_string());
                }

                // failures 区块结束标记
                if line.starts_with("test result:") {
                    in_failures_section = false;
                }
            }

            // 解析测试结果汇总
            if line.starts_with("test result:") {
                result.test_summary = self.parse_test_summary(line);
            }

            i += 1;
        }

        result
    }
}

impl CargoParser {
    /// 解析单个测试用例行
    fn parse_test_case_line(&self, line: &str) -> Option<TestCase> {
        // 匹配: "test <name> ... ok/FAILED/ignored"
        let re = regex::Regex::new(
            r"^test\s+(\S+)\s+\.\.\.\s+(ok|FAILED|ignored)(?:\s*\(([^)]+)\))?"
        ).ok()?;

        let caps = re.captures(line)?;

        let name = caps.get(1)?.as_str().to_string();
        let result_str = caps.get(2)?.as_str();
        let extra = caps.get(3).map(|m| m.as_str());

        let status = match result_str {
            "ok" => TestStatus::Passed,
            "FAILED" => TestStatus::Failed,
            "ignored" => TestStatus::Ignored(extra.map(|s| s.to_string())),
            _ => return None,
        };

        // 尝试从 extra 解析执行时间
        let execution_time = extra.and_then(|e| {
            if e.ends_with("s") {
                e[..e.len()-1].parse().ok()
            } else {
                None
            }
        });

        Some(TestCase {
            name,
            status,
            location: None,
            failure_details: None,
            execution_time,
        })
    }

    /// 从 panic 信息中解析位置
    fn parse_panic_location(&self, detail: &str) -> Option<Location> {
        let re = regex::Regex::new(r"panicked at\s+(\S+):(\d+):(\d+)").ok()?;
        let caps = re.captures(detail)?;

        Some(Location::new(caps.get(1)?.as_str().to_string())
            .with_line(caps.get(2)?.as_str().parse().ok()?)
            .with_column(caps.get(3)?.as_str().parse().ok()?))
    }

    /// 解析测试结果汇总
    fn parse_test_summary(&self, line: &str) -> Option<TestSummary> {
        // 匹配: "test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
        let re = regex::Regex::new(
            r"test result:\s+(ok|FAILED)\.\s+(\d+)\s+passed;\s+(\d+)\s+failed;\s+(\d+)\s+ignored;\s+(\d+)\s+measured;\s+(\d+)\s+filtered out"
        ).ok()?;

        let caps = re.captures(line)?;

        Some(TestSummary {
            total: caps.get(2)?.as_str().parse().ok()?
                + caps.get(3)?.as_str().parse().ok()?
                + caps.get(4)?.as_str().parse().ok()?,
            passed: caps.get(2)?.as_str().parse().ok()?,
            failed: caps.get(3)?.as_str().parse().ok()?,
            ignored: caps.get(4)?.as_str().parse().ok()?,
            measured: caps.get(5)?.as_str().parse().ok()?,
            filtered: caps.get(6)?.as_str().parse().ok()?,
            execution_time: None,
        })
    }
}
```

### 3.3 主程序集成

#### 3.3.1 命令行参数解析

```rust
fn parse_arguments(args: &[String]) -> (String, AnalyzeOptions) {
    // ... 现有逻辑

    // 检测 test 相关子命令
    if subcommand_str == "test" {
        subcommand = Some(SubCommand::Test(TestOptions::default()));
    } else if subcommand_str == "test-lib" {
        subcommand = Some(SubCommand::Test(TestOptions {
            lib_only: true,
            ..Default::default()
        }));
    } else if subcommand_str.starts_with("test-") {
        // 解析其他 test 变体
    }

    // ...
}
```

#### 3.3.2 分析器调用

```rust
fn main() {
    // ... 现有逻辑

    match analyzer.analyze(&options) {
        Ok(result) => {
            // 如果包含测试输出，额外处理
            if result.has_test_output {
                if let Some(ref summary) = result.test_summary {
                    println!("\nTest Summary: {}/{} passed",
                        summary.passed, summary.total);
                }
            }

            // 生成报告...
        }
        // ...
    }
}
```

## 4. 其他技术栈的测试分析扩展

### 4.1 NPM 测试分析（未来扩展）

基于 `TestAnalyzer` trait，NPM 插件可以轻松添加测试支持：

```rust
// plugins/npm/analyzer.rs
impl TestAnalyzer for NpmAnalyzer {
    fn supports_test(&self) -> bool {
        true
    }

    fn run_tests(&self, options: &TestOptions) -> Result<TestResult, TestAnalyzerError> {
        let mut builder = CommandBuilder::new(self.package_manager.as_str());
        builder = builder.arg("test");

        if let Some(ref filter) = options.filter {
            // npm test -- --grep "pattern"
            builder = builder.arg("--").arg("--grep").arg(filter);
        }

        let output = builder.execute()
            .map_err(|e| TestAnalyzerError::CommandFailed(e.to_string()))?;

        let parsed = self.test_parser()
            .ok_or(TestAnalyzerError::NotSupported)?
            .parse_test_output(&output);

        Ok(TestResult::ParsedOutput(parsed))
    }
}
```

### 4.2 Maven 测试分析（未来扩展）

```rust
// plugins/maven/analyzer.rs
impl TestAnalyzer for MavenAnalyzer {
    fn supports_test(&self) -> bool {
        true
    }

    fn run_tests(&self, options: &TestOptions) -> Result<TestResult, TestAnalyzerError> {
        let mut builder = CommandBuilder::new("mvn");
        builder = builder.arg("test");

        if let Some(ref filter) = options.filter {
            // mvn test -Dtest=TestClass#testMethod
            builder = builder.arg(format!("-Dtest={}", filter));
        }

        // ...
    }
}
```

### 4.3 统一测试报告

由于所有技术栈都实现了相同的 `TestAnalyzer` trait，可以生成统一的测试报告格式：

```markdown
# Test Report

## Summary

| Tech Stack | Total | Passed | Failed | Ignored |
| ---------- | ----- | ------ | ------ | ------- |
| Cargo      | 25    | 23     | 2      | 0       |
| NPM        | 50    | 48     | 1      | 1       |
| Maven      | 30    | 30     | 0      | 0       |

## Failed Tests

### Cargo

- `test_divide_by_zero` in `src/math.rs:42`
- `test_overflow` in `src/math.rs:58`

### NPM

- `should calculate total price` in `test/cart.test.js:15`
```

## 5. Cargo Test 输出格式分析

### 5.1 输出结构

`cargo test` 的输出分为两个主要部分：

#### 5.1.1 编译阶段输出

与 `cargo check` 类似，包含编译错误和警告：

```
   Compiling mycrate v0.1.0 (/path/to/project)
error[E0308]: mismatched types
  --> src/lib.rs:15:20
   |
15 |     let x: i32 = "hello";
   |                    ^^^^^^^ expected `i32`, found `&str`

warning: unused variable: `y`
  --> src/main.rs:10:9
   |
10 |     let y = 42;
   |         ^ help: if this is intentional, prefix it with an underscore
```

#### 5.1.2 测试执行阶段输出

```
running 3 tests
test tests::test_add ... ok
test tests::test_subtract ... FAILED
test tests::test_multiply ... ok

failures:

---- tests::test_subtract stdout ----
thread 'tests::test_subtract' panicked at src/lib.rs:25:5:
assertion failed: `(left == right)`
  left: `1`,
 right: `3`
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

failures:
    tests::test_subtract

test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

### 5.2 失败测试的详细模式

#### 5.2.1 测试用例结果行

```
test <测试名称> ... <结果>
```

结果可能是：

- `ok` - 测试通过
- `FAILED` - 测试失败
- `ignored` - 测试被忽略

#### 5.2.3 测试结果汇总行

```
test result: <ok|FAILED>. <passed> passed; <failed> failed; <ignored> ignored; <measured> measured; <filtered> filtered out
```

## 6. 实现细节补充

### 6.1 编译错误复用策略（关键）

`cargo test` 的输出包含两部分，**编译阶段完全复用现有逻辑**：

```rust
impl TestOutputParser for CargoParser {
    fn parse_test_output(&self, output: &str) -> ParsedTestOutput {
        let mut result = ParsedTestOutput::new();

        // 第一步：完全复用现有 parser 解析编译问题
        // 这部分与 cargo check/clippy 的解析完全一致
        result.compile_issues = self.parse(output);

        // 第二步：专门解析测试特有的输出
        result.test_summary = self.parse_test_summary(output);
        result.failed_tests = self.parse_failed_tests(output);

        result
    }
}
```

### 6.2 失败测试定位

从 panic 信息中提取源代码位置：

```
thread 'tests::test_subtract' panicked at src/lib.rs:25:5:
```

解析逻辑：

```rust
fn parse_panic_location(&self, detail: &str) -> Option<Location> {
    let re = regex::Regex::new(r"panicked at\s+(\S+):(\d+):(\d+)").ok()?;
    let caps = re.captures(detail)?;

    Some(Location::new(caps.get(1)?.as_str().to_string())
        .with_line(caps.get(2)?.as_str().parse().ok()?)
        .with_column(caps.get(3)?.as_str().parse().ok()?))
}
```

### 6.3 测试过滤支持

支持 `cargo test <filter>` 模式：

```rust
Some(SubCommand::TestFilter(ref pattern)) => {
    builder = builder.arg("test").arg(pattern);
}
```

### 6.4 原始方案参考（已废弃）

以下是原方案一的架构设计，供对比参考：

```
┌─────────────────────────────────────────────────────────────────┐
│                     CargoAnalyzer                               │
│  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────┐ │
│  │  CommandBuilder │    │  OutputParser   │    │  Reporter   │ │
│  │  (现有)         │    │  (扩展)         │    │  (现有)     │ │
│  └────────┬────────┘    └────────┬────────┘    └─────────────┘ │
│           │                      │                              │
│           ▼                      ▼                              │
│  ┌──────────────────────────────────────────────────────┐      │
│  │              CargoTestParser (新增)                   │      │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────┐  │      │
│  │  │ CompileParser│  │ TestParser   │  │  Merger    │  │      │
│  │  │ (复用现有)   │  │ (新增)       │  │ (新增)     │  │      │
│  │  └──────────────┘  └──────────────┘  └────────────┘  │      │
│  └──────────────────────────────────────────────────────┘      │
└─────────────────────────────────────────────────────────────────┘
```

#### 废弃方案中的类型定义

```rust
/// 测试结果类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestResult {
    Passed,
    Failed,
    Ignored(Option<String>), // 忽略原因
}

/// 测试用例信息
#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub result: TestResult,
    pub location: Option<Location>,
    pub failure_details: Option<String>,
    pub execution_time: Option<f64>,
}

/// 扩展 AnalysisResult 支持测试信息
#[derive(Debug, Default)]
pub struct AnalysisResult {
    // ... 现有字段

    /// 测试相关统计
    pub test_summary: Option<TestSummary>,
    /// 失败的测试用例
    pub failed_tests: Vec<TestCase>,
}
```

    TestOutput,     // 解析编译+测试输出

}

impl CargoParser {
/// 解析 test 输出
pub fn parse_test_output(&self, output: &str) -> TestParseResult {
let mut result = TestParseResult::new();

        // 1. 先解析编译阶段的问题（复用现有逻辑）
        let compile_issues = self.parse(output);
        result.compile_issues = compile_issues;

        // 2. 解析测试执行结果
        let lines: Vec<&str> = output.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            // 识别测试用例行
            if let Some(test_case) = self.parse_test_case_line(lines[i]) {
                if test_case.result == TestResult::Failed {
                    // 向前查找失败详情
                    let details = self.extract_failure_details(&lines, i);
                    result.failed_tests.push(TestCase {
                        failure_details: details,
                        ..test_case
                    });
                }
                i += 1;
                continue;
            }

            // 识别测试结果汇总
            if let Some(summary) = self.parse_test_summary(lines[i]) {
                result.test_summary = Some(summary);
            }

            i += 1;
        }

        result
    }

    /// 解析单个测试用例行
    fn parse_test_case_line(&self, line: &str) -> Option<TestCase> {
        // 匹配格式: "test <name> ... <result>"
        let re = regex::Regex::new(r"^test\s+(\S+)\s+\.\.\.\s+(ok|FAILED|ignored)(?:\s*\((.*)\))?").ok()?;
        let caps = re.captures(line)?;

        let name = caps.get(1)?.as_str().to_string();
        let result_str = caps.get(2)?.as_str();
        let extra = caps.get(3).map(|m| m.as_str());

        let result = match result_str {
            "ok" => TestResult::Passed,
            "FAILED" => TestResult::Failed,
            "ignored" => TestResult::Ignored(extra.map(|s| s.to_string())),
            _ => return None,
        };

        Some(TestCase {
            name,
            result,
            location: None, // 可从失败详情中解析
            failure_details: None,
            execution_time: None,
        })
    }

    /// 提取失败测试的详细信息
    fn extract_failure_details(&self, lines: &[&str], failed_line: usize) -> Option<String> {
        // 在 "failures:" 区块中查找对应测试的详情
        // 格式:
        // ---- <test_name> stdout ----
        // <panic 信息>
        // <堆栈跟踪>

        let test_name = self.parse_test_case_line(lines[failed_line])?.name;

        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("---- ") && line.contains(&test_name) && line.contains("stdout ----") {
                // 收集直到下一个空行或分隔符
                let mut details = Vec::new();
                for j in (i + 1)..lines.len() {
                    if lines[j].trim().is_empty() || lines[j].starts_with("----") {
                        break;
                    }
                    details.push(lines[j].to_string());
                }
                return Some(details.join("\n"));
            }
        }

        None
    }

    /// 解析测试结果汇总
    fn parse_test_summary(&self, line: &str) -> Option<TestSummary> {
        // 匹配: "test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
        let re = regex::Regex::new(
            r"test result:\s+(ok|FAILED)\.\s+(\d+)\s+passed;\s+(\d+)\s+failed;\s+(\d+)\s+ignored"
        ).ok()?;

        let caps = re.captures(line)?;

        Some(TestSummary {
            total: caps.get(2)?.as_str().parse().ok()?
                + caps.get(3)?.as_str().parse().ok()?
                + caps.get(4)?.as_str().parse().ok()?,
            passed: caps.get(2)?.as_str().parse().ok()?,
            failed: caps.get(3)?.as_str().parse().ok()?,
            ignored: caps.get(4)?.as_str().parse().ok()?,
            execution_time: None,
        })
    }

}

````

#### 3.2.4 扩展 CargoAnalyzer

在 [plugins/cargo/analyzer.rs](file:///d:/项目/cli/analyze-cargo/src/plugins/cargo/analyzer.rs) 中添加 test 支持：

```rust
impl CargoAnalyzer {
    fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
        let mut builder = CommandBuilder::new("cargo");

        match options.subcommand {
            // ... 现有命令处理

            Some(SubCommand::Test) => {
                builder = builder.arg("test");
            }
            Some(SubCommand::TestLib) => {
                builder = builder.arg("test").arg("--lib");
            }
            Some(SubCommand::TestBin(ref name)) => {
                builder = builder.arg("test").arg("--bin").arg(name);
            }
            Some(SubCommand::TestDoc) => {
                builder = builder.arg("test").arg("--doc");
            }
            _ => {
                builder = builder.arg("check");
            }
        }

        // test 命令不需要 --message-format=short
        if !matches!(options.subcommand,
            Some(SubCommand::Test) | Some(SubCommand::TestLib) |
            Some(SubCommand::TestBin(_)) | Some(SubCommand::TestDoc)) {
            builder = builder.arg("--message-format=short");
        }

        builder
    }
}

impl BuildAnalyzer for CargoAnalyzer {
    fn analyze(&self, options: &AnalyzeOptions) -> Result<AnalysisResult, AnalyzerError> {
        let builder = self.create_command_builder(options);
        let output = builder.execute()?;

        // 判断是否为 test 命令
        let is_test = matches!(options.subcommand,
            Some(SubCommand::Test) | Some(SubCommand::TestLib) |
            Some(SubCommand::TestBin(_)) | Some(SubCommand::TestDoc)
        );

        if is_test {
            // 使用扩展的 test 解析器
            let test_result = self.parser.parse_test_output(&output);

            // 合并编译问题和测试失败
            let mut result = AnalysisResult::from_issues(test_result.compile_issues);
            result.test_summary = test_result.test_summary;
            result.failed_tests = test_result.failed_tests;

            Ok(self.filter_issues(result, options))
        } else {
            // 原有逻辑
            println!("Parsing output...");
            let issues = self.parser.parse(&output);
            println!("Found {} issues", issues.len());

            let result = AnalysisResult::from_issues(issues);
            Ok(self.filter_issues(result, options))
        }
    }
}
````

### 3.3 报告生成扩展

#### 3.3.1 扩展 MarkdownReporter

在 [core/reporter.rs](file:///d:/项目/cli/analyze-cargo/src/core/reporter.rs) 中扩展报告生成：

````rust
impl Reporter for MarkdownReporter {
    fn generate(&self, result: &AnalysisResult) -> Result<String, ReporterError> {
        let mut report = String::new();

        // 标题
        report.push_str("# Analysis Report\n\n");

        // 如果有测试信息，添加测试摘要
        if let Some(ref summary) = result.test_summary {
            report.push_str("## Test Summary\n\n");
            report.push_str(&format!("- **Total Tests**: {}\n", summary.total));
            report.push_str(&format!("- **Passed**: ✅ {}\n", summary.passed));
            report.push_str(&format!("- **Failed**: ❌ {}\n", summary.failed));
            report.push_str(&format!("- **Ignored**: 🔕 {}\n", summary.ignored));
            if let Some(time) = summary.execution_time {
                report.push_str(&format!("- **Execution Time**: {:.2}s\n", time));
            }
            report.push_str("\n");
        }

        // 失败的测试详情
        if !result.failed_tests.is_empty() {
            report.push_str("## Failed Tests\n\n");
            for test in &result.failed_tests {
                report.push_str(&format!("### `{}`\n\n", test.name));

                if let Some(ref location) = test.location {
                    report.push_str(&format!("**Location**: `{}:{}`\n\n",
                        location.file_path,
                        location.line_number.map(|n| n.to_string()).unwrap_or_default()
                    ));
                }

                if let Some(ref details) = test.failure_details {
                    report.push_str("**Failure Details**:\n");
                    report.push_str("```\n");
                    report.push_str(details);
                    report.push_str("\n```\n\n");
                }
            }
        }

        // 编译问题（原有逻辑）
        if result.total_issues > 0 {
            report.push_str("## Compile Issues\n\n");
            // ... 原有编译问题报告逻辑
        }

        Ok(report)
    }
}
````

## 4. 关键实现细节

### 4.1 编译错误复用策略

`cargo test` 的输出包含两部分：

1. **编译阶段**：与 `cargo check` 格式完全一致
2. **测试阶段**：特有的测试执行输出

## 7. 使用示例

### 7.1 命令行使用

```bash
# 运行所有测试并分析
analyzer cargo test

# 仅运行库测试
analyzer cargo test --lib

# 运行特定二进制文件的测试
analyzer cargo test --bin myapp

# 运行文档测试
analyzer cargo test --doc
```

### 7.2 输出示例

生成的报告将包含：

```markdown
# Analysis Report

## Test Summary

- **Total Tests**: 15
- **Passed**: ✅ 12
- **Failed**: ❌ 2
- **Ignored**: 🔕 1

## Failed Tests

### `tests::test_divide_by_zero`

**Location**: `src/math.rs:42`

**Failure Details**:
```

thread 'tests::test_divide_by_zero' panicked at src/math.rs:42:9:
attempt to divide by zero

```

### `tests::test_overflow`

**Location**: `src/math.rs:58`

**Failure Details**:
```

thread 'tests::test_overflow' panicked at src/math.rs:58:9:
assertion failed: result <= i32::MAX

```

## Compile Issues

- **Total Issues**: 3
- **Errors**: 1
- **Warnings**: 2
...
```

## 8. 扩展建议

### 8.1 未来增强

1. **测试覆盖率集成**：结合 `cargo tarpaulin` 或 `cargo llvm-cov`
2. **基准测试支持**：`cargo bench` 结果分析
3. **Doc-test 代码块提取**：提取失败的 doc-test 中的代码示例
4. **测试历史追踪**：对比多次运行的测试结果

### 8.2 性能考虑

- 对于大型项目，`cargo test` 输出可能很大，建议使用流式解析
- 考虑使用 `--nocapture` 标志时的额外输出处理

## 9. 总结

### 9.1 架构决策回顾

经过三种方案的对比，我们选择了**方案三：核心层扩展 TestAnalyzer trait**：

| 维度           | 方案一 | 方案二 | 方案三（推荐） |
| -------------- | ------ | ------ | -------------- |
| 实现复杂度     | 低     | 中     | 中             |
| 可维护性       | 低     | 中     | **高**         |
| 扩展性         | 差     | 中     | **好**         |
| 多技术栈一致性 | 差     | 中     | **好**         |

### 9.2 核心优势

1. **完全复用**现有的编译错误解析逻辑
2. **统一接口**为所有技术栈提供一致的测试分析能力
3. **灵活组合**编译分析和测试分析在插件层可以按需组合
4. **保持架构**的一致性和可扩展性

### 9.3 关键设计要点

- **TestAnalyzer trait** 定义在核心层，所有技术栈统一实现
- **编译错误解析**完全复用现有 `OutputParser::parse()` 方法
- **测试特有解析**通过 `TestOutputParser` trait 扩展
- **类型系统**扩展 `AnalysisResult` 支持测试信息，保持向后兼容

这种设计遵循了项目现有的插件架构模式，最小化代码重复，最大化功能复用，同时为未来扩展（如 npm test、mvn test）提供了清晰的接口。
