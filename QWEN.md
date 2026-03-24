# QWEN.md - 项目上下文文档

## 项目概述

**Cargo Error Analysis Tool** 是一个用于分析和分类 Rust 编译错误/警告的跨平台命令行工具。该项目采用 **Python** 和 **Rust** 双语言实现，功能对等。

### 核心功能

- **自动分析**: 运行 `cargo test --lib --message-format=short` 并解析输出
- **错误分类**: 将相似的错误和警告按类型和模式归类
- **统计分析**: 提供错误类型分布、问题文件排名等详细统计
- **过滤功能**: 支持按警告/错误类型、文件路径进行过滤
- **Markdown 报告**: 生成结构化的错误分析报告

### 项目结构

```
analyze-cargo/
├── analyze_cargo.py      # Python 版本主程序
├── analyze_cargo.rs      # Rust 版本主程序（二进制入口）
├── lib.rs                # Rust 版本库文件（可复用模块）
├── Cargo.toml            # Rust 项目配置
├── README.md             # 英文文档
├── README_zh.md          # 中文文档
├── cargo_errors_report.md # 生成的错误报告示例
└── LISENSE               # MIT 许可证
```

## 技术栈

- **Python 版本**: Python 3.6+，无外部依赖（仅使用标准库）
- **Rust 版本**: Rust 2021 Edition，无外部依赖（仅使用标准库）
- **构建工具**: Cargo (Rust), 直接执行 (Python)

## 构建与运行

### Python 版本

```bash
# 运行分析（默认分析所有错误和警告）
python analyze_cargo.py

# 仅显示错误，过滤警告
python analyze_cargo.py --filter-warnings

# 按路径过滤
python analyze_cargo.py --filter-paths src/core

# 组合过滤器
python analyze_cargo.py --filter-warnings --filter-paths src/core
```

### Rust 版本

```bash
# 开发构建
rustc analyze_cargo.rs -o analyze_cargo

# 或使用 Cargo 构建发行版（优化代码大小）
cargo build --release

# 运行
./analyze_cargo                          # 默认输出到 cargo_errors_report.md
./analyze_cargo --output report.md       # 指定输出文件
./analyze_cargo --filter-warnings        # 仅显示错误
./analyze_cargo --filter-paths src/main.rs,src/lib.rs  # 按路径过滤
```

## 配置说明

### Cargo.toml 关键配置

```toml
[profile.release]
lto = true              # 链接时优化
panic = "abort"         # panic 时直接终止，减小二进制大小
opt-level = "z"         # 优先优化代码大小
codegen-units = 1       # 单编译单元，利于优化

[lib]
path = "lib.rs"         # 库文件路径

[[bin]]
name = "analyze_cargo"
path = "analyze_cargo.rs"  # 二进制入口
```

## 开发约定

### 代码风格

- **Rust**: 遵循 Rust 官方风格指南，使用 `rustfmt` 格式化
- **Python**: 遵循 PEP 8 风格指南

### 测试实践

Rust 版本包含单元测试，位于各文件的 `#[cfg(test)]` 模块中：

```bash
# 运行 Rust 测试
cargo test
```

### 错误处理

- **Rust**: 使用 `Result<T, Box<dyn std::error::Error>>` 进行错误传播
- **Python**: 使用 `try-except` 捕获异常，返回退出码

### 发布流程

- **Windows**: 提供预编译的 release 包
- **Unix/Linux/macOS**: 用户从源码构建

### 压缩分发

项目配置了最小化依赖和代码大小优化，支持使用 UPX 等工具进一步压缩可执行文件。

## 输出报告格式

工具生成 `cargo_errors_report.md`，包含以下部分：

1. **Summary**: 总错误数、警告数、唯一错误模式数、涉及文件数
2. **Error Statistics**: 错误类型分解、问题文件 Top 10
3. **Warning Statistics**: 警告类型分解、问题文件 Top 10
4. **Detailed Error Categorization**: 按错误类型详细分类，包含文件位置和示例
5. **Detailed Warning Categorization**: 按警告类型详细分类

## 使用场景

- **代码质量评估**: 识别重复错误模式
- **重构规划**: 定位问题最多的文件
- **CI/CD 集成**: 自动化错误报告
- **团队培训**: 分享常见错误模式

