package api

import (
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"github.com/quanttide/qtrecurit-provider/internal/store"
)

func newTestServer(t *testing.T) *httptest.Server {
	t.Helper()
	dir, err := os.MkdirTemp("", "api-test-*")
	if err != nil {
		t.Fatalf("create temp dir: %v", err)
	}
	st, err := store.New(store.Config{Driver: "file", Path: dir})
	if err != nil {
		t.Fatalf("create store: %v", err)
	}
	t.Cleanup(func() {
		st.Close()
		os.RemoveAll(dir)
	})

	standardHandler := NewStandardHandler(st)
	candidateHandler := NewCandidateHandler(st)

	mux := http.NewServeMux()
	mux.HandleFunc("GET /health", Health)

	mux.HandleFunc("GET /api/v1/standards/policies", standardHandler.ListPolicies)
	mux.HandleFunc("GET /api/v1/standards/policies/{id}", standardHandler.GetPolicy)
	mux.HandleFunc("GET /api/v1/standards/criteria", standardHandler.ListCriteria)
	mux.HandleFunc("GET /api/v1/standards/criteria/{id}", standardHandler.GetCriterion)
	mux.HandleFunc("GET /api/v1/standards/assessments", standardHandler.ListAssessments)
	mux.HandleFunc("GET /api/v1/standards/assessments/{id}", standardHandler.GetAssessment)

	mux.HandleFunc("GET /api/v1/candidates", candidateHandler.ListCandidates)
	mux.HandleFunc("POST /api/v1/candidates", candidateHandler.CreateCandidate)
	mux.HandleFunc("GET /api/v1/candidates/{id}", candidateHandler.GetCandidate)
	mux.HandleFunc("PUT /api/v1/candidates/{id}", candidateHandler.UpdateCandidate)
	mux.HandleFunc("DELETE /api/v1/candidates/{id}", candidateHandler.DeleteCandidate)

	srv := httptest.NewServer(mux)
	t.Cleanup(srv.Close)
	return srv
}

func doJSON(t *testing.T, method, url, body string) (*http.Response, map[string]any) {
	t.Helper()
	req, err := http.NewRequest(method, url, strings.NewReader(body))
	if err != nil {
		t.Fatalf("new request: %v", err)
	}
	if body != "" {
		req.Header.Set("Content-Type", "application/json")
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("do request: %v", err)
	}
	t.Cleanup(func() { resp.Body.Close() })

	var v map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&v); err != nil {
		// 空数组或空 body：返回 nil，调用方自行处理
		v = nil
	}
	return resp, v
}

func doJSONAuth(t *testing.T, method, url, token, body string) (*http.Response, map[string]any) {
	t.Helper()
	req, err := http.NewRequest(method, url, strings.NewReader(body))
	if err != nil {
		t.Fatalf("new request: %v", err)
	}
	if body != "" {
		req.Header.Set("Content-Type", "application/json")
	}
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatalf("do request: %v", err)
	}
	t.Cleanup(func() { resp.Body.Close() })

	var v map[string]any
	if err := json.NewDecoder(resp.Body).Decode(&v); err != nil {
		v = nil
	}
	return resp, v
}

func TestHealth(t *testing.T) {
	srv := newTestServer(t)
	resp, v := doJSON(t, "GET", srv.URL+"/health", "")
	if resp.StatusCode != http.StatusOK {
		t.Fatalf("status: got %d, want 200", resp.StatusCode)
	}
	if v["status"] != "ok" {
		t.Errorf("status field: got %v, want ok", v["status"])
	}
}

// --- 考评标准只读 API ---

func TestStandards_EmptyLists(t *testing.T) {
	srv := newTestServer(t)
	for _, path := range []string{
		"/api/v1/standards/policies",
		"/api/v1/standards/criteria",
		"/api/v1/standards/assessments",
	} {
		resp, err := http.Get(srv.URL + path)
		if err != nil {
			t.Fatalf("get %s: %v", path, err)
		}
		body, _ := io.ReadAll(resp.Body)
		resp.Body.Close()
		if resp.StatusCode != http.StatusOK {
			t.Errorf("%s: status %d, want 200", path, resp.StatusCode)
		}
		if strings.TrimSpace(string(body)) != "[]" {
			t.Errorf("%s: expected empty JSON array, got %s", path, body)
		}
	}
}

func TestStandards_NotFound(t *testing.T) {
	srv := newTestServer(t)
	resp, v := doJSON(t, "GET", srv.URL+"/api/v1/standards/policies/none", "")
	if resp.StatusCode != http.StatusNotFound {
		t.Errorf("status: got %d, want 404", resp.StatusCode)
	}
	if _, ok := v["error"]; !ok {
		t.Errorf("expected error body, got %v", v)
	}
}

func TestStandards_WriteNotAllowed(t *testing.T) {
	srv := newTestServer(t)
	resp, _ := doJSON(t, "POST", srv.URL+"/api/v1/standards/policies", `{"title":"x"}`)
	if resp.StatusCode != http.StatusMethodNotAllowed {
		t.Errorf("status: got %d, want 405", resp.StatusCode)
	}
}

