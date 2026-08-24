# CHANGELOG

## [0.1.0-alpha.5] - 2026-08-24

### Changed

- 命令按招聘业务动作动词化：`status` → `report`、`referral send` → `refer`
- 删除通用 `mail` 命令（通道能力不属业务域），话术模板内容移除（模板机制保留）
- 发送日志由 `send_mail` 通道内部处理，业务命令不感知
- `refer` 推荐信正文移除考核评级（责任心评级/配合度）——考核属 access 域，推荐信只给已验证事实
- 新增考核（access）域命令集（话术严格照业务实体手册 `qtrecurit/connect/content.md`）：
  - `access survey`：准入问卷发放
  - `access invite`：实训邀请（邀请进群，可带群二维码附件）
  - `access exam`：笔试（发送笔试邀请）
  - `access interview`：面试通知

### Removed

- `mail` 命令（send/template/log）与三套话术模板内容
- 推荐信中的考核评级结论（access 内容）
- `refer` 的 CSV 台账（referrals.csv 硬编码）——改为无状态，推荐记录待关联 Provider 数据库

### Added

- `access` 域命令集：`survey` / `invite` / `exam` / `interview`（模板渲染 → 通道发送）

## [0.1.0-alpha.4] - 2026-08-24

### Changed

- `main.rs` 改用 lib 模块（对齐 qtcloud-connect 结构），消除 bin 私有模块导致的 dead_code 警告

### Removed

- 无

## [0.1.0-alpha.3] - 2026-08-24

### Added

- CI: `release-cli` 工作流照抄 qtcloud-devops 结构（check → 三平台构建 → `cargo publish` crates.io），tag `cli/*` 触发

### Changed

- 清理 `connect/email.rs` 未用导入
- CI 版本/CHANGELOG 校验脚本落地（`scripts/validate-version.sh` / `validate-changelog.sh`）

### Removed

- 无

## [0.1.0-alpha.2] - 2026-08-24

### Added

- CI: `release-cli` 工作流（tag `cli/*` 触发——版本/CHANGELOG 校验 + `cargo test` + Linux/macOS/Windows 三平台 release 构建）

### Changed

- 无

### Removed

- 无

## [0.1.0-alpha.1] - 2026-08-24

### Added

- `referral.rs`：凭证化人才推荐（自 qtcloud-connect 迁入，issue #1）——凭证号 `REF-YYYYMMDD-NNN`、推荐信正文、台账 `referrals.csv`、fail-closed 写入
- `mail.rs`：招聘沟通邮件命令（referral/training/exam 话术模板发送、模板查看、发送日志）
- `templates.rs`：招聘话术模板（自 qtcloud-connect mail.rs 随业务迁入）与模板渲染机制（render_template/parse_vars）
- `connect/email.rs`：发送方向（send_mail/send_draft）与发送日志，与收件共用 lark-cli 封装，招聘域自包含

### Changed

- 测试从 61 增至 104
- 邮件收发一体（connect/email.rs），不依赖外部发送库

### Removed

- 无

## [0.0.1] - 2026-07-03

### Added

- `connect/email.rs`：分页拉取 `fetch_all_meta`、批量正文下载 `fetch_full`、游标持久化、缓存读写
- `connect/downloader.rs`：邮件附件下载（临时文件→重命名）
- `connect/classifier.rs`：LLM 邮件分类（quanttide-agent，6 种邮件类型）
- `funnel.rs`：招聘漏斗分析（投递→笔试→面试→Offer）

### Changed

- `connect/config.rs`：分类规则从 profile 加载，回退内置规则
- `status.rs`：报告末尾追加漏斗段
- 测试从 47 增至 61

### Removed

- 无

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
