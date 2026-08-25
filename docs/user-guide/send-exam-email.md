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
> 我们认真看了你此前提交的材料及招聘流程中的整体表现，认为你目前展现出的能力和潜力符合量潮进一步招聘考核的要求，因此想邀请你直接参与招聘考核，也想先听听你的想法和意愿。
>
> 量潮的考核以实际成果为核心：不预设题目，在相对开放的环境中自主发现并提出有价值的问题，通过自己的方式创造实际成果。完整的考核机制、岗位要求与报名方式见招聘官网（recurit.quanttide.com/intern/assessment）。如有意参与课题考核，请通过邮箱向我们申请，截止时间为发送本邮件后的下周三 12:00。
>
> 量潮众包可以作为你的选题库和灵感库（crowd.quanttide.com），上面有真实的项目任务供参考，也欢迎你直接参与。
>
> 量潮课堂提供生产实习等课程作为学习资料（class.quanttide.com），可以帮助你提前了解我们的工作方式和技术栈。
>
> 需要提前说明的是，通过招聘考核代表你达到了进入量潮团队的人才选拔标准，但最终是否进入团队，还要看届时公司的岗位和项目情况。如果暂时没有合适岗位，我们也会优先考虑让你进入长期实训，或保留后续合作的可能。
>
> 如果你愿意参与招聘考核，可以直接回复我们，确认意愿后，我们会与你沟通具体考核方式和下一步安排。如果你希望先通过实训、众包或课堂参与量潮，或者暂时不打算继续任何后续安排，也可以直接告诉我们。
>
> 期待你的回复。

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
