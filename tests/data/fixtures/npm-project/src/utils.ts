// Tool Functions Module - contains various code quality issues

// Unused imports
import { readFileSync } from 'fs';

// type definition
interface User {
    id: number;
    name: string;
    email: string;
}

// Functions with missing return types
export function createUser(data: any) {
    return {
        id: Math.random(),
        ...data
    } as User;
}

// Using the any type
export function processUsers(users: any[]): any[] {
    return users.filter(u => u.active);
}

// Unused variables
const API_URL = "https://api.example.com";

// deeply nested function
export function complexFunction(input: number): number {
    if (input > 0) {
        if (input < 100) {
            if (input % 2 === 0) {
                return input * 2;
            }
        }
    }
    return input;
}

// Type-unsafe code
export const unsafeOperation = (x: any, y: any) => {
    return x + y;
};
