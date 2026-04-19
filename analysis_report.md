# Type Check Report

## Type Issues Summary

- **Total**: 22
- **⚠️** warning: 22
- **Categories**: 20
- **Files Affected**: 13

## Breakdown by Category

- **field `base` is never read**: 3 occurrence(s)
- **variants `MvnTest` and `GoLint` are**: 1 occurrence(s)
- **associated functions `new` and `from_compile_result`**: 1 occurrence(s)
- **associated items `new`, `with_location`, `with_failure_details`,**: 1 occurrence(s)
- **fields `passed_tests` and `has_test_output` are**: 1 occurrence(s)
- **unused imports: `CompilerType` and `CppParser`**: 1 occurrence(s)
- **methods `errors` and `warnings` are**: 1 occurrence(s)
- **unused import: `parser::MsvcParser`**: 1 occurrence(s)
- **methods `is_issue_start` and `parse_issue` are**: 1 occurrence(s)
- **methods `supported_commands` and `parser` are**: 1 occurrence(s)
- **associated items `description`, `custom`, and**: 1 occurrence(s)
- **method `parse_failure_line` is never used**: 1 occurrence(s)
- **variants `ParseError` and `NotApplicable` are**: 1 occurrence(s)
- **unused import: `parser::CMakeParser`**: 1 occurrence(s)
- **unused import: `parser::GccParser`**: 1 occurrence(s)
- **unused import: `parser::ClangParser`**: 1 occurrence(s)
- **unused imports: `CommandConfig` and `ConfigError`**: 1 occurrence(s)
- **field `execution_time` is never read**: 1 occurrence(s)
- **unused import: `parser::PytestParser`**: 1 occurrence(s)
- **methods `parse_location` and `find_file_path` are**: 1 occurrence(s)

## Details by File

### `src\core\types.rs` (7 item(s))

- ⚠️ **warning** at line 142:12: methods `errors` and `warnings` are never used
- ⚠️ **warning** at line 174:9: field `execution_time` is never read
- ⚠️ **warning** at line 178:12: associated items `new`, `with_location`, `with_failure_details`, and `with_execution_time` are never used
- ⚠️ **warning** at line 226:9: fields `passed_tests` and `has_test_output` are never read
- ⚠️ **warning** at line 234:12: associated functions `new` and `from_compile_result` are never used
- ⚠️ **warning** at line 323:5: variants `MvnTest` and `GoLint` are never constructed
- ⚠️ **warning** at line 373:12: associated items `description`, `custom`, and `is_custom` are never used

### `src\core\parser.rs` (2 item(s))

- ⚠️ **warning** at line 13:8: methods `is_issue_start` and `parse_issue` are never used
- ⚠️ **warning** at line 30:12: methods `parse_location` and `find_file_path` are never used

### `src\plugins\python\pytest\parser.rs` (2 item(s))

- ⚠️ **warning** at line 10:5: field `base` is never read
- ⚠️ **warning** at line 23:8: method `parse_failure_line` is never used

### `src\core\analyzer.rs` (2 item(s))

- ⚠️ **warning** at line 12:5: variants `ParseError` and `NotApplicable` are never constructed
- ⚠️ **warning** at line 50:8: methods `supported_commands` and `parser` are never used

### `src\plugins\cpp\msvc\mod.rs` (1 item(s))

- ⚠️ **warning** at line 8:9: unused import: `parser::MsvcParser`

### `src\plugins\python\pytest\mod.rs` (1 item(s))

- ⚠️ **warning** at line 8:9: unused import: `parser::PytestParser`

### `src\plugins\cpp\clang\mod.rs` (1 item(s))

- ⚠️ **warning** at line 8:9: unused import: `parser::ClangParser`

### `src\core\mod.rs` (1 item(s))

- ⚠️ **warning** at line 18:26: unused imports: `CommandConfig` and `ConfigError`

### `src\plugins\java\gradle\parser.rs` (1 item(s))

- ⚠️ **warning** at line 7:5: field `base` is never read

### `src\plugins\java\maven\parser.rs` (1 item(s))

- ⚠️ **warning** at line 7:5: field `base` is never read

### `src\plugins\cpp\mod.rs` (1 item(s))

- ⚠️ **warning** at line 10:18: unused imports: `CompilerType` and `CppParser`

### `src\plugins\cpp\cmake\mod.rs` (1 item(s))

- ⚠️ **warning** at line 8:9: unused import: `parser::CMakeParser`

### `src\plugins\cpp\gcc\mod.rs` (1 item(s))

- ⚠️ **warning** at line 8:9: unused import: `parser::GccParser`

