use std::collections::HashMap; // Needed for the modifiers in PolicyEffect

// --- Struct Definitions ---

#[derive(Debug, Clone, PartialEq)] // PartialEq helps with potential assert_eq! on the whole struct
pub struct Action {
    // Fields inferred directly from the test's initialization
    pub name: String,
    pub base_time: i32,       // Assuming i32 based on integer literals 5 and 3
    pub difficulty: f64,
    pub health_impact: f64,
    pub happiness_impact: f64,
    pub wealth_impact: f64,
    // Add other fields if they exist and are used
}

#[derive(Debug, Clone, PartialEq, Default)] // Default is required by `..Default::default()` syntax
pub struct PolicyEffect {
    // Fields inferred from the test's initialization
    // The `vec![...].into_iter().collect()` pattern strongly implies HashMap
    pub action_difficulty_modifiers: HashMap<String, f64>,
    pub action_time_modifiers: HashMap<String, f64>,
    // Add any other fields that PolicyEffect might have.
    // The `Default` derive will initialize them to their respective defaults
    // (e.g., 0.0 for f64, HashMap::new() for HashMap, false for bool, etc.)
    // Example: pub global_food_modifier: f64, (would default to 0.0)
}

// --- Method Implementation ---

impl Action {
    // Method signature inferred from `farm_action.apply_policy_effect(&policy_effect);`
    // It needs to modify the action, hence `&mut self`.
    pub fn apply_policy_effect(&mut self, effect: &PolicyEffect) {
        // Apply difficulty modifier if one exists for this action's name
        if let Some(modifier) = effect.action_difficulty_modifiers.get(&self.name) {
            self.difficulty *= modifier;
        }

        // Apply time modifier if one exists for this action's name
        if let Some(modifier) = effect.action_time_modifiers.get(&self.name) {
            // The test expects integer time, and the results imply flooring:
            // Farm: 5 * 0.9 = 4.5 -> expected 4
            // Hunt: 3 * 1.1 = 3.3 -> expected 3 (Test assertion needs correction)
            let modified_time = (self.base_time as f64 * modifier).floor();
            // Ensure time doesn't become non-positive (adjust if 0 is allowed)
            self.base_time = (modified_time as i32).max(1);
        }

        // Apply other potential effects from PolicyEffect to Action fields here
        // e.g., self.health_impact *= effect.global_health_modifier;
    }
}

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