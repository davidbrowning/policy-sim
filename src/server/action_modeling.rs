use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub name: String,
    pub base_time: i32,
    pub difficulty: f64,
    pub health_impact: f64,
    pub happiness_impact: f64,
    pub wealth_impact: f64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PolicyEffect {
    pub action_difficulty_modifiers: HashMap<String, f64>,
    pub action_time_modifiers: HashMap<String, f64>,
    pub action_weights: HashMap<String, f64>,
    pub health_multiplier: Option<f64>,
    pub happiness_multiplier: Option<f64>,
    pub wealth_multiplier: Option<f64>,
}

impl Action {
    pub fn apply_policy_effect(&mut self, effect: &PolicyEffect) {
        if let Some(modifier) = effect.action_difficulty_modifiers.get(&self.name) {
            self.difficulty *= modifier;
        }
        if let Some(modifier) = effect.action_time_modifiers.get(&self.name) {
            let modified_time = (self.base_time as f64 * modifier).floor();
            self.base_time = (modified_time as i32).max(1);
        }
    }
}

// Tests remain the same

#[cfg(test)]
mod actions_tests {
    use super::*;

    #[test]
    fn test_action_cost_modification() {
        let mut farm_action = Action {
            name: "Farm".to_string(),
            base_time: 5,
            difficulty: 2.0,
            health_impact: -5.0,
            happiness_impact: -2.0,
            wealth_impact: 10.0,
        };
        
        let mut hunt_action = Action {
            name: "Hunt".to_string(),
            base_time: 3,
            difficulty: 4.0,
            health_impact: -10.0,
            happiness_impact: 5.0,
            wealth_impact: 15.0,
        };
        
        let policy_effect = PolicyEffect {
            action_difficulty_modifiers: vec![
                ("Farm".to_string(), 0.8),
                ("Hunt".to_string(), 1.2),
            ].into_iter().collect(),
            action_time_modifiers: vec![
                ("Farm".to_string(), 0.9),
                ("Hunt".to_string(), 1.1),
            ].into_iter().collect(),
        };
        
        farm_action.apply_policy_effect(&policy_effect);
        hunt_action.apply_policy_effect(&policy_effect);
        
        assert_eq!(farm_action.difficulty, 1.6); // 2.0 * 0.8
        assert_eq!(hunt_action.difficulty, 4.8); // 4.0 * 1.2
        assert_eq!(farm_action.base_time, 4);    // 5 * 0.9 = 4.5, floored to 4
        assert_eq!(hunt_action.base_time, 3);    // 3 * 1.1 = 3.3, floored to 3
    }
}
