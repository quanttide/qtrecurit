package api

import (
	"encoding/json"
	"log/slog"
	"net/http"

	"github.com/quanttide/qtrecurit-provider/internal/model"
	"github.com/quanttide/qtrecurit-provider/internal/store"
)

// StandardHandler 提供考评标准只读 API（政策、筛选标准、考核说明）。
// 考评标准是透明公开的基础——无需登录即可读。
type StandardHandler struct {
	store store.Store
}

func NewStandardHandler(st store.Store) *StandardHandler {
	return &StandardHandler{store: st}
}

// --- Policies ---

func (h *StandardHandler) ListPolicies(w http.ResponseWriter, r *http.Request) {
	var policies []model.Policy
	h.list(w, "standards/policies", &policies)
}

func (h *StandardHandler) GetPolicy(w http.ResponseWriter, r *http.Request) {
	var policy model.Policy
	h.get(w, r, "standards/policies", &policy)
}

// --- Criteria ---

func (h *StandardHandler) ListCriteria(w http.ResponseWriter, r *http.Request) {
	var criteria []model.Criterion
	h.list(w, "standards/criteria", &criteria)
}

func (h *StandardHandler) GetCriterion(w http.ResponseWriter, r *http.Request) {
	var criterion model.Criterion
	h.get(w, r, "standards/criteria", &criterion)
}

// --- Assessments ---

func (h *StandardHandler) ListAssessments(w http.ResponseWriter, r *http.Request) {
	var assessments []model.Assessment
	h.list(w, "standards/assessments", &assessments)
}

func (h *StandardHandler) GetAssessment(w http.ResponseWriter, r *http.Request) {
	var assessment model.Assessment
	h.get(w, r, "standards/assessments", &assessment)
}

func (h *StandardHandler) list(w http.ResponseWriter, collection string, target any) {
	data, err := h.store.List(collection)
	if err != nil {
		slog.Error("list "+collection, "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to list "+collection, http.StatusInternalServerError)
		return
	}
	if err := json.Unmarshal(data, target); err != nil {
		slog.Error("parse "+collection, "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to parse "+collection, http.StatusInternalServerError)
		return
	}
	WriteJSON(w, target, http.StatusOK)
}

func (h *StandardHandler) get(w http.ResponseWriter, r *http.Request, collection string, target any) {
	id := r.PathValue("id")
	data, err := h.store.Get(collection, id)
	if err != nil {
		WriteError(w, "NOT_FOUND", "record not found", http.StatusNotFound)
		return
	}
	if err := json.Unmarshal(data, target); err != nil {
		slog.Error("parse "+collection, "error", err)
		WriteError(w, "INTERNAL_ERROR", "failed to parse "+collection, http.StatusInternalServerError)
		return
	}
	WriteJSON(w, target, http.StatusOK)
}
