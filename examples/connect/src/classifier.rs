//! LLM-based email classifier for recruitment emails.
//!
//! Replaces the Python `src/classify.py` with a pure-Rust implementation
//! that calls the OpenAI-compatible chat API directly via `reqwest`.
//!
//! Environment variables:
//! - `LLM_API_KEY`, `AI_REVIEW_API_KEY` or `OPENAI_API_KEY` — required
//! - `AI_REVIEW_MODEL` — defaults to `deepseek-chat`
//! - `AI_REVIEW_BASE_URL` — defaults to `https://api.deepseek.com`

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

/// A single classification record stored in `.classification.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Classification {
    pub message_id: String,
    pub classification: String,
    pub source: String,
    pub updated_at: String,
}

const CATEGORIES: &[&str] = &[
    "resume_submission",
    "interview_scheduling",
    "written_exam",
    "offer_letter",
    "hr_internal",
    "unrelated",
];

const SYSTEM_PROMPT: &str = "\
You are an email classifier for a recruitment system. \
Classify each email into exactly one category:
- resume_submission: Job applications, resumes, cover letters, internship applications
- interview_scheduling: Interview invitations, scheduling, confirmations
- written_exam: Coding tests, written exams, take-home assignments,笔试题目
- offer_letter: Offer letters, employment contracts, onboarding instructions
- hr_internal: Internal HR communications from company domain
- unrelated: Newsletters, notifications, spam, or anything not recruitment-related

