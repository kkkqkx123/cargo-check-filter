# 主模块 - 包含各种类型错误供mypy检测

from typing import List, Optional, Dict, Any
import os


def add_numbers(a: int, b: int) -> int:
    """正确的函数实现"""
    return a + b


def add_with_error(a: int, b: str) -> int:
    """类型错误：返回类型与实际不符"""
    result = a + b  # error: Unsupported operand types
    return result


def process_data(data: List[int]) -> List[str]:
    """处理数据，但包含类型问题"""
    result = []
    for item in data:
        result.append(str(item))
    return result


def unsafe_function(value):
    """缺少类型注解"""
    return value * 2


def optional_param(name: str, age: Optional[int] = None) -> str:
    """使用Optional类型"""
    if age is None:
        return f"Name: {name}"
    return f"Name: {name}, Age: {age}"


def wrong_optional_usage(value: Optional[str]) -> int:
    """错误地使用Optional值"""
    return len(value)  # error: Item "None" of "Optional[str]" has no attribute "__len__"


class User:
    """用户类 - 包含类型问题"""
    
    def __init__(self, name: str, age: int):
        self.name = name
        self.age = age
        self.email = None  # 应该标注类型
    
    def get_info(self) -> str:
        return f"{self.name} ({self.age})"
    
    def update_email(self, email):
        """参数缺少类型注解"""
        self.email = email


class DataProcessor:
    """数据处理器 - 包含多种类型问题"""
    
    def __init__(self):
        self.data: List[Any] = []
    
    def add_item(self, item: int) -> None:
        self.data.append(item)
    
    def get_sum(self) -> int:
        """返回类型可能不匹配"""
        return sum(self.data)  # error: Argument 1 to "sum" has incompatible type
    
    def process_dict(self, data: Dict[str, int]) -> List[str]:
        result = []
        for key, value in data.items():
            result.append(f"{key}: {value}")
        return result


# 全局变量类型问题
GLOBAL_CONFIG = {}


def update_config(key: str, value: str) -> None:
    """更新全局配置"""
    global GLOBAL_CONFIG
    GLOBAL_CONFIG[key] = value


# 调用错误
def test_calls():
    """测试各种调用错误"""
    # 参数类型错误
    add_numbers("10", 20)  # error: Argument 1 to "add_numbers" has incompatible type
    
    # 返回值类型错误
    x: str = add_numbers(1, 2)  # error: Incompatible types in assignment
    
    # 未定义变量
    print(undefined_variable)  # error: Name "undefined_variable" is not defined
