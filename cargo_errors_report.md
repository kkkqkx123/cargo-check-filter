# Cargo Check Error Analysis Report

## Summary

- **Total Errors**: 0
- **Total Warnings**: 3
- **Total Issues**: 3
- **Unique Error Patterns**: 0
- **Unique Warning Patterns**: 3
- **Files with Issues**: 2

## Error Statistics

**Total Errors**: 0

## Warning Statistics

**Total Warnings**: 3

### Warning Type Breakdown

- **warning**: 3 warnings

### Files with Warnings (Top 10)

- `src\core\config.rs`: 2 warnings
- `src\plugins\mod.rs`: 1 warnings

## Detailed Warning Categorization

### warning: function `create_registry` is never used

**Total Occurrences**: 3  
**Unique Files**: 2

#### `src\core\config.rs`: 2 occurrences

- Line 167: this `if` statement can be collapsed
- Line 145: methods `is_command_enabled` and `get_available_commands` are never used

#### `src\plugins\mod.rs`: 1 occurrences

- Line 14: function `create_registry` is never used

