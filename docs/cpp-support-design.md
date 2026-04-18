# C++ 编译检查支持设计文档

## 1. 概述

### 1.1 目标

为 analyze-cargo 项目添加 C++ 编译检查支持，覆盖主流构建工具（CMake）和直接工具链（GCC、Clang、MSVC）。

### 1.2 设计原则

基于项目新的设计思路：**完全基于用户输入判断，取消自动检测逻辑**。用户明确指定分析器和参数，工具不猜测项目类型。

### 1.3 支持范围

| 工具类型 | 具体工具 | 检查命令              | 说明                       |
| -------- | -------- | --------------------- | -------------------------- |
| 构建工具 | CMake    | `cmake --build`       | 需要指定源码目录和构建目录 |
| 编译器   | GCC      | `gcc -Wall -Wextra`   | 直接编译源文件             |
| 编译器   | Clang    | `clang -Wall -Wextra` | 直接编译源文件             |
| 编译器   | MSVC     | `cl.exe /W4`          | 直接编译源文件（Windows）  |

---

## 2. C++ 编译错误输出格式分析

### 2.1 GCC 错误格式

GCC 使用标准格式输出错误和警告：

```
file.cpp:line:column: severity: message [error-code]
```

**示例：**

```
src/main.cpp:10:5: error: 'x' was not declared in this scope
src/utils.cpp:25:12: warning: unused variable 'tmp' [-Wunused-variable]
src/math.cpp:42:8: note: suggested alternative: 'std::max'
```

**特点：**

- 文件路径、行号、列号使用冒号分隔
- severity 可以是：`error`, `warning`, `note`
- 可选的错误代码在方括号中
- 支持多行错误（包含上下文和代码片段）

### 2.2 Clang 错误格式

Clang 格式与 GCC 兼容，但提供更丰富的诊断信息：

```
file.cpp:line:column: severity: message
    code snippet
    ^
    note: additional info
```

**示例：**

```
src/main.cpp:10:5: error: use of undeclared identifier 'x'
    int y = x + 1;
            ^
src/main.cpp:8:9: note: declared here
    int x = 0;
        ^
```

**特点：**

- 基本格式与 GCC 相同
- 提供代码片段和位置标记（^）
- 支持诊断分类（`-fdiagnostics-format`）
- GCC 9+ 和 Clang 支持 JSON 输出：`-fdiagnostics-format=json`

### 2.3 MSVC (cl.exe) 错误格式

MSVC 使用不同的格式：

```
file.cpp(line,column): severity Cxxxx: message
```

**示例：**

```
src\main.cpp(10,5): error C2065: 'x': undeclared identifier
src\utils.cpp(25,12): warning C4101: 'tmp': unreferenced local variable
src\math.cpp(42): fatal error C1083: Cannot open include file: 'header.h': No such file or directory
```

**特点：**

- 使用圆括号包裹行号和列号
- 错误代码格式为 `C` + 四位数字
- severity 可以是：`error`, `warning`, `fatal error`
- 路径使用反斜杠（Windows 风格）

### 2.4 CMake 构建输出

CMake 本身不直接产生编译错误，而是调用底层编译器。错误格式取决于使用的编译器。

CMake 配置阶段错误：

```
CMake Error at CMakeLists.txt:10 (add_executable):
  Cannot find source file:
    src/main.cpp
```

**特点：**

- 构建阶段的错误来自底层编译器
- 需要解析编译器输出而非 CMake 输出
- 支持多种生成器（Ninja、Makefiles、Visual Studio）

---

## 3. 插件架构设计

### 3.1 插件划分策略

基于新的设计思路（无自动检测），采用以下插件划分方案：

```
src/plugins/
├── cpp/                    # C++ 基础模块（共享解析逻辑）
│   ├── mod.rs
│   └── parser.rs           # CppParser 共享解析器
├── cmake/                  # CMake 构建工具
│   ├── mod.rs
│   ├── analyzer.rs
│   └── parser.rs
├── gcc/                    # GCC 编译器
│   ├── mod.rs
│   ├── analyzer.rs
│   └── parser.rs
├── clang/                  # Clang 编译器
│   ├── mod.rs
│   ├── analyzer.rs
│   └── parser.rs
└── msvc/                   # MSVC 编译器
    ├── mod.rs
    ├── analyzer.rs
    └── parser.rs
```

### 3.2 划分理由

