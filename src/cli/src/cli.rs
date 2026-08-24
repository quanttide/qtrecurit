use clap::{Args, Parser, Subcommand};

use crate::{access, refer, report};

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
    /// 考核（access）域：招聘考核流程的沟通命令集
    Access(access::AccessArgs),
    /// 管理本地缓存
    Cache(CacheArgs),
}

#[derive(Args)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub action: CacheAction,
}

#[derive(Subcommand)]
pub enum CacheAction {
    /// 刷新问卷链接缓存（从 HR 邮箱获取最新链接）
    RefreshSurvey,
    /// 查看当前缓存的问卷链接
    ShowSurvey,
    /// 清除问卷链接缓存
    ClearSurvey,
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
        Some(Commands::Access(args)) => access::run(args),
        Some(Commands::Cache(args)) => match &args.action {
            CacheAction::RefreshSurvey => {
                eprintln!("正在从 HR 邮箱获取最新问卷链接...");
                match crate::connect::cache::fetch_survey_url_from_email() {
                    Ok(url) => {
                        if let Err(e) = crate::connect::cache::set_survey_url(&url) {
                            eprintln!("警告: 缓存问卷链接失败: {}", e);
                        }
                        println!("✓ 问卷链接已更新: {}", url);
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("获取问卷链接失败: {e:#}");
                        Err(e)
                    }
                }
            }
            CacheAction::ShowSurvey => {
                match crate::connect::cache::get_survey_url() {
                    Some(url) => {
                        println!("当前缓存的问卷链接: {}", url);
                        Ok(())
                    }
                    None => {
                        eprintln!("缓存中没有问卷链接。请运行 'qtrecurit cache refresh-survey' 获取。");
                        Ok(())
                    }
                }
            }
            CacheAction::ClearSurvey => {
                if let Err(e) = crate::connect::cache::clear_survey_url() {
                    eprintln!("清除缓存失败: {}", e);
                    return;
                }
                println!("✓ 问卷链接缓存已清除");
                Ok(())
            }
        },
        None => Ok(()),
    };
    if let Err(e) = result {
        eprintln!("错误: {e:#}");
    }
}
