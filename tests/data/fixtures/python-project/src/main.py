# Main module - contains various types of errors for mypy to detect

from typing import List, Optional, Dict, Any
import os


def add_numbers(a: int, b: int) -> int:
    """Correct function implementation"""
    return a + b


def add_with_error(a: int, b: str) -> int:
    """Type error: the return type does not match the actual"""
    result = a + b  # error: Unsupported operand types
    return result


def process_data(data: List[int]) -> List[str]:
    """Handles data but contains type issues"""
    result = []
    for item in data:
        result.append(str(item))
    return result


def unsafe_function(value):
    """Missing type annotations"""
    return value * 2


def optional_param(name: str, age: Optional[int] = None) -> str:
    """Using the Optional type"""
    if age is None:
        return f"Name: {name}"
    return f"Name: {name}, Age: {age}"


def wrong_optional_usage(value: Optional[str]) -> int:
    """Incorrect use of Optional values"""
    return len(value)  # error: Item "None" of "Optional[str]" has no attribute "__len__"


class User:
    """User Classes - Contains Type Issues"""
    
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
        self.email = None  # Type should be labeled
    
    def get_info(self) -> str:
        return f"{self.name} ({self.age})"
    
    def update_email(self, email):
        """Parameters missing type annotations"""
        self.email = email


class DataProcessor:
    """Data Processor - Contains multiple types of problems"""
    
    def __init__(self):
        self.data: List[Any] = []
    
    def add_item(self, item: int) -> None:
        self.data.append(item)
    
    def get_sum(self) -> int:
        """Return types may not match"""
        return sum(self.data)  # error: Argument 1 to "sum" has incompatible type
    
    def process_dict(self, data: Dict[str, int]) -> List[str]:
        result = []
        for key, value in data.items():
            result.append(f"{key}: {value}")
        return result


# Global variable type issues
GLOBAL_CONFIG = {}


def update_config(key: str, value: str) -> None:
    """Updating the global configuration"""
    global GLOBAL_CONFIG
    GLOBAL_CONFIG[key] = value


# invocation error
def test_calls():
    """Testing for various call errors"""
    # Wrong parameter type
    add_numbers("10", 20)  # error: Argument 1 to "add_numbers" has incompatible type
    
    # Return value type error
    x: str = add_numbers(1, 2)  # error: Incompatible types in assignment
    
    # undefined variable
    print(undefined_variable)  # error: Name "undefined_variable" is not defined
