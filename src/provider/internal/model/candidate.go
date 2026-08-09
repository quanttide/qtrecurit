package model

// Candidate 是候选人档案（v0.1 共享契约）。
//
// 契约先行：本结构与 Studio 数据模型同源。评估字段与信任字段同构——
// 筛选评估产出的每条记录，将来都是信任档案里的一条记录，
// 故评估与信任共用 Record 单元，以 Type 区分，从第一天避免事后集成。
type Candidate struct {
	ID string `json:"id"`

	// Name 是候选人姓名，内部写入时必填。
	Name string `json:"name"`

	// QueryToken 是候选人查询凭证，创建时自动签发。
	// 候选人凭此凭证查看自己的档案（GET /api/v1/queries/{token}）。
	QueryToken string `json:"query_token"`

	// Level 是考核分层：实训生 / 实习生 / 长期共建者 / 短期打手。
	Level string `json:"level"`

	// Track 是序列轨道：T0/T1/T2 序列，A/B 双轨。
	Track string `json:"track"`

	// Records 是评估记录与信任记录的统一承载（同构契约）。
	Records []Record `json:"records"`

	CreatedAt string `json:"created_at"`
	UpdatedAt string `json:"updated_at"`
}

// Record 是评估与信任共用的记录单元（同构契约）。
type Record struct {
	// Type 是记录类型：评估（考核分层 / 问卷分析 / 政策匹配）或
	// 信任（历史行为 / 背调 / 推荐信）。
	Type string `json:"type"`

	Title       string `json:"title"`
	Description string `json:"description"`
	Date        string `json:"date"`

	// Source 是记录来源：CLI 采集 / 考核沉淀 / 背调报告。
	Source string `json:"source"`

	Status string `json:"status"`
}
