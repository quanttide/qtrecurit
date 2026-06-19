use crate::status;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "qtrecurit", version, about = "量潮招聘 CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 招聘数据统计（面向公开发文）
    Status(status::StatusArgs),
}

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Status(args)) => {
            if let Err(e) = status::run(args) {
                eprintln!("错误: {}", e);
            }
        }
        None => {}
    }
}
