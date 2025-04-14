#[cfg(test)]
mod policy_impact_tests {
    use super::*;

    #[test]
    fn test_apply_single_policy_effect() {
        let mut individual = Individual {
            id: 1,
            health: 100.0,
            happiness: 80.0,
            wealth: 50.0,
            community_id: 1,
            // Other fields...
        };
        
        let policy_effect = PolicyEffect {
            health_multiplier: Some(1.2),
            happiness_multiplier: Some(0.9),
            wealth_multiplier: None,  // No effect on wealth
            ..Default::default()
        };
        
        individual.apply_policy_effect(&policy_effect);
        
        assert_eq!(individual.health, 120.0);   // 100 * 1.2
        assert_eq!(individual.happiness, 72.0); // 80 * 0.9
        assert_eq!(individual.wealth, 50.0);    // Unchanged
    }

    #[test]
    fn test_blend_effects_logic() {
        let policy1 = Policy {
            id: 1,
            name: "Policy 1".to_string(),
            community: "city".to_string(),
            effects: PolicyEffect {
                health_multiplier: Some(1.2),
                happiness_multiplier: Some(0.9),
                ..Default::default()
            },
            adoption_rate: 0.6,
        };
        
        let policy2 = Policy {
            id: 2,
            name: "Policy 2".to_string(),
            community: "city".to_string(),
            effects: PolicyEffect {
                health_multiplier: Some(0.8),
                happiness_multiplier: None,
                wealth_multiplier: Some(1.5),
                ..Default::default()
            },
            adoption_rate: 0.4,
        };
        
        let policies = vec![policy1, policy2];
        let blended_effect = blend_policy_effects(&policies);
        
        // Expected health multiplier: (1.2 * 0.6 + 0.8 * 0.4) / (0.6 + 0.4) = 1.04
        assert!((blended_effect.health_multiplier.unwrap() - 1.04).abs() < 0.001);
        
        // Expected happiness multiplier: 0.9 * 0.6 / 0.6 = 0.9 (only one policy affects it)
        assert!((blended_effect.happiness_multiplier.unwrap() - 0.9).abs() < 0.001);
        
        // Expected wealth multiplier: 1.5 * 0.4 / 0.4 = 1.5 (only one policy affects it)
        assert!((blended_effect.wealth_multiplier.unwrap() - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_blend_effects_zero_adoption() {
        let policy1 = Policy {
            id: 1,
            name: "Zero Adoption".to_string(),
            community: "city".to_string(),
            effects: PolicyEffect {
                health_multiplier: Some(1.2),
                ..Default::default()
            },
            adoption_rate: 0.0,  // Zero adoption rate
        };
        
        let policy2 = Policy {
            id: 2,
            name: "Also Zero".to_string(),
            community: "city".to_string(),
            effects: PolicyEffect {
                health_multiplier: Some(0.8),
                ..Default::default()
            },
            adoption_rate: 0.0,  // Zero adoption rate
        };
        
        let policies = vec![policy1, policy2];
        let blended_effect = blend_policy_effects(&policies);
        
        // When all adoption rates are zero, should return neutral effects (1.0)
        assert_eq!(blended_effect.health_multiplier, None);
    }

    #[test]
    fn test_blend_effects_single_policy() {
        let policy = Policy {
            id: 1,
            name: "Solo Policy".to_string(),
            community: "city".to_string(),
            effects: PolicyEffect {
                health_multiplier: Some(1.5),
                happiness_multiplier: Some(0.75),
                ..Default::default()
            },
            adoption_rate: 0.8,
        };
        
        let policies = vec![policy];
        let blended_effect = blend_policy_effects(&policies);
        
        // With only one policy, effect should be exactly that policy's effect
        assert_eq!(blended_effect.health_multiplier, Some(1.5));
        assert_eq!(blended_effect.happiness_multiplier, Some(0.75));
    }
}