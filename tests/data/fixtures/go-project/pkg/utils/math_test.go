package utils

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAdd(t *testing.T) {
	tests := []struct {
		name     string
		a, b     int
		expected int
	}{
		{"positive numbers", 5, 3, 8},
		{"with zero", 5, 0, 5},
		{"negative numbers", -5, -3, -8},
		{"mixed signs", -5, 3, -2},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := Add(tt.a, tt.b)
			assert.Equal(t, tt.expected, result)
		})
	}
}

func TestDivide(t *testing.T) {
	tests := []struct {
		name        string
		a, b        int
		expected    int
		expectError bool
	}{
		{"normal division", 10, 2, 5, false},
		{"division by one", 10, 1, 10, false},
		{"division by zero", 10, 0, 0, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := Divide(tt.a, tt.b)
			if tt.expectError {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
				assert.Equal(t, tt.expected, result)
			}
		})
	}
}

func TestProcessData(t *testing.T) {
	result := ProcessData("test data")
	assert.Equal(t, "test data processed", result)
}

// Intentionally skipped test
func TestIntegration(t *testing.T) {
	t.Skip("Skipping integration test - requires external service")
}
