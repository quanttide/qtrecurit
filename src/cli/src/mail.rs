//! 招聘沟通邮件：按话术模板发送（referral / training / exam）。
//!
//! 招聘域的业务入口：话术内容在 templates.rs，渲染与发送通道
//! 由 qtcloud-connect-send 提供（沟通云 = 通道，不承载业务）。

use anyhow::Result;
use chrono::Local;
use clap::{Args, Subcommand};

use crate::connect::email::{SendLogEntry, append_send_log, read_send_log, send_mail};
use crate::templates::{self, parse_vars, render_template};

#[derive(Args)]
pub struct MailArgs {
    #[command(subcommand)]
    pub action: MailAction,
}

#[derive(Subcommand)]
pub enum MailAction {
    /// 按话术模板发送邮件：渲染模板 → 生成草稿 → 人工确认 → 发送 → 回写状态
    Send(MailSendArgs),
    /// 查看/列出邮件模板
    Template(MailTemplateArgs),
    /// 查看发送日志
    Log(MailLogArgs),
}

#[derive(Args)]
pub struct MailSendArgs {
    /// 收件人邮箱（逗号分隔多个）
    #[arg(long)]
    pub to: String,

    /// 模板名：referral（内推）/ training（实训邀请）/ exam（考核说明）
    #[arg(long, default_value = "exam")]
    pub template: String,

    /// 模板变量：key=value（逗号分隔多个），如 name=张三
    #[arg(long)]
    pub vars: Option<String>,

    /// 发送日志回写路径（默认 $SEND_LOG_DIR/send.log）
    #[arg(long)]
    pub log_file: Option<String>,

    /// 确认后直接发送（默认只生成草稿）
    #[arg(long)]
    pub confirm_send: bool,

    /// 发送前打印将执行的命令，不执行
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Args)]
pub struct MailTemplateArgs {
    /// 模板名：referral / training / exam
    #[arg(long)]
    pub name: Option<String>,

    /// 列出所有模板
    #[arg(long)]
    pub list: bool,
}

#[derive(Args)]
pub struct MailLogArgs {
    /// 显示最近 N 条日志
    #[arg(long, default_value = "20")]
    pub tail: usize,
}

pub fn run(args: &MailArgs) -> Result<()> {
    match &args.action {
        MailAction::Send(a) => cmd_send(a),
        MailAction::Template(a) => cmd_template(a),
        MailAction::Log(a) => cmd_log(a),
    }
}

fn cmd_send(args: &MailSendArgs) -> Result<()> {
    let tpl = match templates::find_template(&args.template) {
        Some(t) => t,
        None => {
            eprintln!(
                "错误: 未知模板 '{}'。可用: referral / training / exam",
                args.template
            );
            std::process::exit(1);
        }
    };
    let vars = parse_vars(args.vars.as_deref());
    let subject = tpl.subject.to_string();
    let body = render_template(tpl, &vars);

    // training 模板默认附带群二维码（$TRAINING_QR_PATH，敏感内容不放进仓库）
    let attach = if args.template == "training" {
        match std::env::var("TRAINING_QR_PATH") {
            Ok(p) => Some(p),
            Err(_) => {
                eprintln!(
                    "警告: training 模板需要群二维码附件，请设置 TRAINING_QR_PATH 环境变量指向二维码图片"
                );
                None
            }
        }
    } else {
        None
    };

    match send_mail(
        &args.to,
        &subject,
        &body,
        attach.as_deref(),
        args.confirm_send,
        args.dry_run,
    ) {
        Ok((id, sent)) => {
            let status = if args.dry_run {
                "dry-run".to_string()
            } else if sent {
                "sent".to_string()
            } else {
                "draft".to_string()
            };
            println!(
                "{} 收件人: {} | 模板: {} | 状态: {}",
                if args.dry_run {
                    "[dry-run]"
                } else if sent {
                    "✓ 已发送"
                } else {
                    "✓ 已生成草稿"
                },
                args.to,
                args.template,
                status
            );
            if !args.dry_run {
                let entry = SendLogEntry {
                    time: Local::now().to_rfc3339(),
                    to: args.to.clone(),
                    subject: subject.clone(),
                    template: args.template.clone(),
                    status,
                    draft_id: id,
                    note: None,
                };
                if let Err(e) = append_send_log(args.log_file.as_deref(), &entry) {
                    // fail-closed：日志写入失败必须显式报错
                    eprintln!("警告: 发送日志写入失败（发送本身已成功）: {e}");
                    eprintln!(
                        "请手动补记: {}",
                        serde_json::to_string(&entry).unwrap_or_default()
                    );
                }
            }
            Ok(())
        }
        Err(e) => Err(e.context("发送邮件失败")),
    }
}

fn cmd_template(args: &MailTemplateArgs) -> Result<()> {
    if args.list {
        println!("可用模板（招聘话术）:");
        for t in templates::TEMPLATES {
            println!("  {} — {}", t.name, t.description);
        }
        return Ok(());
    }
    let name = match &args.name {
        Some(n) => n,
        None => {
            eprintln!("用法: qtrecurit mail template --list 或 --name <referral|training|exam>");
            std::process::exit(1);
        }
    };
    let tpl = match templates::find_template(name) {
        Some(t) => t,
        None => {
            eprintln!("未知模板: {name}");
            std::process::exit(1);
        }
    };
    println!(
        "=== {} ===\n主题: {}\n\n{}",
        tpl.name, tpl.subject, tpl.body
    );
    Ok(())
}

fn cmd_log(args: &MailLogArgs) -> Result<()> {
    match read_send_log(None, args.tail) {
        Ok(entries) => {
            if entries.is_empty() {
                println!("暂无发送日志");
                return Ok(());
            }
            for e in entries {
                println!(
                    "{} | {} | {} | {} | {}",
                    e.time, e.status, e.to, e.subject, e.draft_id
                );
            }
            Ok(())
        }
        Err(e) => Err(e.context("读取发送日志失败")),
    }
}
