# qtrecurit Provider — 使用文档

## 启动服务

```bash
cd src/provider
go run ./cmd/server
```

服务默认监听 `http://127.0.0.1:8000`，可通过 `QTRECURIT_ADDR` 修改。

## API 端点

### 健康检查

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/health` | 健康检查 |

### 考评标准（只读，透明公开）

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/policies` | 政策列表 |
| GET | `/api/v1/policies/{id}` | 政策详情 |
| GET | `/api/v1/criteria` | 筛选标准列表 |
| GET | `/api/v1/criteria/{id}` | 筛选标准详情 |
| GET | `/api/v1/assessments` | 考核说明列表 |
| GET | `/api/v1/assessments/{id}` | 考核说明详情 |

### 候选人档案（内部写入 + 候选人自查询）

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/v1/candidates` | 候选人列表（内部） |
| POST | `/api/v1/candidates` | 创建候选人（内部写入，自动签发查询凭证） |
| GET | `/api/v1/candidates/{id}` | 候选人凭查询凭证查看自己的档案 |
| PUT | `/api/v1/candidates/{id}` | 更新候选人（内部） |
| DELETE | `/api/v1/candidates/{id}` | 删除候选人（内部） |

## 示例

### 考评标准只读

```bash
curl http://localhost:8000/api/v1/assessments
```

### 内部写入：创建候选人（评估 + 信任记录同构）

```bash
curl -X POST http://localhost:8000/api/v1/candidates \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "张三",
    "level": "实训生",
    "track": "T0-A",
    "records": [
      {"type": "问卷分析", "title": "责任心问卷", "description": "结构化问卷分析结果", "date": "2026-08-01", "source": "CLI 采集", "status": "done"},
      {"type": "历史行为", "title": "实训项目交付", "description": "按时交付任务", "date": "2026-08-05", "source": "考核沉淀", "status": "done"}
    ]
  }'
```

返回：

```json
{
  "id": "3f2a...",
  "name": "张三",
  "query_token": "9c1b...",
  "level": "实训生",
  "track": "T0-A",
  "records": [...],
  "created_at": "2026-08-09T00:00:00Z",
  "updated_at": "2026-08-09T00:00:00Z"
}
```

### 候选人自查询

候选人访问自己的档案时在请求头携带查询凭证：

```bash
curl http://localhost:8000/api/v1/candidates/3f2a... \
  -H 'Authorization: Bearer 9c1b...'
```

凭证缺失或无效时返回 `401`。

## 运行测试

```bash
cd src/provider
go test ./...
```

## 项目结构

```
src/provider/
├── cmd/server/       # 服务入口
├── internal/
│   ├── api/          # HTTP handler（Policy / Criterion / Assessment / Candidates / Health）
│   ├── config/       # 配置管理（文件 + 环境变量）
│   ├── model/        # 档案 schema 契约（候选人与考评标准）
│   ├── store/        # 存储层（FileStore，S3 预留）
│   └── version/      # 版本号
├── testdata/         # 测试配置
├── docs/usage.md     # 本文档
├── go.mod
└── README.md
```