// --- 候选人档案 API ---

func TestCandidate_CreateAndQuery(t *testing.T) {
	srv := newTestServer(t)

	// 内部写入：评估记录 + 信任记录（同构契约）
	resp, v := doJSON(t, "POST", srv.URL+"/api/v1/candidates", `{
		"name": "张三",
		"level": "实训生",
		"track": "T0-A",
		"records": [
			{"type": "问卷分析", "title": "责任心问卷", "description": "结构化问卷分析结果", "date": "2026-08-01", "source": "CLI 采集", "status": "done"},
			{"type": "历史行为", "title": "实训项目交付", "description": "按时交付任务", "date": "2026-08-05", "source": "考核沉淀", "status": "done"}
		]
	}`)
	if resp.StatusCode != http.StatusCreated {
		t.Fatalf("create status: got %d, want 201, body %v", resp.StatusCode, v)
	}
	id, _ := v["id"].(string)
	token, _ := v["query_token"].(string)
	if id == "" {
		t.Fatal("expected non-empty id")
	}
	if token == "" {
		t.Fatal("expected auto-issued query_token")
	}
	if v["created_at"] == nil || v["updated_at"] == nil {
		t.Error("expected created_at/updated_at timestamps")
	}

	// 候选人凭查询凭证查看自己的档案
	resp, v = doJSONAuth(t, "GET", srv.URL+"/api/v1/candidates/"+id, token, "")
	if resp.StatusCode != http.StatusOK {
		t.Fatalf("query status: got %d, want 200", resp.StatusCode)
	}
	if v["id"] != id || v["name"] != "张三" {
		t.Errorf("query result mismatch: %v", v)
	}

	// 错误凭证 → 401
	resp, _ = doJSONAuth(t, "GET", srv.URL+"/api/v1/candidates/"+id, "wrong-token", "")
	if resp.StatusCode != http.StatusUnauthorized {
		t.Errorf("wrong token status: got %d, want 401", resp.StatusCode)
	}

	// 无凭证 → 401
	resp, _ = doJSON(t, "GET", srv.URL+"/api/v1/candidates/"+id, "")
	if resp.StatusCode != http.StatusUnauthorized {
		t.Errorf("no token status: got %d, want 401", resp.StatusCode)
	}
}

func TestCandidate_Validation(t *testing.T) {
	srv := newTestServer(t)
	resp, v := doJSON(t, "POST", srv.URL+"/api/v1/candidates", `{"level":"实训生"}`)
	if resp.StatusCode != http.StatusBadRequest {
		t.Fatalf("status: got %d, want 400", resp.StatusCode)
	}
	detail := v["error"].(map[string]any)
	if detail["code"] != "VALIDATION_ERROR" {
		t.Errorf("code: got %v, want VALIDATION_ERROR", detail["code"])
	}
}

func TestCandidate_UpdateDelete(t *testing.T) {
	srv := newTestServer(t)

	resp, v := doJSON(t, "POST", srv.URL+"/api/v1/candidates", `{"name":"李四"}`)
	if resp.StatusCode != http.StatusCreated {
		t.Fatalf("create status: %d", resp.StatusCode)
	}
	id, _ := v["id"].(string)

	// 更新：追加信任记录（推荐信）
	resp, v = doJSON(t, "PUT", srv.URL+"/api/v1/candidates/"+id, `{
		"id": "`+id+`",
		"name": "李四",
		"query_token": "`+v["query_token"].(string)+`",
		"records": [{"type": "推荐信", "title": "创始人评价", "source": "背调报告", "status": "done"}]
	}`)
	if resp.StatusCode != http.StatusOK {
		t.Fatalf("update status: %d", resp.StatusCode)
	}
	records, _ := v["records"].([]any)
	if len(records) != 1 || records[0].(map[string]any)["type"] != "推荐信" {
		t.Errorf("records mismatch after update: %v", v["records"])
	}

	// 删除
	resp, _ = doJSON(t, "DELETE", srv.URL+"/api/v1/candidates/"+id, "")
	if resp.StatusCode != http.StatusNoContent {
		t.Errorf("delete status: %d, want 204", resp.StatusCode)
	}
	// 已删除：带凭证也查不到
	resp, _ = doJSONAuth(t, "GET", srv.URL+"/api/v1/candidates/"+id, v["query_token"].(string), "")
	if resp.StatusCode != http.StatusNotFound {
		t.Errorf("get after delete status: %d, want 404", resp.StatusCode)
	}
}

func TestCandidate_GetNotFound(t *testing.T) {
	srv := newTestServer(t)
	resp, _ := doJSON(t, "GET", srv.URL+"/api/v1/candidates/none", "")
	if resp.StatusCode != http.StatusNotFound {
		t.Errorf("status: got %d, want 404", resp.StatusCode)
	}
}
