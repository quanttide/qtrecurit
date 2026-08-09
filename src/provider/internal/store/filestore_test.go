package store

import (
	"encoding/json"
	"os"
	"testing"
)

func setupTestStore(t *testing.T) (Store, func()) {
	t.Helper()
	dir, err := os.MkdirTemp("", "filestore-test-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	s, err := New(Config{Driver: "file", Path: dir})
	if err != nil {
		os.RemoveAll(dir)
		t.Fatalf("failed to create store: %v", err)
	}
	return s, func() {
		s.Close()
		os.RemoveAll(dir)
	}
}

func TestCreateAndGet(t *testing.T) {
	s, cleanup := setupTestStore(t)
	defer cleanup()

	data := json.RawMessage(`{"name":"test-entity"}`)
	id, err := s.Create("test_collection", data)
	if err != nil {
		t.Fatalf("Create failed: %v", err)
	}
	if id == "" {
		t.Fatal("expected non-empty id")
	}

	got, err := s.Get("test_collection", id)
	if err != nil {
		t.Fatalf("Get failed: %v", err)
	}
	var want, gotV map[string]string
	json.Unmarshal(data, &want)
	json.Unmarshal(got, &gotV)
	if gotV["name"] != want["name"] {
		t.Errorf("got %s, want %s", got, data)
	}
}

func TestList(t *testing.T) {
	s, cleanup := setupTestStore(t)
	defer cleanup()

	s.Create("test_collection", json.RawMessage(`{"name":"a"}`))
	s.Create("test_collection", json.RawMessage(`{"name":"b"}`))

	got, err := s.List("test_collection")
	if err != nil {
		t.Fatalf("List failed: %v", err)
	}
	var items []map[string]string
	if err := json.Unmarshal(got, &items); err != nil {
		t.Fatalf("parse list: %v", err)
	}
	if len(items) != 2 {
		t.Errorf("got %d items, want 2", len(items))
	}
}

func TestUpdate(t *testing.T) {
	s, cleanup := setupTestStore(t)
	defer cleanup()

	id, _ := s.Create("test_collection", json.RawMessage(`{"name":"old"}`))
	if err := s.Update("test_collection", id, json.RawMessage(`{"name":"new"}`)); err != nil {
		t.Fatalf("Update failed: %v", err)
	}

	got, _ := s.Get("test_collection", id)
	var item map[string]string
	json.Unmarshal(got, &item)
	if item["name"] != "new" {
		t.Errorf("got name %q, want %q", item["name"], "new")
	}

	if err := s.Update("test_collection", "missing", json.RawMessage(`{}`)); err == nil {
		t.Error("expected error updating missing record")
	}
}

func TestDelete(t *testing.T) {
	s, cleanup := setupTestStore(t)
	defer cleanup()

	id, _ := s.Create("test_collection", json.RawMessage(`{"name":"a"}`))
	if err := s.Delete("test_collection", id); err != nil {
		t.Fatalf("Delete failed: %v", err)
	}
	if _, err := s.Get("test_collection", id); err == nil {
		t.Error("expected error getting deleted record")
	}
	if err := s.Delete("test_collection", "missing"); err == nil {
		t.Error("expected error deleting missing record")
	}
}

func TestGetMissing(t *testing.T) {
	s, cleanup := setupTestStore(t)
	defer cleanup()

	if _, err := s.Get("test_collection", "missing"); err == nil {
		t.Error("expected error for missing record")
	}
}

func TestUnknownDriver(t *testing.T) {
	if _, err := New(Config{Driver: "postgres"}); err == nil {
		t.Error("expected error for unknown driver")
	}
}

func TestNestedCollection(t *testing.T) {
	s, cleanup := setupTestStore(t)
	defer cleanup()

	data := json.RawMessage(`{"title":"p1"}`)
	id, err := s.Create("standards/policies", data)
	if err != nil {
		t.Fatalf("Create nested failed: %v", err)
	}
	got, err := s.Get("standards/policies", id)
	if err != nil {
		t.Fatalf("Get nested failed: %v", err)
	}
	var want, gotV map[string]string
	json.Unmarshal(data, &want)
	json.Unmarshal(got, &gotV)
	if gotV["title"] != want["title"] {
		t.Errorf("got %s, want %s", got, data)
	}
}
