#[cfg(test)]
mod individual_tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_choose_action_basic_priorities() {
        // Use a seeded RNG for deterministic tests
        let mut rng = StdRng::seed_from_u64(42);
        
        let mut individual = Individual {
            id: 1,
            health: 100.0,
            happiness: 70.0,
            wealth: 50.0,
            community_id: 1,
            // Other fields...
        };

        let priorities = vec![
            (Action::Farm, 0.6),
            (Action::Hunt, 0.3),
            (Action::Trade, 0.1),
        ];
        
        // Run multiple selections to verify distribution
        let mut action_counts = std::collections::HashMap::new();
        let iterations = 1000;
        
        for _ in 0..iterations {
            let action = individual.choose_action(&priorities, &mut rng);
            *action_counts.entry(action).or_insert(0) += 1;
        }
        
        // Check if distribution roughly matches weights
        let farm_pct = *action_counts.get(&Action::Farm).unwrap_or(&0) as f64 / iterations as f64;
        let hunt_pct = *action_counts.get(&Action::Hunt).unwrap_or(&0) as f64 / iterations as f64;
        let trade_pct = *action_counts.get(&Action::Trade).unwrap_or(&0) as f64 / iterations as f64;
        
        assert!((farm_pct - 0.6).abs() < 0.05);
        assert!((hunt_pct - 0.3).abs() < 0.05);
        assert!((trade_pct - 0.1).abs() < 0.05);
    }

    #[test]
    fn test_choose_action_with_policy_effects() {
        let mut rng = StdRng::seed_from_u64(42);
        
        let mut individual = Individual::new(1, 1);
        
        let base_priorities = vec![
            (Action::Farm, 0.5),
            (Action::Hunt, 0.5),
        ];
        
        // Create policy effect that doubles farming weight
        let policy_effect = PolicyEffect {
            action_weights: vec![(Action::Farm, 2.0)].into_iter().collect(),
            ..Default::default()
        };
        
        // Apply policy effect
        let modified_priorities = individual.apply_policy_to_priorities(&base_priorities, &policy_effect);
        
        // Check modified weights
        let farm_weight = modified_priorities.iter()
            .find(|(a, _)| *a == Action::Farm)
            .map(|(_, w)| *w)
            .unwrap();
        let hunt_weight = modified_priorities.iter()
            .find(|(a, _)| *a == Action::Hunt)
            .map(|(_, w)| *w)
            .unwrap();
        
        assert_eq!(farm_weight, 1.0);  // 0.5 * 2.0
        assert_eq!(hunt_weight, 0.5);  // Unchanged
        
        // Verify distribution with modified weights
        let mut action_counts = std::collections::HashMap::new();
        let iterations = 1000;
        
        for _ in 0..iterations {
            let action = individual.choose_action(&modified_priorities, &mut rng);
            *action_counts.entry(action).or_insert(0) += 1;
        }
        
        let farm_pct = *action_counts.get(&Action::Farm).unwrap_or(&0) as f64 / iterations as f64;
        let hunt_pct = *action_counts.get(&Action::Hunt).unwrap_or(&0) as f64 / iterations as f64;
        
        assert!((farm_pct - 0.67).abs() < 0.05);  // ~2/3
        assert!((hunt_pct - 0.33).abs() < 0.05);  // ~1/3
    }

    #[test]
    fn test_choose_action_edge_cases() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut individual = Individual::new(1, 1);
        
        // Test with zero priorities (empty list)
        let empty_priorities: Vec<(Action, f64)> = vec![];
        assert!(individual.choose_action(&empty_priorities, &mut rng).is_none());
        
        // Test with one priority
        let single_priority = vec![(Action::Farm, 1.0)];
        assert_eq!(individual.choose_action(&single_priority, &mut rng), Some(Action::Farm));
        
        // Test with equal priorities
        let equal_priorities = vec![
            (Action::Farm, 1.0),
            (Action::Hunt, 1.0),
            (Action::Trade, 1.0),
        ];
        
        let mut action_counts = std::collections::HashMap::new();
        let iterations = 3000;
        
        for _ in 0..iterations {
            let action = individual.choose_action(&equal_priorities, &mut rng).unwrap();
            *action_counts.entry(action).or_insert(0) += 1;
        }
        
        // Each action should be chosen approximately 1/3 of the time
        for (_, count) in action_counts.iter() {
            let pct = *count as f64 / iterations as f64;
            assert!((pct - 0.333).abs() < 0.05);
        }
    }

    #[test]
    fn test_apply_environment_multipliers() {
        let mut individual = Individual {
            id: 1,
            health: 100.0,
            happiness: 80.0,
            wealth: 60.0,
            community_id: 1,
            // Other fields...
        };
        
        let geography = Geography {
            name: "Mountain",
            health_multiplier: 0.9,    // Mountains are harsh
            happiness_multiplier: 1.2, // But beautiful
            wealth_multiplier: 0.8,    // Difficult to grow food/trade
        };
        
        individual.apply_environment_multipliers(&geography);
        
        assert_eq!(individual.health, 90.0);    // 100 * 0.9
        assert_eq!(individual.happiness, 96.0); // 80 * 1.2
        assert_eq!(individual.wealth, 48.0);    // 60 * 0.8
    }

    #[test]
    fn test_individual_trait_influence() {
        let mut rng = StdRng::seed_from_u64(42);
        
        // Individual with "industrious" trait that increases farm preference
        let mut industrious_individual = Individual {
            id: 1,
            traits: vec![Trait::Industrious],
            health: 100.0,
            happiness: 80.0,
            wealth: 50.0,
            community_id: 1,
            // Other fields...
        };
        
        // Individual without traits
        let mut regular_individual = Individual {
            id: 2,
            traits: vec![],
            health: 100.0,
            happiness: 80.0,
            wealth: 50.0,
            community_id: 1,
            // Other fields...
        };
        
        let base_priorities = vec![
            (Action::Farm, 0.4),
            (Action::Hunt, 0.3),
            (Action::Trade, 0.3),
        ];
        
        // Get modified priorities for industrious individual
        let industrious_priorities = industrious_individual.apply_trait_effects(&base_priorities);
        
        // The industrious trait should increase farm weight
        let industrious_farm_weight = industrious_priorities.iter()
            .find(|(a, _)| *a == Action::Farm)
            .map(|(_, w)| *w)
            .unwrap();
            
        // Regular individual should have unchanged priorities
        let regular_priorities = regular_individual.apply_trait_effects(&base_priorities);
        let regular_farm_weight = regular_priorities.iter()
            .find(|(a, _)| *a == Action::Farm)
            .map(|(_, w)| *w)
            .unwrap();
        
        assert!(industrious_farm_weight > regular_farm_weight);
        
        // Run simulation to verify trait influence on decisions
        let mut industrious_choices = std::collections::HashMap::new();
        let mut regular_choices = std::collections::HashMap::new();
        let iterations = 1000;
        
        for _ in 0..iterations {
            let i_action = industrious_individual.choose_action(&industrious_priorities, &mut rng).unwrap();
            let r_action = regular_individual.choose_action(&regular_priorities, &mut rng).unwrap();
            
            *industrious_choices.entry(i_action).or_insert(0) += 1;
            *regular_choices.entry(r_action).or_insert(0) += 1;
        }
        
        // Industrious individual should farm more often
        let i_farm_pct = *industrious_choices.get(&Action::Farm).unwrap_or(&0) as f64 / iterations as f64;
        let r_farm_pct = *regular_choices.get(&Action::Farm).unwrap_or(&0) as f64 / iterations as f64;
        
        assert!(i_farm_pct > r_farm_pct);
    }
}