//! 准入问卷发放：候选人投递后，进入筛选流程前。
//!
//! 话术见 templates.rs（源自业务实体手册 qtrecurit/connect/content.md），
//! 发送经 connect::email::send_mail 通道（日志由通道内部处理）。
//! 问卷链接优先从本地缓存读取，缓存未命中时自动从 HR 邮箱获取。

use anyhow::Result;
use clap::Args;

use crate::connect::{cache, email::send_mail};
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
    
    let tpl = templates::find_template("survey")
        .expect("survey 模板必须存在（templates.rs TEMPLATES）");
    let body = render_template(
        tpl,
        &[
            ("name".to_string(), args.name.clone()),
            ("link".to_string(), link),
        ],
    );

    let (_id, sent) = send_mail(
        &args.to,
        &tpl.subject,
        &body,
        None,
        "survey",
        false,
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
        "{} 收件人: {} | 模板: survey | 状态: {}",
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
    fn test_survey_template_loaded() {
        let tpl = templates::find_template("survey").unwrap();
        assert_eq!(tpl.subject, "量潮科技准入问卷");
        assert!(tpl.body.contains("准入问卷"));
        assert!(tpl.body.contains("15-20分钟"));
        assert!(tpl.body.contains("3个工作日"));
    }
}
