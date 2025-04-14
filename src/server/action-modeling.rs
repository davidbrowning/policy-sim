#[cfg(test)]
mod actions_tests {
    use super::*;

    #[test]
    fn test_action_cost_modification() {
        // Create base actions
        let mut farm_action = Action {
            name: "Farm".to_string(),
            base_time: 5,       // Takes 5 ticks
            difficulty: 2.0,    // Medium difficulty
            health_impact: -5.0, // Slight negative health impact
            happiness_impact: -2.0,
            wealth_impact: 10.0, // Good wealth gain
            // Other fields...
        };
        
        let mut hunt_action = Action {
            name: "Hunt".to_string(),
            base_time: 3,       // Takes 3 ticks
            difficulty: 4.0,    // Higher difficulty
            health_impact: -10.0, // Larger health impact
            happiness_impact: 5.0, // Fun but dangerous
            wealth_impact: 15.0,  // Good wealth if successful
            // Other fields...
        };
        
        // Create policy effect that makes farming easier and hunting harder
        let policy_effect = PolicyEffect {
            action_difficulty_modifiers: vec![
                ("Farm".to_string(), 0.8),  // 20% easier
                ("Hunt".to_string(), 1.2),  // 20% harder
            ].into_iter().collect(),
            action_time_modifiers: vec![
                ("Farm".to_string(), 0.9),  // 10% faster
                ("Hunt".to_string(), 1.1),  // 10% slower
            ].into_iter().collect(),
            ..Default::default()
        };
        
        // Apply policy effects to actions
        farm_action.apply_policy_effect(&policy_effect);
        hunt_action.apply_policy_effect(&policy_effect);
        
        // Verify modifications
        assert_eq!(farm_action.difficulty, 1.6);  // 2.0 * 0.8
        assert_eq!(hunt_action.difficulty, 4.8);  // 4.0 * 1.2
        
        // Time adjustments - might need rounding depending on implementation
        assert_eq!(farm_action.base_time, 4);     // 5 * 0.9 rounded down
        assert_eq!(hunt_action.base_time, 3);     // 3 * 1.1 rounded down
    }
}