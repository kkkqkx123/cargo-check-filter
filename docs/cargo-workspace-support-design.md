# Cargo Workspace 与多目标支持设计文档

## 问题分析

当前 Cargo 插件存在以下严重缺陷：

1. **缺少 `--workspace` 支持** - 无法分析多包工作空间中的所有成员
2. **缺少 `--package` 支持** - 无法指定特定包进行分析
3. **目标选择不完整** - 只有 `ClippyAll` 支持 `--all-targets`，`check` 等命令不支持
4. **缺少特性控制** - 无法使用 `--features` 或 `--all-features`

## 设计方案

### 1. 扩展 AnalyzeOptions

在 `AnalyzeOptions` 中添加 Cargo 相关字段，参数直接透传给 Cargo 命令：

```rust
pub struct AnalyzeOptions {
    // ... 现有字段 ...

    // === Cargo Workspace 支持 ===
    /// --workspace
    pub workspace: bool,

    /// --package <SPEC>
    pub package: Vec<String>,

    /// --exclude <SPEC>
    pub exclude: Vec<String>,

    // === Cargo Target 支持 ===
    /// --lib
    pub lib: bool,

    /// --bin <NAME>
    pub bin: Vec<String>,

    /// --bins
    pub bins: bool,

    /// --test <NAME>
    pub test: Vec<String>,

    /// --tests
    pub tests: bool,

    /// --example <NAME>
    pub example: Vec<String>,

    /// --examples
    pub examples: bool,

    /// --bench <NAME>
    pub bench: Vec<String>,

    /// --benches
    pub benches: bool,

    /// --all-targets
    pub all_targets: bool,

    // === Cargo Feature 支持 ===
    /// --features <FEATURES>
    pub features: Vec<String>,

    /// --all-features
    pub all_features: bool,

    /// --no-default-features
    pub no_default_features: bool,
}
```

### 2. 修改 CargoAnalyzer

在 `create_command_builder` 中直接透传参数：

```rust
fn create_command_builder(&self, options: &AnalyzeOptions) -> CommandBuilder {
    // ... 基础命令构建 ...

    // Workspace 选项
    if options.workspace { builder = builder.arg("--workspace"); }
    for pkg in &options.package { builder = builder.arg("--package").arg(pkg); }
    for pkg in &options.exclude { builder = builder.arg("--exclude").arg(pkg); }

    // Target 选项
    if options.lib { builder = builder.arg("--lib"); }
    for name in &options.bin { builder = builder.arg("--bin").arg(name); }
    if options.bins { builder = builder.arg("--bins"); }
    for name in &options.test { builder = builder.arg("--test").arg(name); }
    if options.tests { builder = builder.arg("--tests"); }
    for name in &options.example { builder = builder.arg("--example").arg(name); }
    if options.examples { builder = builder.arg("--examples"); }
    for name in &options.bench { builder = builder.arg("--bench").arg(name); }
    if options.benches { builder = builder.arg("--benches"); }
    if options.all_targets { builder = builder.arg("--all-targets"); }

    // Feature 选项
    if !options.features.is_empty() {
        builder = builder.arg("--features").arg(options.features.join(","));
    }
    if options.all_features { builder = builder.arg("--all-features"); }
    if options.no_default_features { builder = builder.arg("--no-default-features"); }

    builder
}
```

### 3. CLI 参数映射

| CLI 参数 | 对应字段 | Cargo 参数 |
|---------|---------|-----------|
| `--workspace` | `workspace: true` | `--workspace` |
| `--package <SPEC>` | `package: vec![SPEC]` | `--package <SPEC>` |
| `--exclude <SPEC>` | `exclude: vec![SPEC]` | `--exclude <SPEC>` |
| `--lib` | `lib: true` | `--lib` |
| `--bin <NAME>` | `bin: vec![NAME]` | `--bin <NAME>` |
| `--bins` | `bins: true` | `--bins` |
| `--test <NAME>` | `test: vec![NAME]` | `--test <NAME>` |
| `--tests` | `tests: true` | `--tests` |
| `--example <NAME>` | `example: vec![NAME]` | `--example <NAME>` |
| `--examples` | `examples: true` | `--examples` |
| `--bench <NAME>` | `bench: vec![NAME]` | `--bench <NAME>` |
| `--benches` | `benches: true` | `--benches` |
| `--all-targets` | `all_targets: true` | `--all-targets` |
| `--features <FEATURES>` | `features: vec![FEATURES]` | `--features <FEATURES>` |
| `--all-features` | `all_features: true` | `--all-features` |
| `--no-default-features` | `no_default_features: true` | `--no-default-features` |

## 使用示例

### 分析整个工作空间
```bash
analyzer cargo check --workspace
```

### 分析特定包
```bash
analyzer cargo check --package my-crate
analyzer cargo check -p my-crate -p another-crate
```

### 分析特定目标
```bash
analyzer cargo check --lib                    # 仅库
analyzer cargo check --bins                   # 所有二进制
analyzer cargo check --tests                  # 所有测试
analyzer cargo check --examples               # 所有示例
analyzer cargo check --all-targets            # 所有目标
```

### 分析特定二进制/测试/示例
```bash
analyzer cargo check --bin my-app
analyzer cargo check --test integration_test
analyzer cargo check --example demo
```

### 组合使用
```bash
# 分析 workspace 中所有包的测试目标，启用所有特性
analyzer cargo check --workspace --tests --all-features

# 分析特定包的特定二进制
analyzer cargo check --package my-app --bin my-binary

# 排除某些包
analyzer cargo check --workspace --exclude legacy-crate

# 使用特定特性
analyzer cargo check --features "feature1 feature2"
analyzer cargo check --all-features
analyzer cargo check --no-default-features --features minimal
```

## 实施步骤

1. **修改 `AnalyzeOptions`** - 添加 Cargo 相关字段
2. **修改 `CargoAnalyzer`** - 更新命令构建逻辑
3. **修改 CLI 参数解析** - 在 `main.rs` 中添加参数解析
4. **测试验证** - 验证各种参数组合

## 注意事项

- 所有参数直接透传给 Cargo，不进行额外处理
- 参数优先级由 Cargo 自身处理
- 保持与原生 Cargo 命令的参数名称一致
