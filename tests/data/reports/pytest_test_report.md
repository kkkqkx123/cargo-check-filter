# Pytest Test Report

**Command**: `pytest -v`

## Summary

- **Total Tests**: 10
- **Passed**: 7
- **Failed**: 1
- **Skipped/Ignored**: 2
- **Execution Time**: 0.15s

## Failed Tests

| Test Name | File | Line | Error |
|-----------|------|------|-------|
| test_divide_failure | tests/test_example.py | - |  |

## Passed Tests

| Test Name | File | Execution Time |
|-----------|------|----------------|
| test_add_positive_numbers | tests/test_example.py | 0.010s |
| test_add_negative_numbers | tests/test_example.py | 0.010s |
| test_add_zero | tests/test_example.py | 0.010s |
| test_divide_normal | tests/test_example.py | 0.010s |
| test_divide_by_zero | tests/test_example.py | 0.010s |
| test_multiply | tests/test_utils.py | 0.010s |
| test_subtract | tests/test_utils.py | 0.010s |

## Skipped/Ignored Tests

| Test Name | File | Reason |
|-----------|------|--------|
| test_future_feature | tests/test_example.py | not implemented yet |
| test_known_bug | tests/test_example.py | expected failure |

## Failure Details

### test_divide_failure

**File**: `tests/test_example.py`

**Error Details**:

```

    def test_divide_failure():
        """This test is designed to fail for testing purposes."""
        # Intentionally wrong assertion for testing
>       assert divide(10, 2) == 6  # Should be 5
E       AssertionError: assert 5.0 == 6
E        +  where 5.0 = divide(10, 2)

tests/test_example.py:35: AssertionError

============= 8 passed, 1 skipped, 1 xfailed, 1 failed in 0.15s =============
```

## Raw Output

View raw command output: [raw_output/pytest.txt](raw_output/pytest.txt)

---

*Report generated at: 2026-04-21 21:58:19*
