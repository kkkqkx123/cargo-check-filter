# Configuration Guide

## Overview

The analyzer tool uses a multi-level configuration system that allows you to customize command behavior, define custom commands, and configure tech stack-specific settings.

## Configuration Architecture

The configuration system follows a three-tier architecture with increasing priority:

```
┌─────────────────────────────────────────────────────────────┐
│  Level 1: Embedded Defaults (Lowest Priority)               │
│  - Hardcoded in the binary                                  │
│  - Provides default commands for all tech stacks            │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  Level 2: Binary Directory Config                           │
│  - Location: <binary_dir>/analyzer.toml                     │
│  - Applies to all projects using this binary                │
│  - Useful for distribution with custom defaults             │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│  Level 3: Project Config (Highest Priority)                 │
│  - Location: ./.analyzer.toml (preferred)                   │
│  - Or: ./analyzer.toml                                      │
│  - Project-specific settings                                │
│  - Should be committed to version control                   │
└─────────────────────────────────────────────────────────────┘
```

**Important Design Decisions**:
- User directory configuration (e.g., `~/.analyzer.toml`) is intentionally NOT supported to avoid configuration fragmentation and "works on my machine" problems
- There is NO distinction between "global config" and "project config" - all configuration files are project-level
- The `[global]` section in the config file refers to **global settings** (like default format, filter warnings), NOT a global configuration file

## Configuration File Format

Configuration files use TOML format with the following structure:

```toml
# Configuration version
version = "1.0"

# Global settings
[global]
default_format = "markdown"      # Output format: markdown, json, html
filter_warnings = false          # Filter warnings by default
default_output = "report.md"     # Default output file path

# Command definitions
[commands.<command-name>]
exec = "<actual-command>"        # Command to execute
description = "<desc>"           # Command description
tech_stacks = ["<stack>"]        # Applicable tech stacks
enabled = true                   # Enable/disable command

# Tech stack specific configuration
[tech_stack.<stack-name>]
test_framework = "<framework>"   # Test framework type

[tech_stack.<stack-name>.commands.<command-name>]
# Override commands for specific tech stack

[tech_stack.<stack-name>.scripts]
# Script name mappings
"<script-name>" = "<actual-script>"
```

## Configuration Sections

### Global Configuration

Controls default behavior for the analyzer:

```toml
[global]
default_format = "markdown"      # Default report format
filter_warnings = false          # Filter warnings by default
default_output = "report.md"     # Default output path
```

**Options:**
- `default_format`: Report format (`markdown`, `json`, `html`)
- `filter_warnings`: Whether to filter warnings by default
- `default_output`: Default output file path (optional)

### Command Configuration

Define or override commands:

```toml
# Override built-in command
[commands.check]
exec = "cargo check --all-targets"
description = "Check all targets"
tech_stacks = ["cargo"]
enabled = true

# Define custom command
[commands.lint-fix]
exec = "npm run lint -- --fix"
description = "Run linter with auto-fix"
tech_stacks = ["npm", "pnpm", "yarn"]
enabled = true
```

**Properties:**
- `exec`: The actual command to execute (required)
- `description`: Human-readable description (optional)
- `tech_stacks`: List of applicable tech stacks (default: empty)
- `enabled`: Whether command is active (default: true)

### Tech Stack Configuration

Configure tech stack-specific settings:

```toml
[tech_stack.npm]
test_framework = "jest"

[tech_stack.npm.commands.lint]
exec = "npm run lint:strict"
description = "Run strict linting"

[tech_stack.npm.scripts]
"test" = "jest --coverage"
"lint" = "eslint . --ext .ts,.tsx"
"build" = "tsc --noEmit"
```

**Properties:**
- `commands`: Override commands for this tech stack
- `scripts`: Script name mappings for output parsing
- `test_framework`: Test framework type (`jest`, `vitest`, `mocha`, etc.)

## Built-in Commands

### Cargo (Rust)

