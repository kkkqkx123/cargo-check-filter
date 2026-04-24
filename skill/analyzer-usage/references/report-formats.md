# Report Formats

The analyzer supports multiple output formats for different use cases.

## Supported Formats

| Format     | Extension | Use Case                          |
| ---------- | --------- | --------------------------------- |
| Markdown   | `.md`     | Human-readable reports (default)  |
| JSON       | `.json`   | Machine-readable, CI/CD pipelines |
| HTML       | `.html`   | Styled reports for sharing        |

## Specifying Output Format

The format is determined by the file extension:

```bash
# Markdown report (default)
analyzer cargo check
analyzer cargo check --output report.md

# JSON report
analyzer cargo check --output report.json

# HTML report
analyzer cargo check --output report.html
```

## Report Content

All formats include the following information:

### Summary Statistics

- Total number of issues
- Error count
- Warning count
- Issues by level breakdown

### File Analysis

- Top files with most issues
- Issues per file count

### Issue Details

Each issue includes:

- **Level**: Error, Warning, Info, or Hint
- **Code**: Error code (if available)
- **Message**: Human-readable description
- **Location**: File path, line number, column number
- **Context**: Surrounding code context (if available)

## Format Details

### Markdown

- Human-readable format with headers and tables
- Suitable for documentation and code review
- Default filename: `analysis_report.md`

### JSON

```json
{
  "total_issues": 5,
  "issues_by_level": {
    "error": 0,
    "warning": 5
  },
  "issues_by_file": {
    "src/main.rs": [
      {
        "level": "warning",
        "code": "unused_imports",
        "message": "unused import",
        "location": {
          "file_path": "src/main.rs",
          "line_number": 1,
          "column_number": 5
        }
      }
    ]
  }
}
```

### HTML

- Styled HTML page with CSS
- Collapsible sections for easy navigation
- Suitable for sharing reports via web browser

## CI/CD Integration

For CI/CD pipelines, JSON format is recommended:

```bash
# Generate JSON report
analyzer cargo check --output analysis.json

# Check for errors only
analyzer cargo check --filter-warnings --output errors.json
```

## Test Reports

When running test commands, the report includes additional information:

- Test summary (total, passed, failed, ignored)
- Failed test details
- Compilation issues (if any)
- Test execution time (if available)

```bash
# Generate test report
analyzer cargo test --output test_report.md
```
