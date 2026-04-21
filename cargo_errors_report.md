# Cargo Check Error Analysis Report

## Summary

- **Total Errors**: 4
- **Total Warnings**: 39
- **Total Issues**: 43
- **Unique Error Patterns**: 1
- **Unique Warning Patterns**: 33
- **Files with Issues**: 17

## Error Statistics

**Total Errors**: 4

### Error Type Breakdown

- **error[E0425]**: 4 errors

### Files with Errors (Top 10)

- `tests\cpp_integration_tests.rs`: 4 errors

## Warning Statistics

**Total Warnings**: 39

### Warning Type Breakdown

- **warning**: 39 warnings

### Files with Warnings (Top 10)

- `tests\common\mod.rs`: 11 warnings
- `tests\common\vs_env.rs`: 8 warnings
- `tests\go_integration_tests.rs`: 3 warnings
- `src\plugins\npm\parser.rs`: 3 warnings
- `src\plugins\java\maven\parser.rs`: 2 warnings
- `tests\mypy_integration_tests.rs`: 2 warnings
- `src\core\types.rs`: 1 warnings
- `src\core\reporter\json.rs`: 1 warnings
- `tests\pytest_integration_tests.rs`: 1 warnings
- `tests\parser_integration_tests.rs`: 1 warnings

## Detailed Error Categorization

### error[E0425]: cannot find function `save_raw_output` in this scope: not found in this scope

**Total Occurrences**: 4  
**Unique Files**: 1

#### `tests\cpp_integration_tests.rs`: 4 occurrences

- Line 48: cannot find function `save_raw_output` in this scope: not found in this scope
- Line 103: cannot find function `save_raw_output` in this scope: not found in this scope
- Line 158: cannot find function `save_raw_output` in this scope: not found in this scope
- ... 1 more occurrences in this file

## Detailed Warning Categorization

### warning: useless use of `format!`

**Total Occurrences**: 39  
**Unique Files**: 16

#### `tests\common\mod.rs`: 11 occurrences

- Line 8: unused import: `Path`
- Line 42: function `fixtures_dir` is never used
- Line 47: function `save_raw_output` is never used
- ... 8 more occurrences in this file

#### `tests\common\vs_env.rs`: 8 occurrences

- Line 8: constant `VS_DEV_SHELL_PATH` is never used
- Line 11: function `is_vs_dev_shell_available` is never used
- Line 17: function `run_with_vs_env` is never used
- ... 5 more occurrences in this file

#### `tests\go_integration_tests.rs`: 3 occurrences

- Line 88: this `if` statement can be collapsed
- Line 139: this `if` statement can be collapsed
- Line 140: this `if` statement can be collapsed

#### `src\plugins\npm\parser.rs`: 3 occurrences

- Line 665: the loop variable `i` is only used to index `lines`
- Line 669: compiling a regex in a loop
- Line 676: compiling a regex in a loop

#### `tests\mypy_integration_tests.rs`: 2 occurrences

- Line 5: unused import: `std::process::Command`
- Line 254: unnecessary `if let` since only the `Ok` variant of the iterator element is used

#### `src\plugins\java\maven\parser.rs`: 2 occurrences

- Line 30: stripping a prefix manually
- Line 77: length comparison to one

#### `src\core\reporter\json.rs`: 1 occurrences

- Line 71: useless use of `format!`

#### `src\plugins\go\parser.rs`: 1 occurrences

- Line 19: this `impl` can be derived

#### `src\core\types.rs`: 1 occurrences

- Line 125: use of `or_insert_with` to construct default value

#### `tests\cpp_parser_tests.rs`: 1 occurrences

- Line 5: unused import: `std::path::PathBuf`

#### `tests\parser_integration_tests.rs`: 1 occurrences

- Line 399: unnecessary `if let` since only the `Ok` variant of the iterator element is used

#### `src\plugins\go\analyzer.rs`: 1 occurrences

- Line 28: wildcard pattern covers any other pattern as it will match anyway

#### `src\core\parser.rs`: 1 occurrences

- Line 171: stripping a prefix manually

#### `tests\gradle_integration_tests.rs`: 1 occurrences

- Line 22: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do: help: change this to: `&Path`

#### `tests\npm_integration_tests.rs`: 1 occurrences

- Line 261: unnecessary `if let` since only the `Ok` variant of the iterator element is used

#### `tests\pytest_integration_tests.rs`: 1 occurrences

- Line 25: function `run_pytest` is never used

