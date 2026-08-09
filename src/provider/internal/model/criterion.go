package model

// 考评标准（v0.1 只读 API）：政策、筛选标准、考核说明——无需登录即可读，
// 是「考评方式透明公开」的基础。
//
// 契约先行：字段以 data/profile/qtrecurit/criteria.json 为基准，
// 与 Studio 数据模型同源。policy 与 assessment 承载长文（content），
// criterion 承载细则（description）。

// Policy 是政策：知识提炼链产物（政策列表、状态承载、主题总结）的公开化展示。
type Policy struct {
	ID      string `json:"id"`
	Title   string `json:"title"`
	Content string `json:"content"`
	Status  string `json:"status"`
}

// Criterion 是筛选标准：五个检验维度（期望匹配、动机视角、了解程度、学习意向、责任心）的落地细则。
type Criterion struct {
	ID          string `json:"id"`
	Title       string `json:"title"`
	Description string `json:"description"`
	Status      string `json:"status"`
}

// Assessment 是考核说明：考核分层、序列轨道、问卷与 AI 分析说明。
type Assessment struct {
	ID      string `json:"id"`
	Title   string `json:"title"`
	Content string `json:"content"`
	Status  string `json:"status"`
}
