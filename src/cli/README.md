# qtrecurit CLI

量潮招聘命令行工具。

## 安装

```bash
cargo install --path .
```

需要系统中安装 `lark-cli`（用于邮件获取/发送）和 `curl`（用于附件下载）。

## 用法

```bash
# 查看招聘统计数据（基于 Lark 邮箱）
qtrecurit status

# 指定日期范围
qtrecurit status --days 30
qtrecurit status --start 2026-06-01 --end 2026-06-30

# 凭证化人才推荐（凭证号 REF-YYYYMMDD-NNN → 推荐信 → 草稿 → 确认 → 发送 → 台账）
qtrecurit referral send --name 张三 --candidate-email wu@example.com --company 示例企业
qtrecurit referral send --name 张三 --candidate-email wu@example.com --company 示例企业 --confirm-send
qtrecurit referral send --name 张三 --candidate-email wu@example.com --company 示例企业 --dry-run

# 招聘沟通邮件（话术模板：referral 内推 / training 实训邀请 / exam 考核说明）
qtrecurit mail send --to 候选人@example.com --template exam
qtrecurit mail send --to a@x.com --template training --vars name=张三
qtrecurit mail send --to x@example.com --template referral --confirm-send

# 模板管理
qtrecurit mail template --list
qtrecurit mail template --name exam

# 发送日志（只记元数据：时间/收件人/主题/状态，不记正文）
qtrecurit mail log --tail 20
```

输出包含：
- **岗位分布** — 按关键词规则自动分类各岗位投递量
- **投递趋势** — 每日投递数、日均、峰值
- **招聘漏斗** — 投递→笔试→面试→Offer 各阶段转化
- **未识别样本** — 未能匹配岗位的邮件主题（辅助调优分类规则）

## 架构

```
qtrecurit status
  ├── connect/email.rs    — 邮件拉取管道（Lark Mail）
  │   ├── EmailFetcher     — trait 抽象
  │   ├── LarkCliFetcher   — lark-cli 实现
  │   ├── fetch_all_meta   — 分页拉取收件箱/发件箱
  │   ├── fetch_full       — 批量下载完整正文
  │   └── 游标 + 缓存     — 增量同步
  ├── connect/downloader.rs — 附件下载（临时文件→重命名）
  ├── connect/config.rs    — 岗位分类规则（profile 优先，回退内置）
  ├── connect/classifier.rs — LLM 邮件分类（quanttide-agent）
  ├── connect/notice.rs    — 飞书群通知
  ├── human/report.rs      — Markdown 报告格式化
  └── funnel.rs            — 招聘漏斗分析（关键词匹配）
```

`referral`（凭证化推荐）与 `mail`（招聘沟通邮件）业务归属招聘域，发送通道复用
`qtcloud-connect-send`（lark-cli 封装，qtcloud-connect 仓库 src/send/），话术模板见 `src/templates.rs`。

## 数据流

```
Lark 邮箱 → 分页拉取（fetch_all_meta）
          → 正文下载（fetch_full）
          → 关键词分类岗位（config.rs）
          → 可选：LLM 邮件分类（classifier.rs）
          → 漏斗分析（funnel.rs）
          → Markdown 报告（human/report.rs）
```

## 配置

| 环境变量 | 说明 | 默认值 |
|----------|------|--------|
| `QTRECURIT_PROFILE` | profile 仓库路径 | `../../data/profile` |
| `DEEPSEEK_API_KEY` | LLM 分类 API Key | — |

岗位分类规则存放在 `profile/connect/rules.json`，未设置时使用内置 12 个岗位规则。

## 开发

```bash
cargo test     # 61 测试
cargo build    # 编译
```
