# Cargo Check Error Analysis Report

## Summary

- **Total Errors**: 0
- **Total Warnings**: 23
- **Total Issues**: 23
- **Unique Error Patterns**: 0
- **Unique Warning Patterns**: 20
- **Files with Issues**: 15

## Error Statistics

**Total Errors**: 0

## Warning Statistics

**Total Warnings**: 23

### Warning Type Breakdown

- **warning**: 23 warnings

### Files with Warnings (Top 10)

- `src\core\types.rs`: 7 warnings
- `src\core\analyzer.rs`: 2 warnings
- `src\plugins\python\pytest\parser.rs`: 2 warnings
- `src\plugins\python\mypy\analyzer.rs`: 1 warnings
- `src\plugins\cpp\clang\mod.rs`: 1 warnings
- `src\plugins\cpp\cmake\mod.rs`: 1 warnings
- `src\plugins\python\pytest\mod.rs`: 1 warnings
- `src\core\mod.rs`: 1 warnings
- `src\plugins\cpp\mod.rs`: 1 warnings
- `src\core\parser.rs`: 1 warnings

## Detailed Warning Categorization

### warning: field `base` is never read

**Total Occurrences**: 23  
**Unique Files**: 15

#### `src\core\types.rs`: 7 occurrences

- Line 142: methods `errors` and `warnings` are never used
- Line 174: field `execution_time` is never read
- Line 178: associated items `new`, `with_location`, `with_failure_details`, and `with_execution_time` are never used
- ... 4 more occurrences in this file

#### `src\core\analyzer.rs`: 2 occurrences

- Line 12: variants `ParseError` and `NotApplicable` are never constructed
- Line 50: methods `supported_commands` and `parser` are never used

#### `src\plugins\python\pytest\parser.rs`: 2 occurrences

- Line 10: field `base` is never read
- Line 23: method `parse_failure_line` is never used

#### `src\plugins\java\maven\parser.rs`: 1 occurrences

- Line 7: field `base` is never read

#### `src\plugins\cpp\clang\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::ClangParser`

#### `src\plugins\cpp\msvc\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::MsvcParser`

#### `src\core\parser.rs`: 1 occurrences

- Line 16: trait `StreamingOutputParser` is never used

#### `src\plugins\cpp\gcc\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::GccParser`

#### `src\plugins\python\pytest\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::PytestParser`

#### `src\plugins\python\mypy\analyzer.rs`: 1 occurrences

- Line 6: unused import: `SubCommand`

#### `src\plugins\cpp\mod.rs`: 1 occurrences

- Line 10: unused imports: `CompilerType` and `CppParser`

#### `src\plugins\python\pytest\analyzer.rs`: 1 occurrences

- Line 6: unused import: `SubCommand`

#### `src\plugins\java\gradle\parser.rs`: 1 occurrences

- Line 7: field `base` is never read

#### `src\core\mod.rs`: 1 occurrences

- Line 18: unused imports: `CommandConfig` and `ConfigError`

#### `src\plugins\cpp\cmake\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::CMakeParser`

