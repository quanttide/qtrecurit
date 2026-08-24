use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;
 
/// lark-cli 命令超时时间（秒），可根据网络状况调整
pub const LARK_TIMEOUT_SECS: u64 = 60;

use serde_json::Value;

use super::{EmailFetcher, Message};

// ── LarkResponse 用于 fetch_all ─────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct LarkResponse {
    messages: Option<Vec<LarkMessage>>,
    page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LarkMessage {
    #[serde(default)]
    subject: String,
    #[serde(default)]
    date: String,
}

// ── EmailFetcher impl ───────────────────────────────────────────────────

pub struct LarkCliFetcher;

impl EmailFetcher for LarkCliFetcher {
    fn fetch_all(&self) -> Result<Vec<Message>> {
        let mut all = Vec::new();
        let mut token: Option<String> = None;

        loop {
            let resp = run_lark_triage(token.as_deref())?;
            match resp.messages {
                Some(ref msgs) if msgs.is_empty() => break,
                Some(msgs) => {
                    for m in msgs {
                        all.push(Message {
                            subject: m.subject,
                            date: m.date,
                        });
                    }
                }
                None => break,
            }
            match resp.page_token {
                Some(t) if !t.is_empty() => token = Some(t),
                _ => break,
            }
        }

        Ok(all)
    }
}

fn run_lark_triage(page_token: Option<&str>) -> Result<LarkResponse> {
    let mut args = vec![
        "mail",
        "+triage",
        "--mailbox",
        "hr@quanttide.com",
        "--max",
        "50",
        "--format",
        "json",
    ];
    if let Some(token) = page_token {
        args.extend(["--page-token", token]);
    }
    run_lark_cli(&args)
}

/// 运行 lark-cli 命令并解析返回的 LarkResponse JSON。
fn run_lark_cli<T: serde::de::DeserializeOwned>(args: &[&str]) -> Result<T> {
    let output = run_lark_raw(args)?;
    Ok(serde_json::from_slice(&output.stdout).context("lark-cli 返回数据格式异常")?)
}

// ── 通用 lark-cli 调用 ─────────────────────────────────────────────────

/// 调用 lark-cli 命令，返回解析后的 JSON Value。
pub fn run_lark_json(args: &[&str]) -> Result<Value> {
    let output = run_lark_raw(args)?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let filtered: String = stdout
        .lines()
        .filter(|l| !l.starts_with("tip:"))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(serde_json::from_str(&filtered).context("lark-cli 返回数据格式异常")?)
}

/// 调用 lark-cli 命令，返回原始输出。
fn run_lark_raw(args: &[&str]) -> Result<std::process::Output> {
    let child = Command::new("lark-cli")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("无法启动 lark-cli，请确认已安装并完成登录")?;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let result = child.wait_with_output();
        let _ = tx.send(result);
    });

    let output = rx
        .recv_timeout(Duration::from_secs(LARK_TIMEOUT_SECS))
        .map_err(|_| anyhow::anyhow!("lark-cli 请求超时，请检查网络连接或认证状态"))?
        .context("lark-cli 进程异常退出")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("lark-cli 执行失败: {}", stderr.trim());
    }

    Ok(output)
}

// ── 发送通道（发件方向，复用 lark-cli 封装）─────────────────────────────

