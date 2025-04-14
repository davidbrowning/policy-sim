#[cfg(test)]
mod metrics_tests {
    use super::*;
    use chrono::{Utc, TimeZone};
    use std::collections::HashMap;

    #[test]
    fn test_schedule_metric_update_logic() {
        let metrics_system = MetricsSystem::new();
        
        // Daily metrics should update every tick
        assert!(metrics_system.should_update_metric("population_count", 1));
        assert!(metrics_system.should_update_metric("population_count", 2));
        
        // Monthly metrics should update every 30 ticks
        assert!(metrics_system.should_update_metric("average_health", 30));
        assert!(!metrics_system.should_update_metric("average_health", 31));
        assert!(metrics_system.should_update_metric("average_health", 60));
        
        // Yearly metrics should update every 365 ticks
        assert!(metrics_system.should_update_metric("birth_rate", 365));
        assert!(!metrics_system.should_update_metric("birth_rate", 366));
        assert!(metrics_system.should_update_metric("birth_rate", 730));  // 2 years
    }

    #[test]
    fn test_calculate_predefined_metrics() {
        let mut metrics_system = MetricsSystem::new();
        
        // Create test individuals
        let individuals = vec![
            Individual {
                id: 1,
                health: 80.0,
                happiness: 70.0,
                wealth: 100.0,
                community_id: 1,
                age: 30,
                children: 2,
                // Other fields...
            },
            Individual {
                id: 2,
                health: 60.0,
                happiness: 90.0,
                wealth: 50.0,
                community_id: 1,
                age: 25,
                children: 0,
                // Other fields...
            },
            Individual {
                id: 3,
                health: 100.0,
                happiness: 60.0,
                wealth: 150.0,
                community_id: 2,
                age: 40,
                children: 3,
                // Other fields...
            },
        ];
        
        // Calculate metrics
        let metrics = metrics_system.calculate_predefined_metrics(&individuals);
        
        // Verify calculations
        assert_eq!(metrics.get("population_count").unwrap(), &3.0);
        
        // Average health should be (80 + 60 + 100) / 3 = 80
        assert!((metrics.get("average_health").unwrap() - 80.0).abs() < 0.001);
        
        // Average happiness should be (70 + 90 + 60) / 3 = 73.33
        assert!((metrics.get("average_happiness").unwrap() - 73.333).abs() < 0.001);
        
        // Average wealth should be (100 + 50 + 150) / 3 = 100
        assert!((metrics.get("average_wealth").unwrap() - 100.0).abs() < 0.001);
        
        // Community 1 population should be 2
        assert_eq!(metrics.get("community_1_population").unwrap(), &2.0);
        
        // Community 2 population should be 1
        assert_eq!(metrics.get("community_2_population").unwrap(), &1.0);
        
        // Birth rate calculation depends on implementation, but should be 5/3 = 1.67 children per individual
        assert!((metrics.get("birth_rate").unwrap() - 1.667).abs() < 0.01);
    }

    #[test]
    fn test_parse_custom_metric_formula() {
        let yaml = r#"
            name: "custom_wellbeing_index"
            formula: "2 * average_health + average_happiness + 0.5 * average_wealth"
            description: "A weighted index of wellbeing"
            update_frequency: "monthly"
        "#;
        
        let custom_metric = parse_custom_metric(yaml).unwrap();
        
        assert_eq!(custom_metric.name, "custom_wellbeing_index");
        assert_eq!(custom_metric.formula, "2 * average_health + average_happiness + 0.5 * average_wealth");
        assert_eq!(custom_metric.update_frequency, "monthly");
        
        // Test with invalid formula syntax
        let invalid_yaml = r#"
            name: "broken_metric"
            formula: "2 * average_health + /"
            update_frequency: "daily"
        "#;
        
        let result = parse_custom_metric(invalid_yaml);
        assert!(result.is_err());
    }
}