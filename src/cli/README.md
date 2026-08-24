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

# 凭证化人才推荐（凭证号 REF-YYYYMMDD-NNN → 推荐信 → 草稿 → 确认 → 发送，无状态不落库）
qtrecurit refer --name 张三 --candidate-email wu@example.com --company 示例企业
qtrecurit refer --name 张三 --candidate-email wu@example.com --company 示例企业 --confirm-send
qtrecurit refer --name 张三 --candidate-email wu@example.com --company 示例企业 --dry-run

# 考核（access）域——招聘考核流程沟通命令（话术见业务实体手册 qtrecurit/connect/content.md）
qtrecurit access survey    --to 候选人@example.com --name 张三 --link https://问卷链接        # 准入问卷发放
qtrecurit access invite    --to 候选人@example.com --name 张三 [--qr 群二维码.png]            # 实训邀请（进群）
qtrecurit access exam      --to 候选人@example.com                                            # 笔试（发送笔试邀请）
qtrecurit access interview --to 候选人@example.com --name 张三 --position 数据工程师 --time "6月20日 10:00"  # 面试通知
# 均支持 --confirm-send（确认后直接发送）与 --dry-run（预览不发送）
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

命令按「名词域 + 动词动作」组织：`report` / `refer` / `access`（域），access 下
`survey` / `invite` / `assess` / `interview`（动作）。发送日志由
`connect/email.rs::send_mail` 内部处理（业务命令不感知）；沟通话术见
`src/templates.rs`（严格照业务实体手册 `qtrecurit/connect/content.md`）。

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
cargo test     # 70 测试
cargo build    # 编译
```
