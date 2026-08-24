# 发送实训邀请邮件

本指南说明如何使用 `qtrecurit` CLI 向候选人发送实训基地邀请邮件。

## 第一步：缓存问卷名单和二维码

### 获取问卷提交者名单

问卷提交记录存储在飞书多维表格「量潮招聘工作档案」中。

先从缓存获取数据源地址：

```bash
qtrecurit cache show-template-source --name invite
```

然后使用该地址获取数据：

```bash
# 解析多维表格 URL，获取 base_token
lark-cli base +url-resolve --url "<缓存的地址>" --as user

# 获取表格列表
lark-cli base +base-block-list --base-token "<base_token>" --as user

# 获取字段列表，确认姓名字段
lark-cli base +field-list --base-token "<base_token>" --table-id "<table_id>" --as user

# 获取所有记录（提取姓名字段）
lark-cli base +record-list --base-token "<base_token>" --table-id "<table_id>" --as user --format json | jq -r '.data.data[] | select(.[16] != null) | .[16]'
```

### 缓存数据源地址

首次使用时需要缓存数据源地址：

```bash
qtrecurit cache set-template-source --name invite --url "<飞书文档地址>"
```

## 第二步：对比找出未发送邀请的名单

### 获取已发送实训邀请名单

```bash
# 查看"已发送实训邀请"文件夹 ID
lark-cli mail user_mailbox.folders list --user-mailbox-id hr@quanttide.com --format json

# 获取该文件夹中的邮件
lark-cli mail +triage --mailbox hr@quanttide.com --filter '{"folder":"<folder_id>"}' --max 100
```

### 对比找出未发送名单

对比问卷提交者名单和已发送邀请名单，找出已提交问卷但未收到邀请的候选人。

```bash
# 获取已发送实训邀请的邮件主题（包含候选人姓名）
FOLDER_ID=$(qtrecurit cache show-folder-id --name "已发送实训邀请")
lark-cli mail +triage --mailbox hr@quanttide.com --filter "{\"folder\":\"$FOLDER_ID\"}" --max 100
```

将获取的名单与问卷提交者名单对比，找出差集即为待发送邀请的候选人。

## 第三步：发送邀请邮件

```bash
qtrecurit access invite \
  --to candidate@example.com \
  --name 张三
```

CLI 会渲染内置的话术模板，通过 `hr@quanttide.com` 发出邀请邮件。

### 参数说明

- `--to`（必填）候选人邮箱地址
- `--name`（必填）候选人姓名
- `--dry-run`（可选）只打印将执行的命令，不实际执行

### 邮件内容

发送的邮件内容如下：

> 张三你好，感谢你完成量潮科技的准入问卷。经评估，你已通过初筛，正式受邀加入量潮实训基地。
>
> 实训基地是量潮科技对外招聘考核的组成部分。你将在这里通过完成真实的工作任务接受考核，以实际产出代替答卷。
>
> 请扫码加入实训基地群（见附件二维码），进群后修改昵称为「张三-岗位意向」。
>
> 具体考核规则将在群内发布，请关注群公告和资料。
>
> 期待在基地见到你。
>
> 量潮招聘

### 自动归档

发送邀请邮件后，CLI 会自动将候选人的投递邮件移动到「已发送实训邀请」文件夹。

## 验证发送结果

CLI 会在发送后自动验证邮件是否成功，并在输出中返回结果：

```
✓ 已发送 | 收件人: candidate@example.com | 邮件已发送，message_id: xxx
```

如需手动查看已发送邀请文件夹中的邮件：

```bash
FOLDER_ID=$(qtrecurit cache show-folder-id --name "已发送实训邀请")
lark-cli mail +triage --mailbox hr@quanttide.com --filter "{\"folder\":\"$FOLDER_ID\"}" --max 5
```
