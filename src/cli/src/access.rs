//! access（考核）域：招聘考核流程的沟通命令集。
//!
//! 域 = access（考核）；动作 = survey（问卷发放）/ invite（实训邀请）/
//! assess（考核邀请）/ interview（面试通知）。
//! 话术严格照业务实体手册 `qtrecurit/connect/content.md`。

use clap::{Args, Subcommand};

use crate::{exam, interview, invite, survey};

#[derive(Args)]
pub struct AccessArgs {
    #[command(subcommand)]
    pub action: AccessAction,
}

#[derive(Subcommand)]
pub enum AccessAction {
    /// 准入问卷发放：候选人投递后，进入筛选流程前
    Survey(survey::SurveyArgs),
    /// 邀请进群（实训邀请）：准入问卷通过后
    Invite(invite::InviteArgs),
    /// 笔试：发送笔试邀请，候选人以实际成果参与考核
    Exam(exam::ExamArgs),
    /// 面试通知：筛选/考核通过后，安排面试
    Interview(interview::InterviewArgs),
}

pub fn run(args: &AccessArgs) -> anyhow::Result<()> {
    match &args.action {
        AccessAction::Survey(a) => survey::run(a),
        AccessAction::Invite(a) => invite::run(a),
        AccessAction::Exam(a) => exam::run(a),
        AccessAction::Interview(a) => interview::run(a),
    }
}
