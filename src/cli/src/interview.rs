//! 面试通知：筛选/考核通过后，安排面试。
//!
//! 话术见 templates.rs（源自业务实体手册 qtrecurit/connect/content.md），
//! 发送经 connect::email::send_mail 通道（日志由通道内部处理）。

use anyhow::Result;
use clap::Args;

use crate::connect::email::send_mail;
use crate::templates::{self, render_template};

#[derive(Args)]
pub struct InterviewArgs {
    /// 候选人邮箱
    #[arg(long)]
    pub to: String,

    /// 候选人姓名
    #[arg(long)]
    pub name: String,

    /// 应聘岗位
    #[arg(long)]
    pub position: String,

    /// 面试时间
    #[arg(long)]
    pub time: String,

    /// 确认后直接发送（默认只生成草稿）
    #[arg(long)]
    pub confirm_send: bool,

    /// 发送前打印将执行的命令，不执行
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run(args: &InterviewArgs) -> Result<()> {
    let tpl = templates::find_template("interview")
        .expect("interview 模板必须存在（templates.rs TEMPLATES）");
    let body = render_template(
        tpl,
        &[
            ("name".to_string(), args.name.clone()),
            ("position".to_string(), args.position.clone()),
            ("time".to_string(), args.time.clone()),
        ],
    );

    let (_id, sent) = send_mail(
        &args.to,
        &tpl.subject,
        &body,
        None,
        "interview",
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
        "{} 收件人: {} | 模板: interview | 状态: {}",
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
    fn test_interview_template_loaded() {
        let tpl = templates::find_template("interview").unwrap();
        assert_eq!(tpl.subject, "量潮面试通知");
        assert!(tpl.body.contains("飞书线上面试"));
        assert!(tpl.body.contains("{{position}}"));
        assert!(tpl.body.contains("{{time}}"));
    }
}
