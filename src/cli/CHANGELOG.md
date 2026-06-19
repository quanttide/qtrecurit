# CHANGELOG

## [0.0.1] - 2026-06-19

### Added

- `qtrecurit` CLI 二进制入口，基于 clap 的子命令路由（`status` / `meta`）
- `status` 命令：招聘数据统计，支持日期范围过滤（`--days` / `--start` / `--end`）
- `meta` 命令：主数据关联概览，展示分类规则与招聘计划的关系
- 三域架构：`connect`（邮件获取）、`human`（岗位规则/报告）、`meta`（跨域关联索引）
- `human::config`：12 个内置岗位分类规则，支持关键词/排除词/优先级匹配
- `human::report`：Markdown 报告格式化，含岗位分布、投递趋势、未识别样本
- `human::status`：月度招聘计划与进度管理
- `connect::email::lark`：Lark Mail 邮件获取（LarkCliFetcher）
- `connect::notice`：飞书群通知命令
- 主数据管理：`RulePositionIndex`、`PlanPositionIndex` 跨域索引
- 41 个单元测试 + 4 个集成测试
