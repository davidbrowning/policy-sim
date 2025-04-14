#[cfg(test)]
mod events_tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_event_apply_logic() {
        // Create a test earthquake event
        let earthquake = Event {
            id: 1,
            name: "Earthquake".to_string(),
            description: "A major earthquake hits the region".to_string(),
            probability: 0.01,
            effects: EventEffect {
                health_impact: -20.0,
                happiness_impact: -15.0,
                wealth_impact: -30.0,
                duration_ticks: 10,
            },
        };
        
        // Create some test individuals
        let mut individuals = vec![
            Individual {
                id: 1,
                health: 100.0,
                happiness: 80.0,
                wealth: 100.0,
                // Other fields...
            },
            Individual {
                id: 2,
                health: 90.0,
                happiness: 70.0,
                wealth: 120.0,
                // Other fields...
            },
        ];
        
        // Apply earthquake event
        earthquake.apply(&mut individuals);
        
        // Verify impacts on individuals
        assert_eq!(individuals[0].health, 80.0);    // 100 - 20
        assert_eq!(individuals[0].happiness, 65.0); // 80 - 15
        assert_eq!(individuals[0].wealth, 70.0);    // 100 - 30
        
        assert_eq!(individuals[1].health, 70.0);    // 90 - 20
        assert_eq!(individuals[1].happiness, 55.0); // 70 - 15
        assert_eq!(individuals[1].wealth, 90.0);    // 120 - 30
    }

    #[test]
    fn test_event_trigger_probability() {
        // Create events with different probabilities
        let common_event = Event {
            id: 1,
            name: "Common Event".to_string(),
            description: "Happens frequently".to_string(),
            probability: 0.8, // 80% chance
            effects: EventEffect::default(),
        };
        
        let rare_event = Event {
            id: 2,
            name: "Rare Event".to_string(),
            description: "Happens rarely".to_string(),
            probability: 0.05, // 5% chance
            effects: EventEffect::default(),
        };
        
        // Use seeded RNG for deterministic tests
        let mut rng = StdRng::seed_from_u64(42);
        
        // Run many trials to test probability
        let trials = 1000;
        let mut common_triggers = 0;
        let mut rare_triggers = 0;
        
        for _ in 0..trials {
            if common_event.should_trigger(&mut rng) {
                common_triggers += 1;
            }
            
            if rare_event.should_trigger(&mut rng) {
                rare_triggers += 1;
            }
        }
        
        // Common event should trigger close to 80% of the time
        let common_rate = common_triggers as f64 / trials as f64;
        assert!((common_rate - 0.8).abs() < 0.05);
        
        // Rare event should trigger close to 5% of the time
        let rare_rate = rare_triggers as f64 / trials as f64;
        assert!((rare_rate - 0.05).abs() < 0.03);
    }
}