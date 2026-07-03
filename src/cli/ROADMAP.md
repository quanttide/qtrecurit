# ROADMAP — connect 模块合并

## 背景

`connect` 模块有两份源码：

| 来源 | 位置 | 特点 |
|------|------|------|
| **CLI 当前** | `src/cli/src/connect/` | 有 trait 抽象（`EmailFetcher`）、profile 配置加载，但功能少——只有 `LarkCliFetcher` 拉取邮件标题+日期 |
| **实验** | `examples/connect/` | 功能完整——分页拉取、增量同步、附件下载、LLM 分类、候选人整理、漏斗报告，但架构简陋——无 trait、硬编码内置规则、无测试 |

**目标：** 把实验的能力吸收到 CLI，保持 CLI 现有的架构质量（trait 抽象 + profile 配置 + 测试覆盖）。

---

## 差距分析

### 实验有、CLI 无的能力

| 能力 | 实验文件 | 优先级 | 说明 |
|------|---------|--------|------|
| 分页拉取收件箱/发件箱 | `main.rs:fetch_all_meta` (L84-121) | 高 | 现有 CLI 只有 20 页的硬限制，实验做了完整的分页循环 |
| 增量同步（cursor） | `main.rs:load_cursor/save_cursor` (L195-210) | 高 | 记录已同步日期，避免重复拉取 |
| 附件下载 | `downloader.rs` | 中 | 临时文件→重命名模式，防中断产生残文件 |
| LLM 分类 | `classifier.rs`（调用 Python classify.py） | 中 | 实验的 `async` 分类，按 6 种邮件类型分类 |
| `+messages` 批量获取正文 | `main.rs:fetch_full` (L215-238) | 高 | 现有 CLI 只拉标题，实验可以拉全文 |
| 按候选人聚合 | 实验数据产物（`data/journal/_index.json`） | 中 | 邮件→候选人级整理 |
| 漏斗报告 | `scripts/funnel_report.py` | 低 | Markdown 漏斗图，可在 CLI `qtrecurit status` 中增强 |

### CLI 有、实验无的质量

| 质量 | 现状 | 合并原则 |
|------|------|---------|
| Trait 抽象 | `EmailFetcher` trait + `LarkCliFetcher` 实现 | 保留，新功能也通过 trait 注入 |
| 配置加载 | `config.rs` 从 profile 读取 `connect/rules.json` | 保留，替代实验的内置规则 |
| 分类逻辑 | `config.rs:classify()` — 关键词+优先级+排除 | 保留，规则从 profile 加载而非硬编码 |
| 测试覆盖 | email.rs 8 测试 + config.rs 7 测试 + mod.rs 3 测试 | 保留，新功能补齐测试 |

---

## 合并步骤

### Step 1 — 扩充 `email.rs`：添加完整邮件拉取能力

**当前：** `LarkCliFetcher::fetch_all` 硬编码 20 页限制，只拉标题和日期。

**目标：** 新增 `fetch_all_meta`、`fetch_full`、cursor 机制，保持 `EmailFetcher` trait 不变。

**具体：**
1. `connect/email.rs` 新增 `LarkCliFetcher::fetch_all_meta(mailbox, folder)` — 真分页循环（无页数硬限制）
2. 新增 `LarkCliFetcher::fetch_full(mids, mailbox)` — 批量正文下载
3. 新增 cursor 读写函数（`load_cursor` / `save_cursor`）
4. 新增 `MailFetcher` trait 扩展方法（可选）
5. 现有 `fetch_all` 改为调用 `fetch_all_meta` 实现
6. 测试：cursor 读写、分页循环 mock

### Step 2 — 新增 `connect/downloader.rs`：附件下载

**当前：** 无。

**目标：** 移植实验的附件下载逻辑。

**具体：**
1. 新建 `src/connect/downloader.rs`
2. 移植 `download_attachments` 函数（临时文件→重命名模式）
3. 暴露 `Downloader` trait（可选）
4. 测试：下载成功/失败/重名/中断保护

### Step 3 — 扩充 `connect/config.rs`：从 profile 加载分类规则

**当前：** 硬编码 `builtin_rules()` 12 个岗位。

**目标：** 改为从 `data/profile/connect/rules.json` 加载，profile 不存在时回退内置规则。

**参考：** qtadmin CLI `human/config.rs` 的 `load_from_profile()` 实现。

### Step 4 — 新增 `connect/classifier.rs`：邮件分类

**当前：** 实验在 Python 中做 LLM 分类。

**目标：**
1. 移植分类逻辑到 Rust，通过 `quanttide-agent` crate 调用 LLM
2. 定义邮件类型枚举：`ResumeSubmission / WrittenExam / InterviewScheduling / OfferLetter / HrInternal / Unrelated`
3. 分类结果与邮件关联保存
4. 考虑「候选人级分类」vs「单邮件分类」的粒度选择

### Step 5 — `qtrecurit status` 增强

**当前：** 按岗位统计投递量，展示未识别邮件。

**目标：** 新增漏斗报告（投递→联系→笔试→面试→Offer），整合实验的 `scripts/funnel_report.py` 逻辑。

---

## 发布顺序

```
v0.0.3  Step 1 ✅ 完成 — 分页拉取、游标、完整正文、缓存、过滤
v0.0.3  Step 2 ✅ 完成 — 附件下载
v0.0.3  Step 3 ✅ 完成 — profile 配置化（回退内置规则）
v0.0.3  Step 4 ✅ 完成 — LLM 分类（quanttide-agent）
v0.0.3  Step 5 ✅ 完成 — 漏斗报告（投递→笔试→面试→Offer）
```

## 迁移完成后的清理

- 删除 `examples/connect/` 目录
- `examples/connect/ROADMAP.md` 中 Phase 2/3 功能点移到 `ROADMAP.md` 做远期规划

## 不做

- **实验的 Python 分类脚本**（`classify.py`、`classify_journal.py`）不移植，用 Rust 重写调用 LLM
- **`examples/connect/docs/ops.md`** 中的操作步骤转到 `docs/handbook/` 或 `docs/tutorial/`
