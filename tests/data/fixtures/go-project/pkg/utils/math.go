package utils

import (
	"fmt"
	"os"
)

// Add returns the sum of two integers
func Add(a, b int) int {
	return a + b
}

// Divide divides two integers
func Divide(a, b int) (int, error) {
	if b == 0 {
		return 0, fmt.Errorf("cannot divide by zero")
	}
	return a / b, nil
}

// ProcessData processes data with potential issues
func ProcessData(data string) string {
	// Intentional issue: error return value not checked
	file, _ := os.Open("/tmp/test.txt")
	defer file.Close()
	
	return data + " processed"
}
