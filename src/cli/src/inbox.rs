//! 收件箱同步：拉取 HR 邮箱新投递，输出候选人最小字段。

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use clap::{Args, Subcommand, ValueEnum};
use encoding_rs::{Encoding, GB18030};
use serde::Serialize;
use serde_json::Value;

use crate::connect::{config, email};

const DEFAULT_MAILBOX: &str = "hr@quanttide.com";
const DEFAULT_FOLDER: &str = "INBOX";
const DEFAULT_PAGE_SIZE: u32 = 50;

#[derive(Args)]
pub struct InboxArgs {
    #[command(subcommand)]
    pub action: InboxAction,
}

#[derive(Subcommand)]
pub enum InboxAction {
    /// 同步 HR 收件箱中的新投递邮件
    Sync(InboxSyncArgs),
    /// 下载单个简历附件到本地缓存
    Resume(InboxResumeArgs),
}

#[derive(Args, Clone)]
pub struct InboxSyncArgs {
    /// 邮箱地址
    #[arg(long, default_value = DEFAULT_MAILBOX)]
    pub mailbox: String,

    /// 邮件文件夹
    #[arg(long, default_value = DEFAULT_FOLDER)]
    pub folder: String,

    /// 每页拉取邮件数量
    #[arg(long, default_value_t = DEFAULT_PAGE_SIZE)]
    pub page_size: u32,

    /// 缓存目录，默认使用 XDG cache 下的 qtrecurit/inbox
    #[arg(long)]
    pub cache_dir: Option<String>,

    /// 预览命令语义，不读取邮箱、不写缓存
    #[arg(long)]
    pub dry_run: bool,

    /// 输出格式
    #[arg(long, value_enum, default_value_t = InboxOutputFormat::Text)]
    pub format: InboxOutputFormat,
}

#[derive(Args, Clone)]
pub struct InboxResumeArgs {
    /// 邮箱地址
    #[arg(long, default_value = DEFAULT_MAILBOX)]
    pub mailbox: String,

    /// 邮件 ID
    #[arg(long)]
    pub message_id: String,

    /// 附件 ID
    #[arg(long)]
    pub attachment_id: String,

    /// 原始文件名，用于保留 PDF/Word 扩展名
    #[arg(long)]
    pub file_name: String,

    /// 缓存目录，默认使用 XDG cache 下的 qtrecurit/inbox
    #[arg(long)]
    pub cache_dir: Option<String>,

