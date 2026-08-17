# qtrecurit Studio 页面与组件规划

qtrecurit Studio 是**候选人的透明窗口**——让候选人用同一套标准、同一套数据看自己。核心页面从"示例推荐信查看器"演进为"真实信用档案窗口"：凭证查询、评估记录、信任记录、推荐信与授权管理（随多租户平台演进，见 `docs/dev-guide/multi-tenant-platform-architecture.md`）。

## 页面规划

| 页面 | 路由 | 内容 |
|------|------|------|
| 凭证查询首页 | `/` | 输入查询凭证进入个人档案；无凭证时展示产品说明与示例。详细规划见 [screens/home.md](./screens/home.md) |
| 结构化推荐信 | `/recommendation/:id` | 评估通过者获得的可展示、可验证的信用背书：客观行为记录（时间、事项、结果）与我们的评价（署名）。不展示考核标准与评估结论。详细规划见 [screens/recommendation.md](./screens/recommendation.md) |
| 评估记录 | `/records/assessment` | 凭凭证查看自己的评估记录：考核分层、问卷分析、政策匹配。详细规划见 [screens/records.md](./screens/records.md) |
| 信任记录 | `/records/trust` | 凭凭证查看自己的信任记录：历史行为、背调、推荐信。详细规划见 [screens/records.md](./screens/records.md) |
| 授权管理 | `/authorizations` | 查看"我向哪些企业披露过信用摘要"，可撤销。详细规划见 [screens/authorization.md](./screens/authorization.md) |

## 组件规划

- 凭证输入卡：查询凭证输入与校验（首页）
- 档案入口卡：推荐信、评估记录、信任记录、授权管理（首页，凭证验证后）
- 推荐信头部卡：推荐方、被推荐人、编号与评估时点
- 行为记录条目：客观行为（时间、事项、结果）
- 实战记录条目：实战任务、过程与结果
- 署名评价卡：直接指导者评价、创始人评价（署名与身份）
- 验证徽章与入口：企业查证、责任条款、导出
- 授权条目：企业、披露时间、摘要内容、撤销按钮

## 数据模型

与 Provider API schema 同源，保证评估字段与信任字段同构契约：

- `recommendation`：推荐信聚合（客观行为记录 + 署名评价）
- `behavior`：客观行为记录（时间、事项、结果）
- `evaluation`：署名评价（评价者、身份、内容）
- `assessment_record`：评估记录（考核分层、问卷分析、政策匹配）
- `trust_record`：信任记录（历史行为、背调、推荐信）
- `authorization`：信用披露授权（企业租户、披露时间、摘要、状态）

## 演进方向

- 现状（v0.1）：`assets/mock/recommendations.json` 示例推荐信，`recommendationId` 默认 `rec_001`
- 阶段 1：数据源 mock → Provider API（凭证查询、真实推荐信）；评估/信任记录视图
- 阶段 2：统一档案视图（标准 + 过程 + 结果 + 信任）
- 阶段 3：授权管理、信用摘要披露、PDF 导出与分享、企业查证
