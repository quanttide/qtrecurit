use anyhow::Result;
use std::collections::HashMap;

use crate::connect::EmailFetcher;
use crate::human;

/// 岗位主数据
#[derive(Debug, Clone)]
pub struct PositionMaster {
    pub id: String,
    pub name: String,
    pub department: String,
    pub level: Option<String>,
    pub description: Option<String>,
    pub active: bool,
}

/// 招聘计划主数据
#[derive(Debug, Clone)]
pub struct PlanMaster {
    pub id: String,
    pub month: String,
    pub items: Vec<PlanItem>,
}

#[derive(Debug, Clone)]
pub struct PlanItem {
    pub position_id: String,
    pub position_name: String,
    pub headcount: u32,
    pub filled: u32,
    pub in_progress: u32,
}

/// 分类规则（关联 connect 邮件 → human 岗位）
#[derive(Debug, Clone)]
pub struct ClassificationRule {
    pub id: String,
    pub position_id: String,
    pub keywords: Vec<String>,
    pub exclude: Vec<String>,
    pub priority: i32,
}

/// 主数据管理器
pub struct MasterDataManager;

impl MasterDataManager {
    /// 从分类规则加载岗位主数据
    pub fn load_positions() -> Vec<PositionMaster> {
        let rules = human::config::load_config();
        rules
            .rules
            .iter()
            .enumerate()
            .map(|(i, r)| PositionMaster {
                id: format!("pos-{:03}", i + 1),
                name: r.name.clone(),
                department: String::new(),
                level: None,
                description: None,
                active: true,
            })
            .collect()
    }

    /// 从内置计划加载招聘计划主数据
    pub fn load_plan() -> PlanMaster {
        let plan = human::status::default_plan();
        PlanMaster {
            id: "plan-001".into(),
            month: plan.month,
            items: plan
                .positions
                .into_iter()
                .map(|p| PlanItem {
                    position_id: String::new(),
                    position_name: p.name,
                    headcount: p.headcount,
                    filled: p.filled,
                    in_progress: p.in_progress,
                })
                .collect(),
        }
    }

    /// 将分类规则转换为关联元数据
    pub fn load_classification_rules() -> Vec<ClassificationRule> {
        let rules = human::config::load_config();
        rules
            .rules
            .iter()
            .enumerate()
            .map(|(i, r)| ClassificationRule {
                id: format!("rule-{:03}", i + 1),
                position_id: format!("pos-{:03}", i + 1),
                keywords: r.keywords.clone(),
                exclude: r.exclude.clone(),
                priority: r.priority,
            })
            .collect()
    }

    /// 构建岗位索引（名称 → 岗位主数据）
    pub fn build_position_index(positions: &[PositionMaster]) -> HashMap<&str, &PositionMaster> {
        positions.iter().map(|p| (p.name.as_str(), p)).collect()
    }

    /// 构建规则索引（规则 ID → 规则）
    pub fn build_rule_index(rules: &[ClassificationRule]) -> HashMap<&str, &ClassificationRule> {
        rules.iter().map(|r| (r.id.as_str(), r)).collect()
    }
}

/// 报表编排
pub fn generate_report(fetcher: &dyn EmailFetcher) -> Result<String> {
    let cfg = human::config::load_config();
    let msgs = fetcher.fetch_all()?;

    let items: Vec<human::report::MailItem> = msgs
        .into_iter()
        .map(|m| human::report::MailItem {
            subject: m.subject,
            date: m.date,
        })
        .collect();

    let title = human::report::build_title(None, None, None);
    let items_ref: Vec<&human::report::MailItem> = items.iter().collect();
    let report = human::report::format_report(&items_ref, &cfg.rules, &title);

    Ok(report)
}

/// 带日期范围的报表编排
pub fn generate_report_with_range(
    fetcher: &dyn EmailFetcher,
    start: Option<chrono::NaiveDate>,
    end: Option<chrono::NaiveDate>,
    days: Option<u32>,
) -> Result<String> {
    let cfg = human::config::load_config();
    let msgs = fetcher.fetch_all()?;

    let items: Vec<human::report::MailItem> = msgs
        .into_iter()
        .map(|m| human::report::MailItem {
            subject: m.subject,
            date: m.date,
        })
        .collect();

    let filtered = human::report::filter_by_date(&items, start, end);
    let title = human::report::build_title(start, end, days);
    let report = human::report::format_report(&filtered, &cfg.rules, &title);

    Ok(report)
}