1. **CMake 作为独立插件**：CMake 是构建系统生成器，需要处理配置和构建两个阶段
2. **编译器作为独立插件**：GCC、Clang、MSVC 可以直接使用，用户明确指定使用哪个
3. **共享解析代码**：通过 `cpp` 模块共享通用的 C++ 错误解析逻辑（GCC 和 Clang 格式兼容）

### 3.3 与旧设计的区别

| 方面             | 旧设计                       | 新设计                            |
| ---------------- | ---------------------------- | --------------------------------- |
| 检测逻辑         | `is_applicable` 检测项目类型 | 完全基于用户输入                  |
| CMake 编译器选择 | 自动检测底层编译器           | 用户通过参数指定或 CMake 自动处理 |
| 源文件发现       | 自动扫描目录                 | 用户通过参数指定                  |
| 使用灵活性       | 受限于检测逻辑               | 支持任意目录结构和文件名          |

---

## 4. 命令执行设计

### 4.1 CMake 插件

**支持的命令：**
| 命令 | 说明 |
|-----|------|
| `cmake` | 默认执行构建 |
| `cmake-build` | 显式指定 CMake 构建 |
| `cmake-check` | 仅配置，不构建 |

**命令构建逻辑：**

```rust
fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
    let mut builder = CommandBuilder::new("cmake");

    // 获取源码目录（通过参数指定，默认为当前目录）
    let source_dir = options.source_dir.as_deref().unwrap_or(".");
    // 获取构建目录（通过参数指定，默认为 "build"）
    let build_dir = options.build_dir.as_deref().unwrap_or("build");

    match options.subcommand {
        Some(SubCommand::Check) => {
            // 仅配置，不构建
            builder = builder.arg("-B").arg(build_dir);
            builder = builder.arg("-S").arg(source_dir);
            // 可以指定生成器
            if let Some(ref generator) = options.cmake_generator {
                builder = builder.arg("-G").arg(generator);
            }
        }
        _ => {
            // 先配置（如果需要）
            // 然后构建
            builder = builder.arg("--build").arg(build_dir);
            // 可以指定目标
            if let Some(ref target) = options.target {
                builder = builder.arg("--target").arg(target);
            }
        }
    }

    builder
}
```

**关键考虑：**

- CMake 需要先生成构建系统，再执行构建
- 用户可以通过 `--source-dir` 和 `--build-dir` 指定目录
- 用户可以通过 `--cmake-generator` 指定生成器（Ninja、Make、MSBuild）
- 构建输出包含编译器错误，需要透传解析

**CMake 编译器选择策略：**

CMake 的编译器选择有两种方式：

1. **CMake 自动选择**（推荐）：
   - CMake 根据 `CMAKE_C_COMPILER` 和 `CMAKE_CXX_COMPILER` 环境变量选择
   - 或根据系统默认编译器选择
   - 解析器需要自动检测编译器类型（从 CMake 输出或缓存文件）

2. **用户显式指定**：
   - 通过 `--cmake-compiler` 参数指定（gcc/clang/msvc）
   - 插件根据指定值设置对应的环境变量

**推荐方案**：采用方式1，CMake 自动选择编译器，解析器自动检测输出格式。

### 4.2 GCC 插件

**支持的命令：**
| 命令 | 说明 |
|-----|------|
| `gcc` | 默认编译检查 |
| `gcc-check` | 语法检查（-fsyntax-only） |
| `gcc-build` | 完整编译 |

**命令构建逻辑：**

```rust
fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
    let mut builder = CommandBuilder::new("gcc");

    // 基础警告选项
    builder = builder
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Wpedantic");

    match options.subcommand {
        Some(SubCommand::Check) => {
            // 仅语法检查，不生成输出
            builder = builder.arg("-fsyntax-only");
        }
        _ => {
            // 完整编译，指定输出
            builder = builder.arg("-c").arg("-o").arg("/dev/null");
        }
    }

    // 添加头文件搜索路径
    for include_path in &options.include_paths {
        builder = builder.arg("-I").arg(include_path);
    }

    // 添加宏定义
    for define in &options.defines {
        builder = builder.arg(format!("-D{}", define));
    }

    // 添加源文件（通过参数指定）
    if let Some(ref files) = options.target_files {
        for file in files {
            builder = builder.arg(file);
        }
    } else {
        // 如果没有指定源文件，默认编译当前目录所有 .cpp 文件
        builder = builder.arg("*.cpp");
    }

    builder
}
```

