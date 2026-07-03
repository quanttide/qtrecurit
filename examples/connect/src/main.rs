//! # 招聘邮件拉取工具 (qtrecurit)
//!
//! 从 hr@example.com 邮箱按月拉取招聘相关邮件和附件，保存到 `data/YYYY-MM/` 目录。
//! 支持增量同步、断点续传、游标续扫。
//!
//! ## 用法
//!
//! ```bash
//! # 拉取 + 分类本月邮件
//! cargo run
//!
//! # 拉取 + 分类指定月份
//! cargo run 2026-06
//!
//! # 仅分类（无需拉取），基于已有 .full.json
//! cargo run classify 2026-06
//!
//! # 仅分类本月
//! cargo run classify
//! ```
//!
//! ## 增量同步机制
//!
//! 每次运行后会在 `data/YYYY-MM/.cursor` 文件中记录最后一次成功同步的时间。
//! 下次运行时只拉取该时间之后的新邮件，避免重复扫描整个月的邮件列表。
//!
//! ## 输出结构
//!
//! ```text
//! data/2026-06/
//!   .cursor                    ← 游标（最后同步时间戳）
//!   INBOX.json                 ← 收件箱元数据（列表，不含正文）
//!   INBOX.full.json            ← 收件箱完整正文
//!   INBOX.classification.json  ← 收件箱分类结果（独立文件，可手动编辑）
//!   SENT.json                  ← 已发送元数据
//!   SENT.full.json             ← 已发送完整正文
//!   SENT.classification.json   ← 已发送分类结果
//!   attachments/<msg_id>/      ← 下载的附件
//! ```

mod classifier;
mod downloader;
mod lark;

use serde_json::Value;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use classifier::Classification;
use lark::run_lark;

/// 每页拉取的邮件数量上限。
const PAGE_SIZE: u32 = 200;

/// 获取当前月份字符串 YYYY-MM。
fn current_month() -> String {
    let output = Command::new("date")
        .args(["+%Y-%m"])
        .output()
        .expect("failed to get date");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// 从 `+triage` 输出的 JSON 中提取 messages 数组。
fn extract_messages(data: &Value) -> Vec<Value> {
    data.get("messages")
        .and_then(|m| m.as_array())
        .cloned()
        .unwrap_or_default()
}

/// 从 `+triage` 输出中提取下一页的分页令牌（page_token）。
fn extract_page_token(data: &Value) -> Option<String> {
    data.get("page_token")
        .and_then(|t| t.as_str())
        .filter(|t| !t.is_empty())
        .map(String::from)
}

/// 分页拉取指定邮箱文件夹的所有邮件元数据。
///
/// 逐页请求直到服务端不再返回 `page_token`，合并所有页的消息列表。
fn fetch_all_meta(mailbox: &str, folder: &str) -> Result<Vec<Value>, String> {
    let mut all_msgs = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let page_size = PAGE_SIZE.to_string();
        let filter = format!(r#"{{"folder":"{}"}}"#, folder);
        let mut args = vec![
            "mail",
            "+triage",
            "--mailbox",
            mailbox,
            "--max",
            &page_size,
            "--filter",
            &filter,
            "--format",
            "json",
        ];
        if let Some(ref token) = page_token {
            args.push("--page-token");
            args.push(token);
        }

        let data = run_lark(&args).map_err(|e| format!("fetch page failed: {}", e))?;
        let msgs = extract_messages(&data);
        all_msgs.extend(msgs);

        match extract_page_token(&data) {
            Some(next) => page_token = Some(next),
            None => break,
        }
    }

    Ok(all_msgs)
}

/// 从元数据列表中筛选出指定年月（YYYY-MM）的消息。
fn filter_msgs(msgs: &[Value], ym: &str) -> Vec<Value> {
    msgs.iter()
        .filter(|m| {
            m.get("date")
                .and_then(|d| d.as_str())
                .map(|d| d.starts_with(ym))
                .unwrap_or(false)
        })
        .cloned()
        .collect()
}

/// 从元数据列表中筛选出在游标时间之后的消息。
fn filter_msgs_since(msgs: &[Value], since: &str) -> Vec<Value> {
    msgs.iter()
        .filter(|m| {
            m.get("date")
                .and_then(|d| d.as_str())
                .map(|d| d > since)
                .unwrap_or(false)
        })
        .cloned()
        .collect()
}

/// 从本地缓存的 `.full.json` 中读取已下载的 message_id 集合，用于增量去重。
fn load_existing_ids(base: &str, folder: &str) -> HashSet<String> {
    let path = format!("{}/{}.full.json", base, folder);
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return HashSet::new(),
    };
    let data: Value = match serde_json::from_str(&content) {
        Ok(d) => d,
        Err(_) => return HashSet::new(),
    };
    data.get("messages")
        .and_then(|m| m.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| {
                    m.get("message_id")
                        .and_then(|id| id.as_str())
                        .map(String::from)
                })
                .collect()
        })
        .unwrap_or_default()
}

