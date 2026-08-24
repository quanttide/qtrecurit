//! 凭证化人才推荐：生成凭证号 → 推荐信 → 发送。
//!
//! 无状态命令：凭证号仅出现在推荐信与输出中，不持久化（推荐记录待 Provider 关联）。
//! 发送经 connect::email::send_mail 通道（发送日志由通道内部处理）。

use anyhow::Result;
use chrono::Local;
use clap::Args;

use crate::connect::email::send_mail;

#[derive(Args)]
pub struct ReferArgs {
    /// 候选人姓名
    #[arg(long)]
    pub name: String,

    /// 候选人邮箱
    #[arg(long)]
    pub candidate_email: String,

    /// 目标企业名
    #[arg(long)]
    pub company: String,

    /// 确认后直接发送（默认只生成草稿）
    #[arg(long)]
    pub confirm_send: bool,

    /// 发送前打印将执行的命令，不执行
    #[arg(long)]
    pub dry_run: bool,
}

/// 生成凭证号：REF-YYYYMMDD-NNN
pub fn gen_referral_code(now: chrono::DateTime<chrono::Local>, seq: u32) -> String {
    format!("REF-{}-{:03}", now.format("%Y%m%d"), seq)
}

/// 推荐邮件正文（只给已验证事实，不给考核评级/判分细节）
pub fn build_referral_body(name: &str, company: &str, code: &str) -> String {
    format!(
        r#"您好，

我们向贵司（{company}）推荐候选人 {name}。

候选人经量潮招聘流程评估，材料真实。以下为已验证事实：
- 候选人基本信息、简历、问卷材料均与我们核实一致
- 评估过程无诚信问题

如需进一步了解候选人的具体情况，欢迎随时联系我们。

此邮件由量潮推荐系统自动生成，凭证号：{code}。

量潮科技 招聘团队"#
    )
}

pub fn run(args: &ReferArgs) -> Result<()> {
    let now = Local::now();
    let seq = (now.timestamp() % 1000) as u32;
    let code = gen_referral_code(now, seq);

    let subject = format!("人才推荐：{} → {}", args.name, args.company);
    let body = build_referral_body(&args.name, &args.company, &code);

    println!("=== 推荐凭证 ===");
    println!("凭证号: {}", code);
    println!("候选人: {} <{}>", args.name, args.candidate_email);
    println!("企业:   {}", args.company);
    println!();

    let (_id, sent) = send_mail(
        &args.candidate_email,
        &subject,
        &body,
        None,
        "refer",
        args.confirm_send,
        args.dry_run,
    )
    .map_err(|e| anyhow::anyhow!("发送推荐信失败: {e:#}"))?;

    let status = if args.dry_run {
        "dry-run".to_string()
    } else if sent {
        "sent".to_string()
    } else {
        "draft".to_string()
    };
    println!(
        "{} 凭证 {} | 状态: {}",
        if args.dry_run {
            "[dry-run]"
        } else if sent {
            "✓ 已发送"
        } else {
            "✓ 已生成草稿"
        },
        code,
        status
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen_referral_code_format() {
        let now = chrono::DateTime::parse_from_rfc3339("2026-08-22T10:00:00+08:00")
            .unwrap()
            .with_timezone(&chrono::Local);
        let code = gen_referral_code(now, 42);
        assert_eq!(code, "REF-20260822-042");
    }

    #[test]
    fn test_referral_code_uniqueness_with_seq() {
        let now = chrono::DateTime::parse_from_rfc3339("2026-08-22T10:00:00+08:00")
            .unwrap()
            .with_timezone(&chrono::Local);
        let a = gen_referral_code(now, 1);
        let b = gen_referral_code(now, 2);
        assert_ne!(a, b);
    }

    #[test]
    fn test_build_referral_body() {
        let body = build_referral_body("张三", "示例企业", "REF-20260822-001");
        assert!(body.contains("张三"));
        assert!(body.contains("示例企业"));
        assert!(body.contains("REF-20260822-001"));
        // 只给已验证事实，不给考核评级/判分细节（考核属 access 域，不混入推荐信）
        assert!(!body.contains("51 分"));
        assert!(!body.contains("评分"));
        assert!(!body.contains("判分"));
        assert!(!body.contains("责任心评级"));
        assert!(!body.contains("配合度"));
        assert!(body.contains("材料真实"));
    }
}
