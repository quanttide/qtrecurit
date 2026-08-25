# ROADMAP - qtrecurit CLI

## 文档覆盖状态

| 命令 | 用户文档 | 状态 |
|------|----------|------|
| `access survey` | [send-survey-email.md](../../docs/user-guide/send-survey-email.md) | ✓ 完整 |
| `access invite` | [send-invite-email.md](../../docs/user-guide/send-invite-email.md) | ✓ 完整 |
| `access exam` | [send-exam-email.md](../../docs/user-guide/send-exam-email.md) | ✓ 完整 |
| `access interview` | 无 | ⚠ 需要打磨 |
| `report` | 无 | ⚠ 需要打磨 |
| `refer` | 无 | ⚠ 需要打磨 |
| `cache` | 无 | ⚠ 需要打磨 |

## 需要打磨的命令

### 1. access exam（笔试邀请）

**已完成：**
- [x] 添加用户文档 `send-exam-email.md`

**待完善项：**
- [ ] 完善话术模板，支持不同岗位的笔试题差异化
- [ ] 支持附件（笔试题目文件）上传
- [ ] 添加笔试截止时间参数
- [ ] 笔试结果回收和状态跟踪

### 2. access interview（面试通知）

**待完善项：**
- [ ] 添加用户文档 `send-interview-email.md`
- [ ] 支持面试时间、地点/链接参数
- [ ] 支持面试官信息
- [ ] 支持多候选人批量面试安排
- [ ] 面试提醒功能（面试前 N 小时提醒）
- [ ] 自动归档已发送面试通知

### 3. report（招聘统计报告）

**待完善项：**
- [ ] 添加用户文档 `generate-report.md`
- [ ] 完善报告模板，支持多种输出格式（Markdown/HTML/PDF）
- [ ] 添加岗位分布统计
- [ ] 添加投递趋势分析
- [ ] 添加招聘漏斗转化率
- [ ] 支持导出到文件
- [ ] 支持定时生成报告

### 4. refer（凭证化人才推荐）

**待完善项：**
- [ ] 添加用户文档 `refer-candidate.md`
- [ ] 完善推荐信模板
- [ ] 支持推荐理由参数
- [ ] 台账记录和查询
- [ ] 推荐状态跟踪
- [ ] 支持批量推荐

### 5. cache（缓存管理）

**待完善项：**
- [ ] 添加用户文档 `cache-management.md`
- [ ] 完善缓存过期机制
- [ ] 添加缓存状态查看（大小、创建时间）
- [ ] 支持缓存数据导入/导出
- [ ] 添加缓存数据验证

## 近期优先级

### P0 - 必须完成
1. `access exam` 用户文档和基础功能完善
2. `access interview` 用户文档和基础功能完善

### P1 - 高优先级
3. `report` 报告模板和输出格式
4. `refer` 推荐信模板完善

### P2 - 中优先级
5. `cache` 缓存管理和过期机制
6. 各命令的错误处理和边界情况

### P3 - 低优先级
7. 批量操作支持
8. 高级统计和分析功能
