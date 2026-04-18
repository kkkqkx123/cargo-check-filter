package config

import (
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLoad(t *testing.T) {
	// Set test environment variables
	os.Setenv("PORT", "3000")
	os.Setenv("DATABASE_URL", "postgres://localhost/test")
	os.Setenv("DEBUG", "true")

	cfg := Load()

	assert.Equal(t, "3000", cfg.Port)
	assert.Equal(t, "postgres://localhost/test", cfg.DatabaseURL)
	assert.True(t, cfg.Debug)
}

func TestLoad_DefaultPort(t *testing.T) {
	os.Unsetenv("PORT")
	os.Setenv("DATABASE_URL", "postgres://localhost/test")

	cfg := Load()

	assert.Equal(t, "8080", cfg.Port)
}

func TestValidate(t *testing.T) {
	tests := []struct {
		name    string
		config  *Config
		wantErr bool
	}{
		{
			name:    "valid config",
			config:  &Config{DatabaseURL: "postgres://localhost/test"},
			wantErr: false,
		},
		{
			name:    "missing database url",
			config:  &Config{DatabaseURL: ""},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.config.Validate()
			if tt.wantErr {
				assert.Error(t, err)
			} else {
				assert.NoError(t, err)
			}
		})
	}
}
