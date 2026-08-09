package config

import (
	"os"
	"path/filepath"
	"testing"
)

func TestMain(m *testing.M) {
	os.Unsetenv("QTRECURIT_ADDR")
	os.Unsetenv("QTRECURIT_STORE_DRIVER")
	os.Unsetenv("QTRECURIT_STORE_PATH")
	os.Unsetenv("QTRECURIT_LOG_LEVEL")
	os.Unsetenv("QTRECURIT_LOG_FORMAT")
	os.Exit(m.Run())
}

func TestLoad_EmptyPath(t *testing.T) {
	cfg, err := Load("")
	if err != nil {
		t.Fatalf("Load empty path: %v", err)
	}
	if cfg.Server.Addr != ":8000" {
		t.Errorf("addr: got %q, want %q", cfg.Server.Addr, ":8000")
	}
	if cfg.Store.Driver != "file" {
		t.Errorf("store driver: got %q, want %q", cfg.Store.Driver, "file")
	}
	if cfg.Store.Path != "data" {
		t.Errorf("store path: got %q, want %q", cfg.Store.Path, "data")
	}
	if cfg.Log.Level != "info" {
		t.Errorf("log level: got %q, want %q", cfg.Log.Level, "info")
	}
	if cfg.Log.Format != "text" {
		t.Errorf("log format: got %q, want %q", cfg.Log.Format, "text")
	}
}

func TestLoad_CustomPath(t *testing.T) {
	dir := t.TempDir()
	configContent := `{"server":{"addr":":9000"},"store":{"driver":"file","path":"/tmp/store"},"log":{"level":"debug","format":"json"}}`
	configPath := filepath.Join(dir, "config.json")
	if err := os.WriteFile(configPath, []byte(configContent), 0644); err != nil {
		t.Fatalf("write config: %v", err)
	}

	cfg, err := Load(configPath)
	if err != nil {
		t.Fatalf("Load custom path: %v", err)
	}
	if cfg.Server.Addr != ":9000" {
		t.Errorf("addr: got %q, want %q", cfg.Server.Addr, ":9000")
	}
	if cfg.Store.Path != "/tmp/store" {
		t.Errorf("store path: got %q, want %q", cfg.Store.Path, "/tmp/store")
	}
	if cfg.Log.Level != "debug" {
		t.Errorf("log level: got %q, want %q", cfg.Log.Level, "debug")
	}
	if cfg.Log.Format != "json" {
		t.Errorf("log format: got %q, want %q", cfg.Log.Format, "json")
	}
}

func TestLoad_InvalidPath(t *testing.T) {
	_, err := Load("/nonexistent/config.json")
	if err == nil {
		t.Fatal("expected error for nonexistent path, got nil")
	}
}

func TestLoad_InvalidJSON(t *testing.T) {
	dir := t.TempDir()
	configPath := filepath.Join(dir, "bad.json")
	if err := os.WriteFile(configPath, []byte("{invalid"), 0644); err != nil {
		t.Fatalf("write config: %v", err)
	}
	_, err := Load(configPath)
	if err == nil {
		t.Fatal("expected error for invalid JSON, got nil")
	}
}

func TestLoad_EnvOverrides(t *testing.T) {
	os.Setenv("QTRECURIT_ADDR", ":7000")
	os.Setenv("QTRECURIT_STORE_DRIVER", "file")
	os.Setenv("QTRECURIT_STORE_PATH", "/data/db")
	os.Setenv("QTRECURIT_LOG_LEVEL", "warn")
	os.Setenv("QTRECURIT_LOG_FORMAT", "json")
	defer func() {
		os.Unsetenv("QTRECURIT_ADDR")
		os.Unsetenv("QTRECURIT_STORE_DRIVER")
		os.Unsetenv("QTRECURIT_STORE_PATH")
		os.Unsetenv("QTRECURIT_LOG_LEVEL")
		os.Unsetenv("QTRECURIT_LOG_FORMAT")
	}()

	cfg, err := Load("")
	if err != nil {
		t.Fatalf("Load with env overrides: %v", err)
	}
	if cfg.Server.Addr != ":7000" {
		t.Errorf("addr: got %q, want %q", cfg.Server.Addr, ":7000")
	}
	if cfg.Store.Path != "/data/db" {
		t.Errorf("store path: got %q, want %q", cfg.Store.Path, "/data/db")
	}
	if cfg.Log.Level != "warn" {
		t.Errorf("log level: got %q, want %q", cfg.Log.Level, "warn")
	}
	if cfg.Log.Format != "json" {
		t.Errorf("log format: got %q, want %q", cfg.Log.Format, "json")
	}
}
