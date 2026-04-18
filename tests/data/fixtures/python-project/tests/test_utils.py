"""Tests for utility functions."""

import pytest
from src.utils import multiply, subtract


def test_multiply():
    """Test multiplication."""
    assert multiply(3, 4) == 12
    assert multiply(-2, 5) == -10
    assert multiply(0, 100) == 0


def test_subtract():
    """Test subtraction."""
    assert subtract(10, 3) == 7
    assert subtract(5, 10) == -5
    assert subtract(0, 0) == 0


class TestStringOperations:
    """Test class grouping string-related tests."""

    def test_string_concat(self):
        """Test string concatenation."""
        assert "hello" + " " + "world" == "hello world"

    def test_string_upper(self):
        """Test string upper case."""
        assert "hello".upper() == "HELLO"
