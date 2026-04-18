package com.example;

// Error: class name doesn't match filename
public class Broken {
    public void brokenMethod() {
        // Error: undefined variable
        undefinedVar = 10;
        
        // Error: undefined method
        undefinedMethod();
    }
}
