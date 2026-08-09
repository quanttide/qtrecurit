package api

import (
	"encoding/json"
	"log/slog"
	"net/http"

	"github.com/quanttide/qtrecurit-provider/internal/model"
	"github.com/quanttide/qtrecurit-provider/internal/store"
)

// 考评准则只读 API：政策、筛选标准、考核说明——无需登录即可读，
// 是「考评方式透明公开」的基础。三个资源平级，各自独立。

// PolicyHandler 提供政策只读 API。
type PolicyHandler struct {
	store store.Store
}

func NewPolicyHandler(st store.Store) *PolicyHandler {
	return &PolicyHandler{store: st}
}

func (h *PolicyHandler) ListPolicies(w http.ResponseWriter, r *http.Request) {
	var policies []model.Policy
	list(w, h.store, "policies", &policies)
}

func (h *PolicyHandler) GetPolicy(w http.ResponseWriter, r *http.Request) {
	var policy model.Policy
	get(w, r, h.store, "policies", &policy)
}

// CriterionHandler 提供筛选标准只读 API。
type CriterionHandler struct {
	store store.Store
}

func NewCriterionHandler(st store.Store) *CriterionHandler {
	return &CriterionHandler{store: st}
}

func (h *CriterionHandler) ListCriteria(w http.ResponseWriter, r *http.Request) {
	var criteria []model.Criterion
	list(w, h.store, "criteria", &criteria)
}

func (h *CriterionHandler) GetCriterion(w http.ResponseWriter, r *http.Request) {
	var criterion model.Criterion
	get(w, r, h.store, "criteria", &criterion)
}

// AssessmentHandler 提供考核说明只读 API。
type AssessmentHandler struct {
	store store.Store
}

func NewAssessmentHandler(st store.Store) *AssessmentHandler {
	return &AssessmentHandler{store: st}
}

func (h *AssessmentHandler) ListAssessments(w http.ResponseWriter, r *http.Request) {
	var assessments []model.Assessment
	list(w, h.store, "assessments", &assessments)
}

func (h *AssessmentHandler) GetAssessment(w http.ResponseWriter, r *http.Request) {
	var assessment model.Assessment
	get(w, r, h.store, "assessments", &assessment)
}

func list(w http.ResponseWriter, st store.Store, collection string, target any) {
	data, err := st.List(collection)
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

func get(w http.ResponseWriter, r *http.Request, st store.Store, collection string, target any) {
	id := r.PathValue("id")
	data, err := st.Get(collection, id)
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
