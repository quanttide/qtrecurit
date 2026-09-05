//! 准入问卷发放：候选人投递后，进入筛选流程前。
//!
//! 话术见 templates.rs（源自业务实体手册 qtrecurit/connect/content.md），
//! 发送经 connect::email::send_mail 通道（日志由通道内部处理）。
//! 问卷链接优先从本地缓存读取，缓存未命中时自动从 HR 邮箱获取。

use anyhow::Result;
use clap::Args;

use crate::connect::{
    cache,
    email::{
        find_candidate_submission, mark_as_read, move_message_to_folder, send_mail,
        verify_sent_mail,
    },
};
use crate::templates::{self, render_template};

#[derive(Args)]
pub struct SurveyArgs {
    /// 候选人邮箱
    #[arg(long)]
    pub to: String,

    /// 候选人姓名
    #[arg(long)]
    pub name: String,

    /// 准入问卷链接（可选，留空时从缓存或 HR 邮箱获取）
    #[arg(long)]
    pub link: Option<String>,

    /// 发送前打印将执行的命令，不执行
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run(args: &SurveyArgs) -> Result<()> {
    // 获取问卷链接：优先使用参数，其次从缓存获取，最后从 HR 邮箱获取
    let link = match &args.link {
        Some(link) => link.clone(),
        None if args.dry_run => "https://example.com/survey-dry-run".to_string(),
        None => {
            cache::get_survey_url().or_else(|| {
                eprintln!("缓存未命中，正在从 HR 邮箱获取问卷链接...");
                cache::fetch_survey_url_from_email().ok().map(|url| {
                    // 缓存获取到的链接
                    if let Err(e) = cache::set_survey_url(&url) {
                        eprintln!("警告: 缓存问卷链接失败: {}", e);
                    }
                    url
                })
            }).unwrap_or_else(|| {
                eprintln!("错误: 无法获取问卷链接。请使用 --link 参数指定，或运行 'qtrecurit survey --refresh-cache' 刷新缓存。");
                std::process::exit(1);
            })
        }
    };

    let tpl =
        templates::find_template("survey").expect("survey 模板必须存在（templates.rs TEMPLATES）");
    let body = render_template(
        tpl,
        &[
            ("name".to_string(), args.name.clone()),
            ("link".to_string(), link),
        ],
    );

    let (msg_id, sent) = send_mail(
        &args.to,
        &tpl.subject,
        &body,
        None,
        "survey",
        true,
        args.dry_run,
    )?;

    let status = if args.dry_run {
        "dry-run".to_string()
    } else if sent {
        "sent".to_string()
    } else {
        "draft".to_string()
    };

    // 验证发送结果并标注已读
    if sent && !args.dry_run {
        // 标注已发送的问卷邮件为已读
        if !msg_id.is_empty() {
            if let Err(e) = mark_as_read(&msg_id, false) {
                eprintln!("警告: 标注已读失败: {e:#}");
            }
        }

        match verify_sent_mail(&args.to, &tpl.subject) {
            Ok(result) => {
                if result.success {
                    println!("✓ 已发送 | 收件人: {} | {}", args.to, result.message);
                } else {
                    println!(
                        "⚠ 已发送但无法验证 | 收件人: {} | {}",
                        args.to, result.message
                    );
                }
            }
            Err(e) => {
                println!("✓ 已发送 | 收件人: {} | 验证失败: {}", args.to, e);
            }
        }
    } else if args.dry_run {
        println!(
            "[dry-run] 收件人: {} | 模板: survey | 状态: {}",
            args.to, status
        );
    } else {
        println!("✓ 已生成草稿 | 收件人: {} | 模板: survey", args.to);
    }

    // 归档候选人的投递邮件到「已发送问卷」文件夹
    if sent && !args.dry_run {
        if let Err(e) = archive_candidate_submission(&args.to) {
            eprintln!("警告: 归档投递邮件失败: {e:#}");
        }
    }

    Ok(())
}

/// 归档候选人的投递邮件到「已发送问卷」文件夹
fn archive_candidate_submission(candidate_email: &str) -> Result<()> {
    // 获取文件夹 ID
    let folder_id = cache::get_folder_id("已发送问卷")
        .or_else(|| {
            eprintln!("缓存未命中，正在获取已发送问卷文件夹 ID...");
            cache::fetch_folder_id_from_email("已发送问卷")
                .ok()
                .map(|id| {
                    if let Err(e) = cache::set_folder_id("已发送问卷", &id) {
                        eprintln!("警告: 缓存文件夹 ID 失败: {}", e);
                    }
                    id
                })
        })
        .ok_or_else(|| anyhow::anyhow!("无法获取已发送问卷文件夹 ID"))?;

    // 搜索候选人的投递邮件
    match find_candidate_submission(candidate_email)? {
        Some(message_id) => {
            eprintln!("正在归档投递邮件: {}", message_id);
            move_message_to_folder(&message_id, &folder_id, false)?;
            eprintln!("✓ 投递邮件已归档到已发送问卷文件夹");
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
    fn test_survey_template_loaded() {
        let tpl = templates::find_template("survey").unwrap();
        assert_eq!(tpl.subject, "量潮科技准入问卷");
        assert!(tpl.body.contains("准入问卷"));
        assert!(tpl.body.contains("15-20分钟"));
        assert!(tpl.body.contains("3个工作日"));
    }
}
