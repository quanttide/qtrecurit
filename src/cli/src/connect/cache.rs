//! XDG 兼容的本地缓存模块。
//!
//! 缓存目录遵循 XDG Base Directory Specification：
//! - Linux/macOS: `~/.cache/qtrecurit/`
//! - 可通过 `XDG_CACHE_HOME` 环境变量覆盖

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