/// 从本地缓存的 `.full.json` 中读取已下载的完整消息列表，用于与新消息合并。
fn load_existing_msgs(base: &str, folder: &str) -> Vec<Value> {
    let path = format!("{}/{}.full.json", base, folder);
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let data: Value = match serde_json::from_str(&content) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };
    data.get("messages")
        .and_then(|m| m.as_array())
        .cloned()
        .unwrap_or_default()
}

/// 读取游标文件，返回上次成功同步的时间戳字符串。
///
/// 游标文件 `data/YYYY-MM/.cursor` 保存了最后一次拉取邮件的日期（如 `2026-06-29`）。
/// 如果文件不存在或格式异常，返回 `ym-01`（当月第一天）作为兜底。
fn load_cursor(base: &str, ym: &str) -> String {
    let path = format!("{}/.cursor", base);
    fs::read_to_string(&path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("{}-01", ym))
}

/// 写入游标文件。
///
/// 在当前月的所有邮件处理完成后更新游标，标记已处理到的最大日期。
fn save_cursor(base: &str, max_date: &str) {
    let path = format!("{}/.cursor", base);
    fs::write(&path, max_date).ok();
}

/// 批量获取邮件的完整正文（每批最多 20 封）。
///
/// 先通过元数据拿到 message_id 列表，再逐批调用 mail +messages 获取完整内容。
fn fetch_full(mids: &[String], mailbox: &str) -> Result<Vec<Value>, String> {
    let mut all = Vec::new();
    for chunk in mids.chunks(20) {
        let ids = chunk.join(",");
        let v = run_lark(&[
            "mail",
            "+messages",
            "--mailbox",
            mailbox,
            "--format",
            "json",
            "--message-ids",
            &ids,
        ])?;
        if let Some(msgs) = v
            .get("data")
            .and_then(|d| d.get("messages"))
            .and_then(|m| m.as_array())
        {
            all.extend(msgs.iter().cloned());
        }
    }
    Ok(all)
}

/// Run the full fetch + classify pipeline for a month.
async fn run_fetch(year_month: &str) {
    let mailbox = "hr@example.com";
    let base = format!("data/{}", year_month);
    fs::create_dir_all(&base).expect("failed to create data dir");

    let cursor = load_cursor(&base, year_month);
    println!("=== {} {}（游标: {}）===", mailbox, year_month, cursor);

    let mut total_new = 0;
    let mut total_att = 0;
    let mut total_classified = 0;

    for folder in &["INBOX", "SENT"] {
        println!("\n--- {} ---", folder);

        let existing_ids = load_existing_ids(&base, folder);
        let existing_count = existing_ids.len();

        let all_meta = fetch_all_meta(mailbox, folder).expect("failed to fetch metadata");
        let month_msgs = filter_msgs(&all_meta, year_month);
        let new_msgs = filter_msgs_since(&month_msgs, &cursor);

        let new_mids: Vec<String> = new_msgs
            .iter()
            .filter_map(|m| {
                m.get("message_id")
                    .and_then(|id| id.as_str())
                    .map(String::from)
            })
            .filter(|id| !existing_ids.contains(id))
            .collect();

        if new_mids.is_empty() {
            println!("  无新邮件（已有 {} 封）", existing_count);
            // Still save classification summary if there are existing msgs
            let all_msgs = load_existing_msgs(&base, folder);
            if !all_msgs.is_empty() {
                let existing = classifier::load_classifications(&base, folder);
                let new = classifier::classify_pending(&all_msgs, &existing).await;
                if !new.is_empty() {
                    let mut all: Vec<Classification> = existing.into_values().collect();
                    all.extend(new);
                    classifier::save_classifications(&base, folder, &all);
                    total_classified += all.len();
                    println!("  分类: {} 封", all.len());
                } else {
                    println!("  全部已分类 ({} 封)", existing.len());
                }
            }
            continue;
        }

        println!("  已有 {} 封，新增 {} 封", existing_count, new_mids.len());

        let new_full = fetch_full(&new_mids, mailbox).expect("failed to fetch full content");
        total_new += new_full.len();

        let mut all_msgs = load_existing_msgs(&base, folder);
        all_msgs.extend(new_full.clone());

        // Classify only pending messages — save separately from full.json
        if !all_msgs.is_empty() {
            let existing = classifier::load_classifications(&base, folder);
            let new = classifier::classify_pending(&all_msgs, &existing).await;
            let count = new.len();
            if !new.is_empty() {
                let mut all: Vec<Classification> = existing.into_values().collect();
                let prev = all.len();
                all.extend(new);
                classifier::save_classifications(&base, folder, &all);
                total_classified += count;
                println!("  分类完成: 新增 {} 封，共 {} 封", count, prev + count);
            } else {
                println!("  全部已分类 ({} 封)", existing.len());
            }
        }

        // 保存元数据（邮件列表，不含正文）
        let meta_out =
            serde_json::json!({"folder": folder, "count": all_msgs.len(), "messages": month_msgs});
        fs::write(
            format!("{}/{}.json", base, folder),
            serde_json::to_string_pretty(&meta_out).unwrap(),
        )
        .ok();

        // 保存完整数据（含正文，不含分类）
        // Classification is stored separately in .classification.json
        let full_out =
            serde_json::json!({"folder": folder, "count": all_msgs.len(), "messages": all_msgs});
        fs::write(
            format!("{}/{}.full.json", base, folder),
            serde_json::to_string_pretty(&full_out).unwrap(),
        )
        .ok();

        // 下载新增邮件的附件
        let att_dir = Path::new(&base).join("attachments");
        let att = downloader::download_attachments(&new_full, &att_dir, mailbox);
        total_att += att;
        println!("  新增附件: {} 个", att);
    }

    let end_date = format!("{}-31", year_month);
    save_cursor(&base, &end_date);

    println!("\n=== 完成 ===");
    println!("新增邮件: {} 封", total_new);
    println!("新增附件: {} 个", total_att);
    if total_classified > 0 {
        println!("分类邮件: {} 封", total_classified);
    }
    println!("游标更新至: {}", end_date);
}

