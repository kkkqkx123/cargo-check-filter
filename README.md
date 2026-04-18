# Analyzer - Multi-language Build Tool Error Analyzer

A multilingual build tool error analyzer with a plugin-based architecture, supporting technology stacks such as Cargo, NPM, Maven, Gradle, Mypy, Go, and Pytest.

## Features

- **Multi-language Support**: Analyze errors from various build tools
  - Rust/Cargo: `cargo check`, `cargo clippy`, `cargo test`
  - Python/Mypy: `mypy`, `mypy --strict`
  - Node.js/NPM: `npm lint`, `npm type-check`, `npm audit`
  - Java/Maven: `mvn compile`, `mvn test`
  - Java/Gradle: `gradle compileJava`, `gradle test`
  - Go: `go build`, `go test`, `go vet`
  - Python/Pytest: `pytest`
- **Plugin-based Architecture**: Easily extendable for new tools
- **Multiple Report Formats**: Markdown, JSON, HTML
- **Flexible Filtering**: Filter by warnings or specific file paths
- **Configuration Support**: `.analyzer.toml` for custom configurations

## Installation

### From Source

```bash
cargo build --release
```

The compiled binary will be at `target/release/analyzer`.

### Pre-built

Pre-compiled release packages are provided for Windows users.

## Usage

```bash
# Basic usage
analyzer <tech-stack> <subcommand> [options]

# Analyze Rust project
analyzer cargo check
analyzer cargo clippy
analyzer cargo test

# Analyze Python/Mypy project
analyzer mypy check
analyzer mypy --strict

# Analyze Node.js project
analyzer npm lint
analyzer npm type-check
analyzer npm audit

# Analyze Java/Maven project
analyzer maven compile
analyzer maven test

# Analyze Java/Gradle project
analyzer gradle compile
analyzer gradle test

# Analyze Go project
analyzer go build
analyzer go test
analyzer go vet

# Analyze Python/Pytest
analyzer pytest
```

### Options

- `--filter-warnings`: Filter out all warnings, only show errors
- `--filter-paths <paths>`: Filter errors by file paths (comma-separated)
- `--output <file>`: Specify output file path (default: analysis_report.md)

## Configuration

Create `.analyzer.toml` in your project root to customize behavior:

```toml
version = "1.0"

[global]
default_format = "markdown"
filter_warnings = false

[commands.type-check]
exec = "npm run typecheck"
description = "Run TypeScript type checker"
tech_stacks = ["npm", "pnpm", "yarn"]

[tech_stack.npm]
test_framework = "jest"
```

## Report Output

The tool generates comprehensive reports in multiple formats:

- **Markdown**: Human-readable reports with statistics and categorization
- **JSON**: Machine-readable format for CI/CD integration
- **HTML**: Styled HTML reports for web viewing

Reports include:
- Summary statistics
- Error and warning type breakdown
- Top files with issues
- Detailed categorization with examples
- Line numbers and descriptions for each error

## Architecture

```
CLI Entry → Core Module → Plugin Module
```

### Core Module (core/)

| Component          | Description                                            |
|--------------------|--------------------------------------------------------|
| `types.rs`         | Common data types (Issue, Location, AnalysisResult)   |
| `parser.rs`        | Output parsing interface                               |
| `analyzer.rs`      | Unified analyzer interface                             |
| `reporter/*`       | Report generation (Markdown/JSON/HTML)                |
| `command.rs`       | Command construction and execution                    |
| `base_analyzer.rs` | Generic analyzer implementation                        |

### Plugin Module (plugins/)

| Plugin   | Supported Commands                        |
|----------|------------------------------------------|
| Cargo    | `check`, `clippy`, `test`                 |
| Mypy     | `mypy`, `mypy --strict`                  |
| NPM      | `lint`, `type-check`, `audit`            |
| Maven    | `compile`, `test`                        |
| Gradle   | `compileJava`, `test`                    |
| Go       | `build`, `test`, `vet`                   |
| Pytest   | `pytest`                                 |

## Use Cases

- **Code Quality Assessment**: Identify recurring error patterns across your codebase
- **Refactoring Planning**: Focus on files with the most errors/warnings
- **CI/CD Integration**: Automated error reporting in build pipelines
- **Team Onboarding**: Share common error patterns with team members

## License

This project is licensed under the MIT License.
