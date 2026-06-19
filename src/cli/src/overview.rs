use crate::human;
use crate::meta::MasterDataManager;

/// 主数据关联概览
pub fn generate_master_data_overview() -> String {
    let rules = human::config::load_config();
    let plan = human::status::default_plan();
    let plan_index = MasterDataManager::build_plan_position_index(&plan);

    let mut out = String::new();
    out.push_str("# 主数据关联概览\n\n");

    out.push_str(&format!("## 分类规则（{} 条）\n\n", rules.rules.len()));
    for r in &rules.rules {
        let in_plan = if MasterDataManager::is_position_in_plan(&plan_index, &r.name) {
            "✓ 在计划中"
        } else {
            "✗ 不在计划中"
        };
        out.push_str(&format!(
            "- {} [{}] 关键词: {}\n",
            r.name,
            in_plan,
            r.keywords.join(", ")
        ));
    }

    out.push('\n');
    out.push_str(&format!("## 招聘计划（{}）\n\n", plan.month));
    for p in &plan.positions {
        out.push_str(&format!(
            "- {} : 编制 {} · 已入职 {} · 进行中 {}\n",
            p.name, p.headcount, p.filled, p.in_progress
        ));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_master_data_overview_contains_sections() {
        let overview = generate_master_data_overview();
        assert!(overview.contains("主数据关联概览"));
        assert!(overview.contains("分类规则"));
        assert!(overview.contains("招聘计划"));
        assert!(overview.contains("全栈工程师"));
        assert!(overview.contains("数据工程师"));
    }

    #[test]
    fn test_generate_master_data_overview_shows_plan_status() {
        let overview = generate_master_data_overview();
        assert!(overview.contains("✓ 在计划中") || overview.contains("✗ 不在计划中"));
    }
}