### 4.3 Clang 插件

**支持的命令：**
| 命令 | 说明 |
|-----|------|
| `clang` | 默认编译检查 |
| `clang-check` | 语法检查 |
| `clang-tidy` | 静态分析（如可用） |

**命令构建逻辑：**

```rust
fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
    let mut builder = CommandBuilder::new("clang++");

    builder = builder
        .arg("-Wall")
        .arg("-Wextra")
        .arg("-Wpedantic");

    // Clang 特有选项
    if options.json_output {
        builder = builder.arg("-fdiagnostics-format=json");
    }

    match options.subcommand {
        Some(SubCommand::Check) => {
            builder = builder.arg("-fsyntax-only");
        }
        _ => {
            builder = builder.arg("-c").arg("-o").arg("/dev/null");
        }
    }

    // 添加头文件搜索路径、宏定义、源文件...
    // 同 GCC 插件

    builder
}
```

### 4.4 MSVC 插件

**支持的命令：**
| 命令 | 说明 |
|-----|------|
| `msvc` | 默认编译检查 |
| `cl` | 同 msvc |
| `msvc-check` | 语法检查 |

**命令构建逻辑：**

```rust
fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
    let mut builder = CommandBuilder::new("cl");

    // MSVC 警告选项
    builder = builder
        .arg("/W4")           // 最高警告级别
        .arg("/EHsc")         // 异常处理
        .arg("/nologo");      // 不显示版权信息

    match options.subcommand {
        Some(SubCommand::Check) => {
            // 仅语法检查
            builder = builder.arg("/Zs");
        }
        _ => {
            // 编译但不链接
            builder = builder.arg("/c");
            // 输出到临时目录
            builder = builder.arg("/Fo").arg("NUL");
        }
    }

    // 添加头文件搜索路径
    for include_path in &options.include_paths {
        builder = builder.arg("/I").arg(include_path);
    }

    // 添加宏定义
    for define in &options.defines {
        builder = builder.arg(format!("/D{}", define));
    }

    // 添加源文件
    if let Some(ref files) = options.target_files {
        for file in files {
            builder = builder.arg(file);
        }
    }

    builder
}
```

---

## 5. 新增参数设计

### 5.1 C++ 相关参数

| 参数                       | 说明                                               | 适用插件         |
| -------------------------- | -------------------------------------------------- | ---------------- |
| `--source-dir <path>`      | 源码目录                                           | cmake            |
| `--build-dir <path>`       | 构建目录                                           | cmake            |
| `--cmake-generator <name>` | CMake 生成器（Ninja/Unix Makefiles/Visual Studio） | cmake            |
| `--target-files <files>`   | 要编译的源文件列表                                 | gcc, clang, msvc |
| `--include-paths <paths>`  | 头文件搜索路径                                     | gcc, clang, msvc |
| `--defines <defs>`         | 宏定义                                             | gcc, clang, msvc |
| `--std <standard>`         | C++ 标准（c++11/c++14/c++17/c++20）                | gcc, clang, msvc |
| `--json-output`            | 使用 JSON 格式输出诊断                             | clang            |

### 5.2 参数示例

**CMake 示例：**

```bash
# 配置并构建
analyzer cmake --source-dir ./src --build-dir ./build

# 使用 Ninja 生成器
analyzer cmake --cmake-generator Ninja --build-dir ./build

# 仅配置
analyzer cmake-check --source-dir ./src

# 构建特定目标
analyzer cmake-build --build-dir ./build --target myapp
```

**GCC/Clang 示例：**

```bash
# 检查单个文件
analyzer gcc --target-files src/main.cpp

# 检查多个文件并指定头文件路径
analyzer gcc --target-files "src/*.cpp" --include-paths ./include,/usr/local/include

# 指定宏定义和标准
analyzer clang --target-files src/main.cpp --defines DEBUG,VERSION=1 --std c++17

# JSON 输出（Clang 特有）
analyzer clang --target-files src/main.cpp --json-output
```

**MSVC 示例：**

```bash
# Windows 平台
analyzer msvc --target-files src\main.cpp --include-paths .\include
```

---

## 6. 解析器设计

### 6.1 基础 C++ 解析器

