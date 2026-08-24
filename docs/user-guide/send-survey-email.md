# 发送准入问卷邮件

本指南说明如何使用 `qtrecurit` CLI 向候选人发送准入问卷通知邮件。

## 获取问卷链接

如果需要手动获取问卷链接，可按以下步骤操作：

```bash
# 步骤 1：搜索包含「准入问卷」的邮件
lark-cli mail +triage --mailbox hr@quanttide.com --query "准入问卷" --max 10

# 步骤 2：从输出中找到邮件的 message_id（例如：ZTExOWFiMmQtMjM2My00YzA1LWE1MzAtOWY2YjE0ODdhNTE4）

# 步骤 3：获取邮件完整内容
lark-cli mail +messages --mailbox hr@quanttide.com --message-ids "<message_id>" --format json

# 步骤 4：从邮件正文中提取问卷链接（形如 https://quanttide.feishu.cn/share/base/form/...）

# 步骤 5：缓存获取到的链接
qtrecurit cache refresh-survey
```


## 获取候选人信息

候选人的邮箱地址和姓名从 HR 邮箱收到的投递邮件中获取：

```bash
# 查看最近的投递邮件
lark-cli mail +triage --mailbox hr@quanttide.com --max 10

# 搜索特定候选人的邮件
lark-cli mail +triage --mailbox hr@quanttide.com --query "候选人姓名"
```

邮件主题通常包含候选人姓名和应聘岗位，例如「数据工程师-张三-清华大学-6个月」。

## 发送问卷邮件

```bash
qtrecurit access survey \
  --to candidate@example.com \
  --name 张三 \
  --link https://survey.quanttide.com/abc123
```

CLI 会渲染内置的「量潮科技准入问卷」话术模板，将候选人姓名和问卷链接填入对应位置，然后通过 `lark-cli` 从 `hr@quanttide.com` 邮箱发出。

### 参数说明

- `--to`（必填）候选人邮箱地址
- `--name`（必填）候选人姓名
- `--link`（可选）准入问卷链接，留空时自动从缓存或 HR 邮箱获取
- `--dry-run`（可选）只打印将执行的 `lark-cli` 命令，不实际执行

### 邮件内容

发送的邮件使用内置的 `survey` 话术模板，内容如下：

> 张三你好，感谢你对量潮科技的关注与投递。在继续招聘流程之前，请先完成以下准入问卷：https://survey.quanttide.com/abc123
>
> 问卷大约需要15-20分钟，请基于真实想法认真作答。这是进入筛选流程的必要条件。未在3个工作日内提交的，申请将被视为未完成。仅提醒一次。
>
> 量潮招聘

模板中的 `{{name}}` 和 `{{link}}` 会自动替换为 `--name` 和 `--link` 参数的值。

## 验证发送结果

发送邮件后，可通过以下方式验证：

```bash
# 查看最近发送的邮件
lark-cli mail +triage --mailbox hr@quanttide.com --query "量潮科技准入问卷" --max 5

# 查看特定邮件的详细内容
lark-cli mail +messages --mailbox hr@quanttide.com --message-ids "<message_id>" --format json
```
