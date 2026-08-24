# AGENTS.md - AI 工作指南

## 工作纪律

### 发布纪律

1. **不删除已发布的版本**：如果发布过程中出现问题，不要删除已创建的标签或 Release，而是创建新版本继续发布
2. **标签格式**：CLI 发布使用 `cli/vX.Y.Z-qualifier` 格式（如 `cli/v0.1.0-rc.1`）
3. **预发布版本**：使用 `-alpha.N`、`-beta.N`、`-rc.N` 后缀标识预发布版本

### 提交纪律

1. **Conventional Commits**：使用规范的提交信息格式
   - `feat:` 新功能
   - `fix:` 修复 bug
   - `docs:` 文档更新
   - `chore:` 构建/工具
2. **原子提交**：每次提交包含完整独立变更
3. **提交前检查**：确保工作区干净、版本号正确、CHANGELOG 已更新

### 子模块操作纪律

1. **独立提交**：子模块内容变更在子模块内提交推送
2. **引用同步**：子模块提交后，父仓库更新引用
3. **标签位置**：CI/CD 标签应创建在子模块仓库中，而非父仓库

## 常用命令

```bash
# 预检查
git status
cargo build
cargo test

# 提交
git add -A
git commit -m "feat: 描述"

# 发布
git tag cli/vX.Y.Z-qualifier
git push origin cli/vX.Y.Z-qualifier

# 验证
gh release view <tag> --repo quanttide/qtrecurit
gh run list --repo quanttide/qtrecurit --limit 5
```
