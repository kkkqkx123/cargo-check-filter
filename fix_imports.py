#!/usr/bin/env python3
"""
Fix import paths in test files after directory restructuring.
"""

import re
from pathlib import Path

# Mapping of old import paths to new import paths
IMPORT_MAPPINGS = {
    'analyzer::plugins::mypy::': 'analyzer::plugins::python::mypy::',
    'analyzer::plugins::pytest::': 'analyzer::plugins::python::pytest::',
    'analyzer::plugins::maven::': 'analyzer::plugins::java::maven::',
    'analyzer::plugins::gradle::': 'analyzer::plugins::java::gradle::',
}

def fix_imports_in_file(file_path: Path) -> bool:
    """Fix imports in a single file. Returns True if changes were made."""
    content = file_path.read_text(encoding='utf-8')
    original_content = content
    
    for old_import, new_import in IMPORT_MAPPINGS.items():
        content = content.replace(old_import, new_import)
    
    if content != original_content:
        file_path.write_text(content, encoding='utf-8')
        print(f"✓ Fixed imports in: {file_path}")
        return True
    return False

def main():
    tests_dir = Path('d:/项目/cli/analyze-cargo/tests')
    
    # Find all .rs files in tests directory
    rust_files = list(tests_dir.glob('*.rs'))
    
    fixed_count = 0
    for file_path in rust_files:
        if fix_imports_in_file(file_path):
            fixed_count += 1
    
    print(f"\n✓ Fixed imports in {fixed_count} files")

if __name__ == '__main__':
    main()