```rust
// plugins/cpp/parser.rs

pub enum CompilerType {
    Gcc,
    Clang,
    Msvc,
}

pub struct CppParser {
    compiler_type: CompilerType,
}

impl CppParser {
    pub fn new(compiler_type: CompilerType) -> Self {
        Self { compiler_type }
    }

    pub fn with_gcc() -> Self {
        Self::new(CompilerType::Gcc)
    }

    pub fn with_clang() -> Self {
        Self::new(CompilerType::Clang)
    }

    pub fn with_msvc() -> Self {
        Self::new(CompilerType::Msvc)
    }
}

impl OutputParser for CppParser {
    fn parse(&self, output: &str) -> Vec<Issue> {
        match self.compiler_type {
            CompilerType::Gcc | CompilerType::Clang => {
                self.parse_gcc_style(output)
            }
            CompilerType::Msvc => {
                self.parse_msvc_style(output)
            }
        }
    }

    fn is_issue_start(&self, line: &str) -> bool {
        match self.compiler_type {
            CompilerType::Gcc | CompilerType::Clang => {
                line.contains(": error:")
                    || line.contains(": warning:")
                    || line.contains(": note:")
            }
            CompilerType::Msvc => {
                let msvc_pattern = regex::Regex::new(
                    r"\(\d+\s*(,\s*\d+)?\)\s*:\s*(error|warning|fatal error)"
                ).unwrap();
                msvc_pattern.is_match(line)
            }
        }
    }
}
```

### 6.2 GCC/Clang 解析实现

```rust
impl CppParser {
    fn parse_gcc_style(&self, output: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        let issue_regex = regex::Regex::new(
            r"^(.*?):(\d+):(\d+):\s*(error|warning|note):\s*(.*?)(?:\s*\[(.*?)\])?$"
        ).unwrap();

        for line in output.lines() {
            if let Some(caps) = issue_regex.captures(line) {
                let file_path = caps[1].to_string();
                let line_num = caps[2].parse::<u32>().ok();
                let col_num = caps[3].parse::<u32>().ok();
                let severity = &caps[4];
                let message = caps[5].to_string();
                let code = caps.get(6).map(|m| m.as_str().to_string());

                let level = match severity {
                    "error" => IssueLevel::Error,
                    "warning" => IssueLevel::Warning,
                    "note" => IssueLevel::Info,
                    _ => IssueLevel::Hint,
                };

                let location = Location::new(file_path)
                    .with_line(line_num.unwrap_or(0))
                    .with_column(col_num.unwrap_or(0));

                let mut issue = Issue::new(level, message, location);
                if let Some(c) = code {
                    issue = issue.with_code(c);
                }

                issues.push(issue);
            }
        }

        issues
    }
}
```

### 6.3 MSVC 解析实现

```rust
impl CppParser {
    fn parse_msvc_style(&self, output: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        let issue_regex = regex::Regex::new(
            r"^(.*?)\((\d+)\s*(?:,\s*(\d+))?\)\s*:\s*(error|warning|fatal error)\s+(\w+)?\s*:\s*(.*)$"
        ).unwrap();

        for line in output.lines() {
            if let Some(caps) = issue_regex.captures(line) {
                let file_path = caps[1].to_string();
                let line_num = caps[2].parse::<u32>().ok();
                let col_num = caps.get(3)
                    .and_then(|m| m.as_str().parse::<u32>().ok());
                let severity = &caps[4];
                let code = caps.get(5).map(|m| m.as_str().to_string());
                let message = caps[6].to_string();

                let level = match severity {
                    "error" | "fatal error" => IssueLevel::Error,
                    "warning" => IssueLevel::Warning,
                    _ => IssueLevel::Hint,
                };

                let mut location = Location::new(file_path);
                if let Some(ln) = line_num {
                    location = location.with_line(ln);
                }
                if let Some(cn) = col_num {
                    location = location.with_column(cn);
                }

                let mut issue = Issue::new(level, message, location);
                if let Some(c) = code {
                    issue = issue.with_code(c);
                }

                issues.push(issue);
            }
        }

        issues
    }
}
```

### 6.4 CMake 解析器

CMake 的解析器需要处理两部分输出：

1. CMake 配置阶段的错误（CMake 格式）
2. 编译阶段的错误（底层编译器格式）

