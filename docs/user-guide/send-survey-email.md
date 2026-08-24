# 发送准入问卷邮件

本指南说明如何使用 `qtrecurit` CLI 向候选人发送准入问卷通知邮件。

## 前置条件

1. 已安装 `lark-cli` 并完成登录认证
2. 已构建 `qtrecurit` CLI
3. 候选人的邮箱地址、姓名、准入问卷链接

## 发送问卷邮件

```bash
qtrecurit access survey \
  --to candidate@example.com \
  --name 张三 \
  --link https://survey.quanttide.com/abc123
```

CLI 会渲染内置的「量潮科技准入问卷」话术模板，将候选人姓名和问卷链接填入对应位置，然后通过 `lark-cli` 从 `hr@quanttide.com` 邮箱发出。

参数说明：

- `--to`（必填）候选人邮箱地址
- `--name`（必填）候选人姓名
- `--link`（必填）准入问卷链接
- `--dry-run`（可选）只打印将执行的 `lark-cli` 命令，不实际执行

## 邮件内容

发送的邮件使用内置的 `survey` 话术模板，内容如下：

> 张三你好，感谢你对量潮科技的关注与投递。在继续招聘流程之前，请先完成以下准入问卷：https://survey.quanttide.com/abc123
>
> 问卷大约需要15-20分钟，请基于真实想法认真作答。这是进入筛选流程的必要条件。未在3个工作日内提交的，申请将被视为未完成。仅提醒一次。
>
> 量潮招聘

模板中的 `{{name}}` 和 `{{link}}` 会自动替换为 `--name` 和 `--link` 参数的值。

## 常见问题

**Q: 发送失败，提示「无法启动 lark-cli」？**

确认 `lark-cli` 已安装且在 PATH 中。运行 `lark-cli --version` 检查。

**Q: 邮件没有发出去？**

检查 `lark-cli` 的认证状态，运行 `lark-cli mail +send --help` 查看详细参数。

**Q: 想修改邮件话术？**

话术模板存储在 `src/cli/templates/` 目录下的文本文件中（源自业务实体手册 `qtrecurit/connect/content.md`）。每个模板文件格式：第一行为邮件主题，其余为邮件正文。修改文本文件后无需重新编译。

模板文件示例（`survey.txt`）：
```
量潮科技准入问卷
{{name}}你好，感谢你对量潮科技的关注与投递...
```
