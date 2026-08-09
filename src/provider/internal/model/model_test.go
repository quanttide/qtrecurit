package model

import (
	"encoding/json"
	"testing"
)

// TestCandidateContract 验证候选人档案契约：评估字段与信任字段同构——
// 评估记录（考核分层、问卷分析）与信任记录（历史行为、背调、推荐信）
// 共用 Record 单元，均可被序列化为统一 JSON。
func TestCandidateContract(t *testing.T) {
	cand := Candidate{
		ID:         "c1",
		Name:       "张三",
		QueryToken: "token-abc",
		Level:      "实训生",
		Track:      "T0-A",
		Records: []Record{
			// 评估记录：筛选评估产出
			{Type: "问卷分析", Title: "责任心问卷", Description: "结构化问卷分析结果", Date: "2026-08-01", Source: "CLI 采集", Status: "done"},
			// 信任记录：评估沉淀为信任档案的记录
			{Type: "历史行为", Title: "实训项目交付", Description: "按时交付数据分析任务", Date: "2026-08-05", Source: "考核沉淀", Status: "done"},
			{Type: "背调", Title: "指导者背调", Description: "直接指导者评价", Date: "2026-08-08", Source: "背调报告", Status: "done"},
		},
		CreatedAt: "2026-08-09T00:00:00Z",
		UpdatedAt: "2026-08-09T00:00:00Z",
	}

	data, err := json.Marshal(cand)
	if err != nil {
		t.Fatalf("marshal candidate: %v", err)
	}

	var decoded Candidate
	if err := json.Unmarshal(data, &decoded); err != nil {
		t.Fatalf("unmarshal candidate: %v", err)
	}
	if decoded.Name != "张三" || decoded.Level != "实训生" || decoded.Track != "T0-A" {
		t.Errorf("scalar fields mismatch: %+v", decoded)
	}
	if len(decoded.Records) != 3 {
		t.Fatalf("got %d records, want 3", len(decoded.Records))
	}
	// 同构校验：评估与信任记录字段结构一致
	first := decoded.Records[0]
	trust := decoded.Records[1]
	if first.Type == "" || first.Title == "" || first.Source == "" || first.Status == "" {
		t.Errorf("evaluation record fields incomplete: %+v", first)
	}
	if trust.Type != "历史行为" || trust.Source != "考核沉淀" {
		t.Errorf("trust record mismatch: %+v", trust)
	}
}

// TestStandardContract 验证考评标准三类资源的契约字段。
func TestStandardContract(t *testing.T) {
	policy := Policy{ID: "p1", Title: "考核政策", Content: "……", Status: "active"}
	criterion := Criterion{ID: "c1", Title: "责任心标准", Description: "……", Dimension: "责任心", Status: "active"}
	assessment := Assessment{ID: "a1", Title: "考核分层说明", Description: "……", Kind: "考核分层", Content: "实训生/实习生/长期共建者/短期打手", Status: "active"}

	for name, v := range map[string]any{"policy": policy, "criterion": criterion, "assessment": assessment} {
		data, err := json.Marshal(v)
		if err != nil {
			t.Fatalf("marshal %s: %v", name, err)
		}
		if len(data) == 0 {
			t.Errorf("%s: empty json", name)
		}
	}
}
