# qtrecurit CLI

量潮招聘命令行工具。

## 安装

```bash
cargo install qtrecurit-cli
```

需要系统中安装 `lark-cli`（用于邮件获取/发送）和 `curl`（用于附件下载）。

## 用法

命令按招聘业务动作命名（动词）：

```bash
# 生成招聘统计报告（基于 Lark 邮箱）
qtrecurit report

# 指定日期范围
qtrecurit report --days 30
qtrecurit report --start 2026-06-01 --end 2026-06-30

# 收件箱同步（最近一批，不跟随分页扫全邮箱）
qtrecurit inbox sync --mailbox hr@quanttide.com --folder INBOX --page-size 50 --format json
qtrecurit inbox sync --dry-run --format json

# 人才推荐（推荐信 → 发送，无状态、无凭证号/台账）
qtrecurit refer --name 张三 --candidate-email wu@example.com --company 示例企业
qtrecurit refer --name 张三 --candidate-email wu@example.com --company 示例企业 --dry-run

# 考核（access）域——招聘考核流程沟通命令（话术见业务实体手册 qtrecurit/connect/content.md）
qtrecurit access survey    --to 候选人@example.com --name 张三 [--link https://问卷链接]  # 准入问卷发放（链接可选，优先从缓存获取）
qtrecurit access invite    --to 候选人@example.com --name 张三 [--qr 群二维码.png]            # 实训邀请（进群）
qtrecurit access exam      --to 候选人@example.com                                            # 笔试（发送笔试邀请）
qtrecurit access interview --to 候选人@example.com --name 张三 --position 数据工程师 --time "6月20日 10:00"  # 面试通知
# 均支持 --dry-run（预览不发送）
```

输出包含：
- **岗位分布** — 按关键词规则自动分类各岗位投递量
- **投递趋势** — 每日投递数、日均、峰值
- **招聘漏斗** — 投递→笔试→面试→Offer 各阶段转化
- **未识别样本** — 未能匹配岗位的邮件主题（辅助调优分类规则）

## 架构

```
qtrecurit report
  ├── connect/email.rs    — 邮件拉取/发送管道（Lark Mail，收发一体）
  │   ├── EmailFetcher     — trait 抽象（收件）
  │   ├── LarkCliFetcher   — lark-cli 实现
  │   ├── send_mail        — 发送通道（草稿/确认，内部写发送日志）
  │   ├── fetch_recent_meta — 拉取收件箱/发件箱最近一批邮件
  │   ├── fetch_all_meta    — 分页拉取收件箱/发件箱
  │   ├── fetch_full       — 批量下载完整正文
  │   └── 游标 + 缓存     — 增量同步
  ├── connect/downloader.rs — 附件下载（临时文件→重命名）
  ├── connect/config.rs    — 岗位分类规则（profile 优先，回退内置）
  ├── connect/classifier.rs — LLM 邮件分类（quanttide-agent）
  ├── connect/notice.rs    — 飞书群通知
  ├── human/report.rs      — Markdown 报告格式化
  └── funnel.rs            — 招聘漏斗分析（关键词匹配）
```

命令按「名词域 + 动词动作」组织：`report` / `refer` / `access`（域），access 下
`survey` / `invite` / `assess` / `interview`（动作）。发送日志由
`connect/email.rs::send_mail` 内部处理（业务命令不感知）；沟通话术存储在
`templates/` 目录下的文本文件中（严格照业务实体手册 `qtrecurit/connect/content.md`）。

模板文件格式：第一行为邮件主题，其余为邮件正文。支持 `{{variable}}` 占位符变量。

## 数据流

```
Lark 邮箱 → 最近一批拉取（fetch_recent_meta）
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
| `XDG_CACHE_HOME` | 缓存目录 | `~/.cache` |

岗位分类规则存放在 `profile/connect/rules.json`，未设置时使用内置 12 个岗位规则。

## 缓存

问卷链接缓存遵循 XDG Base Directory Specification，存储在 `~/.cache/qtrecurit/survey_url`。

```bash
# 查看当前缓存的问卷链接
qtrecurit cache show-survey

# 从 HR 邮箱获取最新问卷链接并缓存
qtrecurit cache refresh-survey

# 清除问卷链接缓存
qtrecurit cache clear-survey
```

使用 `access survey` 命令时，问卷链接优先级：
1. `--link` 参数指定的链接
2. 本地缓存的链接
3. 自动从 HR 邮箱获取（并缓存）

## 开发

```bash
cargo test     # 70 测试
cargo build    # 编译
```

## 模板系统

邮件话术模板存储在 `templates/` 目录下，每个模板为独立的文本文件：

- `survey.txt` - 准入问卷发放
- `invite.txt` - 实训邀请
- `exam.txt` - 笔试邀请
- `interview.txt` - 面试通知

模板文件格式：
```
第一行：邮件主题
其余行：邮件正文（支持 {{variable}} 占位符）
```

运行时通过环境变量 `QTRECURIT_TEMPLATES_DIR` 可指定模板目录路径。
