package config

import "os"

// Config holds application configuration
type Config struct {
	DatabaseURL string
	Port        string
	Debug       bool
}

// Load loads configuration from environment
func Load() *Config {
	// Intentional issue: error return value not checked
	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	return &Config{
		DatabaseURL: os.Getenv("DATABASE_URL"),
		Port:        port,
		Debug:       os.Getenv("DEBUG") == "true",
	}
}

// Validate validates the configuration
func (c *Config) Validate() error {
	if c.DatabaseURL == "" {
		return os.ErrInvalid
	}
	return nil
}
