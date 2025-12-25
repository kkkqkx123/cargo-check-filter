#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Cargo Check Error Analysis Tool
Automatically categorize, count and generate MD format report
"""

import subprocess
import re
import sys
import os
import argparse
from collections import defaultdict
from typing import List, Dict, Tuple, Set, Optional
from pathlib import Path


def parse_arguments():
    """Parse command line arguments"""
    parser = argparse.ArgumentParser(
        description='Cargo Check Error Analysis Tool',
        epilog='''
Examples:
  # Default: analyze all errors and warnings
  python analyze_cargo_errors.py
  
  # Filter out warnings, only show errors
  python analyze_cargo_errors.py --filter-warnings
  
  # Only show errors from specific paths
  python analyze_cargo_errors.py --filter-paths src/core
  python analyze_cargo_errors.py --filter-paths src/core src/query
  
  # Combine filters
  python analyze_cargo_errors.py --filter-warnings --filter-paths src/core
        '''
    )
    parser.add_argument('--filter-warnings', action='store_true',
                       help='Filter out all warnings, only show errors')
    parser.add_argument('--filter-paths', nargs='+',
                       help='Filter errors by file paths (absolute or relative paths)')
    return parser.parse_args()


def normalize_path(path: str, base_dir: str) -> str:
    """Normalize path to absolute path"""
    if os.path.isabs(path):
        return os.path.normpath(path)
    else:
        return os.path.normpath(os.path.join(base_dir, path))


def should_include_error(file_path: str, error_type: str, args, base_dir: str) -> bool:
    """Determine if an error should be included based on filters"""
    # Filter warnings if requested
    if args.filter_warnings and error_type == 'warning':
        return False
    
    # Filter by paths if requested
    if args.filter_paths:
        abs_file_path = normalize_path(file_path, base_dir)
        for filter_path in args.filter_paths:
            abs_filter_path = normalize_path(filter_path, base_dir)
            # Check if file path starts with filter path
            if abs_file_path.startswith(abs_filter_path):
                return True
        return False
    
    return True


def run_cargo_test() -> str:
    """Execute cargo test --lib command with short message format and return output"""
    try:
        # Use PowerShell compatible command on Windows
        # Run cargo test --lib with short message format to get parseable output
        result = subprocess.run(
            ['cargo', 'test', '--lib', '--message-format=short'],
            capture_output=True,
            text=True,
            encoding='utf-8'
        )
        return result.stdout + result.stderr
    except Exception as e:
        return f"Failed to execute cargo test: {e}"


def parse_error_line(line: str) -> Optional[Tuple[str, str, str, str, str]]:
    """
    Parse single error line
    Returns: (file_path, line_num, col_num, error_type, error_desc)
    """
    # Match format: file_path:line:column: type: description
    # Supports both warning and error[EXXXX] formats
    pattern = r'^([^:]+):(\d+):(\d+):\s*(\w+(?:\[E\d+\])?):\s*(.+)$'
    match = re.match(pattern, line.strip())

    if match:
        file_path = match.group(1)
        line_num = match.group(2)
        col_num = match.group(3)
        error_type = match.group(4)
        error_desc = match.group(5)
        return file_path, line_num, col_num, error_type, error_desc

    return None


def extract_error_pattern(desc: str) -> str:
    """
    Extract error pattern while preserving meaningful details
    Remove line numbers but keep method names, types, etc.
    """
    # Remove line number references but keep method/type names
    desc = re.sub(r':\d+', ':[line]', desc)
    
    # Remove help suggestions but keep the main error
    desc = re.sub(r': help: .+', '', desc)
    
    # Keep method names, type names, etc. but remove specific variable names
    # Only remove very generic variable names that don't provide useful info
    desc = re.sub(r'`\w+`', '[identifier]', desc)
    
    # But preserve method names in error messages like "no method named X"
    method_match = re.search(r"no method named `([^`]+)`", desc)
    if method_match:
        method_name = method_match.group(1)
        desc = desc.replace(f"`{method_name}`", f"`{method_name}`")
    
    # Preserve type names in error messages
    type_match = re.search(r"in the current scope.*?`([^`]+)`", desc)
    if type_match:
        type_name = type_match.group(1)
        desc = desc.replace(f"`{type_name}`", f"`{type_name}`")
    
    return desc.strip()


def categorize_errors(output: str) -> Dict[str, Dict[str, List[Tuple[str, str, str]]]]:
    """
    Categorize error information with multiple levels of grouping
    Returns: {error_pattern: {file_path: [(line_num, col_num, original_desc)]}}
    """
    categorized: Dict[str, Dict[str, List[Tuple[str, str, str]]]] = defaultdict(lambda: defaultdict(list))

    # Parse the multi-line cargo output format
    lines = output.split('\n')
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        if not line:
            i += 1
            continue

        # Check if this line starts with error/warning level
        if line.startswith(('error:', 'warning:')):
            # Extract error type and description
            parts = line.split(':', 1)
            if len(parts) >= 2:
                error_type = parts[0].strip()
                error_desc = parts[1].strip()

                # Look for the next line with the arrow format pointing to the file location
                if i + 1 < len(lines):
                    next_line = lines[i + 1].strip()
                    arrow_match = re.match(r'^\s*-->\s*([^:]+):(\d+):(\d+)\s*$', next_line)
                    if arrow_match:
                        file_path = arrow_match.group(1)
                        line_num = arrow_match.group(2)
                        col_num = arrow_match.group(3)

                        if file_path and error_type and error_desc:
                            # Extract error pattern while preserving meaningful details
                            error_pattern = extract_error_pattern(error_desc)

                            # Create category key with error type
                            category_key = f"{error_type}: {error_pattern}"

                            categorized[category_key][file_path].append((line_num, col_num, error_desc))

        # Also try the old format in case cargo outputs in that format
        else:
            parsed = parse_error_line(line)
            if parsed is not None:
                file_path, line_num, col_num, error_type, error_desc = parsed

                if file_path and error_type and error_desc:
                    # Extract error pattern while preserving meaningful details
                    error_pattern = extract_error_pattern(error_desc)

                    # Create category key with error type
                    category_key = f"{error_type}: {error_pattern}"

                    categorized[category_key][file_path].append((line_num, col_num, error_desc))

        i += 1

    return dict(categorized)


def generate_markdown_report(categorized_errors: Dict[str, Dict[str, List[Tuple[str, str, str]]]], args=None) -> str:
    """Generate MD format report"""
    if not categorized_errors:
        return "# Cargo Check Error Analysis Report\n\nNo errors found"
    
    report = ["# Cargo Check Error Analysis Report\n"]
    
    # Filter information
    if args:
        report.append("## Filter Settings\n")
        if args.filter_warnings:
            report.append("- **Warnings Filter**: Enabled (warnings are filtered out)")
        if args.filter_paths:
            report.append(f"- **Path Filter**: {', '.join(args.filter_paths)}")
        if not args.filter_warnings and not args.filter_paths:
            report.append("- **No filters applied**")
        report.append("")
    
    # Separate errors and warnings
    errors = {}
    warnings = {}
    
    for category_key, files in categorized_errors.items():
        error_type = category_key.split(':')[0]
        if error_type == 'warning':
            warnings[category_key] = files
        else:
            errors[category_key] = files
    
    # Calculate statistics
    total_errors = sum(sum(len(lines) for lines in files.values()) for files in errors.values())
    total_warnings = sum(sum(len(lines) for lines in files.values()) for files in warnings.values())
    
    error_type_stats: Dict[str, int] = defaultdict(int)
    warning_type_stats: Dict[str, int] = defaultdict(int)
    file_stats: Dict[str, int] = defaultdict(int)
    
    # Error statistics
    for category_key, files in errors.items():
        error_type = category_key.split(':')[0]
        category_count = sum(len(lines) for lines in files.values())
        error_type_stats[error_type] += category_count
        
        for file_path in files:
            file_stats[file_path] += len(files[file_path])
    
    # Warning statistics
    for category_key, files in warnings.items():
        warning_type = category_key.split(':')[0]
        category_count = sum(len(lines) for lines in files.values())
        warning_type_stats[warning_type] += category_count
        
        for file_path in files:
            file_stats[file_path] += len(files[file_path])
    
    # Overall statistics
    report.append("## Summary\n")
    report.append(f"- **Total Errors**: {total_errors}")
    report.append(f"- **Total Warnings**: {total_warnings}")
    report.append(f"- **Total Issues**: {total_errors + total_warnings}")
    report.append(f"- **Unique Error Patterns**: {len(errors)}")
    report.append(f"- **Unique Warning Patterns**: {len(warnings)}")
    report.append(f"- **Files with Issues**: {len(file_stats)}\n")
    
    # Error type statistics (only if there are errors)
    if errors:
        report.append("## Error Statistics\n")
        report.append(f"**Total Errors**: {total_errors}\n")
        
        if error_type_stats:
            report.append("### Error Type Breakdown\n")
            for error_type, count in sorted(error_type_stats.items(), key=lambda x: x[1], reverse=True):
                report.append(f"- **{error_type}**: {count} errors")
        
        # File statistics for errors
        error_file_stats = {file: count for file, count in file_stats.items() 
                           if any(file in files for files in errors.values())}
        if error_file_stats:
            report.append("\n### Files with Errors (Top 10)\n")
            for file_path, count in sorted(error_file_stats.items(), key=lambda x: x[1], reverse=True)[:10]:
                report.append(f"- `{file_path}`: {count} errors")
    
    # Warning type statistics (only if there are warnings)
    if warnings:
        report.append("\n## Warning Statistics\n")
        report.append(f"**Total Warnings**: {total_warnings}\n")
        
        if warning_type_stats:
            report.append("### Warning Type Breakdown\n")
            for warning_type, count in sorted(warning_type_stats.items(), key=lambda x: x[1], reverse=True):
                report.append(f"- **{warning_type}**: {count} warnings")
        
        # File statistics for warnings
        warning_file_stats = {file: count for file, count in file_stats.items() 
                             if any(file in files for files in warnings.values())}
        if warning_file_stats:
            report.append("\n### Files with Warnings (Top 10)\n")
            for file_path, count in sorted(warning_file_stats.items(), key=lambda x: x[1], reverse=True)[:10]:
                report.append(f"- `{file_path}`: {count} warnings")
    
    # Detailed error categorization
    if errors:
        report.append("\n## Detailed Error Categorization\n")
        
        # Sort errors by total occurrences
        sorted_errors = sorted(
            errors.items(),
            key=lambda x: sum(len(lines) for lines in x[1].values()),
            reverse=True
        )
        
        for category_key, files in sorted_errors:
            total_occurrences = sum(len(lines) for lines in files.values())
            unique_files = len(files)
            
            report.append(f"### {category_key}\n")
            report.append(f"**Total Occurrences**: {total_occurrences}  ") 
            report.append(f"**Unique Files**: {unique_files}\n")
            
            # Show files with this error pattern
            for file_path, lines in sorted(files.items(), key=lambda x: len(x[1]), reverse=True):
                file_count = len(lines)
                report.append(f"#### `{file_path}`: {file_count} occurrences\n")
                
                # Show first few examples
                max_examples = min(3, file_count)
                for i, (line_num, col_num, original_desc) in enumerate(lines[:max_examples]):
                    report.append(f"- Line {line_num}: {original_desc}")
                
                if file_count > max_examples:
                    report.append(f"- ... {file_count - max_examples} more occurrences in this file")
                
                report.append("")
            
            report.append("")
    
    # Detailed warning categorization
    if warnings:
        report.append("\n## Detailed Warning Categorization\n")
        
        # Sort warnings by total occurrences
        sorted_warnings = sorted(
            warnings.items(),
            key=lambda x: sum(len(lines) for lines in x[1].values()),
            reverse=True
        )
        
        for category_key, files in sorted_warnings:
            total_occurrences = sum(len(lines) for lines in files.values())
            unique_files = len(files)
            
            report.append(f"### {category_key}\n")
            report.append(f"**Total Occurrences**: {total_occurrences}  ") 
            report.append(f"**Unique Files**: {unique_files}\n")
            
            # Show files with this warning pattern
            for file_path, lines in sorted(files.items(), key=lambda x: len(x[1]), reverse=True):
                file_count = len(lines)
                report.append(f"#### `{file_path}`: {file_count} occurrences\n")
                
                # Show first few examples
                max_examples = min(3, file_count)
                for i, (line_num, col_num, original_desc) in enumerate(lines[:max_examples]):
                    report.append(f"- Line {line_num}: {original_desc}")
                
                if file_count > max_examples:
                    report.append(f"- ... {file_count - max_examples} more occurrences in this file")
                
                report.append("")
            
            report.append("")
    
    return '\n'.join(report)


def main():
    """Main function"""
    args = parse_arguments()
    base_dir = os.getcwd()

    print("Running cargo test --lib...")
    output = run_cargo_test()

    if not output:
        print("No output from cargo test")
        return 0

    print("Parsing errors...")
    categorized_errors = categorize_errors(output)
    
    # Apply filters
    filtered_errors = defaultdict(lambda: defaultdict(list))
    for category_key, files in categorized_errors.items():
        for file_path, lines in files.items():
            error_type = category_key.split(':')[0]
            if should_include_error(file_path, error_type, args, base_dir):
                filtered_errors[category_key][file_path].extend(lines)
    
    if not filtered_errors:
        print("No errors remain after applying filters")
        return 0
    
    print("Generating report...")
    report = generate_markdown_report(filtered_errors, args)
    
    with open('cargo_errors_report.md', 'w', encoding='utf-8') as f:
        f.write(report)
    
    print(f"Report generated: cargo_errors_report.md")
    
    # Console summary
    total_errors = 0
    for category_key, files in filtered_errors.items():
        total_errors += sum(len(lines) for lines in files.values())
    
    unique_patterns = len(filtered_errors)
    unique_files = len(set(
        file_path 
        for category in filtered_errors.values() 
        for file_path in category.keys()
    ))
    
    print(f"Total errors: {total_errors}")
    print(f"Unique error patterns: {unique_patterns}")
    print(f"Files with errors: {unique_files}")
    
    # Print filter summary
    if args.filter_warnings:
        print("Filter: Warnings are filtered out")
    if args.filter_paths:
        print(f"Filter: Only errors from paths: {', '.join(args.filter_paths)}")
    
    if filtered_errors:
        print("\nTop Error Patterns:")
        for category_key, files in sorted(
            filtered_errors.items(),
            key=lambda x: sum(len(lines) for lines in x[1].values()),
            reverse=True
        )[:5]:
            count = sum(len(lines) for lines in files.values())
            print(f"- {category_key}: {count} occurrences")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())