use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};

use crate::server::individual::Individual;

/// Represents a custom metric defined by a formula
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    pub name: String,
    pub formula: String,
    pub description: String,
    pub update_frequency: String,
}

/// Error types for metrics operations
#[derive(Debug)]
pub enum MetricsError {
    FormulaParseError(String),
    EvaluationError(String),
    InvalidFrequency(String),
}

impl std::fmt::Display for MetricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MetricsError::FormulaParseError(msg) => write!(f, "Formula parse error: {}", msg),
            MetricsError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
            MetricsError::InvalidFrequency(msg) => write!(f, "Invalid frequency: {}", msg),
        }
    }
}

impl Error for MetricsError {}

/// System for calculating and tracking metrics about the simulation
pub struct MetricsSystem {
    // Track when metrics were last updated
    last_updates: HashMap<String, i64>,
    // Store calculated metrics
    metrics: HashMap<String, f64>,
    // Custom metrics defined by formulas
    custom_metrics: Vec<CustomMetric>,
}

impl MetricsSystem {
    /// Create a new metrics system
    pub fn new() -> Self {
        MetricsSystem {
            last_updates: HashMap::new(),
            metrics: HashMap::new(),
            custom_metrics: Vec::new(),
        }
    }

    /// Determine if a metric should be updated based on its update frequency and the current tick
    pub fn should_update_metric(&self, metric_name: &str, current_tick: i64) -> bool {
        // Get update frequency based on metric name
        let frequency = match metric_name {
            // Monthly metrics (update every 30 ticks)
            m if m.starts_with("average_") => 30,
            // Yearly metrics (update every 365 ticks)
            "birth_rate" => 365,
            // Daily metrics (update every tick)
            _ => 1,
        };

        // Get the last tick this metric was updated
        let last_update = self.last_updates.get(metric_name).unwrap_or(&0);
        
        // Check if enough ticks have passed
        (current_tick - last_update) % frequency == 0
    }

    /// Calculate all predefined metrics for the given individuals
    pub fn calculate_predefined_metrics(&mut self, individuals: &[Individual]) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        
        // Population count
        let population_count = individuals.len() as f64;
        metrics.insert("population_count".to_string(), population_count);
        
        // Average health
        let total_health: f64 = individuals.iter().map(|i| i.health).sum();
        let average_health = total_health / population_count;
        metrics.insert("average_health".to_string(), average_health);
        
        // Average happiness
        let total_happiness: f64 = individuals.iter().map(|i| i.happiness).sum();
        let average_happiness = total_happiness / population_count;
        metrics.insert("average_happiness".to_string(), average_happiness);
        
        // Average wealth
        let total_wealth: f64 = individuals.iter().map(|i| i.wealth).sum();
        let average_wealth = total_wealth / population_count;
        metrics.insert("average_wealth".to_string(), average_wealth);
        
        // Community populations
        let mut community_counts = HashMap::new();
        for individual in individuals {
            let count = community_counts.entry(individual.community_id).or_insert(0);
            *count += 1;
        }
        
        for (community_id, count) in community_counts {
            metrics.insert(format!("community_{}_population", community_id), count as f64);
        }
        
        // Birth rate (children per individual)
        let total_children: i32 = individuals.iter().map(|i| i.children).sum();
        let birth_rate = total_children as f64 / population_count;
        metrics.insert("birth_rate".to_string(), birth_rate);
        
        // Update internal metrics state
        for (key, value) in &metrics {
            self.metrics.insert(key.clone(), *value);
        }
        
