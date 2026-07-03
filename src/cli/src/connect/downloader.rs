//! 附件下载模块。
//!
//! 通过 Lark 邮件 API 获取附件临时下载链接，用 curl 下载到本地。
//! 先写 `.tmp` 文件，完成后重命名，避免中断产生残文件。

use std::fs;
use std::path::Path;

use serde_json::Value;

use super::email::run_lark_json;

/// 下载邮件附件到 `attachments/<message_id>/` 目录。
///
/// - 跳过内联附件（如签名图）
/// - 跳过已存在的文件
/// - 先下载到临时文件（`.tmp`），完成后重命名，防止中断产生残文件
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
            if outpath.exists() {
                continue;
            }
            // 临时文件路径，防中断产生不完整文件
            let tmppath = outdir.join(format!("{fname}.tmp"));

            let url_data = match run_lark_json(&[
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
                Ok(d) => d,
                Err(_) => continue,
            };
            let url = match url_data
                .get("data")
                .and_then(|d| d.get("download_urls"))
                .and_then(|u| u.as_array())
                .and_then(|arr| arr.first())
                .and_then(|u| u.get("download_url"))
                .and_then(|u| u.as_str())
            {
                Some(u) => u,
                None => continue,
            };

            // 先下载到 .tmp 文件
            let _ = std::process::Command::new("curl")
                .args(["-s", "-o", &tmppath.to_string_lossy(), url])
                .output();
            // 下载完成后重命名
            if tmppath.exists() {
                if let Ok(meta) = fs::metadata(&tmppath) {
                    if meta.len() > 0 {
                        fs::rename(&tmppath, &outpath).ok();
                        count += 1;
                    } else {
                        fs::remove_file(&tmppath).ok();
                    }
                }
            }
        }
    }
    count
}
