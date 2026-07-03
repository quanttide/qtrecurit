//! 附件下载模块。
//!
//! 通过 Lark 邮件 API 获取附件临时下载链接，用 curl 下载到本地。
//! 支持断点续传：先写 `.tmp` 文件，完成后重命名，避免中断产生残文件。

use std::fs;
use std::path::Path;
use std::process::Command;

use crate::lark::run_lark;
use serde_json::Value;

/// 下载邮件附件到 `attachments/<message_id>/` 目录。
///
/// - 跳过内联附件（如签名图）
/// - 跳过已存在的文件（支持断点续传）
/// - 先下载到临时文件（`.tmp`），完成后重命名，防止中断产生残文件
/// - 通过 mail attachment download_url 接口获取临时下载链接，再用 curl 下载
pub fn download_attachments(msgs: &[Value], att_dir: &Path, mailbox: &str) -> u32 {
    let mut count = 0;
    for msg in msgs {
        let mid = match msg.get("message_id").and_then(|m| m.as_str()) {
            Some(id) => id,
            None => continue,
        };
        let attachments = match msg.get("attachments").and_then(|a| a.as_array()) {
            Some(a) => a,
            None => continue,
        };
        for att in attachments {
            // 跳过内联附件（如邮件中嵌入的图片）
            if att
                .get("is_inline")
                .and_then(|i| i.as_bool())
                .unwrap_or(false)
            {
                continue;
            }
            let aid = match att.get("id").and_then(|a| a.as_str()) {
                Some(id) => id,
                None => continue,
            };
            let fname = match att.get("filename").and_then(|f| f.as_str()) {
                Some(f) => f,
                None => continue,
            };
            let outdir = att_dir.join(mid);
            fs::create_dir_all(&outdir).ok();
            let outpath = outdir.join(fname);
            // 已下载完成，跳过
            if outpath.exists() {
                continue;
            }
            // 临时文件路径（下载中），防止中断产生不完整的"已完成"文件
            let tmppath = outdir.join(format!("{}.tmp", fname));
            // 获取临时下载链接
            if let Ok(url_data) = run_lark(&[
                "mail",
                "user_mailbox.message.attachments",
                "download_url",
                "--user-mailbox-id",
                mailbox,
                "--message-id",
                mid,
                "--attachment-ids",
                aid,
            ]) {
                if let Some(url) = url_data
                    .get("data")
                    .and_then(|d| d.get("download_urls"))
                    .and_then(|u| u.as_array())
                    .and_then(|arr| arr.first())
                    .and_then(|u| u.get("download_url"))
                    .and_then(|u| u.as_str())
                {
                    // 先下载到 .tmp 文件
                    let _ = Command::new("curl")
                        .args(["-s", "-o", &tmppath.to_string_lossy(), url])
                        .output();
                    // 下载完成后重命名，确保不会产生不完整的文件
                    if tmppath.exists() {
                        if let Ok(meta) = fs::metadata(&tmppath) {
                            if meta.len() > 0 {
                                fs::rename(&tmppath, &outpath).ok();
                                println!("  ↓ {}", fname);
                                count += 1;
                            } else {
                                // 空文件说明下载失败，清理临时文件
                                fs::remove_file(&tmppath).ok();
                            }
                        }
                    }
                }
            }
        }
    }
    count
}
