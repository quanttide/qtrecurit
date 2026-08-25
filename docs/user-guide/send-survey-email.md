# 发送准入问卷邮件

本指南说明如何使用 `qtrecurit` CLI 向候选人发送准入问卷通知邮件。

## 获取问卷链接

问卷链接会自动从 HR 邮箱缓存。如需手动刷新：

```bash
qtrecurit cache refresh-survey
```

查看当前缓存的链接：

```bash
qtrecurit cache show-survey
```

## 获取候选人信息

从 HR 邮箱查看最近的投递邮件：

```bash
lark-cli mail +triage --mailbox hr@quanttide.com --max 10
```

邮件主题通常包含候选人姓名和应聘岗位，例如「数据工程师-张三-清华大学-6个月」。

## 发送问卷邮件

```bash
qtrecurit access survey \
  --to candidate@example.com \
  --name 张三
```

CLI 会渲染内置的话术模板，通过 `hr@quanttide.com` 发出问卷通知邮件。

### 参数说明

- `--to`（必填）候选人邮箱地址
- `--name`（必填）候选人姓名
- `--link`（可选）准入问卷链接，留空时自动从缓存获取
- `--dry-run`（可选）只打印将执行的命令，不实际执行

### 邮件内容

发送的邮件内容如下：

> 张三你好，感谢你对量潮科技的关注与投递。在继续招聘流程之前，请先完成以下准入问卷：https://survey.quanttide.com/abc123
>
> 问卷大约需要15-20分钟，请基于真实想法认真作答。这是进入筛选流程的必要条件。未在3个工作日内提交的，申请将被视为未完成。仅提醒一次。
>
> 量潮招聘

### 自动归档

发送问卷邮件后，CLI 会自动执行以下归档操作：

1. 将候选人的投递邮件从收件箱移动到「已发送问卷」文件夹

## 验证发送结果

CLI 会在发送后自动验证邮件是否成功，并在输出中返回结果：

```
✓ 已发送 | 收件人: candidate@example.com | 邮件已发送，message_id: xxx
```

如需手动查看已发送问卷文件夹中的邮件：

```bash
FOLDER_ID=$(qtrecurit cache show-folder-id --name "已发送问卷")
lark-cli mail +triage --mailbox hr@quanttide.com --filter "{\"folder\":\"$FOLDER_ID\"}" --max 5
```