| Command | Execution | Description |
|---------|-----------|-------------|
| `check` | `cargo check` | Fast syntax and type checking |
| `clippy` | `cargo clippy` | Run Clippy linter |
| `clippy-all` | `cargo clippy --all-targets --all-features` | Full Clippy check |
| `check-test` | `cargo check --tests` | Check test code |
| `test` | `cargo test` | Run tests |

### NPM/Node.js

| Command | Execution | Tech Stacks |
|---------|-----------|-------------|
| `lint` | `npm run lint` | npm, pnpm, yarn |
| `type-check` | `npm run type-check` | npm, pnpm, yarn |
| `audit` | `npm audit` | npm, pnpm, yarn |

### Mypy (Python)

| Command | Execution | Description |
|---------|-----------|-------------|
| `mypy` | `mypy` | Run mypy type checker |
| `mypy-strict` | `mypy --strict` | Strict mode type checking |

## Command Resolution

When requesting a command, the system checks in this order:

1. **Tech stack specific command**: `tech_stack.<stack>.commands.<name>`
2. **Global command**: `commands.<name>`
3. **Built-in default**: Embedded in the binary

Example resolution for `analyzer cargo check`:

```
1. Check: tech_stack.cargo.commands.check
2. Check: commands.check
3. Use: Built-in "cargo check"
```

## Configuration Merging

When multiple configuration levels exist, they are merged with these rules:

### Global Settings
- Higher priority values override lower priority
- `default_format`: Non-default values override
- `filter_warnings`: Direct override
- `default_output`: `Some` values override

### Commands
- **Complete override**: Same command name is replaced
- **Additive**: New commands are added

### Tech Stacks
- **Incremental merge**: Commands and scripts are added
- **Override**: `test_framework` is replaced

## Configuration File Locations

### Project Configuration

The analyzer looks for project configuration in this order:

1. `./.analyzer.toml` (hidden file, preferred)
2. `./analyzer.toml` (regular file)

**Recommendation**: Use `.analyzer.toml` to keep configuration hidden and avoid cluttering the project root.

### Binary Directory Configuration

Location: Same directory as the `analyzer` executable

```
/usr/local/bin/analyzer          # Binary
/usr/local/bin/analyzer.toml     # Config
```

This is useful for:
- Distribution with custom defaults
- Organization-wide settings
- Pre-configured installations

## Error Handling

Configuration errors are handled gracefully:

- **File not found**: Uses default configuration
- **Parse error**: Falls back to default configuration
- **Invalid values**: Uses default values for that field

The analyzer will never fail to run due to configuration issues.

## Best Practices

### 1. Version Control

Always commit project configuration:

```bash
git add .analyzer.toml
git commit -m "Add analyzer configuration"
```

### 2. Minimal Configuration

Only override what you need:

```toml
# Good: Only override necessary commands
[commands.type-check]
exec = "npm run typecheck"
tech_stacks = ["npm"]
```

### 3. Team Consistency

Use project configuration to ensure consistent behavior across the team:

```toml
[global]
filter_warnings = true
default_format = "json"

[commands.test]
exec = "cargo test --all-features"
tech_stacks = ["cargo"]
```

### 4. Tech Stack Specificity

Use tech stack configuration for project-specific overrides:

```toml
[tech_stack.cargo.commands.check]
exec = "cargo check --all-targets --all-features"
description = "Check all targets and features"
```

## Examples

See the `assets/` directory for example configuration files:

- `minimal-config.toml` - Minimal configuration example
- `full-config.toml` - Complete configuration example
- `custom-commands.toml` - Custom command definitions
- `tech-stack-config.toml` - Tech stack specific configuration

## Troubleshooting

### Configuration Not Loading

1. Check file location (`.analyzer.toml` or `analyzer.toml`)
2. Verify TOML syntax
3. Check file permissions

### Command Not Found

1. Verify command is defined in configuration
2. Check `tech_stacks` list includes your stack
3. Ensure `enabled = true`

### Unexpected Behavior

1. Check configuration merge order
2. Verify no conflicting definitions
3. Use minimal configuration to isolate issues

---

*For detailed implementation analysis, see: `docs/config-loading-logic.md`*
