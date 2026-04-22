// This file has intentional ESLint errors for testing

// Unused variable - should trigger @typescript-eslint/no-unused-vars
const unusedVariable = "test";

// Function with unused parameter
function greet(name: string, unusedParam: string): string {
  return `Hello ${name}`;
}

// Using any type - should trigger @typescript-eslint/no-explicit-any
function processData(data: any): any {
  return data;
}

// Unused function
function unusedFunction(): void {
  console.log("never called");
}

export { greet, processData };
