# qtrecurit 部署选型（IaC）

对齐 qtdata 与 qtclass 的部署模式，作为 Terraform 基础设施代码的设计依据。

## 部署选型

| 维度 | 选型 | 说明 |
|------|------|------|
| Site 客户端 | React（招聘官网） | `src/site`，`npm run build` 产出 dist/，域名 `recurit.quanttide.com`（二级主站） |
| Studio 客户端 | Flutter Web（推荐信工作台） | `src/studio`，`flutter build web --release` 产出站点，域名 `studio.recurit.quanttide.com` |
| 发布分发 | 阿里云 OSS 桶 `qtrecurit-site` / `qtrecurit-studio` | 静态网站托管（index.html 默认页）+ 公共读 |
| CDN | 阿里云 CDN `recurit.quanttide.com`（site）/ `studio.recurit.quanttide.com`（studio） | 源站 OSS（域名回源）；site 用泛域名证书 `*.quanttide.com`，studio 用 SAN 证书 `recurit.quanttide.com + *.recurit.quanttide.com`（acme.sh 签发，续期后重跑 `scripts/configure-*.sh`） |
| 服务端 | Go provider（`src/provider`） | 独立部署，见 `.github/workflows/provider.yml`，不在本 IaC 范围 |

## 本 IaC 范围

- **应用级**（`qtrecurit-<env>` 命名）：OSS 发布分发桶 `qtrecurit-site`（`site.tf`）、`qtrecurit-studio`（`studio.tf`）：桶 + 静态网站托管 + 公共读 + 关闭阻止公共访问
- **不含** CDN / DNS / 证书（无组织级 IaC 先例，在控制台配置并记录于本文件）

## site 客户端发布

- 基础设施：`terraform apply`（`site.tf`）
- 构建上传：`.github/workflows/deploy-site.yml`（推送 tag `site/*` 触发 → npm build → ossutil cp → 刷新 CDN）

## studio 客户端发布

- 基础设施：`terraform apply`（`studio.tf`）
- 构建上传：`.github/workflows/deploy-studio.yml`（推送 tag `studio/*` 触发 → flutter build web → ossutil cp → 刷新 CDN）

## 关键操作记录（手动部署踩坑）

1. **阻止公共访问**：2023 后新 OSS 桶默认开启"阻止公共访问"，即使 ACL=public-read 匿名访问也返回 `AccessDenied`。需用 `alicloud_oss_bucket_public_access_block` 独立资源显式关闭。
2. **ACL drift**：桶创建后 ACL 可能回退为 private，`terraform plan` 可检测并修复。
3. **CDN 配置**（控制台/CLI 完成）：
   - site：`AddCdnDomain`：`recurit.quanttide.com`，源站 OSS `qtrecurit-site.oss-cn-hangzhou.aliyuncs.com`（type=oss, port=443）；HTTPS 上传 `*.quanttide.com` 证书（`scripts/configure-recurit-cdn.sh`，acme.sh 3 个月续期）；DNS：`recurit.quanttide.com` CNAME → `recurit.quanttide.com.w.kunlunaq.com`（注意 RR 精确匹配，避免被 `_acme-challenge` 前缀记录误判）
   - studio：`AddCdnDomain`：`studio.recurit.quanttide.com`，源站 OSS `qtrecurit-studio.oss-cn-hangzhou.aliyuncs.com`（type=oss, port=443）；HTTPS 上传 SAN 证书（`scripts/configure-studio-cdn.sh`）；DNS：`studio.recurit.quanttide.com` CNAME → `studio.recurit.quanttide.com.w.kunlunaq.com`

## 使用

```sh
terraform init \
  -backend-config="bucket=quanttide-terraform-state" \
  -backend-config="key=qtrecurit/terraform.tfstate" \
  -backend-config="region=cn-hangzhou"
terraform plan
terraform apply
```
