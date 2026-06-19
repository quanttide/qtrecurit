use std::collections::HashMap;

use crate::human;

/// 岗位 ↔ 分类规则 的关联索引
pub struct RulePositionIndex {
    pub rule_to_position: HashMap<String, String>,
    pub position_to_rules: HashMap<String, Vec<String>>,
}

/// 岗位 ↔ 计划条目 的关联索引
pub struct PlanPositionIndex {
    pub position_to_plan: HashMap<String, human::status::PositionPlan>,
}

/// 主数据关联管理器
pub struct MasterDataManager;

impl MasterDataManager {
    pub fn build_rule_position_index() -> RulePositionIndex {
        let rules = human::config::load_config();
        let mut rule_to_position = HashMap::new();
        let mut position_to_rules: HashMap<String, Vec<String>> = HashMap::new();

        for r in &rules.rules {
            let pos_name = r.name.clone();
            rule_to_position.insert(pos_name.clone(), pos_name.clone());
            position_to_rules
                .entry(pos_name)
                .or_default()
                .push(r.name.clone());
        }

        RulePositionIndex {
            rule_to_position,
            position_to_rules,
        }
    }

    pub fn build_plan_position_index(
        plan: &human::status::RecruitmentPlan,
    ) -> PlanPositionIndex {
        let mut position_to_plan = HashMap::new();
        for p in &plan.positions {
            position_to_plan.insert(p.name.clone(), p.clone());
        }
        PlanPositionIndex {
            position_to_plan,
        }
    }

    pub fn is_position_in_plan(
        index: &PlanPositionIndex,
        position_name: &str,
    ) -> bool {
        index.position_to_plan.contains_key(position_name)
    }
}
