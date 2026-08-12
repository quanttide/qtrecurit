# 部署参数配置最佳实践

部署参数（OSS 桶、endpoint、CDN 域名）不硬编码在 workflow 中，使用 GitHub variables 承载，从 git 历史移除。源自 qtrecurit site/studio 域名拆分时的实践（2026-08-12）。

## 问题

- 部署参数写死在 `.github/workflows/*.yml` 里，改配置要发版、要改历史
- 参数进入 git 历史后无法移除：仓库一旦公开或迁移，内部域名/桶名成为永久历史遗留
- workflow 无法跨仓库复制：每个仓库的域名、桶名不同，复制后要手改

## 做法

### 1. 用 gh 设置 repo 级 variables

```bash
# 示例值用占位符，实际值只存在于 GitHub variables，不写入任何仓库文件
gh variable set OSS_ENDPOINT -R <org>/<repo> --body "oss-cn-<region>.aliyuncs.com"
gh variable set OSS_BUCKET_<SCOPE> -R <org>/<repo> --body "<repo>-<scope>"
gh variable set CDN_DOMAIN_<SCOPE> -R <org>/<repo> --body "<subdomain>.<domain>"
```

- 变量按 scope 区分命名：同域多端时加后缀（如 `OSS_BUCKET_SITE` / `OSS_BUCKET_STUDIO`、`CDN_DOMAIN_SITE` / `CDN_DOMAIN_STUDIO`）
- 敏感凭据（AK/SK）用 secrets（`${{ secrets.X }}`），非敏感配置用 variables（`${{ vars.X }}`）
- 多仓库共用时提升到 org 级：`gh variable set X --org <org>`

### 2. workflow 只引用变量

```yaml
env:
  OSS_BUCKET: ${{ vars.OSS_BUCKET_<SCOPE> }}
  OSS_ENDPOINT: ${{ vars.OSS_ENDPOINT }}
  CDN_DOMAIN: ${{ vars.CDN_DOMAIN_<SCOPE> }}
```

- 注释中也不写具体值：注释同样进入历史，泛化描述（如「部署参数见 GitHub variables」）
- 文档示例一律用占位符：文档也在 git 历史里，写真实值等于把配置放回历史

### 3. 已提交的历史重写移除

```bash
git stash push -m "workflow vars"   # 保存工作区修改
GIT_SEQUENCE_EDITOR="sed -i 's/^pick <sha>/edit <sha>/'" git rebase -i <parent-sha>
git stash pop
git add .github/workflows/ && git commit --amend --no-edit
git rebase --continue
# 验证：git show <new-sha>:.github/workflows/... | grep -cE "硬编码pattern"  # 应为 0
git reflog expire --expire=now --all && git gc --prune=now
git push --force-with-lease origin main
```

- 只重写引入硬编码的提交及其后的提交；`--force-with-lease` 保护协作分支
- 重写后同步子模块指针（主仓库 `git add <submodule>` + 提交）
- 文档中出现真实值同样需要重写历史移除，不只限 workflow

## 边界

- Terraform（`manifests/terraform/`）与运维脚本（`scripts/`）中的桶名、域名是 IaC 声明与脚本固有内容，保留在仓库
- 重写历史前确认：无其他协作者基于旧历史工作、tag 未指向被重写提交（tag 指向的提交不重写）
