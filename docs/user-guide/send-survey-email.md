# 发送准入问卷邮件

本指南说明如何使用 `qtrecurit` CLI 向候选人发送准入问卷通知邮件。

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

## 邮件内容

发送的邮件使用内置的 `survey` 话术模板，内容如下：

> 张三你好，感谢你对量潮科技的关注与投递。在继续招聘流程之前，请先完成以下准入问卷：https://survey.quanttide.com/abc123
>
> 问卷大约需要15-20分钟，请基于真实想法认真作答。这是进入筛选流程的必要条件。未在3个工作日内提交的，申请将被视为未完成。仅提醒一次。
>
> 量潮招聘

模板中的 `{{name}}` 和 `{{link}}` 会自动替换为 `--name` 和 `--link` 参数的值。

## 获取问卷链接

问卷链接可通过以下方式获取：

### 方式一：自动获取（推荐）

直接运行 `access survey` 命令，不指定 `--link` 参数，CLI 会自动按以下顺序获取：
1. 检查本地缓存（`~/.cache/qtrecurit/survey_url`）
2. 缓存未命中时，自动从 HR 邮箱搜索包含「准入问卷」的邮件并提取链接
3. 获取后自动缓存，下次直接使用

```bash
# 直接发送，CLI 自动获取问卷链接
qtrecurit access survey --to candidate@example.com --name 张三
```

### 方式二：手动缓存

提前获取并缓存问卷链接，后续发送时直接使用：

```bash
# 从 HR 邮箱获取最新问卷链接并缓存
qtrecurit cache refresh-survey

# 查看当前缓存的问卷链接
qtrecurit cache show-survey

# 清除问卷链接缓存
qtrecurit cache clear-survey
```

### 方式三：手动指定

直接使用 `--link` 参数指定问卷链接：

```bash
qtrecurit access survey --to candidate@example.com --name 张三 --link https://具体链接
```

### 链接获取优先级

使用 `access survey` 命令时，问卷链接按以下优先级获取：
1. `--link` 参数指定的链接（最高优先级）
2. 本地缓存的链接
3. 自动从 HR 邮箱获取（并缓存）

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