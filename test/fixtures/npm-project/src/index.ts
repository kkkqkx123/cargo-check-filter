// 故意包含ESLint错误和类型错误的测试文件

const unusedVariable = "This variable is never used";

function greet(name: string): string {
    console.log("Greeting:", name);
    return `Hello, ${name}!`;
}

// 类型错误：参数类型不匹配
greet(123);

// 未使用的函数
function unusedFunction(): void {
    const x = 1;
}

// 隐式any类型
function processData(data) {
    return data.map(item => item.value);
}

// 缺少返回类型注解
export function calculate(a: number, b: number) {
    return a + b;
}

// 使用var（不推荐）
var oldStyle = true;

// 正确的代码
export function validFunction(userName: string): string {
    return `Welcome, ${userName}`;
}
