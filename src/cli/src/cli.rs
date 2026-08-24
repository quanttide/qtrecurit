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
    /// 刷新文件夹 ID 缓存（从 HR 邮箱获取最新 ID）
    RefreshFolderId(RefreshFolderIdArgs),
    /// 查看当前缓存的文件夹 ID
    ShowFolderId(ShowFolderIdArgs),
    /// 清除文件夹 ID 缓存
    ClearFolderId(ClearFolderIdArgs),
    /// 设置模板数据源 URL
    SetTemplateSource(SetTemplateSourceArgs),
    /// 查看当前缓存的模板数据源 URL
    ShowTemplateSource(ShowTemplateSourceArgs),
    /// 清除模板数据源缓存
    ClearTemplateSource(ClearTemplateSourceArgs),
}

#[derive(Args)]
pub struct RefreshFolderIdArgs {
    /// 文件夹名称（如：sent-survey）
    #[arg(long)]
    pub name: String,
}

#[derive(Args)]
pub struct ShowFolderIdArgs {
    /// 文件夹名称（如：sent-survey）
    #[arg(long)]
    pub name: String,
}

#[derive(Args)]
pub struct ClearFolderIdArgs {
    /// 文件夹名称（如：sent-survey）
    #[arg(long)]
    pub name: String,
}

#[derive(Args)]
pub struct SetTemplateSourceArgs {
    /// 模板名称（如：invite）
    #[arg(long)]
    pub name: String,
    /// 数据源 URL
    #[arg(long)]
    pub url: String,
}

#[derive(Args)]
pub struct ShowTemplateSourceArgs {
    /// 模板名称（如：invite）
    #[arg(long)]
    pub name: String,
}

#[derive(Args)]
pub struct ClearTemplateSourceArgs {
    /// 模板名称（如：invite）
    #[arg(long)]
    pub name: String,
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
            CacheAction::RefreshFolderId(args) => {
                eprintln!("正在从 HR 邮箱获取文件夹 '{}' 的 ID...", args.name);
                match crate::connect::cache::fetch_folder_id_from_email(&args.name) {
                    Ok(id) => {
                        if let Err(e) = crate::connect::cache::set_folder_id(&args.name, &id) {
                            eprintln!("警告: 缓存文件夹 ID 失败: {}", e);
                        }
                        println!("✓ 文件夹 ID 已更新: {}", id);
                        Ok(())
                    }
                    Err(e) => {
                        eprintln!("获取文件夹 ID 失败: {e:#}");
                        Err(e)
                    }
                }
            }
            CacheAction::ShowFolderId(args) => {
                match crate::connect::cache::get_folder_id(&args.name) {
                    Some(id) => {
                        println!("{}", id);
                        Ok(())
                    }
                    None => {
                        eprintln!("缓存中没有文件夹 '{}' 的 ID。请运行 'qtrecurit cache refresh-folder-id --name {}' 获取。", args.name, args.name);
                        Ok(())
                    }
                }
            }
            CacheAction::ClearFolderId(args) => {
                if let Err(e) = crate::connect::cache::clear_folder_id(&args.name) {
                    eprintln!("清除缓存失败: {}", e);
                    return;
                }
                println!("✓ 文件夹 '{}' 的 ID 缓存已清除", args.name);
                Ok(())
            }
            CacheAction::SetTemplateSource(args) => {
                if let Err(e) = crate::connect::cache::set_template_source(&args.name, &args.url) {
                    eprintln!("缓存模板数据源失败: {}", e);
                    return;
                }
                println!("✓ 模板 '{}' 数据源已缓存: {}", args.name, args.url);
                Ok(())
            }
            CacheAction::ShowTemplateSource(args) => {
                match crate::connect::cache::get_template_source(&args.name) {
                    Some(url) => {
                        println!("{}", url);
                        Ok(())
                    }
                    None => {
                        eprintln!("缓存中没有模板 '{}' 的数据源。", args.name);
                        Ok(())
                    }
                }
            }
            CacheAction::ClearTemplateSource(args) => {
                if let Err(e) = crate::connect::cache::clear_template_source(&args.name) {
                    eprintln!("清除缓存失败: {}", e);
                    return;
                }
                println!("✓ 模板 '{}' 数据源缓存已清除", args.name);
                Ok(())
            }
        },
        None => Ok(()),
    };
    if let Err(e) = result {
        eprintln!("错误: {e:#}");
    }
}