    /// 输出格式
    #[arg(long, value_enum, default_value_t = InboxOutputFormat::Text)]
    pub format: InboxOutputFormat,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum InboxOutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct InboxResumeAttachment {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub attachment_id: String,
    pub file_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub content_type: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct InboxResumeResult {
    pub status: String,
    pub path: String,
    pub file_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct InboxCandidate {
    pub id: String,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub subject: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub body: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub position: String,
    pub stage: String,
    pub status: String,
    pub has_resume: bool,
    pub has_cover_letter: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resume_attachments: Vec<InboxResumeAttachment>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub source_message_id: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct InboxSyncResult {
    pub status: String,
    pub mailbox: String,
    pub folder: String,
    pub scanned: usize,
    pub imported: usize,
    pub candidates: Vec<InboxCandidate>,
}

pub fn run(args: &InboxArgs) -> Result<()> {
    match &args.action {
        InboxAction::Sync(sync_args) => {
            let result = sync(sync_args)?;
            print_result(&result, sync_args.format)?;
        }
        InboxAction::Resume(resume_args) => {
            let result = download_resume(resume_args)?;
            print_resume_result(&result, resume_args.format)?;
        }
    }
    Ok(())
}

pub fn download_resume(args: &InboxResumeArgs) -> Result<InboxResumeResult> {
    let file_name = safe_file_name(&args.file_name)?;
    if !is_resume_file(&file_name) {
        anyhow::bail!("unsupported resume file type");
    }
    let base = resolve_resume_cache_dir(args.cache_dir.as_deref())?;
    let key = stable_id(&format!("{}:{}", args.message_id, args.attachment_id));
    let outdir = PathBuf::from(base).join("resume-files").join(key);
    fs::create_dir_all(&outdir).context("create resume cache directory failed")?;
    let outpath = outdir.join(&file_name);

    if !outpath.exists() || fs::metadata(&outpath).map(|meta| meta.len()).unwrap_or(0) == 0 {
        let download_url = fetch_attachment_download_url(args)?;
        let tmppath = outdir.join(format!("{file_name}.tmp"));
        let output = Command::new("curl")
            .arg("-fL")
            .arg("-sS")
            .arg("-o")
            .arg(&tmppath)
            .arg(&download_url)
            .output()
            .context("download resume attachment failed")?;
        if !output.status.success() {
            let _ = fs::remove_file(&tmppath);
            anyhow::bail!("download resume attachment failed");
        }
        let size = fs::metadata(&tmppath).map(|meta| meta.len()).unwrap_or(0);
        if size == 0 {
            let _ = fs::remove_file(&tmppath);
            anyhow::bail!("downloaded resume attachment is empty");
        }
        fs::rename(&tmppath, &outpath).context("save resume attachment failed")?;
    }

    let size_bytes = fs::metadata(&outpath).ok().map(|meta| meta.len());
    Ok(InboxResumeResult {
        status: "downloaded".to_string(),
        path: path_to_string(outpath),
        file_name,
        content_type: content_type_for_file_name(&args.file_name),
        size_bytes,
    })
}

pub fn sync(args: &InboxSyncArgs) -> Result<InboxSyncResult> {
    if args.dry_run {
        return Ok(InboxSyncResult {
            status: "dry_run".to_string(),
            mailbox: args.mailbox.clone(),
            folder: args.folder.clone(),
            scanned: 0,
            imported: 0,
            candidates: Vec::new(),
        });
    }

    let base = resolve_cache_dir(args)?;
    email::ensure_data_dir(&base);
    let existing_ids = email::load_existing_ids(&base, &args.folder);
    let existing_msgs = email::load_existing_msgs(&base, &args.folder);
    let meta = email::fetch_recent_meta(&args.mailbox, &args.folder, args.page_size)?;
    let new_ids = new_message_ids(&meta, &existing_ids);
    let fetch_ids = message_ids_to_fetch(&meta, &existing_msgs, &existing_ids);
    let full = email::fetch_full(&fetch_ids, &args.mailbox)?;

    let merged = merge_messages(existing_msgs, &full);
    email::save_meta(&base, &args.folder, &meta);
    email::save_full(&base, &args.folder, &merged);
    if let Some(max_date) = max_date(&meta) {
        email::save_cursor(&base, &max_date);
    }

    let imported = candidates_from_messages(&messages_matching_ids(&full, &new_ids)).len();
    let current_messages = messages_matching_meta_order(&meta, &merged);
    let candidates = candidates_from_messages(&current_messages);
    Ok(InboxSyncResult {
        status: "synced".to_string(),
        mailbox: args.mailbox.clone(),
        folder: args.folder.clone(),
        scanned: meta.len(),
        imported,
        candidates,
    })
}

fn merge_messages(mut existing: Vec<Value>, new_messages: &[Value]) -> Vec<Value> {
    let replacements: HashMap<String, Value> = new_messages
        .iter()
        .filter_map(|message| message_id(message).map(|id| (id.to_string(), message.clone())))
        .collect();
    let mut seen = HashSet::new();
    let mut merged = Vec::with_capacity(existing.len() + new_messages.len());
    for message in existing.drain(..) {
        if let Some(id) = message_id(&message) {
            seen.insert(id.to_string());
            if let Some(replacement) = replacements.get(id) {
                merged.push(replacement.clone());
                continue;
            }
        }
        merged.push(message);
    }
    for message in new_messages {
        match message_id(message) {
            Some(id) if seen.contains(id) => {}
            Some(id) => {
                seen.insert(id.to_string());
                merged.push(message.clone());
            }
            None => merged.push(message.clone()),
        }
    }
    merged
}

fn message_ids_to_fetch(
    meta: &[Value],
    existing_msgs: &[Value],
    existing_ids: &HashSet<String>,
) -> Vec<String> {
    let cached: HashMap<String, &Value> = existing_msgs
        .iter()
        .filter_map(|message| message_id(message).map(|id| (id.to_string(), message)))
        .collect();
    meta.iter()
        .filter_map(message_id)
        .filter(|id| !existing_ids.contains(*id) || cached.get(*id).is_some_and(needs_refresh))
        .map(String::from)
        .collect()
}

fn messages_matching_ids(messages: &[Value], ids: &[String]) -> Vec<Value> {
    let ids: HashSet<&str> = ids.iter().map(String::as_str).collect();
    messages
        .iter()
        .filter(|message| message_id(message).is_some_and(|id| ids.contains(id)))
        .cloned()
        .collect()
}

fn messages_matching_meta_order(meta: &[Value], messages: &[Value]) -> Vec<Value> {
    let by_id: HashMap<String, &Value> = messages
        .iter()
        .filter_map(|message| message_id(message).map(|id| (id.to_string(), message)))
        .collect();
    meta.iter()
        .filter_map(message_id)
        .filter_map(|id| by_id.get(id).map(|message| (*message).clone()))
        .collect()
}

fn message_id(message: &Value) -> Option<&str> {
    message
        .get("message_id")
        .or_else(|| message.get("id"))
        .and_then(|id| id.as_str())
        .filter(|id| !id.trim().is_empty())
}

fn needs_refresh(message: &&Value) -> bool {
    contains_garbled_text(message)
}

fn contains_garbled_text(value: &Value) -> bool {
    match value {
        Value::String(text) => is_garbled(text),
        Value::Array(items) => items.iter().any(contains_garbled_text),
        Value::Object(object) => object.values().any(contains_garbled_text),
        _ => false,
    }
}

pub fn candidates_from_messages(messages: &[Value]) -> Vec<InboxCandidate> {
    let cfg = config::load_config();
    messages
        .iter()
        .filter_map(|message| candidate_from_message(message, &cfg.rules))
        .collect()
}

fn candidate_from_message(
    message: &Value,
    rules: &[config::PositionRule],
) -> Option<InboxCandidate> {
    let message_id = string_field(message, &["message_id", "id"]);
    let subject = clean_display_text(&decode_mime_words(&string_field(message, &["subject"])));
    let body = clean_display_text(&normalize_body_text(&decode_mime_words(&text_field(
        message,
        &[
            "body_plain_text",
            "plain_text",
            "text",
            "body",
            "content",
            "body_html",
            "body_preview",
            "preview",
        ],
    ))));
    let from = mail_address_field(message, &["head_from", "from"]);
    let email = extract_email(&from)?;
    let name = extract_name(&from)
        .or_else(|| extract_name_from_subject(&subject))
        .unwrap_or_else(|| email.clone());
    let position = config::classify(&subject, rules).unwrap_or("").to_string();
    let updated_at = string_field(
        message,
        &["date", "date_formatted", "created_at", "received_at"],
    );
    let has_cover_letter = body.chars().count() > 20;
    let resume_attachments = resume_attachments(message);
    Some(InboxCandidate {
        id: if message_id.is_empty() {
            format!("cand_{}", stable_id(&email))
        } else {
            format!("cand_{}", stable_id(&message_id))
        },
        name,
        email,
        subject,
        body,
        position,
        stage: "new".to_string(),
        status: "pending".to_string(),
        has_resume: !resume_attachments.is_empty(),
        has_cover_letter,
        resume_attachments,
        source_message_id: message_id,
        updated_at,
    })
}

fn new_message_ids(meta: &[Value], existing_ids: &HashSet<String>) -> Vec<String> {
    meta.iter()
        .filter_map(|message| message.get("message_id").and_then(|id| id.as_str()))
        .filter(|id| !existing_ids.contains(*id))
        .map(String::from)
        .collect()
}

fn max_date(meta: &[Value]) -> Option<String> {
    meta.iter()
        .filter_map(|message| message.get("date").and_then(|date| date.as_str()))
        .max()
        .map(String::from)
}

fn resolve_cache_dir(args: &InboxSyncArgs) -> Result<String> {
    if let Some(dir) = &args.cache_dir {
        return Ok(dir.clone());
    }
    let mut path = crate::connect::cache::cache_dir()?;
    path.push("inbox");
    Ok(path_to_string(path))
}

fn resolve_resume_cache_dir(cache_dir: Option<&str>) -> Result<String> {
    if let Some(dir) = cache_dir {
        return Ok(dir.to_string());
    }
    let mut path = crate::connect::cache::cache_dir()?;
    path.push("inbox");
    Ok(path_to_string(path))
}

fn path_to_string(path: PathBuf) -> String {
    path.to_string_lossy().to_string()
}

fn print_result(result: &InboxSyncResult, format: InboxOutputFormat) -> Result<()> {
    match format {
        InboxOutputFormat::Json => {
            print!("{}", serde_json::to_string(result)?);
        }
        InboxOutputFormat::Text => {
            println!(
                "{} | mailbox: {} | folder: {} | scanned: {} | imported: {}",
                result.status, result.mailbox, result.folder, result.scanned, result.imported
            );
        }
    }
    Ok(())
}

fn print_resume_result(result: &InboxResumeResult, format: InboxOutputFormat) -> Result<()> {
    match format {
        InboxOutputFormat::Json => {
            print!("{}", serde_json::to_string(result)?);
        }
        InboxOutputFormat::Text => {
            println!(
                "{} | file: {} | path: {}",
                result.status, result.file_name, result.path
            );
        }
    }
    Ok(())
}

fn string_field(message: &Value, fields: &[&str]) -> String {
    for field in fields {
        if let Some(value) = message.get(*field).and_then(|value| value.as_str()) {
            return value.to_string();
        }
    }
    String::new()
}

fn text_field(message: &Value, fields: &[&str]) -> String {
    for field in fields {
        let Some(value) = message.get(*field) else {
            continue;
        };
        if let Some(text) = text_from_value(value) {
            return text;
        }
    }
    String::new()
}

fn text_from_value(value: &Value) -> Option<String> {
    if let Some(text) = value.as_str() {
        return non_empty(text);
    }
    if let Some(object) = value.as_object() {
        for field in [
            "body_plain_text",
            "plain_text",
            "text",
            "content",
            "body",
            "html",
            "body_html",
            "body_preview",
            "preview",
        ] {
            if let Some(text) = object.get(field).and_then(text_from_value) {
                return Some(text);
            }
        }
    }
    if let Some(array) = value.as_array() {
        for item in array {
            if let Some(text) = text_from_value(item) {
                return Some(text);
            }
        }
    }
    None
}

fn non_empty(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn mail_address_field(message: &Value, fields: &[&str]) -> String {
    for field in fields {
        let Some(value) = message.get(*field) else {
            continue;
        };
        if let Some(text) = value.as_str() {
            return text.to_string();
        }
        if let Some(mail_address) = value.get("mail_address").and_then(|v| v.as_str()) {
            let name = value.get("name").and_then(|v| v.as_str()).unwrap_or("");
            if name.trim().is_empty() {
                return mail_address.to_string();
            }
            return format!("{} <{}>", name.trim(), mail_address);
        }
    }
    String::new()
}

fn extract_email(from: &str) -> Option<String> {
    let re = regex::Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").ok()?;
    re.find(from).map(|found| found.as_str().to_string())
}

fn extract_name(from: &str) -> Option<String> {
    let trimmed = from.trim();
    if trimmed.is_empty() || trimmed.contains('@') && !trimmed.contains('<') {
        return None;
    }
    let raw_name = trimmed.split('<').next()?.trim().trim_matches('"').trim();
    let decoded = decode_mime_words(raw_name);
    let name = decoded.trim();
    if name.is_empty() || is_garbled(name) {
        None
    } else {
        Some(name.to_string())
    }
}

fn extract_name_from_subject(subject: &str) -> Option<String> {
    let bracketed = regex::Regex::new(r"\[([^\]]+)\]").ok()?;
    let parts: Vec<String> = bracketed
        .captures_iter(subject)
        .filter_map(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
        .filter(|value| !value.is_empty())
        .collect();
    if parts.len() >= 2 {
        return Some(parts[1].clone());
    }
    None
}

fn clean_display_text(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() || is_garbled(trimmed) {
        String::new()
    } else {
        trimmed.to_string()
    }
}

fn is_garbled(value: &str) -> bool {
    value.chars().any(|ch| ch == '\u{FFFD}')
}

fn decode_mime_words(input: &str) -> String {
    let re = match regex::Regex::new(r"=\?([^?]+)\?([bBqQ])\?([^?]*)\?=") {
        Ok(re) => re,
        Err(_) => return input.to_string(),
    };
    let mut output = String::new();
    let mut last = 0;
    for caps in re.captures_iter(input) {
        let Some(found) = caps.get(0) else {
            continue;
        };
        output.push_str(&input[last..found.start()]);
        let charset = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let encoding = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        let encoded = caps.get(3).map(|m| m.as_str()).unwrap_or("");
        match decode_mime_word(charset, encoding, encoded) {
            Some(decoded) => output.push_str(&decoded),
            None => output.push_str(found.as_str()),
        }
        last = found.end();
    }
    output.push_str(&input[last..]);
    output
}

fn decode_mime_word(charset: &str, encoding: &str, encoded: &str) -> Option<String> {
    let bytes = if encoding.eq_ignore_ascii_case("b") {
        decode_base64(encoded)?
    } else {
        decode_q_encoding(encoded)?
    };
    decode_charset(charset, &bytes)
}

fn decode_charset(charset: &str, bytes: &[u8]) -> Option<String> {
    if charset.eq_ignore_ascii_case("utf-8") || charset.eq_ignore_ascii_case("us-ascii") {
        return String::from_utf8(bytes.to_vec()).ok();
    }
    if charset.eq_ignore_ascii_case("gb2312")
        || charset.eq_ignore_ascii_case("gbk")
        || charset.eq_ignore_ascii_case("gb18030")
    {
        let (decoded, _, _) = GB18030.decode(bytes);
        return Some(decoded.into_owned());
    }
    let encoding = Encoding::for_label(charset.as_bytes())?;
    let (decoded, _, had_errors) = encoding.decode(bytes);
    if had_errors {
        None
    } else {
        Some(decoded.into_owned())
    }
}

fn normalize_body_text(input: &str) -> String {
    let decoded = decode_html_entities(input);
    let stripped = strip_html_tags(&decoded);
    collapse_whitespace(&stripped)
}

fn strip_html_tags(input: &str) -> String {
    let re = match regex::Regex::new(r"<[^>]+>") {
        Ok(re) => re,
        Err(_) => return input.to_string(),
    };
    re.replace_all(input, " ").to_string()
}

fn decode_html_entities(input: &str) -> String {
    input
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
}

fn collapse_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn decode_q_encoding(input: &str) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    let raw = input.as_bytes();
    let mut index = 0;
    while index < raw.len() {
        match raw[index] {
            b'_' => {
                bytes.push(b' ');
                index += 1;
            }
            b'=' if index + 2 < raw.len() => {
                let high = hex_value(raw[index + 1])?;
                let low = hex_value(raw[index + 2])?;
                bytes.push((high << 4) | low);
                index += 3;
            }
            b'=' => return None,
            value => {
                bytes.push(value);
                index += 1;
            }
        }
    }
    Some(bytes)
}

fn decode_base64(input: &str) -> Option<Vec<u8>> {
    let mut buffer: u32 = 0;
    let mut bits: u8 = 0;
    let mut bytes = Vec::new();
    for byte in input.bytes().filter(|b| !b.is_ascii_whitespace()) {
        if byte == b'=' {
            break;
        }
        let value = u32::from(base64_value(byte)?);
        buffer = (buffer << 6) | value;
        bits += 6;
        while bits >= 8 {
            bits -= 8;
            bytes.push(((buffer >> bits) & 0xff) as u8);
            buffer &= (1 << bits) - 1;
        }
    }
    Some(bytes)
}

fn base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn resume_attachments(message: &Value) -> Vec<InboxResumeAttachment> {
    message
        .get("attachments")
        .and_then(|attachments| attachments.as_array())
        .map(|attachments| {
            attachments
                .iter()
                .filter(|attachment| !is_inline_attachment(attachment))
                .filter_map(resume_attachment_from_value)
                .collect()
        })
        .unwrap_or_default()
}

fn resume_attachment_from_value(attachment: &Value) -> Option<InboxResumeAttachment> {
    let file_name = clean_display_text(&decode_mime_words(&string_field(
        attachment,
        &["file_name", "filename", "name"],
    )));
    if file_name.is_empty() || !is_resume_file(&file_name) {
        return None;
    }
    Some(InboxResumeAttachment {
        attachment_id: string_field(attachment, &["attachment_id", "id"]),
        file_name,
        content_type: string_field(attachment, &["content_type", "mime_type", "mime"]),
        url: string_field(attachment, &["url", "download_url", "preview_url"]),
        size_bytes: numeric_field(attachment, &["size_bytes", "size"]),
    })
}

fn fetch_attachment_download_url(args: &InboxResumeArgs) -> Result<String> {
    let data = email::run_lark_json(&[
        "mail",
        "user_mailbox.message.attachments",
        "download_url",
        "--user-mailbox-id",
        &args.mailbox,
        "--message-id",
        &args.message_id,
        "--attachment-ids",
        &args.attachment_id,
    ])?;
    data.get("data")
        .and_then(|data| data.get("download_urls"))
        .and_then(|urls| urls.as_array())
        .and_then(|urls| urls.first())
        .and_then(|url| url.get("download_url"))
        .and_then(|url| url.as_str())
        .map(str::to_string)
        .filter(|url| !url.trim().is_empty())
        .context("resume attachment download url missing")
}

fn safe_file_name(file_name: &str) -> Result<String> {
    let normalized = file_name.replace(['\\', '/'], "_");
    let name = Path::new(&normalized)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .trim()
        .replace('\0', "");
    if name.is_empty() {
        anyhow::bail!("resume file name is required");
    }
    Ok(name)
}

fn content_type_for_file_name(file_name: &str) -> String {
    let lower = file_name.to_lowercase();
    if lower.ends_with(".pdf") {
        return "application/pdf".to_string();
    }
    if lower.ends_with(".docx") {
        return "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            .to_string();
    }
    if lower.ends_with(".doc") {
        return "application/msword".to_string();
    }
    String::new()
}

fn is_inline_attachment(attachment: &Value) -> bool {
    attachment
        .get("is_inline")
        .or_else(|| attachment.get("inline"))
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
}

fn is_resume_file(file_name: &str) -> bool {
    let lower = file_name.to_lowercase();
    [".pdf", ".doc", ".docx"]
        .iter()
        .any(|suffix| lower.ends_with(suffix))
}

fn numeric_field(value: &Value, fields: &[&str]) -> Option<u64> {
    for field in fields {
        let Some(raw) = value.get(*field) else {
            continue;
        };
        if let Some(number) = raw.as_u64() {
            return Some(number);
        }
        if let Some(text) = raw.as_str().and_then(|text| text.parse::<u64>().ok()) {
            return Some(text);
        }
    }
    None
}

fn stable_id(input: &str) -> String {
    let mut hash: u64 = 14695981039346656037;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(1099511628211);
    }
    format!("{hash:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidates_from_messages_extracts_minimal_fields() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_1",
                    "head_from":"张三 <zhangsan@example.com>",
                    "subject":"应聘数据工程师 - 张三",
                    "body":"您好，我对数据工程师岗位很感兴趣，附件是我的简历。",
                    "date":"2026-09-04",
                    "attachments":[{"name":"resume.pdf"}]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].name, "张三");
        assert_eq!(candidates[0].email, "zhangsan@example.com");
        assert_eq!(candidates[0].position, "数据工程师");
        assert!(candidates[0].has_resume);
        assert!(candidates[0].has_cover_letter);
        assert_eq!(candidates[0].source_message_id, "m_1");
    }

    #[test]
    fn test_candidates_from_lark_messages_output_shape() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_2",
                    "head_from":{"mail_address":"candidate@example.com","name":"候选人"},
                    "subject":"[产品经理] - [张心洁] - [浙江越秀外国语学院] - [3个月以上]",
                    "body_plain_text":"您好，我是张心洁，希望加入团队并参与真实商业项目。",
                    "date_formatted":"2026-09-04 16:26",
                    "attachments":[{"filename":"简历.pdf"},{"filename":"求职信.docx"}]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].name, "候选人");
        assert_eq!(candidates[0].email, "candidate@example.com");
        assert_eq!(candidates[0].position, "产品经理");
        assert_eq!(candidates[0].updated_at, "2026-09-04 16:26");
        assert!(candidates[0].has_resume);
        assert!(candidates[0].has_cover_letter);
    }

    #[test]
    fn test_candidates_expose_resume_attachment_metadata() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_resume",
                    "head_from":"张三 <zhangsan@example.com>",
                    "subject":"应聘后端开发",
                    "body_plain_text":"HR 您好，我想投递后端开发岗位，附件是我的简历。",
                    "date":"2026-09-04",
                    "attachments":[
                        {
                            "id":"att_resume_001",
                            "filename":"张三-后端开发简历.pdf",
                            "content_type":"application/pdf",
                            "download_url":"https://files.example.test/resumes/m_resume.pdf",
                            "size":245760
                        }
                    ]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert!(candidates[0].has_resume);
        assert_eq!(candidates[0].resume_attachments.len(), 1);
        assert_eq!(
            candidates[0].resume_attachments[0].file_name,
            "张三-后端开发简历.pdf"
        );
        assert_eq!(
            candidates[0].resume_attachments[0].url,
            "https://files.example.test/resumes/m_resume.pdf"
        );
        assert_eq!(
            candidates[0].resume_attachments[0].attachment_id,
            "att_resume_001"
        );
    }

    #[test]
    fn test_candidates_ignore_inline_non_resume_attachments() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_inline",
                    "head_from":"candidate@example.com",
                    "subject":"应聘后端开发",
                    "body_plain_text":"HR 您好，我想投递后端开发岗位。",
                    "date":"2026-09-04",
                    "attachments":[
                        {"filename":"logo.png","is_inline":true},
                        {"filename":"signature.jpg"}
                    ]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert!(!candidates[0].has_resume);
        assert!(candidates[0].resume_attachments.is_empty());
    }

    #[test]
    fn test_candidates_decode_mime_names_and_expose_subject_body() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_3",
                    "head_from":"=?UTF-8?B?5byg5LiJ?= <zhangsan@example.com>",
                    "subject":"应聘后端开发",
                    "body_plain_text":"HR 您好，我想投递后端开发岗位，附件是我的简历。",
                    "date":"2026-09-04",
                    "attachments":[{"filename":"resume.pdf"}]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].name, "张三");
        assert_eq!(candidates[0].subject, "应聘后端开发");
        assert_eq!(
            candidates[0].body,
            "HR 您好，我想投递后端开发岗位，附件是我的简历。"
        );
    }

    #[test]
    fn test_candidates_decode_gb2312_mime_names() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_4",
                    "head_from":"=?GB2312?B?1cXI/Q==?= <zhangsan@example.com>",
                    "subject":"应聘后端开发",
                    "body_plain_text":"HR 您好，我想投递后端开发岗位，附件是我的简历。",
                    "date":"2026-09-04",
                    "attachments":[{"filename":"resume.pdf"}]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].name, "张三");
    }

    #[test]
    fn test_candidates_fallback_to_subject_name_when_sender_name_is_garbled() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_5",
                    "head_from":"���� <candidate@example.com>",
                    "subject":"[产品经理] - [王五] - [浙江越秀外国语学院] - [3个月以上]",
                    "body_plain_text":"您好，希望投递产品经理岗位。",
                    "date":"2026-09-04",
                    "attachments":[{"filename":"resume.pdf"}]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].name, "王五");
    }

    #[test]
    fn test_candidates_do_not_expose_garbled_subject_or_body() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_5_bad_text",
                    "head_from":"candidate@example.com",
                    "subject":"����",
                    "body_plain_text":"����",
                    "date":"2026-09-04",
                    "attachments":[{"filename":"resume.pdf"}]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].name, "candidate@example.com");
        assert!(candidates[0].subject.is_empty());
        assert!(candidates[0].body.is_empty());
        assert!(!candidates[0].has_cover_letter);
    }

    #[test]
    fn test_candidates_extract_nested_plain_body() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_6",
                    "head_from":{"mail_address":"candidate@example.com","name":"候选人"},
                    "subject":"应聘数据工程师",
                    "body":{"body_plain_text":"HR 您好，我有正文自荐内容，希望参与真实商业项目并长期投入。"},
                    "date":"2026-09-04",
                    "attachments":[{"filename":"resume.pdf"}]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert_eq!(
            candidates[0].body,
            "HR 您好，我有正文自荐内容，希望参与真实商业项目并长期投入。"
        );
        assert!(candidates[0].has_cover_letter);
    }

    #[test]
    fn test_candidates_extract_html_body_as_text() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {
                    "message_id":"m_7",
                    "head_from":{"mail_address":"candidate@example.com","name":"候选人"},
                    "subject":"应聘数据工程师",
                    "body":{"content":"<p>HR&nbsp;您好，</p><p>我想投递数据工程师岗位。</p>"},
                    "date":"2026-09-04",
                    "attachments":[{"filename":"resume.pdf"}]
                }
            ]"#,
        )
        .unwrap();

        let candidates = candidates_from_messages(&messages);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].body, "HR 您好， 我想投递数据工程师岗位。");
    }

    #[test]
    fn test_new_message_ids_keeps_batch_bounded() {
        let meta: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"m_1"},
                {"message_id":"m_2"},
                {"message_id":"m_3"}
            ]"#,
        )
        .unwrap();
        let existing = HashSet::from(["m_2".to_string()]);

        let new_ids = new_message_ids(&meta, &existing);

        assert_eq!(new_ids, vec!["m_1".to_string(), "m_3".to_string()]);
    }

    #[test]
    fn test_merge_messages_returns_existing_and_new_messages() {
        let existing: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"m_1","subject":"应聘后端开发"}
            ]"#,
        )
        .unwrap();
        let new_messages: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"m_2","subject":"应聘产品经理"}
            ]"#,
        )
        .unwrap();

        let merged = merge_messages(existing, &new_messages);

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0]["message_id"], "m_1");
        assert_eq!(merged[1]["message_id"], "m_2");
    }

    #[test]
    fn test_message_ids_to_fetch_refreshes_garbled_cached_messages() {
        let meta: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"m_1"},
                {"message_id":"m_2"},
                {"message_id":"m_3"}
            ]"#,
        )
        .unwrap();
        let cached: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"m_1","subject":"应聘后端开发"},
                {"message_id":"m_2","subject":"����"}
            ]"#,
        )
        .unwrap();
        let existing = HashSet::from(["m_1".to_string(), "m_2".to_string()]);

        let ids = message_ids_to_fetch(&meta, &cached, &existing);

        assert_eq!(ids, vec!["m_2".to_string(), "m_3".to_string()]);
    }

    #[test]
    fn test_merge_messages_replaces_cached_message_with_refetched_message() {
        let existing: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"m_1","subject":"����"},
                {"message_id":"m_2","subject":"应聘产品经理"}
            ]"#,
        )
        .unwrap();
        let refetched: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"m_1","subject":"应聘后端开发"}
            ]"#,
        )
        .unwrap();

        let merged = merge_messages(existing, &refetched);

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0]["subject"], "应聘后端开发");
        assert_eq!(merged[1]["subject"], "应聘产品经理");
    }

    #[test]
    fn test_messages_matching_ids_keeps_imported_count_to_new_messages() {
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"m_1","subject":"应聘后端开发"},
                {"message_id":"m_2","subject":"应聘产品经理"}
            ]"#,
        )
        .unwrap();
        let ids = vec!["m_2".to_string()];

        let matched = messages_matching_ids(&messages, &ids);

        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0]["message_id"], "m_2");
    }

    #[test]
    fn test_messages_matching_meta_order_returns_current_scan_only() {
        let meta: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"m_2"},
                {"message_id":"m_1"}
            ]"#,
        )
        .unwrap();
        let messages: Vec<Value> = serde_json::from_str(
            r#"[
                {"message_id":"old","subject":"旧缓存"},
                {"message_id":"m_1","subject":"应聘后端开发"},
                {"message_id":"m_2","subject":"应聘产品经理"}
            ]"#,
        )
        .unwrap();

        let matched = messages_matching_meta_order(&meta, &messages);

        assert_eq!(matched.len(), 2);
        assert_eq!(matched[0]["message_id"], "m_2");
        assert_eq!(matched[1]["message_id"], "m_1");
    }

    #[test]
    fn test_sync_dry_run_does_not_read_mail() {
        let result = sync(&InboxSyncArgs {
            mailbox: "hr@example.com".to_string(),
            folder: "INBOX".to_string(),
            page_size: 50,
            cache_dir: None,
            dry_run: true,
            format: InboxOutputFormat::Json,
        })
        .unwrap();

        assert_eq!(result.status, "dry_run");
        assert_eq!(result.scanned, 0);
        assert!(result.candidates.is_empty());
    }
}
