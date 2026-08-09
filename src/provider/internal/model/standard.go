package model

// 考评标准（v0.1 只读 API）：政策、筛选标准、考核说明——无需登录即可读，
// 是「考评方式透明公开」的基础。数据由 CLI 通过知识提炼链采集后写入，Provider 不重复造数据。

// Policy 是政策：知识提炼链产物（政策列表、状态承载、主题总结）的公开化展示。
type Policy struct {
	ID      string `json:"id"`
	Title   string `json:"title"`
	Content string `json:"content"`
	Status  string `json:"status"`

	CreatedAt string `json:"created_at"`
	UpdatedAt string `json:"updated_at"`
}

// Criterion 是筛选标准：考核分层与序列轨道的落地细则。
type Criterion struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`

	// Dimension 是考察维度，如责任心。
	Dimension string `json:"dimension"`
	Status    string `json:"status"`

	CreatedAt string `json:"created_at"`
	UpdatedAt string `json:"updated_at"`
}

// Assessment 是考核说明：考核分层（实训生/实习生/长期共建者/短期打手）、
// 序列轨道（T0/T1/T2、A/B 双轨）、问卷与 AI 分析说明。
type Assessment struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`

	// Kind 是考核说明类别：考核分层 / 序列轨道 / 问卷与 AI 分析说明。
	Kind    string `json:"kind"`
	Content string `json:"content"`
	Status  string `json:"status"`

	CreatedAt string `json:"created_at"`
	UpdatedAt string `json:"updated_at"`
}
