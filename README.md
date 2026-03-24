# Cargo Error Analysis Tool

This project contains cli tool written by two language (Python and Rust)  for analyzing and categorizing Rust compilation errors and warnings. This tool automatically runs cargo commands, categorizes the errors/warnings, and generates a detailed Markdown report.

## Release Information (build by rust)

- **Windows**: Pre-compiled release packages are provided for Windows users
- **Unix (Linux/macOS)**: Unix users need to build from source using the provided Rust or Python files

## Features

- **Multiple Analysis Modes**:
  - Default: `cargo test --lib` - Analyze test-related errors
  - Minimal: `cargo check` - Quick syntax and type checking
  - Full: `cargo clippy --all-targets --all-features` - Comprehensive lint analysis
- **Categorization**: Groups similar errors and warnings together
- **Statistics**: Provides detailed statistics on error types and affected files
- **Filtering**: Filter by warnings/errors or specific file paths
- **Markdown Reports**: Generates comprehensive reports in Markdown format
- **Cross-platform**: Available in both Python and Rust implementations

## Python Version

### Installation
- Python 3.6 or higher
- No additional dependencies required

### Usage
```bash
# Default mode: analyze all errors and warnings (cargo test --lib)
python analyze_cargo.py

# Minimal mode: quick check (cargo check)
python analyze_cargo.py --minimal

# Full mode: comprehensive analysis (cargo clippy --all-targets --all-features)
python analyze_cargo.py --full

# Filter out warnings, only show errors
python analyze_cargo.py --filter-warnings

# Only show errors from specific paths
python analyze_cargo.py --filter-paths src/core
python analyze_cargo.py --filter-paths src/core src/query

# Combine filters
python analyze_cargo.py --filter-warnings --filter-paths src/core
```

### Options
- `--minimal`: Minimal mode - run `cargo check` instead of `cargo test --lib`
- `--full`: Full mode - run `cargo clippy --all-targets --all-features` for comprehensive analysis
- `--filter-warnings`: Filter out all warnings, only show errors
- `--filter-paths [PATHS ...]`: Filter errors by file paths (absolute or relative paths)

## Rust Version

### Installation
- Rust toolchain installed
- Compatible with stable Rust

### Compilation
```bash
rustc analyze_cargo.rs -o analyze_cargo
```

### build release
```bash
cargo build --release
```

### Usage
```bash
# Default mode: analyze all errors and warnings (cargo test --lib)
./analyze_cargo

# Minimal mode: quick check (cargo check)
./analyze_cargo --minimal

# Full mode: comprehensive analysis (cargo clippy --all-targets --all-features)
./analyze_cargo --full

# Specify output file
./analyze_cargo --output report.md

# Filter warnings only
./analyze_cargo --filter-warnings

# Filter by specific paths
./analyze_cargo --filter-paths src/main.rs,src/lib.rs

# Combine filters
./analyze_cargo --filter-warnings --output errors_only.md
```

### Options
- `--output <file>`: Specify output file path (default: cargo_errors_report.md)
- `--minimal`: Minimal mode - run `cargo check` instead of `cargo test --lib`
- `--full`: Full mode - run `cargo clippy --all-targets --all-features` for comprehensive analysis
- `--filter-warnings`: Filter warnings, only show errors
- `--filter-paths <paths>`: Filter errors by file paths (comma-separated)

## Report Output

The tool generates a comprehensive Markdown report (`cargo_errors_report.md`) containing:

- Summary statistics
- Error and warning type breakdown
- Top files with issues
- Detailed categorization with examples
- Line numbers and descriptions for each error

## Use Cases

- **Code Quality Assessment**: Identify recurring error patterns across your codebase
- **Refactoring Planning**: Focus on files with the most errors/warnings
- **Team Onboarding**: Share common error patterns with team members
- **CI/CD Integration**: Automated error reporting in build pipelines

## Contributing

Both implementations are designed to have similar functionality. Feel free to contribute by:

- Adding new filtering options
- Improving error categorization algorithms
- Enhancing the report format
- Adding support for additional Cargo output formats
- Add the compressed version of the distribution executable (like upx)

## License

This project is licensed under the MIT License.