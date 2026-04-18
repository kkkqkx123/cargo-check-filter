package main

import (
	"fmt"
	"os"

	"example.com/myproject/internal/config"
	"example.com/myproject/pkg/utils"
)

func main() {
	cfg := config.Load()
	
	// Intentional issue: unused variable
	unusedVar := "this is unused"
	_ = unusedVar
	
	result := utils.Add(5, 3)
	fmt.Printf("Result: %d\n", result)
	
	// Intentional issue: Printf format mismatch
	fmt.Printf("String value: %d\n", "hello")
	
	// Intentional issue: error return value not checked
	os.Setenv("TEST", "value")
	
	fmt.Printf("Config: %+v\n", cfg)
}
