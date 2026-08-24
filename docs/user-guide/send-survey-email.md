# 发送准入问卷邮件

本指南说明如何使用 `qtrecurit` CLI 向候选人发送准入问卷通知邮件。

## 前置条件

1. 已安装 `lark-cli` 并完成登录认证
2. 已构建 `qtrecurit` CLI（`cargo build --release`）
3. 候选人的邮箱地址、姓名、准入问卷链接

## 发送问卷邮件

```bash
qtrecurit access survey \
  --to candidate@example.com \
  --name 张三 \
  --link https://survey.quanttide.com/abc123
```

CLI 会渲染内置的「量潮科技准入问卷」话术模板，将候选人姓名和问卷链接填入对应位置，然后通过 `lark-cli` 从 `hr@quanttide.com` 邮箱发出。

### 参数说明

| 参数 | 必填 | 说明 |
|:-----|:----:|:-----|
| `--to` | 是 | 候选人邮箱地址 |
| `--name` | 是 | 候选人姓名 |
| `--link` | 是 | 准入问卷链接 |
| `--confirm-send` | 否 | 加上此参数后直接发送邮件（默认只生成草稿） |
| `--dry-run` | 否 | 只打印将执行的 `lark-cli` 命令，不实际执行 |

### 默认行为：生成草稿

不加 `--confirm-send` 时，CLI 会调用 `lark-cli mail +send` 生成一封草稿邮件，但**不会自动发送**。你可以先在飞书中检查草稿内容，确认无误后再手动发送或重新执行命令并加上 `--confirm-send`。

```bash
# 1. 先生成草稿，检查内容
qtrecurit access survey \
  --to candidate@example.com \
  --name 张三 \
  --link https://survey.quanttide.com/abc123

# 2. 确认无误后，直接发送
qtrecurit access survey \
  --to candidate@example.com \
  --name 张三 \
  --link https://survey.quanttide.com/abc123 \
  --confirm-send
```

### 预览命令（dry-run）

不实际执行，只打印将要调用的 `lark-cli` 命令：

```bash
qtrecurit access survey \
  --to candidate@example.com \
  --name 张三 \
  --link https://survey.quanttide.com/abc123 \
  --dry-run
```

输出示例：

```
[dry-run] lark-cli mail +send --to candidate@example.com --subject 量潮科技准入问卷 --body 张三你好... --mailbox hr@quanttide.com --confirm-send --as user --format json
```

## 邮件内容

发送的邮件使用内置的 `survey` 话术模板，内容如下：

> 张三你好，感谢你对量潮科技的关注与投递。在继续招聘流程之前，请先完成以下准入问卷：https://survey.quanttide.com/abc123
>
> 问卷大约需要15-20分钟，请基于真实想法认真作答。这是进入筛选流程的必要条件。未在3个工作日内提交的，申请将被视为未完成。仅提醒一次。
>
> 量潮科技HR

模板中的 `{{name}}` 和 `{{link}}` 会自动替换为 `--name` 和 `--link` 参数的值。

## 发送日志

每次发送（或生成草稿）后，CLI 会自动将发送记录追加到 `.quanttide/logs/send.log`（JSONL 格式）。记录包含：

- 发送时间
- 收件人邮箱
- 邮件主题
- 使用的模板名称（`survey`）
- 状态（`sent` / `draft`）
- 草稿 ID

可通过环境变量 `SEND_LOG_DIR` 自定义日志目录。

## 常见问题

**Q: 发送失败，提示「无法启动 lark-cli」？**

确认 `lark-cli` 已安装且在 PATH 中。运行 `lark-cli --version` 检查。

**Q: 邮件没有发出去？**

默认行为是生成草稿而非直接发送。加上 `--confirm-send` 参数才会实际发送。

**Q: 想修改邮件话术？**

话术模板硬编码在 `src/cli/src/templates.rs` 中（源自业务实体手册 `qtrecurit/connect/content.md`）。修改后需重新编译。
