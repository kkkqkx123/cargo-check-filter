# Cargo 错误分析工具

该项目包含 Python 和 Rust 2个版本的命令行工具，用于分析和分类 Rust 编译错误和警告。该工具自动运行 `cargo check`，对错误/警告进行分类，并生成详细的 Markdown 报告。

## 发布版本信息 (使用rust构建)

- **Windows**: 为 Windows 用户提供预编译的发布包
- **Unix (Linux/macOS)**: Unix 用户需要从源代码构建，使用提供的 Rust 或 Python 文件

## 功能特性

- **自动分析**：运行 `cargo check` 并解析输出
- **分类整理**：将相似的错误和警告归类
- **统计信息**：提供错误类型和受影响文件的详细统计数据
- **过滤功能**：按警告/错误或特定文件路径进行过滤
- **Markdown 报告**：生成综合报告，格式为 Markdown
- **跨平台**：提供 Python 和 Rust 两种实现

## Python 版本

### 安装
- Python 3.6 或更高版本
- 无需额外依赖

### 使用方法
```bash
python analyze_cargo.py
```

### 选项
- `--filter-warnings`：过滤所有警告，仅显示错误
- `--filter-paths [PATHS ...]`：按文件路径过滤错误（绝对或相对路径）

### 示例
```bash
# 默认：分析所有错误和警告
python analyze_cargo.py

# 过滤警告，仅显示错误
python analyze_cargo.py --filter-warnings

# 仅显示特定路径的错误
python analyze_cargo.py --filter-paths src/core
python analyze_cargo.py --filter-paths src/core src/query

# 组合过滤器
python analyze_cargo.py --filter-warnings --filter-paths src/core
```

## Rust 版本

### 安装
- 已安装 Rust 工具链
- 兼容稳定版 Rust

### 编译
```bash
rustc analyze_cargo.rs -o analyze_cargo
```

### 使用方法
```bash
./analyze_cargo
```

### 选项
- `--output <file>`：指定输出文件路径（默认：cargo_errors_report.md）
- `--filter-warnings`：过滤警告，仅显示错误
- `--filter-paths <paths>`：按文件路径过滤错误（逗号分隔）

### 示例
```bash
# 默认使用
./analyze_cargo

# 指定输出文件
./analyze_cargo --output report.md

# 仅过滤警告
./analyze_cargo --filter-warnings

# 按特定路径过滤
./analyze_cargo --filter-paths src/main.rs,src/lib.rs

# 组合过滤器
./analyze_cargo --filter-warnings --output errors_only.md
```

## 报告输出

该工具生成一个综合的 Markdown 报告（`cargo_errors_report.md`），包含：

- 摘要统计
- 错误和警告类型分解
- 问题文件排名
- 详细分类和示例
- 每个错误的行号和描述

## 使用场景

- **代码质量评估**：识别代码库中的重复错误模式
- **重构规划**：重点关注错误/警告最多的文件
- **团队培训**：与团队成员分享常见错误模式
- **CI/CD 集成**：构建管道中的自动错误报告

## 贡献

两种实现都旨在具有相似的功能。欢迎通过以下方式贡献：

- 添加新的过滤选项
- 改进错误分类算法
- 增强报告格式
- 添加对其他 Cargo 输出格式的支持
- 添加压缩版的发行版可执行文件(例如使用upx)

## 许可证

该项目采用 MIT 许可证。