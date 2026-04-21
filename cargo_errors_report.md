# Cargo Check Error Analysis Report

## Summary

- **Total Errors**: 0
- **Total Warnings**: 57
- **Total Issues**: 57
- **Unique Error Patterns**: 0
- **Unique Warning Patterns**: 42
- **Files with Issues**: 20

## Error Statistics

**Total Errors**: 0

## Warning Statistics

**Total Warnings**: 57

### Warning Type Breakdown

- **warning**: 57 warnings

### Files with Warnings (Top 10)

- `tests\common\mod.rs`: 11 warnings
- `tests\common\vs_env.rs`: 8 warnings
- `src\plugins\npm\parser.rs`: 7 warnings
- `tests\cpp_integration_tests.rs`: 4 warnings
- `tests\go_integration_tests.rs`: 4 warnings
- `tests\cmake_real_integration_tests.rs`: 3 warnings
- `tests\cmake_parser_tests.rs`: 3 warnings
- `tests\mypy_integration_tests.rs`: 2 warnings
- `src\plugins\cargo\parser.rs`: 2 warnings
- `src\plugins\java\maven\parser.rs`: 2 warnings

## Detailed Warning Categorization

### warning: function `generate_test_report` is never used

**Total Occurrences**: 57  
**Unique Files**: 20

#### `tests\common\mod.rs`: 11 occurrences

- Line 145: function `generate_test_report` is never used
- Line 29: function `samples_dir` is never used
- Line 54: function `read_sample` is never used
- ... 8 more occurrences in this file

#### `tests\common\vs_env.rs`: 8 occurrences

- Line 8: constant `VS_DEV_SHELL_PATH` is never used
- Line 11: function `is_vs_dev_shell_available` is never used
- Line 17: function `run_with_vs_env` is never used
- ... 5 more occurrences in this file

#### `src\plugins\npm\parser.rs`: 7 occurrences

- Line 130: this manual char comparison can be written more succinctly: help: consider using an array of `char`: `['-', ':']`
- Line 619: stripping a suffix manually
- Line 639: called `map(..).flatten()` on `Option`: help: try replacing `map` with `and_then` and remove the `.flatten()`: `and_then(|m| m.as_str().parse().ok())`
- ... 4 more occurrences in this file

#### `tests\go_integration_tests.rs`: 4 occurrences

- Line 7: unused import: `raw_output_dir`
- Line 89: redundant pattern matching, consider using `is_ok()`
- Line 140: redundant pattern matching, consider using `is_ok()`
- ... 1 more occurrences in this file

#### `tests\cpp_integration_tests.rs`: 4 occurrences

- Line 8: unused import: `raw_output_dir`
- Line 196: the borrowed expression implements the required traits: help: change this to: `["--version"]`
- Line 209: the borrowed expression implements the required traits: help: change this to: `["--version"]`
- ... 1 more occurrences in this file

#### `tests\cmake_parser_tests.rs`: 3 occurrences

- Line 30: this `map_or` can be simplified
- Line 36: this `map_or` can be simplified
- Line 42: this `map_or` can be simplified

#### `tests\cmake_real_integration_tests.rs`: 3 occurrences

- Line 9: unused import: `raw_output_dir`
- Line 15: unused import: `CompilerType`
- Line 23: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do: help: change this to: `&Path`

#### `tests\mypy_integration_tests.rs`: 2 occurrences

- Line 5: unused import: `std::process::Command`
- Line 254: unnecessary `if let` since only the `Ok` variant of the iterator element is used

#### `src\plugins\java\maven\parser.rs`: 2 occurrences

- Line 30: stripping a prefix manually
- Line 77: length comparison to one: help: using `!is_empty` is clearer and more explicit: `!parts.is_empty()`

#### `src\plugins\cargo\parser.rs`: 2 occurrences

- Line 76: stripping a prefix manually
- Line 255: stripping a suffix manually

#### `tests\parser_integration_tests.rs`: 2 occurrences

- Line 4: unused import: `std::collections::HashMap`
- Line 400: unnecessary `if let` since only the `Ok` variant of the iterator element is used

#### `tests\gradle_integration_tests.rs`: 1 occurrences

- Line 22: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do: help: change this to: `&Path`

#### `src\core\types.rs`: 1 occurrences

- Line 125: use of `or_insert_with` to construct default value: help: try: `or_default()`

#### `src\core\parser.rs`: 1 occurrences

- Line 171: stripping a prefix manually

#### `tests\pytest_integration_tests.rs`: 1 occurrences

- Line 25: function `run_pytest` is never used

#### `tests\cpp_parser_tests.rs`: 1 occurrences

- Line 5: unused import: `std::path::PathBuf`

#### `src\plugins\go\parser.rs`: 1 occurrences

- Line 19: this `impl` can be derived

#### `src\core\reporter\json.rs`: 1 occurrences

- Line 71: useless use of `format!`: help: consider using `.to_string()`: `"      \"location\": {\n".to_string()`

#### `src\plugins\go\analyzer.rs`: 1 occurrences

- Line 28: wildcard pattern covers any other pattern as it will match anyway

#### `tests\npm_integration_tests.rs`: 1 occurrences

- Line 261: unnecessary `if let` since only the `Ok` variant of the iterator element is used

