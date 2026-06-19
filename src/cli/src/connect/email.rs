use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;

use super::{EmailFetcher, Message};

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

pub struct LarkCliFetcher;

impl EmailFetcher for LarkCliFetcher {
    fn fetch_all(&self) -> Result<Vec<Message>> {
        let mut all = Vec::new();
        let mut token: Option<String> = None;

        for _ in 0..20 {
            let resp = run_lark_cli(token.as_deref())?;
            if let Some(batch) = resp.messages {
                if batch.is_empty() {
                    break;
                }
                for m in batch {
                    all.push(Message { subject: m.subject, date: m.date });
                }
            } else {
                break;
            }
            match resp.page_token {
                Some(t) if !t.is_empty() => token = Some(t),
                _ => break,
            }
        }

        Ok(all)
    }
}

fn run_lark_cli(page_token: Option<&str>) -> Result<LarkResponse> {
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

    let child = Command::new("lark-cli")
        .args(&args)
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
        .recv_timeout(Duration::from_secs(15))
        .map_err(|_| anyhow::anyhow!("lark-cli 请求超时（15s），请检查网络连接或认证状态"))?
        .context("lark-cli 进程异常退出")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("lark-cli 执行失败: {}", stderr.trim());
    }

    let data: LarkResponse =
        serde_json::from_slice(&output.stdout).context("lark-cli 返回数据格式异常")?;
    Ok(data)
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

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

    #[test]
    fn test_filter_by_date() {
        let items = vec![
            MailItem { subject: "a".into(), date: "2026-06-14".into() },
            MailItem { subject: "b".into(), date: "2026-06-15".into() },
            MailItem { subject: "c".into(), date: "2026-06-16".into() },
        ];
        let start = chrono::NaiveDate::from_ymd_opt(2026, 6, 15);
        let end = chrono::NaiveDate::from_ymd_opt(2026, 6, 15);
        let filtered = filter_by_date(&items, start, end);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].subject, "b");
    }

    #[test]
    fn test_filter_by_date_no_match() {
        let items = vec![MailItem { subject: "a".into(), date: "2026-06-14".into() }];
        let start = chrono::NaiveDate::from_ymd_opt(2026, 6, 15);
        let end = chrono::NaiveDate::from_ymd_opt(2026, 6, 15);
        let filtered = filter_by_date(&items, start, end);
        assert!(filtered.is_empty());
    }

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
        let (s, e) = resolve_date_range(
            Some("2026-06-01".into()),
            Some("2026-06-16".into()),
            None,
        );
        assert_eq!(s.unwrap().to_string(), "2026-06-01");
        assert_eq!(e.unwrap().to_string(), "2026-06-16");
    }
}
