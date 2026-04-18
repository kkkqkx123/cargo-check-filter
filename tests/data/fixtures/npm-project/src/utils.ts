// 工具函数模块 - 包含各种代码质量问题

// 未使用的导入
import { readFileSync } from 'fs';

// 类型定义
interface User {
    id: number;
    name: string;
    email: string;
}

// 缺少返回类型的函数
export function createUser(data: any) {
    return {
        id: Math.random(),
        ...data
    } as User;
}

// 使用any类型
export function processUsers(users: any[]): any[] {
    return users.filter(u => u.active);
}

// 未使用的变量
const API_URL = "https://api.example.com";

// 嵌套过深的函数
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

// 类型不安全的代码
export const unsafeOperation = (x: any, y: any) => {
    return x + y;
};