/// 发送邮件。默认生成草稿；confirm_send 时确认后直接发送。
/// 返回 (draft_id 或 message_id, 是否实际发送)。
///
/// 发送日志（元数据）由通道内部写入，业务命令不感知。
pub fn send_mail(
    to: &str,
    subject: &str,
    body: &str,
    attach: Option<&str>,
    template: &str,
    confirm_send: bool,
    dry_run: bool,
) -> Result<(String, bool)> {
    let mut args = vec![
        "mail",
        "+send",
        "--to",
        to,
        "--subject",
        subject,
        "--body",
        body,
        "--mailbox",
        "hr@quanttide.com",
    ];

    if let Some(att) = attach {
        args.extend(["--attach", att]);
    }
    if confirm_send {
        args.push("--confirm-send");
    }
    args.extend(["--as", "user", "--format", "json"]);

    if dry_run {
        eprintln!("[dry-run] lark-cli {}", args.join(" "));
        return Ok(("dry-run".to_string(), false));
    }

    let output = run_lark_raw(&args)?;
    let data: Value =
        serde_json::from_slice(&output.stdout).context("lark-cli +send 返回数据格式异常")?;

    let id = data["data"]["draft_id"]
        .as_str()
        .or_else(|| data["data"]["message_id"].as_str())
        .unwrap_or("")
        .to_string();

    // 通道内部写发送日志（fail-closed 语义：写失败显式警告，不阻塞发送）
    let status = if confirm_send { "sent" } else { "draft" };
    let entry = SendLogEntry {
        time: chrono::Local::now().to_rfc3339(),
        to: to.to_string(),
        subject: subject.to_string(),
        template: template.to_string(),
        status: status.to_string(),
        draft_id: id.clone(),
        note: None,
    };
    if let Err(e) = append_send_log(None, &entry) {
        eprintln!("警告: 发送日志写入失败（发送本身已成功）: {e:#}");
        eprintln!("请手动补记: {}", serde_json::to_string(&entry).unwrap_or_default());
    }

    Ok((id, confirm_send))
}

/// 发送已存在的草稿（+draft-send）
pub fn send_draft(draft_id: &str, dry_run: bool) -> Result<()> {
    let args = [
        "mail",
        "+draft-send",
        "--draft-id",
        draft_id,
        "--as",
        "user",
        "--format",
        "json",
    ];
    if dry_run {
        eprintln!("[dry-run] lark-cli {}", args.join(" "));
        return Ok(());
    }
    let output = run_lark_raw(&args)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("草稿发送失败: {}", stderr.trim());
    }
    Ok(())
}

// ── 发送日志（只记元数据，不记正文）──────────────────────────────────────

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SendLogEntry {
    pub time: String,
    pub to: String,
    pub subject: String,
    pub template: String,
    pub status: String,
    pub draft_id: String,
    pub note: Option<String>,
}

