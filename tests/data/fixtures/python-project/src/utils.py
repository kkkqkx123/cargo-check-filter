# tool function module

from typing import Tuple, Union, Callable
import json


def parse_json(data: str) -> dict:
    """Parsing JSON - Insufficiently precise return types"""
    return json.loads(data)


def add(a: float, b: float) -> float:
    """Add two numbers."""
    return a + b


def subtract(a: float, b: float) -> float:
    """Subtract b from a."""
    return a - b


def multiply(a: float, b: float) -> float:
    """Multiply two numbers."""
    return a * b


def divide(a: float, b: float) -> float:
    """Divide a by b."""
    if b == 0:
        raise ValueError("Cannot divide by zero")
    return a / b


def format_name(first: str, last: str, middle: str = "") -> str:
    """Formatting names"""
    if middle:
        return f"{first} {middle} {last}"
    return f"{first} {last}"


def flexible_function(*args, **kwargs):
    """Functions missing type annotations altogether"""
    return args, kwargs


def union_type_example(value: Union[str, int]) -> str:
    """Using the Union type"""
    if isinstance(value, str):
        return value.upper()
    return str(value)


def tuple_unpacking() -> Tuple[int, str, float]:
    """Return tuple"""
    return 1, "test", 3.14


def callback_example(fn: Callable[[int], str], value: int) -> str:
    """Using the Callable type"""
    return fn(value)


# Type issues in class inheritance
class BaseHandler:
    def handle(self, data: str) -> str:
        return data.lower()


class DerivedHandler(BaseHandler):
    def handle(self, data) -> str:  # Parameters missing type annotations
        return data.upper()


# Generalized Usage
from typing import TypeVar, Generic

T = TypeVar('T')


class Container(Generic[T]):
    def __init__(self, value: T) -> None:
        self.value = value
    
    def get(self) -> T:
        return self.value
    
    def set(self, value) -> None:  # Parameters missing type
        self.value = value


# type inference problem
def inferred_issues():
    """Issues related to type inference"""
    numbers = [1, 2, 3]
    # The following code may be wrong at runtime, but mypy can catch it
    numbers.append("four")  # error: Argument 1 to "append" of "list" has incompatible type
    
    # Dictionary type issues
    mapping = {"a": 1, "b": 2}
    result = mapping.get("c") + 10  # error: Unsupported operand types
    
    return numbers
