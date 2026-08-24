# 用户指南

## 前置条件

1. 已安装 `lark-cli` 并完成登录认证
2. 已构建 `qtrecurit` CLI

## 文档目录

- [发送准入问卷邮件](./send-survey-email.md) - 向候选人发送准入问卷通知

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