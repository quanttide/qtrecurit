# Changelog

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
