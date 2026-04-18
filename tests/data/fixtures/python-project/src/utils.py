# 工具函数模块

from typing import Tuple, Union, Callable
import json


def parse_json(data: str) -> dict:
    """解析JSON - 返回类型不够精确"""
    return json.loads(data)


def divide(a: float, b: float) -> float:
    """除法函数 - 可能的除零错误"""
    return a / b


def format_name(first: str, last: str, middle: str = "") -> str:
    """格式化姓名"""
    if middle:
        return f"{first} {middle} {last}"
    return f"{first} {last}"


def flexible_function(*args, **kwargs):
    """完全缺少类型注解的函数"""
    return args, kwargs


def union_type_example(value: Union[str, int]) -> str:
    """使用Union类型"""
    if isinstance(value, str):
        return value.upper()
    return str(value)


def tuple_unpacking() -> Tuple[int, str, float]:
    """返回元组"""
    return 1, "test", 3.14


def callback_example(fn: Callable[[int], str], value: int) -> str:
    """使用Callable类型"""
    return fn(value)


# 类继承中的类型问题
class BaseHandler:
    def handle(self, data: str) -> str:
        return data.lower()


class DerivedHandler(BaseHandler):
    def handle(self, data) -> str:  # 参数缺少类型注解
        return data.upper()


# 泛型使用
from typing import TypeVar, Generic

T = TypeVar('T')


class Container(Generic[T]):
    def __init__(self, value: T) -> None:
        self.value = value
    
    def get(self) -> T:
        return self.value
    
    def set(self, value) -> None:  # 参数缺少类型
        self.value = value


# 类型推断问题
def inferred_issues():
    """类型推断相关的问题"""
    numbers = [1, 2, 3]
    # 下面的代码在运行时可能出错，但mypy可以捕获
    numbers.append("four")  # error: Argument 1 to "append" of "list" has incompatible type
    
    # 字典类型问题
    mapping = {"a": 1, "b": 2}
    result = mapping.get("c") + 10  # error: Unsupported operand types
    
    return numbers