Respond with ONLY a JSON array of objects, each with \"message_id\" and \"classification\" fields. \
Example:
[{\"message_id\": \"abc123\", \"classification\": \"resume_submission\"}, ...]";

/// Path to the classification file for a given folder.
fn classification_path(base: &str, folder: &str) -> String {
    format!("{}/{}.classification.json", base, folder)
}

/// Load existing classifications from disk.
///
/// Returns a map keyed by message_id. Returns an empty map if the file
/// doesn't exist or can't be parsed.
pub fn load_classifications(base: &str, folder: &str) -> HashMap<String, Classification> {
    let path = classification_path(base, folder);
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return HashMap::new(),
    };
    let list: Vec<Classification> = match serde_json::from_str(&content) {
        Ok(l) => l,
        Err(_) => return HashMap::new(),
    };
    list.into_iter()
        .map(|c| (c.message_id.clone(), c))
        .collect()
}

/// Save classifications to disk as a JSON array.
///
/// The file is a simple, flat array — easy to open in an editor and modify manually.
pub fn save_classifications(base: &str, folder: &str, classifications: &[Classification]) {
    let path = classification_path(base, folder);
    if let Ok(json) = serde_json::to_string_pretty(classifications) {
        fs::write(&path, json).ok();
    }
}

/// Classify only messages that don't already have a classification entry.
///
/// * `msgs` — all messages (both classified and unclassified)
/// * `existing` — known classifications (keyed by message_id)
///
/// Returns new `Classification` entries for messages that were classified by the LLM.
/// Messages already present in `existing` are skipped entirely.
pub async fn classify_pending(
    msgs: &[Value],
    existing: &HashMap<String, Classification>,
) -> Vec<Classification> {
    let pending: Vec<&Value> = msgs
        .iter()
        .filter(|m| {
            m.get("message_id")
                .and_then(|id| id.as_str())
                .map(|id| !existing.contains_key(id))
                .unwrap_or(false)
        })
        .collect();

    if pending.is_empty() {
        return Vec::new();
    }

    let pending_owned: Vec<Value> = pending.into_iter().cloned().collect();

    // 分批处理，每批最多 30 封，防止 LLM 输出截断
    let batch_size = 30;
    let mut all_results = Vec::new();
    for chunk in pending_owned.chunks(batch_size) {
        match classify_llm_batch(chunk).await {
            Ok(results) => {
                let now = timestamp_now();
                for (mid, label) in results {
                    all_results.push(Classification {
                        message_id: mid,
                        classification: label,
                        source: "llm".to_string(),
                        updated_at: now.clone(),
                    });
                }
            }
            Err(e) => {
                eprintln!("  分类失败 (跳过): {}", e);
            }
        }
    }
    all_results
}

/// Build a short text representation of one email for the LLM prompt.
fn email_text(email: &Value) -> String {
    let subj = email.get("subject").and_then(|s| s.as_str()).unwrap_or("");
    let sender = email
        .get("head_from")
        .and_then(|hf| hf.as_object())
        .and_then(|o| o.get("mail_address"))
        .and_then(|m| m.as_str())
        .unwrap_or("");
    let body = email
        .get("body_plain_text")
        .or_else(|| email.get("body_preview"))
        .and_then(|b| b.as_str())
        .unwrap_or("");
    // Truncate body to ~2000 Unicode characters (safe with multi-byte UTF-8)
    // ponytail: char-counting instead of byte-slicing avoids UTF-8 panics;
    // ceiling: allocates a new String on truncation, fine for email bodies.
    let body: &str = &if body.len() > 2000 {
        body.chars().take(2000).collect::<String>()
    } else {
        body.to_string()
    };
    format!("Subject: {}\nFrom: {}\n\nBody:\n{}", subj, sender, body)
}

/// Strip markdown code fences (```json ... ```) from LLM output.
/// Also handles plain JSON arrays without fences.
fn strip_fences(s: &str) -> &str {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix("```json").or_else(|| s.strip_prefix("```")) {
        if let Some(end) = inner.rfind("```") {
            return inner[..end].trim();
        }
        return inner.trim();
    }
    s
}

/// Call the LLM API and return a map of message_id → category label.
async fn classify_llm_batch(msgs: &[Value]) -> Result<HashMap<String, String>, String> {
    if msgs.is_empty() {
        return Ok(HashMap::new());
    }

    let api_key = std::env::var("LLM_API_KEY")
        .or_else(|_| std::env::var("AI_REVIEW_API_KEY"))
        .or_else(|_| std::env::var("OPENAI_API_KEY"))
        .map_err(|_| "LLM_API_KEY not set — cannot classify".to_string())?;

    let model = std::env::var("AI_REVIEW_MODEL")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "deepseek-chat".into());

    let base_url = std::env::var("AI_REVIEW_BASE_URL")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "https://api.deepseek.com".into());

    let url = format!("{}/v1/chat/completions", base_url.trim_end_matches('/'));

    // Build a single user message listing all pending emails
    // ponytail: single batch call instead of N per-email calls — efficient enough
    let mut user_content = String::from("Classify the following emails:\n\n");
    for (i, msg) in msgs.iter().enumerate() {
        let mid = msg.get("message_id").and_then(|m| m.as_str()).unwrap_or("");
        user_content.push_str(&format!("--- Email {} ---\n", i + 1));
        user_content.push_str(&format!("Message ID: {}\n", mid));
        user_content.push_str(&email_text(msg));
        user_content.push('\n');
    }

    let body = serde_json::json!({
        "model": model,
        "messages": [
            {"role": "system", "content": SYSTEM_PROMPT},
            {"role": "user", "content": user_content}
        ],
        "temperature": 0.0,
        "max_tokens": 4096,
    });

    let client = reqwest::Client::new();
    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("LLM API request failed: {}", e))?;

    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| format!("read response body failed: {}", e))?;

    if !status.is_success() {
        return Err(format!("LLM API error {}: {}", status, text));
    }

    let data: Value =
        serde_json::from_str(&text).map_err(|e| format!("parse LLM response failed: {}", e))?;

    let content = data
        .pointer("/choices/0/message/content")
        .and_then(|c| c.as_str())
        .ok_or_else(|| {
            format!(
                "unexpected LLM response structure — missing choices[0].message.content\nraw: {}",
                text
            )
        })?;

    // 剥离 markdown 代码块标记（LLM 经常输出 ```json ... ```）
    let content = strip_fences(content);

    let classifications: Vec<Value> = serde_json::from_str(content).map_err(|e| {
        format!(
            "LLM output is not valid JSON array: {}\nraw: {}",
            e, content
        )
    })?;

    let mut map = HashMap::new();
    for entry in &classifications {
        let mid = entry
            .get("message_id")
            .and_then(|m| m.as_str())
            .unwrap_or("");
        let label = entry
            .get("classification")
            .and_then(|c| c.as_str())
            .unwrap_or("unrelated")
            .to_lowercase();
        let label = if CATEGORIES.contains(&label.as_str()) {
            label
        } else {
            "unrelated".to_string()
        };
        map.insert(mid.to_string(), label);
    }
    Ok(map)
}

/// Get current timestamp string (YYYY-MM-DD HH:MM).
///
/// ponytail: avoids pulling in `chrono` — shell `date` is sufficient
/// for a display-only timestamp. Ceiling: no timezone awareness; if
/// cross-timezone consistency is needed, switch to `chrono::Utc::now()`.
fn timestamp_now() -> String {
    let output = std::process::Command::new("date")
        .args(["+%Y-%m-%d %H:%M"])
        .output()
        .expect("failed to get date");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
