# CHANGELOG

## [0.0.2] - 2026-06-19

### Changed

- 架构简化为两层：`connect`（邮件+分类规则）、`human`（岗位+报告）
- `config`（分类规则）从 `human` 移至 `connect`
- `meta` 模块移除，关联索引功能合并到 `connect/config`
- `report` 编排合并到 `human/report`
- `overview` 概览命令移除
- `connect/email` 合并为单文件，内聚 `MailItem`、`extract_date`、`filter_by_date`、`resolve_date_range`
- `status` 拆分 connect 抓取 + human 报告两步
- 移除未使用的 `department`、`employee`、`notice` 模块
- 移除服务端依赖（reqwest、tokio、sqlx、serde_yaml、toml）

### Tests

- 30 个单元测试 + 3 个集成测试

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
