# Cargo Workspace & Target Options

Cargo analyzer supports extensive options for workspace management, target selection, and feature configuration.

## Workspace Options

| Option                 | Description                                           |
| ---------------------- | ----------------------------------------------------- |
| `--workspace`          | Analyze all workspace members                         |
| `-p, --package <SPEC>` | Analyze specific package (can be used multiple times) |
| `--exclude <SPEC>`     | Exclude specific package from analysis                |

### Examples

```bash
# Analyze entire workspace
analyzer cargo check --workspace

# Analyze specific package
analyzer cargo check --package my-crate

# Analyze multiple packages
analyzer cargo check -p crate1 -p crate2

# Exclude packages from workspace analysis
analyzer cargo check --workspace --exclude legacy-crate
```

## Target Options

| Option             | Description                       |
| ------------------ | --------------------------------- |
| `--lib`            | Analyze only the library target   |
| `--bin <NAME>`     | Analyze specific binary target    |
| `--bins`           | Analyze all binary targets        |
| `--test <NAME>`    | Analyze specific test target      |
| `--tests`          | Analyze all test targets          |
| `--example <NAME>` | Analyze specific example target   |
| `--examples`       | Analyze all example targets       |
| `--bench <NAME>`   | Analyze specific benchmark target |
| `--benches`        | Analyze all benchmark targets     |
| `--all-targets`    | Analyze all targets               |

### Examples

```bash
# Analyze only library
analyzer cargo check --lib

# Analyze specific binary
analyzer cargo check --bin my-app

# Analyze all binaries
analyzer cargo check --bins

# Analyze specific test
analyzer cargo check --test integration_test

# Analyze all tests
analyzer cargo check --tests

# Analyze specific example
analyzer cargo check --example demo

# Analyze all examples
analyzer cargo check --examples

# Analyze specific benchmark
analyzer cargo check --bench perf

# Analyze all benchmarks
analyzer cargo check --benches

# Analyze all targets
analyzer cargo check --all-targets
```

## Feature Options

| Option                  | Description                                |
| ----------------------- | ------------------------------------------ |
| `--features <FEATURES>` | Space-separated list of features to enable |
| `--all-features`        | Enable all available features              |
| `--no-default-features` | Do not enable the default feature          |

### Examples

```bash
# Enable specific features
analyzer cargo check --features "feat1 feat2"

# Enable all features
analyzer cargo check --all-features

# Disable default features
analyzer cargo check --no-default-features

# Disable default features and enable specific ones
analyzer cargo check --no-default-features --features minimal
```

## Combined Examples

```bash
# Analyze workspace tests with all features
analyzer cargo check --workspace --tests --all-features

# Analyze specific package with specific features
analyzer cargo check --package my-app --features "auth db"

# Analyze all targets in workspace
analyzer cargo clippy --workspace --all-targets

# Check library with no default features
analyzer cargo check --lib --no-default-features
```
