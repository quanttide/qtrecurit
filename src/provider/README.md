# qtrecurit Provider

qtrecurit Provider 是招聘系统的数据服务端（Go）：内部写入考评数据（评估记录 + 信任记录），候选人凭查询凭证自查询。版本目标见 `ROADMAP.md`，产品上下文见仓库根 `ROADMAP.md` 与 `data/roadmap/qtrecurit/product.md`。

## 启动

```bash
cd src/provider
go run ./cmd/server
```

配置通过环境变量设定（见下方说明），也支持 JSON 配置文件（`CONFIG_PATH` 环境变量指定）。

## 环境变量

| 变量 | 默认值 | 说明 |
|:-----|:-------|:-----|
| `QTRECURIT_ADDR` | `:8000` | 监听地址 |
| `QTRECURIT_STORE_DRIVER` | `file` | 存储驱动（`file` / `s3`） |
| `QTRECURIT_STORE_PATH` | `data` | 数据存储目录 |
| `QTRECURIT_LOG_LEVEL` | `info` | 日志级别 |
| `QTRECURIT_LOG_FORMAT` | `text` | 日志格式，`text` 或 `json` |

## API

| 域 | 端点 | 鉴权 | 说明 |
|:---|:-----|:-----|:-----|
| Health | `GET /health` | 否 | 健康检查 |
| Criteria | `GET /api/v1/policies` | 否 | 政策列表（只读） |
| Criteria | `GET /api/v1/policies/{id}` | 否 | 政策详情 |
| Criteria | `GET /api/v1/criteria` | 否 | 筛选标准列表（只读） |
| Criteria | `GET /api/v1/criteria/{id}` | 否 | 筛选标准详情 |
| Criteria | `GET /api/v1/assessments` | 否 | 考核说明列表（只读） |
| Criteria | `GET /api/v1/assessments/{id}` | 否 | 考核说明详情 |
| Candidates | `GET/POST /api/v1/candidates` | 否 | 候选人列表 / 创建（内部写入） |
| Candidates | `GET /api/v1/candidates/{id}` | 查询凭证 | 候选人凭查询凭证查看自己的档案 |
| Candidates | `PUT/DELETE /api/v1/candidates/{id}` | 否 | 候选人更新 / 删除（内部） |

考评标准是透明公开的基础——无需登录即可读。候选人档案的评估字段与信任字段同构（统一 `records` 记录单元），筛选评估的产出即信任档案的记录。

候选人查询：创建候选人时自动签发 `query_token`，候选人访问自己的档案时在请求头携带 `Authorization: Bearer <query_token>`；凭证缺失或无效返回 `401`。详细用法见 `docs/usage.md`。

## 架构说明

Provider 只做两件事：

1. **持久化** — 接收 CLI 采集的数据，存入本地 JSON 文件（S3 接口预留，参照 qtadmin provider 模式）
2. **权限控制** — 创建候选人时自动签发 `query_token`，候选人凭此凭证（`Authorization: Bearer`）访问自己的档案；凭证缺失或无效一律 `401`

内部写入端点（列表、创建、更新、删除）v0.1 不做应用层鉴权，与 qtadmin provider 一致——生产环境通过反向代理加鉴权。CLI 负责采集（邮件拉取、漏斗分析、知识提炼链）并加工整理，然后通过 API 写入 Provider。Provider 不重复造数据。