pub fn default_log_dir() -> PathBuf {
    std::env::var("SEND_LOG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".quanttide/logs"))
}

/// 追加一条发送日志（fail-closed：写入失败不静默）
pub fn append_send_log(log_file: Option<&str>, entry: &SendLogEntry) -> Result<()> {
    let path = match log_file {
        Some(f) => PathBuf::from(f),
        None => {
            let dir = default_log_dir();
            std::fs::create_dir_all(&dir).context("创建日志目录失败")?;
            dir.join("send.log")
        }
    };
    let line = serde_json::to_string(entry).context("序列化日志失败")?;
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .context("打开发送日志失败")?;
    writeln!(f, "{}", line).context("写入发送日志失败")?;
    Ok(())
}

/// 读取最近 N 条发送日志
pub fn read_send_log(log_file: Option<&str>, tail: usize) -> Result<Vec<SendLogEntry>> {
    let path = match log_file {
        Some(f) => PathBuf::from(f),
        None => default_log_dir().join("send.log"),
    };
    let content = std::fs::read_to_string(&path).context("读取发送日志失败")?;
    let mut entries: Vec<SendLogEntry> = content
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let start = entries.len().saturating_sub(tail);
    entries.drain(..start);
    Ok(entries)
}

// ── 邮件拉取管道 ──────────────────────────────────────────────────

/// 分页拉取指定邮箱文件夹的所有邮件元数据，返回 JSON Value 列表。
///
/// 每页 `page_size` 封，逐页请求直到服务端不再返回 `page_token`。
pub fn fetch_all_meta(mailbox: &str, folder: &str, page_size: u32) -> Result<Vec<Value>> {
    let mut all_msgs = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let page_size = page_size.to_string();
        let filter = format!(r#"{{"folder":"{folder}"}}"#);
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

        let data: Value =
            run_lark_cli(&args).map_err(|e| anyhow::anyhow!("fetch page failed: {e}"))?;
        let msgs = extract_messages(&data);
        all_msgs.extend(msgs);

        match extract_page_token(&data) {
            Some(next) => page_token = Some(next),
            None => break,
        }
    }

    Ok(all_msgs)
}

/// 批量获取邮件的完整正文（每批最多 20 封）。
///
/// 先通过元数据拿到 message_id 列表，再逐批调用 mail +messages 获取完整内容。
pub fn fetch_full(mids: &[String], mailbox: &str) -> Result<Vec<Value>> {
    let mut all = Vec::new();
    for chunk in mids.chunks(20) {
        let ids = chunk.join(",");
        let args = [
            "mail",
            "+messages",
            "--mailbox",
            mailbox,
            "--format",
            "json",
            "--message-ids",
            &ids,
        ];
        let v: Value =
            run_lark_cli(&args).map_err(|e| anyhow::anyhow!("fetch full failed: {e}"))?;
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

// ── 邮件过滤 ──────────────────────────────────────────────────────

/// 从 `+triage` 输出的 JSON 中提取 messages 数组。
fn extract_messages(data: &Value) -> Vec<Value> {
    data.get("messages")
        .and_then(|m| m.as_array())
        .cloned()
        .unwrap_or_default()
}

/// 从 `+triage` 输出中提取下一页的分页令牌。
fn extract_page_token(data: &Value) -> Option<String> {
    data.get("page_token")
        .and_then(|t| t.as_str())
        .filter(|t| !t.is_empty())
        .map(String::from)
}

/// 从元数据列表中筛选出指定年月（YYYY-MM）的消息。
pub fn filter_msgs_by_ym(msgs: &[Value], ym: &str) -> Vec<Value> {
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
pub fn filter_msgs_since(msgs: &[Value], since: &str) -> Vec<Value> {
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

// ── 缓存与游标 ─────────────────────────────────────────────────────

/// 从本地缓存的 `.full.json` 中读取已下载的 message_id 集合，用于增量去重。
pub fn load_existing_ids(base: &str, folder: &str) -> HashSet<String> {
    let path = format!("{base}/{folder}.full.json");
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
pub fn load_existing_msgs(base: &str, folder: &str) -> Vec<Value> {
    let path = format!("{base}/{folder}.full.json");
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
/// 游标文件 `<base>/.cursor` 保存了最后一次拉取邮件的日期（如 `2026-06-29`）。
/// 如果文件不存在或格式异常，返回 `{ym}-01`（当月第一天）作为兜底。
pub fn load_cursor(base: &str, ym: &str) -> String {
    let path = format!("{base}/.cursor");
    fs::read_to_string(&path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| format!("{ym}-01"))
}

/// 写入游标文件。
pub fn save_cursor(base: &str, max_date: &str) {
    let path = format!("{base}/.cursor");
    fs::write(&path, max_date).ok();
}

/// 确保数据目录存在。
pub fn ensure_data_dir(base: &str) {
    fs::create_dir_all(base).ok();
}

/// 保存邮件元数据（列表，不含正文）到 `<base>/<folder>.json`。
pub fn save_meta(base: &str, folder: &str, msgs: &[Value]) {
    let out = serde_json::json!({"folder": folder, "count": msgs.len(), "messages": msgs});
    fs::write(
        format!("{base}/{folder}.json"),
        serde_json::to_string_pretty(&out).unwrap(),
    )
    .ok();
}

/// 保存完整邮件数据（含正文）到 `<base>/<folder>.full.json`。
pub fn save_full(base: &str, folder: &str, msgs: &[Value]) {
    let out = serde_json::json!({"folder": folder, "count": msgs.len(), "messages": msgs});
    fs::write(
        format!("{base}/{folder}.full.json"),
        serde_json::to_string_pretty(&out).unwrap(),
    )
    .ok();
}

// ── 原有工具函数 ─────────────────────────────────────────────────────

pub fn extract_date(date_str: &str) -> Option<chrono::NaiveDate> {
    if date_str.is_empty() {
        return None;
    }

    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_str) {
        return Some(dt.date_naive());
    }

    if let Ok(d) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Some(d);
    }

    let re = regex::Regex::new(r"(\d{4}-\d{2}-\d{2})").ok()?;
    let cap = re.find(date_str)?;
    chrono::NaiveDate::parse_from_str(cap.as_str(), "%Y-%m-%d").ok()
}

pub struct MailItem {
    pub subject: String,
    pub date: String,
}

pub fn filter_by_date<'a>(
    items: &'a [MailItem],
    start: Option<chrono::NaiveDate>,
    end: Option<chrono::NaiveDate>,
) -> Vec<&'a MailItem> {
    items
        .iter()
        .filter(|m| {
            let date = extract_date(&m.date);
            match (date, start, end) {
                (Some(d), Some(s), Some(e)) => d >= s && d <= e,
                (Some(d), Some(s), None) => d >= s,
                (Some(d), None, Some(e)) => d <= e,
                (Some(_), None, None) => true,
                (None, _, _) => false,
            }
        })
        .collect()
}

/// 将 CLI 日期参数解析为日期范围
pub fn resolve_date_range(
    start: Option<String>,
    end: Option<String>,
    days: Option<u32>,
) -> (Option<chrono::NaiveDate>, Option<chrono::NaiveDate>) {
    use chrono::Datelike;

    if let (Some(start), Some(end)) = (&start, &end) {
        let s = chrono::NaiveDate::parse_from_str(start, "%Y-%m-%d").ok();
        let e = chrono::NaiveDate::parse_from_str(end, "%Y-%m-%d").ok();
        return (s, e);
    }

    if let Some(days) = days {
        let end = chrono::Local::now().date_naive();
        let start = end - chrono::Duration::days(days as i64);
        return (Some(start), Some(end));
    }

    let now = chrono::Local::now().date_naive();
    let start = chrono::NaiveDate::from_ymd_opt(now.year(), now.month(), 1).unwrap_or(now);
    (Some(start), Some(now))
}

// ── 投递邮件搜索与归档 ─────────────────────────────────────────────

/// 搜索候选人邮箱对应的投递邮件，返回最新的 message_id
pub fn find_candidate_submission(email: &str) -> Result<Option<String>> {
    let data: Value = run_lark_json(&[
        "mail", "+triage",
        "--mailbox", "hr@quanttide.com",
        "--max", "20",
        "--format", "json",
    ])?;
    
    let messages = data["messages"]
        .as_array()
        .context("无法解析邮件列表")?;
    
    // 从发件人邮箱匹配候选人的投递邮件
    for msg in messages {
        if let Some(from) = msg["from"].as_str() {
            if from == email {
                if let Some(message_id) = msg["message_id"].as_str() {
                    return Ok(Some(message_id.to_string()));
                }
            }
        }
    }
    
    Ok(None)
}

/// 移动邮件到指定文件夹，并标记为已读
pub fn move_message_to_folder(message_id: &str, folder_id: &str, dry_run: bool) -> Result<()> {
    let data = serde_json::json!({
        "add_folder": folder_id,
        "remove_label_ids": ["UNREAD"]
    });
    let data_str = data.to_string();
    
    let args = vec![
        "mail", "user_mailbox.messages", "modify",
        "--message-id", message_id,
        "--user-mailbox-id", "hr@quanttide.com",
        "--data", &data_str,
        "--as", "user",
        "--format", "json",
    ];
    
    if dry_run {
        eprintln!("[dry-run] lark-cli {}", args.join(" "));
        return Ok(());
    }
    
    let _output = run_lark_raw(&args)?;
    Ok(())
}

/// 验证邮件是否成功发送，返回验证结果
pub fn verify_sent_mail(to: &str, subject: &str) -> Result<VerifyResult> {
    let data: Value = run_lark_json(&[
        "mail", "+triage",
        "--mailbox", "hr@quanttide.com",
        "--filter", r#"{"folder":"SENT"}"#,
        "--max", "10",
        "--format", "json",
    ])?;
    
    let messages = data["messages"]
        .as_array()
        .context("无法解析邮件列表")?;
    
    for msg in messages {
        let msg_subject = msg["subject"].as_str().unwrap_or("");
        let msg_to = msg["to"].as_str().unwrap_or("");
        let message_id = msg["message_id"].as_str().unwrap_or("");
        
        if msg_subject == subject && msg_to == to {
            return Ok(VerifyResult {
                success: true,
                message_id: message_id.to_string(),
                message: format!("邮件已发送，message_id: {}", message_id),
            });
        }
    }
    
    Ok(VerifyResult {
        success: false,
        message_id: String::new(),
        message: "未在已发送邮件中找到匹配的邮件".to_string(),
    })
}

/// 邮件验证结果
#[derive(Debug)]
pub struct VerifyResult {
    pub success: bool,
    pub message_id: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    // ── extract_date ──

    #[test]
    fn test_extract_date_iso8601() {
        let d = extract_date("2026-06-15T10:30:00+08:00");
        assert!(d.is_some());
        assert_eq!(d.unwrap().to_string(), "2026-06-15");
    }

    #[test]
    fn test_extract_date_ymd() {
        let d = extract_date("2026-06-15");
        assert!(d.is_some());
        assert_eq!(d.unwrap().to_string(), "2026-06-15");
    }

    #[test]
    fn test_extract_date_empty() {
        assert!(extract_date("").is_none());
    }

    #[test]
    fn test_extract_date_regex_fallback() {
        let d = extract_date("some text 2026-06-15 more text");
        assert!(d.is_some());
        assert_eq!(d.unwrap().to_string(), "2026-06-15");
    }

    // ── filter_by_date ──

    #[test]
    fn test_filter_by_date() {
        let items = vec![
            MailItem {
                subject: "a".into(),
                date: "2026-06-14".into(),
            },
            MailItem {
                subject: "b".into(),
                date: "2026-06-15".into(),
            },
            MailItem {
                subject: "c".into(),
                date: "2026-06-16".into(),
            },
        ];
        let start = chrono::NaiveDate::from_ymd_opt(2026, 6, 15);
        let end = chrono::NaiveDate::from_ymd_opt(2026, 6, 15);
        let filtered = filter_by_date(&items, start, end);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].subject, "b");
    }

    #[test]
    fn test_filter_by_date_no_match() {
        let items = vec![MailItem {
            subject: "a".into(),
            date: "2026-06-14".into(),
        }];
        let start = chrono::NaiveDate::from_ymd_opt(2026, 6, 15);
        let end = chrono::NaiveDate::from_ymd_opt(2026, 6, 15);
        let filtered = filter_by_date(&items, start, end);
        assert!(filtered.is_empty());
    }

    // ── resolve_date_range ──

    #[test]
    fn test_resolve_date_range_default_this_month() {
        let (s, e) = resolve_date_range(None, None, None);
        assert!(s.is_some());
        assert!(e.is_some());
        let now = chrono::Local::now().date_naive();
        assert_eq!(s.unwrap().month(), now.month());
        assert_eq!(s.unwrap().year(), now.year());
        assert_eq!(s.unwrap().day(), 1);
    }

    #[test]
    fn test_resolve_date_range_with_days() {
        let (s, e) = resolve_date_range(None, None, Some(7));
        assert!(s.is_some());
        assert!(e.is_some());
        let diff = e.unwrap().signed_duration_since(s.unwrap()).num_days();
        assert_eq!(diff, 7);
    }

    #[test]
    fn test_resolve_date_range_explicit() {
        let (s, e) = resolve_date_range(Some("2026-06-01".into()), Some("2026-06-16".into()), None);
        assert_eq!(s.unwrap().to_string(), "2026-06-01");
        assert_eq!(e.unwrap().to_string(), "2026-06-16");
    }

    // ── filter_msgs_by_ym ──

    #[test]
    fn test_filter_msgs_by_ym() {
        let msgs: Vec<Value> = serde_json::from_str(
            r#"[
            {"message_id": "1", "date": "2026-06-15"},
            {"message_id": "2", "date": "2026-07-01"},
            {"message_id": "3", "date": "2026-06-30"}
        ]"#,
        )
        .unwrap();
        let filtered = filter_msgs_by_ym(&msgs, "2026-06");
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_msgs_by_ym_no_match() {
        let msgs: Vec<Value> = serde_json::from_str(
            r#"[
            {"message_id": "1", "date": "2026-07-01"}
        ]"#,
        )
        .unwrap();
        let filtered = filter_msgs_by_ym(&msgs, "2026-06");
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_filter_msgs_by_ym_no_date_field() {
        let msgs: Vec<Value> = serde_json::from_str(
            r#"[
            {"message_id": "1"}
        ]"#,
        )
        .unwrap();
        let filtered = filter_msgs_by_ym(&msgs, "2026-06");
        assert!(filtered.is_empty());
    }

    // ── filter_msgs_since ──

    #[test]
    fn test_filter_msgs_since() {
        let msgs: Vec<Value> = serde_json::from_str(
            r#"[
            {"message_id": "1", "date": "2026-06-15"},
            {"message_id": "2", "date": "2026-06-20"},
            {"message_id": "3", "date": "2026-06-25"}
        ]"#,
        )
        .unwrap();
        let filtered = filter_msgs_since(&msgs, "2026-06-20");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["message_id"], "3");
    }

    #[test]
    fn test_filter_msgs_since_all_after() {
        let msgs: Vec<Value> = serde_json::from_str(
            r#"[
            {"message_id": "1", "date": "2026-06-20"},
            {"message_id": "2", "date": "2026-06-21"}
        ]"#,
        )
        .unwrap();
        // "since" is exclusive: date > since
        let filtered = filter_msgs_since(&msgs, "2026-06-19");
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_msgs_since_none_after() {
        let msgs: Vec<Value> = serde_json::from_str(
            r#"[
            {"message_id": "1", "date": "2026-06-15"}
        ]"#,
        )
        .unwrap();
        let filtered = filter_msgs_since(&msgs, "2026-06-20");
        assert!(filtered.is_empty());
    }

    // ── cursor ──

    #[test]
    fn test_load_cursor_file_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let cursor = load_cursor(dir.path().to_str().unwrap(), "2026-06");
        assert_eq!(cursor, "2026-06-01");
    }

    #[test]
    fn test_save_and_load_cursor() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();
        save_cursor(base, "2026-06-30");
        let cursor = load_cursor(base, "2026-06");
        assert_eq!(cursor, "2026-06-30");
    }

    #[test]
    fn test_save_cursor_overwrites() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();
        save_cursor(base, "2026-06-15");
        save_cursor(base, "2026-06-30");
        assert_eq!(load_cursor(base, "2026-06"), "2026-06-30");
    }

    // ── cache helpers ──

    #[test]
    fn test_load_existing_ids_no_file() {
        let dir = tempfile::tempdir().unwrap();
        let ids = load_existing_ids(dir.path().to_str().unwrap(), "INBOX");
        assert!(ids.is_empty());
    }

    #[test]
    fn test_load_existing_ids_with_file() {
        let dir = tempfile::tempdir().unwrap();
        let base = dir.path().to_str().unwrap();
        save_full(base, "INBOX", &[]); // creates INBOX.full.json
        let ids = load_existing_ids(base, "INBOX");
        assert!(ids.is_empty());
    }

    #[test]
    fn test_ensure_data_dir_creates() {
        let dir = tempfile::tempdir().unwrap();
        let base = format!("{}/new_dir", dir.path().to_str().unwrap());
        ensure_data_dir(&base);
        assert!(std::path::Path::new(&base).exists());
    }

    // ── extract_messages / extract_page_token ──

    #[test]
    fn test_extract_messages_empty() {
        let data: Value = serde_json::json!({});
        assert!(extract_messages(&data).is_empty());
    }

    #[test]
    fn test_extract_page_token_none() {
        let data: Value = serde_json::json!({});
        assert!(extract_page_token(&data).is_none());
    }

    #[test]
    fn test_extract_page_token_non_empty() {
        let data: Value = serde_json::json!({"page_token": "abc123"});
        assert_eq!(extract_page_token(&data), Some("abc123".into()));
    }

    #[test]
    fn test_extract_page_token_empty_string() {
        let data: Value = serde_json::json!({"page_token": ""});
        assert!(extract_page_token(&data).is_none());
    }
}
