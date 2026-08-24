use clap::{Args, Parser, Subcommand};

use crate::{refer, report};

#[derive(Parser)]
#[command(name = "qtrecurit", version, about = "量潮招聘 CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 生成招聘统计报告（面向公开发文）
    Report(ReportArgs),
    /// 凭证化人才推荐：凭证号 → 推荐信 → 发送 → 台账
    Refer(refer::ReferArgs),
}

#[derive(Args)]
pub struct ReportArgs {
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
        Some(Commands::Report(args)) => report::run(args),
        Some(Commands::Refer(args)) => refer::run(args),
        None => Ok(()),
    };
    if let Err(e) = result {
        eprintln!("错误: {e:#}");
    }
}
