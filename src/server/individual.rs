use rand::Rng;
use std::collections::HashMap;
use crate::server::action_modeling::{Action, PolicyEffect};
use crate::server::types::Trait;

#[derive(Debug, Clone)]
pub struct Geography {
    pub name: String,
    pub health_multiplier: f64,
    pub happiness_multiplier: f64,
    pub wealth_multiplier: f64,
}

#[derive(Debug, Clone)]
pub struct Individual {
    pub id: u64,
    pub health: f64,
    pub happiness: f64,
    pub wealth: f64,
    pub community_id: u64,
    pub traits: Vec<Trait>,
    pub age: i32,
    pub children: i32,
}

impl Individual {
    pub fn new(id: u64, community_id: u64) -> Self {
        Self {
            id,
            health: 100.0,
            happiness: 80.0,
            wealth: 50.0,
            community_id,
            traits: Vec::new(),
            age: 20,
            children: 0,
        }
    }

    pub fn choose_action<R: Rng>(&self, priorities: &[(Action, f64)], rng: &mut R) -> Option<Action> {
        if priorities.is_empty() {
            return None;
        }
        let total_weight: f64 = priorities.iter().map(|(_, weight)| weight).sum();
        if total_weight <= 0.0 {
            return None;
        }
        let random_value = rng.gen::<f64>() * total_weight;
        let mut cumulative_weight = 0.0;
        for (action, weight) in priorities {
            cumulative_weight += weight;
            if random_value <= cumulative_weight {
                return Some(action.clone());
            }
        }
        priorities.last().map(|(action, _)| action.clone())
    }

    pub fn apply_policy_to_priorities(&self, base_priorities: &[(Action, f64)], policy_effect: &PolicyEffect) -> Vec<(Action, f64)> {
        base_priorities
            .iter()
            .map(|(action, base_weight)| {
                let multiplier = policy_effect.action_weights.get(&action.name).copied().unwrap_or(1.0);
                (action.clone(), base_weight * multiplier)
            })
            .collect()
    }

    pub fn apply_environment_multipliers(&mut self, geography: &Geography) {
        self.health *= geography.health_multiplier;
        self.happiness *= geography.happiness_multiplier;
        self.wealth *= geography.wealth_multiplier;
    }

    pub fn apply_trait_effects(&self, base_priorities: &[(Action, f64)]) -> Vec<(Action, f64)> {
        let mut modified_priorities = base_priorities.to_vec();
        for trait_value in &self.traits {
            match trait_value {
                Trait::Industrious => {
                    if let Some(index) = modified_priorities.iter().position(|(action, _)| action.name == "Farm") {
                        let (action, weight) = modified_priorities[index].clone();
                        modified_priorities[index] = (action, weight * 1.5);
                    }
                }
            }
        }
        modified_priorities
    }

    pub fn apply_policy_effect(&mut self, policy_effect: &PolicyEffect) {
        if let Some(multiplier) = policy_effect.health_multiplier {
            self.health *= multiplier;
        }
        if let Some(multiplier) = policy_effect.happiness_multiplier {
            self.happiness *= multiplier;
        }
        if let Some(multiplier) = policy_effect.wealth_multiplier {
            self.wealth *= multiplier;
        }
    }
}

