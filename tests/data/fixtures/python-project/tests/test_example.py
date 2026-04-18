"""Example test file for pytest testing."""

import pytest
from src.utils import add, divide


def test_add_positive_numbers():
    """Test adding positive numbers."""
    assert add(2, 3) == 5
    assert add(10, 20) == 30


def test_add_negative_numbers():
    """Test adding negative numbers."""
    assert add(-2, -3) == -5
    assert add(-10, 5) == -5


def test_add_zero():
    """Test adding zero."""
    assert add(0, 5) == 5
    assert add(5, 0) == 5


def test_divide_normal():
    """Test normal division."""
    assert divide(10, 2) == 5
    assert divide(9, 3) == 3


def test_divide_by_zero():
    """Test division by zero raises exception."""
    with pytest.raises(ValueError, match="Cannot divide by zero"):
        divide(10, 0)


def test_divide_failure():
    """This test is designed to fail for testing purposes."""
    # Intentionally wrong assertion for testing
    assert divide(10, 2) == 6  # Should be 5


@pytest.mark.skip(reason="Feature not implemented yet")
def test_future_feature():
    """This test is skipped."""
    assert False  # Would fail if not skipped


@pytest.mark.xfail(reason="Known bug: issue #123")
def test_known_bug():
    """This test is expected to fail."""
    assert 1 == 2  # Known to fail
