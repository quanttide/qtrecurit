# CHANGELOG

## [provider/v0.0.1] - 2026-08-09

### 新增
- Go 服务脚手架：`cmd/server/main.go` 入口 + `internal/` 分层模块（参照 qtadmin provider 模式）
- 基础设施：健康检查（`GET /health`）、配置管理（JSON 文件 + `QTRECURIT_*` 环境变量覆盖）、`log/slog` 日志
- 存储层：`internal/store` — Store 接口 + FileStore（本地 JSON 文件），S3 预留
- 契约先行：`internal/model` 定义候选人档案 schema（评估字段 + 信任字段同构，统一 `records` 记录单元）

#### Standards 域（考评标准只读 API，透明公开）

- 政策：`GET /api/v1/standards/policies`、`GET /api/v1/standards/policies/{id}`
- 筛选标准：`GET /api/v1/standards/criteria`、`GET /api/v1/standards/criteria/{id}`
- 考核说明：`GET /api/v1/standards/assessments`、`GET /api/v1/standards/assessments/{id}`

#### Candidates 域（候选人档案 API）

- 候选人 CRUD：`GET/POST /api/v1/candidates`、`GET/PUT/DELETE /api/v1/candidates/{id}`（内部写入）
- 候选人查询：创建时自动签发 `query_token`，`GET /api/v1/candidates/{id}` 凭查询凭证（`Authorization: Bearer`）查看自己的档案，凭证缺失或无效返回 `401`

### 测试
- 集成测试（httptest 启动完整 server）：健康检查、标准只读、候选人 CRUD、查询凭证
- 单元测试：Config、Store（FileStore）、Model 契约

### 基础设施
- CI: `.github/workflows/provider.yml`（build + vet + test）
- 使用文档：`docs/usage.md`、`README.md`