/// Run classify-only mode — no fetch, just classify pending messages from existing .full.json.
async fn run_classify(year_month: &str) {
    let base = format!("data/{}", year_month);
    println!("=== 仅分类: {} ===", year_month);

    let mut total = 0;

    for folder in &["INBOX", "SENT"] {
        let full_path = format!("{}/{}.full.json", base, folder);
        let content = match fs::read_to_string(&full_path) {
            Ok(c) => c,
            Err(_) => {
                println!("  {}: 未找到 {}，跳过", folder, full_path);
                continue;
            }
        };
        let data: Value = match serde_json::from_str(&content) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("  {}: 解析失败 (跳过): {}", folder, e);
                continue;
            }
        };
        let msgs = data
            .get("messages")
            .and_then(|m| m.as_array())
            .cloned()
            .unwrap_or_default();

        if msgs.is_empty() {
            println!("  {}: 无邮件数据", folder);
            continue;
        }

        let existing = classifier::load_classifications(&base, folder);
        let existing_count = existing.len();
        let new = classifier::classify_pending(&msgs, &existing).await;

        if new.is_empty() {
            println!("  {}: 全部已分类（{} 封）", folder, existing_count);
        } else {
            let mut all: Vec<Classification> = existing.into_values().collect();
            let prev = all.len();
            all.extend(new);
            classifier::save_classifications(&base, folder, &all);
            total += all.len() - prev;
            println!(
                "  {}: 新增 {} 封，共 {} 封",
                folder,
                all.len() - prev,
                all.len()
            );
        }
    }

    println!("\n=== 完成 ===");
    if total > 0 {
        println!("新增分类: {} 封", total);
    } else {
        println!("无需新增分类");
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse mode and year-month
    //   cargo run                → fetch + classify (current month)
    //   cargo run 2026-06        → fetch + classify
    //   cargo run classify       → classify only (current month)
    //   cargo run classify 2026-06 → classify only
    let (is_classify_only, year_month) = if args.len() > 1 && args[1] == "classify" {
        let ym = if args.len() > 2 {
            args[2].clone()
        } else {
            current_month()
        };
        (true, ym)
    } else {
        let ym = if args.len() > 1 {
            args[1].clone()
        } else {
            current_month()
        };
        (false, ym)
    };

    // Validate year_month format
    if !year_month.contains('-') || year_month.len() != 7 {
        eprintln!("错误: 年月格式应为 YYYY-MM，得到: {}", year_month);
        std::process::exit(1);
    }

    if is_classify_only {
        run_classify(&year_month).await;
    } else {
        run_fetch(&year_month).await;
    }
}
