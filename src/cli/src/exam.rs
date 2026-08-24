//! 笔试（access 域动作）：发送笔试邀请，候选人以实际成果参与考核。
//!
//! 话术内容见 templates.rs（源自业务实体手册 qtrecurit/connect/content.md），
//! 发送经 connect::email::send_mail 通道（日志由通道内部处理）。

use anyhow::Result;
use clap::Args;

use crate::connect::email::send_mail;
use crate::templates;

#[derive(Args)]
pub struct ExamArgs {
    /// 候选人邮箱
    #[arg(long)]
    pub to: String,

    /// 确认后直接发送（默认只生成草稿）
    #[arg(long)]
    pub confirm_send: bool,

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
        args.confirm_send,
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
    Ok(())
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
