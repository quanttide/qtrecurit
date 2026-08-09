package api

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"log/slog"
	"net/http"
	"strings"
	"time"

	"github.com/quanttide/qtrecurit-provider/internal/model"
	"github.com/quanttide/qtrecurit-provider/internal/store"
)

// CandidateHandler 提供候选人档案 API。
//
// 内部写入：CLI 采集结果通过 API 写入（评估记录 + 信任记录同构契约）。
// 候选人查询：GET /api/v1/candidates/{id} 凭查询凭证（Authorization: Bearer）查看自己的档案。
type CandidateHandler struct {
	store store.Store
}

func NewCandidateHandler(st store.Store) *CandidateHandler {
	return &CandidateHandler{store: st}
}

func (h *CandidateHandler) ListCandidates(w http.ResponseWriter, r *http.Request) {
	data, err := h.store.List("candidates")
	if err != nil {
		slog.Error("list candidates", "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to list candidates", http.StatusInternalServerError)
		return
	}
	var candidates []model.Candidate
	if err := json.Unmarshal(data, &candidates); err != nil {
		slog.Error("parse candidates", "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to parse candidates", http.StatusInternalServerError)
		return
	}
	WriteJSON(w, candidates, http.StatusOK)
}

func (h *CandidateHandler) CreateCandidate(w http.ResponseWriter, r *http.Request) {
	var cand model.Candidate
	if err := json.NewDecoder(r.Body).Decode(&cand); err != nil {
		WriteError(w, "INVALID_INPUT", "invalid request body", http.StatusBadRequest)
		return
	}
	if cand.Name == "" {
		WriteError(w, "VALIDATION_ERROR", "name is required", http.StatusBadRequest)
		return
	}

	now := time.Now().UTC().Format(time.RFC3339)
	cand.CreatedAt = now
	cand.UpdatedAt = now
	// 查询凭证由 Provider 签发，客户端不可自定。
	cand.QueryToken = generateQueryToken()

	data, err := json.Marshal(cand)
	if err != nil {
		slog.Error("encode candidate", "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to encode data", http.StatusInternalServerError)
		return
	}

	id, err := h.store.Create("candidates", data)
	if err != nil {
		slog.Error("create candidate", "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to create candidate", http.StatusInternalServerError)
		return
	}

	cand.ID = id
	data, err = json.Marshal(cand)
	if err != nil {
		slog.Error("encode candidate with id", "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to encode data", http.StatusInternalServerError)
		return
	}
	if err := h.store.Update("candidates", id, data); err != nil {
		slog.Error("persist candidate id", "error", err)
	}

	WriteJSON(w, cand, http.StatusCreated)
}

func (h *CandidateHandler) GetCandidate(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")
	data, err := h.store.Get("candidates", id)
	if err != nil {
		WriteError(w, "NOT_FOUND", "candidate not found", http.StatusNotFound)
		return
	}
	var cand model.Candidate
	if err := json.Unmarshal(data, &cand); err != nil {
		slog.Error("parse candidate", "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to parse candidate", http.StatusInternalServerError)
		return
	}
	// 候选人凭查询凭证查看自己的档案；凭证无效一律 401。
	token := bearerToken(r)
	if token == "" || token != cand.QueryToken {
		WriteError(w, "UNAUTHORIZED", "invalid query token", http.StatusUnauthorized)
		return
	}
	WriteJSON(w, cand, http.StatusOK)
}

func (h *CandidateHandler) UpdateCandidate(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")
	var cand model.Candidate
	if err := json.NewDecoder(r.Body).Decode(&cand); err != nil {
		WriteError(w, "INVALID_INPUT", "invalid request body", http.StatusBadRequest)
		return
	}
	cand.ID = id
	cand.UpdatedAt = time.Now().UTC().Format(time.RFC3339)

	data, err := json.Marshal(cand)
	if err != nil {
		slog.Error("encode candidate", "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to encode data", http.StatusInternalServerError)
		return
	}
	if err := h.store.Update("candidates", id, data); err != nil {
		WriteError(w, "NOT_FOUND", "candidate not found", http.StatusNotFound)
		return
	}
	WriteJSON(w, cand, http.StatusOK)
}

func (h *CandidateHandler) DeleteCandidate(w http.ResponseWriter, r *http.Request) {
	id := r.PathValue("id")
	if err := h.store.Delete("candidates", id); err != nil {
		WriteError(w, "NOT_FOUND", "candidate not found", http.StatusNotFound)
		return
	}
	w.WriteHeader(http.StatusNoContent)
}

// generateQueryToken 生成查询凭证：32 字节随机十六进制串。
func generateQueryToken() string {
	b := make([]byte, 32)
	rand.Read(b)
	return hex.EncodeToString(b)
}

// bearerToken 从 Authorization: Bearer <token> 头中提取凭证。
func bearerToken(r *http.Request) string {
	h := r.Header.Get("Authorization")
	if !strings.HasPrefix(h, "Bearer ") {
		return ""
	}
	return strings.TrimSpace(strings.TrimPrefix(h, "Bearer "))
}