```rust
pub struct CMakeParser {
    // 根据检测到的编译器类型使用对应的解析器
    cpp_parser: Option<CppParser>,
}

impl CMakeParser {
    pub fn new() -> Self {
        Self { cpp_parser: None }
    }

    pub fn with_compiler(compiler_type: CompilerType) -> Self {
        Self {
            cpp_parser: Some(CppParser::new(compiler_type)),
        }
    }
}

impl OutputParser for CMakeParser {
    fn parse(&self, output: &str) -> Vec<Issue> {
        let mut issues = Vec::new();

        // 解析 CMake 配置错误
        issues.extend(self.parse_cmake_errors(output));

        // 解析编译器错误
        if let Some(ref cpp_parser) = self.cpp_parser {
            issues.extend(cpp_parser.parse(output));
        } else {
            // 尝试自动检测编译器类型
            let compiler_type = self.detect_compiler_type(output);
            let parser = CppParser::new(compiler_type);
            issues.extend(parser.parse(output));
        }

        issues
    }

    fn is_issue_start(&self, line: &str) -> bool {
        // CMake 错误
        if line.starts_with("CMake Error") || line.starts_with("CMake Warning") {
            return true;
        }

        // 编译器错误
        if let Some(ref cpp_parser) = self.cpp_parser {
            cpp_parser.is_issue_start(line)
        } else {
            // 尝试通用检测
            line.contains(": error:")
                || line.contains(": warning:")
                || line.contains("(error")
                || line.contains("(warning")
        }
    }

    fn detect_compiler_type(&self, output: &str) -> CompilerType {
        // 通过输出特征检测编译器类型
        if output.contains("clang version") || output.contains("clang++") {
            CompilerType::Clang
        } else if output.contains("gcc version") || output.contains("g++") {
            CompilerType::Gcc
        } else if output.contains("Microsoft") || output.contains("cl.exe") {
            CompilerType::Msvc
        } else {
            // 默认使用 GCC 格式解析
            CompilerType::Gcc
        }
    }
}
```

---

## 7. 插件注册

```rust
// plugins/mod.rs

pub mod cpp;
pub mod cmake;
pub mod gcc;
pub mod clang;
pub mod msvc;

pub fn create_registry() -> PluginRegistry {
    let mut registry = PluginRegistry::new();

    // 现有插件...

    // 注册 C++ 插件（无优先级顺序，完全由用户指定）
    registry.register(Box::new(cmake::CMakeAnalyzer::new()));
    registry.register(Box::new(gcc::GccAnalyzer::new()));
    registry.register(Box::new(clang::ClangAnalyzer::new()));
    registry.register(Box::new(msvc::MsvcAnalyzer::new()));

    registry
}
```

---

## 8. 实现步骤建议

### Phase 1: 基础架构

1. 创建 `plugins/cpp/` 模块，实现共享解析器
2. 定义 `CompilerType` 枚举
3. 实现 GCC 风格的解析器

### Phase 2: GCC 支持

1. 实现 `plugins/gcc/` 插件
2. 添加 `--target-files`、`--include-paths`、`--defines` 参数支持
3. 添加 GCC 解析器单元测试

### Phase 3: Clang 支持

1. 实现 `plugins/clang/` 插件
2. 复用 GCC 解析器（格式兼容）
3. 添加 `--json-output` 参数支持

### Phase 4: MSVC 支持

1. 实现 `plugins/msvc/` 插件
2. 实现 MSVC 专用解析器
3. Windows 平台测试

### Phase 5: CMake 支持

1. 实现 `plugins/cmake/` 插件
2. 添加 `--source-dir`、`--build-dir`、`--cmake-generator` 参数
3. 实现编译器类型自动检测
4. 透传编译器错误解析

---

## 9. 注意事项

### 9.1 跨平台考虑

- Windows 路径使用反斜杠，需要统一处理
- MSVC 仅在 Windows 平台可用
- 命令查找使用 `resolve_command` 工具

### 9.2 编译器版本差异

- GCC 4.x 与 9+ 输出格式略有不同
- Clang 与 GCC 高度兼容，但诊断信息更丰富
- MSVC 不同版本错误代码可能变化

### 9.3 头文件依赖

- 直接使用编译器时需要通过 `--include-paths` 指定头文件路径
- CMake 会自动处理依赖关系
- 考虑支持 `compile_commands.json` 解析以获取准确的编译参数

### 9.4 性能考虑

- 大型项目编译可能耗时较长
- 建议支持超时配置
- 考虑增量编译或并行编译选项

### 9.5 与旧设计的兼容性

- 完全移除了 `is_applicable` 检测逻辑
- 所有配置必须通过参数显式指定
- 用户需要了解项目结构并正确指定参数