/// 主数据概览
pub fn generate_master_data_overview() -> String {
    let positions = MasterDataManager::load_positions();
    let plan = MasterDataManager::load_plan();
    let rules = MasterDataManager::load_classification_rules();

    let mut out = String::new();
    out.push_str("# 主数据概览\n\n");

    out.push_str(&format!("## 岗位（{} 个）\n\n", positions.len()));
    out.push_str("| ID | 名称 | 状态 |\n");
    out.push_str("|----|------|------|\n");
    for p in &positions {
        let status = if p.active { "启用" } else { "停用" };
        out.push_str(&format!("| {} | {} | {} |\n", p.id, p.name, status));
    }
    out.push('\n');

    out.push_str(&format!("## 招聘计划（{}）\n\n", plan.month));
    out.push_str("| 岗位 | 编制 | 已入职 | 进行中 |\n");
    out.push_str("|------|------|--------|--------|\n");
    for item in &plan.items {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            item.position_name, item.headcount, item.filled, item.in_progress
        ));
    }
    out.push('\n');

    out.push_str(&format!("## 分类规则（{} 条）\n\n", rules.len()));
    for r in &rules {
        out.push_str(&format!(
            "- {} → {} : [{}]\n",
            r.id,
            r.position_id,
            r.keywords.join(", ")
        ));
    }

    out
}

/// 主数据统计
pub struct MasterDataStats {
    pub total_positions: usize,
    pub active_positions: usize,
    pub plan_month: String,
    pub total_headcount: u32,
    pub total_filled: u32,
    pub total_in_progress: u32,
    pub rule_count: usize,
}

pub fn collect_stats() -> MasterDataStats {
    let positions = MasterDataManager::load_positions();
    let plan = MasterDataManager::load_plan();
    let rules = MasterDataManager::load_classification_rules();

    MasterDataStats {
        total_positions: positions.len(),
        active_positions: positions.iter().filter(|p| p.active).count(),
        plan_month: plan.month,
        total_headcount: plan.items.iter().map(|i| i.headcount).sum(),
        total_filled: plan.items.iter().map(|i| i.filled).sum(),
        total_in_progress: plan.items.iter().map(|i| i.in_progress).sum(),
        rule_count: rules.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_positions_count() {
        let positions = MasterDataManager::load_positions();
        assert_eq!(positions.len(), 12);
        assert!(positions.iter().all(|p| p.active));
    }

    #[test]
    fn test_load_positions_id_format() {
        let positions = MasterDataManager::load_positions();
        assert_eq!(positions[0].id, "pos-001");
        assert_eq!(positions[0].name, "全栈工程师");
        assert_eq!(positions[11].id, "pos-012");
        assert_eq!(positions[11].name, "法务实习生");
    }

    #[test]
    fn test_load_plan_month() {
        let plan = MasterDataManager::load_plan();
        assert_eq!(plan.id, "plan-001");
        assert_eq!(plan.month, "2026-06");
    }

    #[test]
    fn test_load_plan_items() {
        let plan = MasterDataManager::load_plan();
        assert_eq!(plan.items.len(), 8);
        assert_eq!(plan.items[0].position_name, "数据工程师");
        assert_eq!(plan.items[0].headcount, 2);
    }

    #[test]
    fn test_load_classification_rules_count() {
        let rules = MasterDataManager::load_classification_rules();
        assert_eq!(rules.len(), 12);
    }

    #[test]
    fn test_load_classification_rules_mapping() {
        let rules = MasterDataManager::load_classification_rules();
        assert_eq!(rules[0].id, "rule-001");
        assert_eq!(rules[0].position_id, "pos-001");
        assert!(rules[0].keywords.contains(&"全栈".to_string()));
    }

    #[test]
    fn test_build_position_index() {
        let positions = MasterDataManager::load_positions();
        let index = MasterDataManager::build_position_index(&positions);
        assert_eq!(index.len(), 12);
        assert!(index.contains_key("全栈工程师"));
        assert_eq!(index.get("全栈工程师").unwrap().id, "pos-001");
    }

    #[test]
    fn test_build_rule_index() {
        let rules = MasterDataManager::load_classification_rules();
        let index = MasterDataManager::build_rule_index(&rules);
        assert_eq!(index.len(), 12);
        assert!(index.contains_key("rule-001"));
    }

    #[test]
    fn test_generate_master_data_overview_contains_sections() {
        let overview = generate_master_data_overview();
        assert!(overview.contains("# 主数据概览"));
        assert!(overview.contains("## 岗位"));
        assert!(overview.contains("## 招聘计划"));
        assert!(overview.contains("## 分类规则"));
        assert!(overview.contains("全栈工程师"));
        assert!(overview.contains("pos-001"));
    }

    #[test]
    fn test_collect_stats_counts() {
        let stats = collect_stats();
        assert_eq!(stats.total_positions, 12);
        assert_eq!(stats.active_positions, 12);
        assert_eq!(stats.plan_month, "2026-06");
        assert_eq!(stats.rule_count, 12);
    }

    #[test]
    fn test_collect_stats_headcount() {
        let stats = collect_stats();
        assert!(stats.total_headcount > 0);
        assert_eq!(stats.total_filled, 0);
    }
}