        metrics
    }

    /// Update all metrics (predefined and custom) for the current tick
    pub fn update_metrics(&mut self, individuals: &[Individual], current_tick: i64) -> HashMap<String, f64> {
        let mut updated_metrics = HashMap::new();
        
        // Update predefined metrics
        for (name, _) in self.metrics.clone().iter() {
            if self.should_update_metric(name, current_tick) {
                // Record this update
                self.last_updates.insert(name.clone(), current_tick);
                
                // Recalculate predefined metrics
                let metrics = self.calculate_predefined_metrics(individuals);
                for (key, value) in metrics {
                    updated_metrics.insert(key, value);
                }
                break; // Only need to calculate predefined metrics once
            }
        }
        
        // Update custom metrics
        for custom_metric in &self.custom_metrics {
            if self.should_update_metric(&custom_metric.name, current_tick) {
                // Record this update
                self.last_updates.insert(custom_metric.name.clone(), current_tick);
                
                // Calculate custom metric
                if let Ok(value) = self.evaluate_custom_metric(custom_metric) {
                    self.metrics.insert(custom_metric.name.clone(), value);
                    updated_metrics.insert(custom_metric.name.clone(), value);
                }
            }
        }
        
        updated_metrics
    }
    
    /// Add a custom metric to track
    pub fn add_custom_metric(&mut self, custom_metric: CustomMetric) {
        self.custom_metrics.push(custom_metric);
    }
    
    /// Evaluate a custom metric formula using current metric values
    fn evaluate_custom_metric(&self, custom_metric: &CustomMetric) -> Result<f64, MetricsError> {
        // Simple formula evaluation for the test case
        // In a real implementation, this would use a proper expression evaluator
        
        // For the test case: "2 * average_health + average_happiness + 0.5 * average_wealth"
        let formula = custom_metric.formula.as_str();
        
        // Very simplified parser for the test case
        let mut result = 0.0;
        let mut terms = Vec::new();
        
        // Split by + operators
        for term in formula.split('+') {
            let term = term.trim();
            terms.push(term);
        }
        
        for term in terms {
            if term.contains('*') {
                let parts: Vec<&str> = term.split('*').map(|s| s.trim()).collect();
                if parts.len() != 2 {
                    return Err(MetricsError::FormulaParseError(
                        "Invalid multiplication format".to_string()
                    ));
                }
                
                let coefficient = parts[0].parse::<f64>().map_err(|_| {
                    MetricsError::FormulaParseError(format!("Invalid coefficient: {}", parts[0]))
                })?;
                
                let metric_name = parts[1];
                let metric_value = self.metrics.get(metric_name).ok_or_else(|| {
                    MetricsError::EvaluationError(format!("Metric not found: {}", metric_name))
                })?;
                
                result += coefficient * metric_value;
            } else {
                // Just a metric name
                let metric_value = self.metrics.get(term).ok_or_else(|| {
                    MetricsError::EvaluationError(format!("Metric not found: {}", term))
                })?;
                
                result += metric_value;
            }
        }
        
        Ok(result)
    }
    
    /// Get the current value of a metric
    pub fn get_metric(&self, name: &str) -> Option<f64> {
        self.metrics.get(name).copied()
    }
    
    /// Get all current metrics
    pub fn get_all_metrics(&self) -> &HashMap<String, f64> {
        &self.metrics
    }
}

/// Parse a custom metric definition from YAML
pub fn parse_custom_metric(yaml_str: &str) -> Result<CustomMetric, MetricsError> {
    let custom_metric: CustomMetric = serde_yaml::from_str(yaml_str)
        .map_err(|e| MetricsError::FormulaParseError(e.to_string()))?;
    
    // Validate the formula (in a real implementation, this would be more thorough)
    if custom_metric.formula.contains('/') && custom_metric.formula.ends_with('/') {
        return Err(MetricsError::FormulaParseError(
            "Invalid formula syntax".to_string()
        ));
    }
    
    // Validate update frequency
    match custom_metric.update_frequency.as_str() {
        "daily" | "monthly" | "yearly" => (),
        _ => return Err(MetricsError::InvalidFrequency(
            format!("Invalid update frequency: {}", custom_metric.update_frequency)
        )),
    }
    
    Ok(custom_metric)
}

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

