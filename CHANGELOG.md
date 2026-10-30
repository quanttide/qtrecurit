# Changelog

## [Unreleased]

### Removed

- Studio 移除「候选人透明窗口」整套实现（凭证查询/推荐信/评估记录/信任记录/授权管理五页，含 lib/、test/、docs/ 规划与 mock 数据）——该设计已判过时；Studio 改为企业/招聘者工作台，第一版只服务量潮自己，Flutter 脚手架保留
- 三篇商业化文档（BRD/服务设计/多租户架构）顶部标注暂缓：暂时不考虑商业模式，多租户形态一并暂缓，多租户后端路线不再作数
- CLI README 与 roadmap 删除指向三篇商业文档的「商业化设计链」引用，定位改为 Provider 后端本体 + Site 门户 + Studio/Lite 两端待建
- CLI 停用邮件驱动招聘流程：移除 access 域四命令（问卷/邀请/笔试/面试）、模板系统与 cache 命令，保留 report/refer/inbox sync（详见 src/cli/CHANGELOG.md）

### Fixed

- 从 Word 简历 MIME 类型补齐缺失的 `.doc` 或 `.docx` 文件扩展名。

## [0.1.0-beta.4] - 2026-08-25

### Fixed

- 修复版本号不匹配问题（Cargo.toml 与 tag）

## [0.1.0-beta.3] - 2026-08-25

### Refactor

- 移除发送日志写入，信息直接打印到控制台
- 模板目录移至 src/cli/templates/，与源码同级
- 使用 include_str! 编译时嵌入模板，移除运行时文件加载

## [0.1.0-beta.2] - 2026-08-25

### Fixed

- survey 默认直接发送邮件，不再只创建草稿
- 发送后自动标注已读（移除 UNREAD 标签）
- 修复验证匹配逻辑（triage 响应不含 to 字段）
- 修复投递邮件搜索（支持 name <email> 格式）
- 添加 CHANGELOG 验证脚本

## [0.1.0-beta.1] - 2026-08-25

### Features

- 发送问卷邮件后自动验证结果
- 自动归档候选人投递邮件到「已发送问卷」文件夹
- 支持文件夹 ID 缓存（`cache refresh-folder-id`/`show-folder-id`/`clear-folder-id`）

### Docs

- 简化用户指南，与工作手册形成差异化
- 更新验证说明

## [0.1.0-alpha.4] - 2026-08-24

### Features

- 初始版本，支持发送准入问卷邮件
