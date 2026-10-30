# qtrecurit CLI

量潮招聘命令行工具。

> 招聘流程沟通命令（access 域：问卷/邀请/笔试/面试）已于 2026-10-29 移除——邮件驱动的招聘流程停用，选拔统一走选题制考核。本 CLI 保留统计、推荐与收件箱同步三条命令。

## 定位

本 CLI 是**数据采集管道**，不是招聘流程的执行者。平台的业务本体是 Provider（多租户后端原型），招聘前台是 Site，候选人窗口是 Studio；CLI 为它们采集投递数据、漏斗统计与信用凭证。招聘的定位是「内部系统招聘 + 服务推荐给其他企业」，目标是摸索可商业化、可定量核算的招聘业务。

判断依据与「流程入手册、资产入代码」的铁律见 [`docs/dev-guide/roadmap.md`](docs/dev-guide/roadmap.md)。边界：本 CLI 的批量处理与统计能力属采集管道的工程目标，不构成课题立项评审的方向依据——依章程第七条，效率与自动化导向不纳入体系支持范围。商业化设计链：

- [`docs/dev-guide/recommendation-service-brd.md`](docs/dev-guide/recommendation-service-brd.md)：推荐服务化 BRD
- [`docs/dev-guide/recommendation-service-design.md`](docs/dev-guide/recommendation-service-design.md)：商业模式与三端角色
- [`docs/dev-guide/multi-tenant-platform-architecture.md`](docs/dev-guide/multi-tenant-platform-architecture.md)：多租户平台架构，CLI 在其中的角色

## 安装

```bash
cargo install qtrecurit-cli
```

需要系统中安装 `lark-cli`（用于邮件获取/发送）和 `curl`（用于附件下载）。

## 用法

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
```

report 输出包含：
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
  │   ├── send_mail        — 发送通道（refer 使用，内部写发送日志）
  │   ├── fetch_recent_meta — 拉取收件箱/发件箱最近一批邮件
  │   ├── fetch_all_meta    — 分页拉取收件箱/发件箱
  │   ├── fetch_full       — 批量下载完整正文
  │   └── 游标 + 缓存     — 增量同步
  ├── connect/downloader.rs — 附件下载（临时文件→重命名）
  ├── connect/config.rs    — 岗位分类规则（profile 优先，回退内置）
  ├── connect/classifier.rs — LLM 邮件分类（quanttide-agent）
  ├── human/report.rs      — Markdown 报告格式化
  └── funnel.rs            — 招聘漏斗分析（关键词匹配）
```

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

## 开发

```bash
cargo test     # 单元 + 集成测试
cargo build    # 编译
```