#[cfg(test)]
mod individual_tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_choose_action_basic_priorities() {
        let mut rng = StdRng::seed_from_u64(42);
        let individual = Individual::new(1, 1);
        let priorities = vec![
            (
                Action {
                    name: "Farm".to_string(),
                    base_time: 5,
                    difficulty: 2.0,
                    health_impact: -5.0,
                    happiness_impact: -2.0,
                    wealth_impact: 10.0,
                },
                0.6,
            ),
            (
                Action {
                    name: "Hunt".to_string(),
                    base_time: 3,
                    difficulty: 4.0,
                    health_impact: -10.0,
                    happiness_impact: 5.0,
                    wealth_impact: 15.0,
                },
                0.3,
            ),
            (
                Action {
                    name: "Trade".to_string(),
                    base_time: 4,
                    difficulty: 3.0,
                    health_impact: 0.0,
                    happiness_impact: 3.0,
                    wealth_impact: 12.0,
                },
                0.1,
            ),
        ];
        let mut action_counts = HashMap::new();
        let iterations = 1000;
        for _ in 0..iterations {
            let action = individual.choose_action(&priorities, &mut rng).unwrap();
            *action_counts.entry(action.name.clone()).or_insert(0) += 1;
        }
        let farm_pct = *action_counts.get("Farm").unwrap_or(&0) as f64 / iterations as f64;
        let hunt_pct = *action_counts.get("Hunt").unwrap_or(&0) as f64 / iterations as f64;
        let trade_pct = *action_counts.get("Trade").unwrap_or(&0) as f64 / iterations as f64;
        assert!((farm_pct - 0.6).abs() < 0.05, "Farm: {}", farm_pct);
        assert!((hunt_pct - 0.3).abs() < 0.05, "Hunt: {}", hunt_pct);
        assert!((trade_pct - 0.1).abs() < 0.05, "Trade: {}", trade_pct);
    }

    #[test]
    fn test_choose_action_with_policy_effects() {
        let mut rng = StdRng::seed_from_u64(42);
        let individual = Individual::new(1, 1);
        let base_priorities = vec![
            (
                Action {
                    name: "Farm".to_string(),
                    base_time: 5,
                    difficulty: 2.0,
                    health_impact: -5.0,
                    happiness_impact: -2.0,
                    wealth_impact: 10.0,
                },
                0.5,
            ),
            (
                Action {
                    name: "Hunt".to_string(),
                    base_time: 3,
                    difficulty: 4.0,
                    health_impact: -10.0,
                    happiness_impact: 5.0,
                    wealth_impact: 15.0,
                },
                0.5,
            ),
        ];
        let policy_effect = PolicyEffect {
            action_weights: vec![("Farm".to_string(), 2.0)].into_iter().collect(),
            action_difficulty_modifiers: HashMap::new(),
            action_time_modifiers: HashMap::new(),
            health_multiplier: None,
            happiness_multiplier: None,
            wealth_multiplier: None,
        };
        let modified_priorities = individual.apply_policy_to_priorities(&base_priorities, &policy_effect);
        let farm_weight = modified_priorities
            .iter()
            .find(|(a, _)| a.name == "Farm")
            .map(|(_, w)| *w)
            .unwrap();
        let hunt_weight = modified_priorities
            .iter()
            .find(|(a, _)| a.name == "Hunt")
            .map(|(_, w)| *w)
            .unwrap();
        assert_eq!(farm_weight, 1.0);
        assert_eq!(hunt_weight, 0.5);
        let mut action_counts = HashMap::new();
        let iterations = 1000;
        for _ in 0..iterations {
            let action = individual.choose_action(&modified_priorities, &mut rng).unwrap();
            *action_counts.entry(action.name.clone()).or_insert(0) += 1;
        }
        let farm_pct = *action_counts.get("Farm").unwrap_or(&0) as f64 / iterations as f64;
        let hunt_pct = *action_counts.get("Hunt").unwrap_or(&0) as f64 / iterations as f64;
        assert!((farm_pct - 0.67).abs() < 0.05, "Farm: {}", farm_pct);
        assert!((hunt_pct - 0.33).abs() < 0.05, "Hunt: {}", hunt_pct);
    }

    #[test]
    fn test_choose_action_edge_cases() {
        let mut rng = StdRng::seed_from_u64(42);
        let individual = Individual::new(1, 1);
        let empty_priorities: Vec<(Action, f64)> = vec![];
        assert!(individual.choose_action(&empty_priorities, &mut rng).is_none());
        let single_priority = vec![(
            Action {
                name: "Farm".to_string(),
                base_time: 5,
                difficulty: 2.0,
                health_impact: -5.0,
                happiness_impact: -2.0,
                wealth_impact: 10.0,
            },
            1.0,
        )];
        assert_eq!(
            individual.choose_action(&single_priority, &mut rng).unwrap().name,
            "Farm"
        );
        let equal_priorities = vec![
            (
                Action {
                    name: "Farm".to_string(),
                    base_time: 5,
                    difficulty: 2.0,
                    health_impact: -5.0,
                    happiness_impact: -2.0,
                    wealth_impact: 10.0,
                },
                1.0,
            ),
            (
                Action {
                    name: "Hunt".to_string(),
                    base_time: 3,
                    difficulty: 4.0,
                    health_impact: -10.0,
                    happiness_impact: 5.0,
                    wealth_impact: 15.0,
                },
                1.0,
            ),
            (
                Action {
                    name: "Trade".to_string(),
                    base_time: 4,
                    difficulty: 3.0,
                    health_impact: 0.0,
                    happiness_impact: 3.0,
                    wealth_impact: 12.0,
                },
                1.0,
            ),
        ];
        let mut action_counts = HashMap::new();
        let iterations = 3000;
        for _ in 0..iterations {
            let action = individual.choose_action(&equal_priorities, &mut rng).unwrap();
            *action_counts.entry(action.name.clone()).or_insert(0) += 1;
        }
        for (_, count) in action_counts.iter() {
            let pct = *count as f64 / iterations as f64;
            assert!((pct - 0.333).abs() < 0.05, "Pct: {}", pct);
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
            traits: vec![],
            age: 20,
            children: 0,
        };
        let geography = Geography {
            name: "Mountain".to_string(),
            health_multiplier: 0.9,
            happiness_multiplier: 1.2,
            wealth_multiplier: 0.8,
        };
        individual.apply_environment_multipliers(&geography);
        assert_eq!(individual.health, 90.0);
        assert_eq!(individual.happiness, 96.0);
        assert_eq!(individual.wealth, 48.0);
    }

    #[test]
    fn test_individual_trait_influence() {
        let mut rng = StdRng::seed_from_u64(42);
        let industrious_individual = Individual {
            id: 1,
            traits: vec![Trait::Industrious],
            health: 100.0,
            happiness: 80.0,
            wealth: 50.0,
            community_id: 1,
            age: 20,
            children: 0,
        };
        let regular_individual = Individual {
            id: 2,
            traits: vec![],
            health: 100.0,
            happiness: 80.0,
            wealth: 50.0,
            community_id: 1,
            age: 20,
            children: 0,
        };
        let base_priorities = vec![
            (
                Action {
                    name: "Farm".to_string(),
                    base_time: 5,
                    difficulty: 2.0,
                    health_impact: -5.0,
                    happiness_impact: -2.0,
                    wealth_impact: 10.0,
                },
                0.4,
            ),
            (
                Action {
                    name: "Hunt".to_string(),
                    base_time: 3,
                    difficulty: 4.0,
                    health_impact: -10.0,
                    happiness_impact: 5.0,
                    wealth_impact: 15.0,
                },
                0.3,
            ),
            (
                Action {
                    name: "Trade".to_string(),
                    base_time: 4,
                    difficulty: 3.0,
                    health_impact: 0.0,
                    happiness_impact: 3.0,
                    wealth_impact: 12.0,
                },
                0.3,
            ),
        ];
        let industrious_priorities = industrious_individual.apply_trait_effects(&base_priorities);
        let regular_priorities = regular_individual.apply_trait_effects(&base_priorities);
        let industrious_farm_weight = industrious_priorities
            .iter()
            .find(|(a, _)| a.name == "Farm")
            .map(|(_, w)| *w)
            .unwrap();
        let regular_farm_weight = regular_priorities
            .iter()
            .find(|(a, _)| a.name == "Farm")
            .map(|(_, w)| *w)
            .unwrap();
        assert!((industrious_farm_weight - 0.6).abs() < 0.0001);
        assert_eq!(regular_farm_weight, 0.4);
        let mut industrious_choices = HashMap::new();
        let mut regular_choices = HashMap::new();
        let iterations = 1000;
        for _ in 0..iterations {
            let i_action = industrious_individual
                .choose_action(&industrious_priorities, &mut rng)
                .unwrap();
            let r_action = regular_individual
                .choose_action(&regular_priorities, &mut rng)
                .unwrap();
            *industrious_choices.entry(i_action.name.clone()).or_insert(0) += 1;
            *regular_choices.entry(r_action.name.clone()).or_insert(0) += 1;
        }
        let i_farm_pct = *industrious_choices.get("Farm").unwrap_or(&0) as f64 / iterations as f64;
        let r_farm_pct = *regular_choices.get("Farm").unwrap_or(&0) as f64 / iterations as f64;
        assert!(i_farm_pct > r_farm_pct, "Industrious: {}, Regular: {}", i_farm_pct, r_farm_pct);
    }
}
