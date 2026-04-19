# Cargo Check Error Analysis Report

## Summary

- **Total Errors**: 0
- **Total Warnings**: 58
- **Total Issues**: 58
- **Unique Error Patterns**: 0
- **Unique Warning Patterns**: 51
- **Files with Issues**: 28

## Error Statistics

**Total Errors**: 0

## Warning Statistics

**Total Warnings**: 58

### Warning Type Breakdown

- **warning**: 58 warnings

### Files with Warnings (Top 10)

- `src\core\config.rs`: 11 warnings
- `src\core\types.rs`: 10 warnings
- `src\plugins\npm\analyzer.rs`: 3 warnings
- `src\plugins\go\parser.rs`: 3 warnings
- `src\core\test_analyzer.rs`: 3 warnings
- `src\core\parser.rs`: 2 warnings
- `src\core\reporter\mod.rs`: 2 warnings
- `src\core\base_analyzer.rs`: 2 warnings
- `src\plugins\python\pytest\parser.rs`: 2 warnings
- `src\core\analyzer.rs`: 2 warnings

## Detailed Warning Categorization

### warning: method `create_test_command` is never used

**Total Occurrences**: 58  
**Unique Files**: 28

#### `src\core\config.rs`: 11 occurrences

- Line 7: unused import: `PathBuf`
- Line 9: unused import: `crate::core::ReportFormat`
- Line 13: struct `Config` is never constructed
- ... 8 more occurrences in this file

#### `src\core\types.rs`: 10 occurrences

- Line 142: methods `errors` and `warnings` are never used
- Line 174: field `execution_time` is never read
- Line 178: associated items `new`, `with_location`, `with_failure_details`, and `with_execution_time` are never used
- ... 7 more occurrences in this file

#### `src\plugins\npm\analyzer.rs`: 3 occurrences

- Line 21: constant `TEST_ALIASES` is never used
- Line 39: associated function `detect` is never used
- Line 180: associated items `create_test_command` and `find_script_name` are never used

#### `src\core\test_analyzer.rs`: 3 occurrences

- Line 8: enum `TestAnalyzerError` is never used
- Line 57: struct `TestOptions` is never constructed
- Line 82: trait `TestAnalyzer` is never used

#### `src\plugins\go\parser.rs`: 3 occurrences

- Line 428: unused variable: `command_type`: help: if this is intentional, prefix it with an underscore: `_command_type`
- Line 26: field `base` is never read
- Line 39: associated items `with_command_type` and `set_command_type` are never used

#### `src\plugins\python\pytest\parser.rs`: 2 occurrences

- Line 10: field `base` is never read
- Line 23: method `parse_failure_line` is never used

#### `src\core\base_analyzer.rs`: 2 occurrences

- Line 10: trait `BaseBuildAnalyzer` is never used
- Line 78: trait `CommandBuilderAnalyzer` is never used

#### `src\core\parser.rs`: 2 occurrences

- Line 13: methods `is_issue_start` and `parse_issue` are never used
- Line 30: methods `parse_location` and `find_file_path` are never used

#### `src\core\analyzer.rs`: 2 occurrences

- Line 13: variants `ParseError` and `NotApplicable` are never constructed
- Line 52: methods `parser`, `set_config`, and `config` are never used

#### `src\core\reporter\mod.rs`: 2 occurrences

- Line 19: variant `FormatError` is never constructed
- Line 45: methods `generate_test_report` and `format` are never used

#### `src\plugins\python\pytest\analyzer.rs`: 1 occurrences

- Line 52: method `create_test_command` is never used

#### `src\plugins\cpp\msvc\parser.rs`: 1 occurrences

- Line 5: unused import: `CompilerType`

#### `src\plugins\java\maven\parser.rs`: 1 occurrences

- Line 7: field `base` is never read

#### `src\plugins\cpp\mod.rs`: 1 occurrences

- Line 10: unused imports: `CompilerType` and `CppParser`

#### `src\core\mod.rs`: 1 occurrences

- Line 19: unused imports: `CommandConfig`, `ConfigError`, and `Config`

#### `src\plugins\cpp\clang\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::ClangParser`

#### `src\plugins\npm\parser.rs`: 1 occurrences

- Line 719: variable does not need to be mutable

#### `src\plugins\cpp\gcc\parser.rs`: 1 occurrences

- Line 5: unused import: `CompilerType`

#### `src\plugins\cpp\clang\parser.rs`: 1 occurrences

- Line 5: unused import: `CompilerType`

#### `src\plugins\cpp\gcc\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::GccParser`

#### `src\plugins\cargo\analyzer.rs`: 1 occurrences

- Line 50: method `create_test_command` is never used

#### `src\core\command.rs`: 1 occurrences

- Line 80: multiple methods are never used

#### `src\plugins\java\gradle\parser.rs`: 1 occurrences

- Line 7: field `base` is never read

#### `src\plugins\cpp\cmake\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::CMakeParser`

#### `src\plugins\cpp\msvc\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::MsvcParser`

#### `src\plugins\python\mypy\parser.rs`: 1 occurrences

- Line 4: unused import: `IssueLevel`

#### `src\plugins\python\pytest\mod.rs`: 1 occurrences

- Line 8: unused import: `parser::PytestParser`

#### `src\plugins\go\analyzer.rs`: 1 occurrences

- Line 47: method `create_test_command` is never used

