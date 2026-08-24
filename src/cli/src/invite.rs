//! 邀请进群（实训邀请）：准入问卷通过后，正式受邀加入量潮实训基地。
//!
//! 话术见 templates.rs（源自业务实体手册 qtrecurit/connect/content.md），
//! 发送经 connect::email::send_mail 通道（日志由通道内部处理）。

use anyhow::Result;
use clap::Args;

use crate::connect::email::send_mail;
use crate::templates::{self, render_template};

#[derive(Args)]
pub struct InviteArgs {
    /// 候选人邮箱
    #[arg(long)]
    pub to: String,

    /// 候选人姓名
    #[arg(long)]
    pub name: String,

    /// 实训基地群二维码图片路径（附件）
    #[arg(long)]
    pub qr: Option<String>,

    /// 确认后直接发送（默认只生成草稿）
    #[arg(long)]
    pub confirm_send: bool,

    /// 发送前打印将执行的命令，不执行
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run(args: &InviteArgs) -> Result<()> {
    let tpl = templates::find_template("invite")
        .expect("invite 模板必须存在（templates.rs TEMPLATES）");
    let body = render_template(tpl, &[("name".to_string(), args.name.clone())]);

    let (_id, sent) = send_mail(
        &args.to,
        &tpl.subject,
        &body,
        args.qr.as_deref(),
        "invite",
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
        "{} 收件人: {} | 模板: invite | 状态: {}",
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
    fn test_invite_template_loaded() {
        let tpl = templates::find_template("invite").unwrap();
        assert_eq!(tpl.subject, "量潮实训基地邀请");
        assert!(tpl.body.contains("实训基地"));
        assert!(tpl.body.contains("岗位意向"));
        assert!(tpl.body.contains("{{name}}"));
    }
}
