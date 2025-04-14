use crate::server::action_modeling::PolicyEffect;
use crate::server::individual::Individual;
use crate::server::policy_parser::Policy;

pub fn blend_policy_effects(policies: &[Policy]) -> PolicyEffect {
    let mut total_adoption = 0.0;
    let mut health_sum = 0.0;
    let mut happiness_sum = 0.0;
    let mut wealth_sum = 0.0;
    let mut health_count = 0.0;
    let mut happiness_count = 0.0;
    let mut wealth_count = 0.0;

    for policy in policies {
        let adoption = policy.adoption_rate;
        total_adoption += adoption;
        if let Some(health) = policy.effects.get("health_multiplier") {
            health_sum += health * adoption;
            health_count += adoption;
        }
        if let Some(happiness) = policy.effects.get("happiness_multiplier") {
            happiness_sum += happiness * adoption;
            happiness_count += adoption;
        }
        if let Some(wealth) = policy.effects.get("wealth_multiplier") {
            wealth_sum += wealth * adoption;
            wealth_count += adoption;
        }
    }

    PolicyEffect {
        action_difficulty_modifiers: Default::default(),
        action_time_modifiers: Default::default(),
        action_weights: Default::default(),
        health_multiplier: if health_count > 0.0 {
            Some(health_sum / health_count)
        } else {
            None
        },
        happiness_multiplier: if happiness_count > 0.0 {
            Some(happiness_sum / happiness_count)
        } else {
            None
        },
        wealth_multiplier: if wealth_count > 0.0 {
            Some(wealth_sum / wealth_count)
        } else {
            None
        },
    }
}

#[cfg(test)]
mod policy_impact_tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_apply_single_policy_effect() {
        let mut individual = Individual {
            id: 1,
            health: 100.0,
            happiness: 80.0,
            wealth: 50.0,
            community_id: 1,
            traits: vec![],
            age: 20,
            children: 0,
        };
        
        let policy_effect = PolicyEffect {
            health_multiplier: Some(1.2),
            happiness_multiplier: Some(0.9),
            wealth_multiplier: None,
            ..Default::default()
        };
        
        individual.apply_policy_effect(&policy_effect);
        
        assert_eq!(individual.health, 120.0);
        assert_eq!(individual.happiness, 72.0);
        assert_eq!(individual.wealth, 50.0);
    }

    #[test]
    fn test_blend_effects_logic() {
        let policy1 = Policy {
            id: 1,
            name: "Policy 1".to_string(),
            community: "city".to_string(),
            effects: HashMap::from([
                ("health_multiplier".to_string(), 1.2),
                ("happiness_multiplier".to_string(), 0.9),
            ]),
            adoption_rate: 0.6,
        };
        
        let policy2 = Policy {
            id: 2,
            name: "Policy 2".to_string(),
            community: "city".to_string(),
            effects: HashMap::from([
                ("health_multiplier".to_string(), 0.8),
                ("wealth_multiplier".to_string(), 1.5),
            ]),
            adoption_rate: 0.4,
        };
        
        let policies = vec![policy1, policy2];
        let blended_effect = blend_policy_effects(&policies);
        
        assert!((blended_effect.health_multiplier.unwrap() - 1.04).abs() < 0.001);
        assert!((blended_effect.happiness_multiplier.unwrap() - 0.9).abs() < 0.001);
        assert!((blended_effect.wealth_multiplier.unwrap() - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_blend_effects_zero_adoption() {
        let policy1 = Policy {
            id: 1,
            name: "Zero Adoption".to_string(),
            community: "city".to_string(),
            effects: HashMap::from([("health_multiplier".to_string(), 1.2)]),
            adoption_rate: 0.0,
        };
        
        let policy2 = Policy {
            id: 2,
            name: "Also Zero".to_string(),
            community: "city".to_string(),
            effects: HashMap::from([("health_multiplier".to_string(), 0.8)]),
            adoption_rate: 0.0,
        };
        
        let policies = vec![policy1, policy2];
        let blended_effect = blend_policy_effects(&policies);
        
        assert_eq!(blended_effect.health_multiplier, None);
    }

    #[test]
    fn test_blend_effects_single_policy() {
        let policy = Policy {
            id: 1,
            name: "Solo Policy".to_string(),
            community: "city".to_string(),
            effects: HashMap::from([
                ("health_multiplier".to_string(), 1.5),
                ("happiness_multiplier".to_string(), 0.75),
            ]),
            adoption_rate: 0.8,
        };
        
        let policies = vec![policy];
        let blended_effect = blend_policy_effects(&policies);
        
        assert_eq!(blended_effect.health_multiplier, Some(1.5));
        assert_eq!(blended_effect.happiness_multiplier, Some(0.75));
    }
}
