// Test files that intentionally contain ESLint errors and type errors

const unusedVariable = "This variable is never used";

function greet(name: string): string {
    console.log("Greeting:", name);
    return `Hello, ${name}!`;
}

// Type error: The parameter types do not match.
greet(123);

// Unused functions
function unusedFunction(): void {
    const x = 1;
}

// Implicit any type
function processData(data) {
    return data.map(item => item.value);
}

// Lack of return type annotations
export function calculate(a: number, b: number) {
    return a + b;
}

// Using `var` (not recommended)
var oldStyle = true;

// The correct code
export function validFunction(userName: string): string {
    return `Welcome, ${userName}`;
}
