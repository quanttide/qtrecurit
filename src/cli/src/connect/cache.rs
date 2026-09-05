//! XDG 兼容的本地缓存模块。
//!
//! 缓存目录遵循 XDG Base Directory Specification：
//! - Linux/macOS: `~/.cache/qtrecurit/`
//! - 可通过 `XDG_CACHE_HOME` 环境变量覆盖
//!
//! 缓存内容：
//! - `survey_url` - 最新的准入问卷链接

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

/// 获取缓存目录路径
pub fn cache_dir() -> Result<PathBuf> {
    // 优先使用环境变量
    if let Ok(env_dir) = std::env::var("XDG_CACHE_HOME") {
        return Ok(PathBuf::from(env_dir).join("qtrecurit"));
    }

    // 回退到默认路径
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .context("无法获取用户主目录")?;

    Ok(PathBuf::from(home).join(".cache").join("qtrecurit"))
}

/// 确保缓存目录存在
fn ensure_cache_dir() -> Result<PathBuf> {
    let dir = cache_dir()?;
    fs::create_dir_all(&dir).context(format!("创建缓存目录失败: {}", dir.display()))?;
    Ok(dir)
}

/// 读取缓存的问卷链接
pub fn get_survey_url() -> Option<String> {
    let path = cache_dir().ok()?.join("survey_url");
    fs::read_to_string(&path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// 写入问卷链接到缓存
pub fn set_survey_url(url: &str) -> Result<()> {
    let dir = ensure_cache_dir()?;
    let path = dir.join("survey_url");
    fs::write(&path, url).context(format!("写入缓存失败: {}", path.display()))?;
    Ok(())
}

/// 清除问卷链接缓存
pub fn clear_survey_url() -> Result<()> {
    let path = cache_dir()?.join("survey_url");
    if path.exists() {
        fs::remove_file(&path).context(format!("删除缓存失败: {}", path.display()))?;
    }
    Ok(())
}

// ── 文件夹 ID 缓存 ────────────────────────────────────────────────

/// 读取缓存的文件夹 ID
pub fn get_folder_id(name: &str) -> Option<String> {
    let path = cache_dir().ok()?.join(format!("folder_{}", name));
    fs::read_to_string(&path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// 写入文件夹 ID 到缓存
pub fn set_folder_id(name: &str, id: &str) -> Result<()> {
    let dir = ensure_cache_dir()?;
    let path = dir.join(format!("folder_{}", name));
    fs::write(&path, id).context(format!("写入缓存失败: {}", path.display()))?;
    Ok(())
}

/// 清除文件夹 ID 缓存
pub fn clear_folder_id(name: &str) -> Result<()> {
    let path = cache_dir()?.join(format!("folder_{}", name));
    if path.exists() {
        fs::remove_file(&path).context(format!("删除缓存失败: {}", path.display()))?;
    }
    Ok(())
}

/// 从 HR 邮箱获取指定名称的文件夹 ID
pub fn fetch_folder_id_from_email(name: &str) -> Result<String> {
    use super::email::run_lark_json;
    use serde_json::Value;

    let data: Value = run_lark_json(&[
        "mail",
        "user_mailbox.folders",
        "list",
        "--user-mailbox-id",
        "hr@quanttide.com",
        "--format",
        "json",
    ])?;

    let folders = data["data"]["items"]
        .as_array()
        .context("无法解析文件夹列表")?;

    for folder in folders {
        if let Some(folder_name) = folder["name"].as_str() {
            if folder_name == name {
                if let Some(id) = folder["id"].as_str() {
                    return Ok(id.to_string());
                }
            }
        }
    }

    anyhow::bail!("未找到名为 '{}' 的文件夹", name)
}

// ── 模板数据源缓存 ────────────────────────────────────────────────

/// 读取缓存的模板数据源 URL
pub fn get_template_source(name: &str) -> Option<String> {
    let path = cache_dir().ok()?.join(format!("template_source_{}", name));
    fs::read_to_string(&path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// 写入模板数据源 URL 到缓存
pub fn set_template_source(name: &str, url: &str) -> Result<()> {
    let dir = ensure_cache_dir()?;
    let path = dir.join(format!("template_source_{}", name));
    fs::write(&path, url).context(format!("写入缓存失败: {}", path.display()))?;
    Ok(())
}

/// 清除模板数据源缓存
pub fn clear_template_source(name: &str) -> Result<()> {
    let path = cache_dir()?.join(format!("template_source_{}", name));
    if path.exists() {
        fs::remove_file(&path).context(format!("删除缓存失败: {}", path.display()))?;
    }
    Ok(())
}

// ── 二维码图片缓存 ────────────────────────────────────────────────

/// 读取缓存的二维码图片路径
pub fn get_qr() -> Option<String> {
    let path = cache_dir().ok()?.join("invite_qr");
    fs::read_to_string(&path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// 写入二维码图片路径到缓存（同时复制图片到缓存目录）
pub fn set_qr(src_path: &str) -> Result<()> {
    let dir = ensure_cache_dir()?;
    let dest = dir.join("invite_qr.png");

    // 复制图片文件到缓存目录
    fs::copy(src_path, &dest).context(format!(
        "复制二维码图片失败: {} -> {}",
        src_path,
        dest.display()
    ))?;

    // 写入路径到缓存文件
    let path = dir.join("invite_qr");
    fs::write(&path, dest.to_str().unwrap_or_default())
        .context(format!("写入缓存失败: {}", path.display()))?;
    Ok(())
}

/// 清除二维码图片缓存
pub fn clear_qr() -> Result<()> {
    let dir = cache_dir()?;
    let path = dir.join("invite_qr");
    let img_path = dir.join("invite_qr.png");

    if img_path.exists() {
        fs::remove_file(&img_path).context(format!("删除缓存图片失败: {}", img_path.display()))?;
    }
    if path.exists() {
        fs::remove_file(&path).context(format!("删除缓存失败: {}", path.display()))?;
    }
    Ok(())
}

/// 从 HR 邮箱获取最新的问卷链接
pub fn fetch_survey_url_from_email() -> Result<String> {
    use super::email::run_lark_json;
    use serde_json::Value;

    // 搜索包含 "准入问卷" 的邮件
    let data: Value = run_lark_json(&[
        "mail",
        "+triage",
        "--mailbox",
        "hr@quanttide.com",
        "--query",
        "准入问卷",
        "--max",
        "10",
        "--format",
        "json",
    ])?;

    let messages = data["messages"].as_array().context("无法解析邮件列表")?;

    if messages.is_empty() {
        anyhow::bail!("未找到包含准入问卷的邮件");
    }

    // 遍历邮件查找问卷链接
    for msg in messages {
        let message_id = msg["message_id"].as_str().context("无法获取邮件 ID")?;

        // 获取邮件完整内容
        let full_data: Value = run_lark_json(&[
            "mail",
            "+messages",
            "--mailbox",
            "hr@quanttide.com",
            "--message-ids",
            message_id,
            "--format",
            "json",
        ])?;

        // 从正文中提取问卷链接
        if let Some(messages) = full_data["data"]["messages"].as_array() {
            for msg in messages {
                if let Some(body) = msg["body_plain_text"].as_str() {
                    // 查找飞书表单链接
                    if let Some(url) = extract_survey_url(body) {
                        return Ok(url);
                    }
                }
            }
        }
    }

    anyhow::bail!("未在邮件中找到问卷链接")
}

/// 从文本中提取问卷链接
fn extract_survey_url(text: &str) -> Option<String> {
    // 查找飞书多维表格表单链接
    let patterns = [
        "https://quanttide.feishu.cn/share/base/form/",
        "https://quanttide.larksuite.com/share/base/form/",
    ];

    for pattern in patterns {
        if let Some(start) = text.find(pattern) {
            let remaining = &text[start..];
            // 找到链接结尾（空格、换行、引号等）
            let end = remaining
                .find(|c: char| c.is_whitespace() || c == '"' || c == '>' || c == '<')
                .unwrap_or(remaining.len());
            let url = &remaining[..end];
            // 移除末尾可能的标点
            let url = url.trim_end_matches(|c: char| c == '.' || c == ')' || c == '!' || c == '。');
            if url.starts_with(pattern) {
                return Some(url.to_string());
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_survey_url() {
        let text = "请完成准入问卷：https://quanttide.feishu.cn/share/base/form/shrcn7RjQlUfhtS2PMophVyXm2j 问卷大约需要15-20分钟";
        let url = extract_survey_url(text);
        assert!(url.is_some());
        assert_eq!(
            url.unwrap(),
            "https://quanttide.feishu.cn/share/base/form/shrcn7RjQlUfhtS2PMophVyXm2j"
        );
    }

    #[test]
    fn test_extract_survey_url_with_punctuation() {
        let text = "请访问 https://quanttide.feishu.cn/share/base/form/abc123。";
        let url = extract_survey_url(text);
        assert!(url.is_some());
        assert_eq!(
            url.unwrap(),
            "https://quanttide.feishu.cn/share/base/form/abc123"
        );
    }

    #[test]
    fn test_extract_survey_url_not_found() {
        let text = "这是一段没有链接的文本";
        let url = extract_survey_url(text);
        assert!(url.is_none());
    }
}
