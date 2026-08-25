//! 笔试（access 域动作）：发送笔试邀请，候选人以实际成果参与考核。
//!
//! 话术内容见 templates.rs（源自业务实体手册 qtrecurit/connect/content.md），
//! 发送经 connect::email::send_mail 通道（日志由通道内部处理）。

use anyhow::Result;
use clap::Args;

use crate::connect::{cache, email::{send_mail, find_candidate_submission, move_message_to_folder}};
use crate::templates;

#[derive(Args)]
pub struct ExamArgs {
    /// 候选人邮箱
    #[arg(long)]
    pub to: String,

    /// 发送前打印将执行的命令，不执行
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run(args: &ExamArgs) -> Result<()> {
    let tpl = templates::find_template("exam")
        .expect("exam 模板必须存在（templates.rs TEMPLATES）");
    let subject = tpl.subject.to_string();
    let body = tpl.body.to_string();

    let (_id, sent) = send_mail(
        &args.to,
        &subject,
        &body,
        None,
        "exam",
        !args.dry_run, // 非 dry-run 时直接发送
        args.dry_run,
    )?;

    let status = if args.dry_run {
        "dry-run".to_string()
    } else if sent {
        "sent".to_string()
    } else {
        "draft".to_string()
    };
    println!(
        "{} 收件人: {} | 模板: exam | 状态: {}",
        if args.dry_run {
            "[dry-run]"
        } else if sent {
            "✓ 已发送"
        } else {
            "✓ 已生成草稿"
        },
        args.to,
        status
    );

    // 归档候选人的投递邮件到「已发送笔试」文件夹
    if sent && !args.dry_run {
        if let Err(e) = archive_candidate_submission(&args.to) {
            eprintln!("警告: 归档投递邮件失败: {e:#}");
        }
    }

    Ok(())
}

/// 归档候选人的投递邮件到「已发送笔试」文件夹
fn archive_candidate_submission(candidate_email: &str) -> Result<()> {
    // 获取文件夹 ID
    let folder_id = cache::get_folder_id("已发送笔试")
        .or_else(|| {
            eprintln!("缓存未命中，正在获取已发送笔试文件夹 ID...");
            cache::fetch_folder_id_from_email("已发送笔试").ok().map(|id| {
                if let Err(e) = cache::set_folder_id("已发送笔试", &id) {
                    eprintln!("警告: 缓存文件夹 ID 失败: {}", e);
                }
                id
            })
        })
        .ok_or_else(|| anyhow::anyhow!("无法获取已发送笔试文件夹 ID"))?;

    // 搜索候选人的投递邮件
    match find_candidate_submission(candidate_email)? {
        Some(message_id) => {
            eprintln!("正在归档投递邮件: {}", message_id);
            move_message_to_folder(&message_id, &folder_id, false)?;
            eprintln!("✓ 投递邮件已归档到已发送笔试文件夹");
            Ok(())
        }
        None => {
            eprintln!("未找到候选人 {} 的投递邮件", candidate_email);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exam_template_loaded() {
        let tpl = templates::find_template("exam").unwrap();
        assert_eq!(tpl.subject, "量潮招聘考核邀请");
        assert!(tpl.body.contains("实际成果为核心"));
        assert!(tpl.body.contains("实训"));
    }
}
