pub mod index;

pub use index::{MasterDataManager, PlanPositionIndex, RulePositionIndex};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::human;

    #[test]
    fn test_build_rule_position_index() {
        let index = MasterDataManager::build_rule_position_index();
        assert!(index.rule_to_position.contains_key("全栈工程师"));
        assert!(index.position_to_rules.contains_key("全栈工程师"));
    }

    #[test]
    fn test_build_plan_position_index() {
        let plan = human::status::default_plan();
        let index = MasterDataManager::build_plan_position_index(&plan);
        assert!(index.position_to_plan.contains_key("数据工程师"));
        assert_eq!(index.position_to_plan.len(), 8);
    }

    #[test]
    fn test_is_position_in_plan() {
        let plan = human::status::default_plan();
        let index = MasterDataManager::build_plan_position_index(&plan);
        assert!(MasterDataManager::is_position_in_plan(&index, "数据工程师"));
        assert!(!MasterDataManager::is_position_in_plan(&index, "全栈工程师"));
    }
}
