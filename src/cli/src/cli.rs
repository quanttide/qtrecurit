use clap::{Args, Parser, Subcommand};

use crate::{mail, referral, status};

#[derive(Parser)]
#[command(name = "qtrecurit", version, about = "量潮招聘 CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 招聘数据统计（面向公开发文）
    Status(StatusArgs),
    /// 凭证化人才推荐：凭证号 → 推荐信 → 发送 → 台账
    Referral(referral::ReferralArgs),
    /// 招聘沟通邮件：按话术模板发送/查看
    Mail(mail::MailArgs),
}

#[derive(Args)]
pub struct StatusArgs {
    /// 统计最近 N 天
    #[arg(long)]
    pub days: Option<u32>,
    /// 开始日期 (YYYY-MM-DD)
    #[arg(long)]
    pub start: Option<String>,
    /// 结束日期 (YYYY-MM-DD)
    #[arg(long)]
    pub end: Option<String>,
}

pub fn run() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Some(Commands::Status(args)) => status::run(args),
        Some(Commands::Referral(args)) => referral::run(args),
        Some(Commands::Mail(args)) => mail::run(args),
        None => Ok(()),
    };
    if let Err(e) = result {
        eprintln!("错误: {e:#}");
    }
}
