# 发送考核邀请邮件

本指南说明如何使用 `qtrecurit` CLI 向候选人发送招聘考核邀请邮件。

## 适用场景

当候选人已完成准入问卷并通过初筛后，可发送考核邀请邮件，邀请其参与招聘考核。

## 发送考核邀请

```bash
qtrecurit access exam \
  --to candidate@example.com
```

CLI 会渲染内置的话术模板，通过 `hr@quanttide.com` 发出考核邀请邮件。

### 参数说明

- `--to`（必填）候选人邮箱地址
- `--dry-run`（可选）只打印将执行的命令，不实际执行

### 邮件内容

发送的邮件内容如下：

> 你好，
>
> 我们认真看过你的材料，认为你有潜力直接参与量潮的招聘考核，所以想邀请你尝试一下，也想先听听你的想法和意愿。
>
> 量潮的考核以实际成果为核心：不预设题目，在相对开放的环境中自主发现并提出有价值的问题，通过自己的方式创造实际成果。完整的考核机制见招聘官网（recurit.quanttide.com/intern/assessment）。如有意参与课题考核，请通过邮箱向我们申请，截止时间为发送本邮件后的下周三 12:00，过期未申请视为放弃。
>
> 量潮众包（crowd.quanttide.com）是一个真实的项目任务库，可以作为你的选题库和灵感库。我们推荐以众包任务提出的需求为基础申请课题，任务完成也可以作为考核证明的一部分——众包是量潮为候选人搭建的阶梯化成长路径，欢迎你直接参与。
>
> 量潮课堂（class.quanttide.com）提供生产实习等系列课程，可以作为你的学习资料。我们建议你在准备课题时参考相关课程内容，提前熟悉我们的工作方式和技术栈——课堂是量潮培养路径的起点，为后续成长打基础。

### 自动归档

发送考核邀请邮件后，CLI 会自动将候选人的投递邮件移动到「已发送笔试」文件夹。

## 验证发送结果

CLI 会在发送后自动验证邮件是否成功，并在输出中返回结果：

```
✓ 已发送 | 收件人: candidate@example.com | 模板: exam | 状态: sent
```

如需手动查看已发送笔试文件夹中的邮件：

```bash
FOLDER_ID=$(qtrecurit cache show-folder-id --name "已发送笔试")
lark-cli mail +triage --mailbox hr@quanttide.com --filter "{\"folder\":\"$FOLDER_ID\"}" --max 5
```

## 常见问题

### 1. 发送失败：Concurrent write conflict

lark-cli 存在并发写入限制。批量发送时建议每封邮件间隔 1-2 秒：

```bash
# 错误做法
for item in ...; do qtrecurit access exam ...; done

# 正确做法
for item in ...; do qtrecurit access exam ...; sleep 1; done
```

### 2. 候选人回复后的处理

候选人收到考核邀请后，可能会回复确认参与意愿。此时需要：

1. 确认候选人选择（考核 or 实训）
2. 根据选择安排后续流程
3. 将邮件移动到对应的文件夹进行归档

### 3. 查看当前缓存状态

```bash
qtrecurit cache show-survey      # 问卷链接
qtrecurit cache show-qr          # 二维码图片
qtrecurit cache show-folder-id --name "已发送笔试"  # 文件夹ID
```
