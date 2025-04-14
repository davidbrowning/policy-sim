use chrono::Utc;
use std::collections::HashMap;
use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::server::individual::Individual;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    pub name: String,
    pub formula: String,
    pub description: String,
    pub update_frequency: String,
}

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

pub struct MetricsSystem {
    last_updates: HashMap<String, i64>,
    metrics: HashMap<String, f64>,
    custom_metrics: Vec<CustomMetric>,
}

impl MetricsSystem {
    pub fn new() -> Self {
        MetricsSystem {
            last_updates: HashMap::new(),
            metrics: HashMap::new(),
            custom_metrics: Vec::new(),
        }
    }

    pub fn should_update_metric(&self, metric_name: &str, current_tick: i64) -> bool {
        let frequency = match metric_name {
            m if m.starts_with("average_") => 30,
            "birth_rate" => 365,
            _ => 1,
        };
        let last_update = self.last_updates.get(metric_name).unwrap_or(&0);
        (current_tick - last_update) % frequency == 0
    }

    pub fn calculate_predefined_metrics(&mut self, individuals: &[Individual]) -> HashMap<String, f64> {
        let mut metrics = HashMap::new();
        let population_count = individuals.len() as f64;
        metrics.insert("population_count".to_string(), population_count);
        let total_health: f64 = individuals.iter().map(|i| i.health).sum();
        let average_health = if population_count > 0.0 { total_health / population_count } else { 0.0 };
        metrics.insert("average_health".to_string(), average_health);
        let total_happiness: f64 = individuals.iter().map(|i| i.happiness).sum();
        let average_happiness = if population_count > 0.0 { total_happiness / population_count } else { 0.0 };
        metrics.insert("average_happiness".to_string(), average_happiness);
        let total_wealth: f64 = individuals.iter().map(|i| i.wealth).sum();
        let average_wealth = if population_count > 0.0 { total_wealth / population_count } else { 0.0 };
        metrics.insert("average_wealth".to_string(), average_wealth);
        let mut community_counts = HashMap::new();
        for individual in individuals {
            let count = community_counts.entry(individual.community_id).or_insert(0);
            *count += 1;
        }
        for (community_id, count) in community_counts {
            metrics.insert(format!("community_{}_population", community_id), count as f64);
        }
        let total_children: i32 = individuals.iter().map(|i| i.children).sum();
        let birth_rate = if population_count > 0.0 { total_children as f64 / population_count } else { 0.0 };
        metrics.insert("birth_rate".to_string(), birth_rate);
        for (key, value) in &metrics {
            self.metrics.insert(key.clone(), *value);
        }
        metrics
    }

    pub fn update_metrics(&mut self, individuals: &[Individual], current_tick: i64) -> HashMap<String, f64> {
        let mut updated_metrics = HashMap::new();
        for (name, _) in self.metrics.clone().iter() {
            if self.should_update_metric(name, current_tick) {
                self.last_updates.insert(name.clone(), current_tick);
                let metrics = self.calculate_predefined_metrics(individuals);
                for (key, value) in metrics {
                    updated_metrics.insert(key, value);
                }
                break;
            }
        }
        for custom_metric in &self.custom_metrics {
            if self.should_update_metric(&custom_metric.name, current_tick) {
                self.last_updates.insert(custom_metric.name.clone(), current_tick);
                if let Ok(value) = self.evaluate_custom_metric(custom_metric) {
                    self.metrics.insert(custom_metric.name.clone(), value);
                    updated_metrics.insert(custom_metric.name.clone(), value);
                }
            }
        }
        updated_metrics
    }
    
    pub fn add_custom_metric(&mut self, custom_metric: CustomMetric) {
        self.custom_metrics.push(custom_metric);
    }
    
    fn evaluate_custom_metric(&self, custom_metric: &CustomMetric) -> Result<f64, MetricsError> {
        let formula = custom_metric.formula.as_str();
        let mut result = 0.0;
        let mut terms = Vec::new();
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
                let metric_value = self.metrics.get(term).ok_or_else(|| {
                    MetricsError::EvaluationError(format!("Metric not found: {}", term))
                })?;
                result += metric_value;
            }
        }
        Ok(result)
    }
    
    pub fn get_metric(&self, name: &str) -> Option<f64> {
        self.metrics.get(name).copied()
    }
    
    pub fn get_all_metrics(&self) -> &HashMap<String, f64> {
        &self.metrics
    }
}

pub fn parse_custom_metric(yaml_str: &str) -> Result<CustomMetric, MetricsError> {
    let custom_metric: CustomMetric = serde_yaml::from_str(yaml_str)
        .map_err(|e| MetricsError::FormulaParseError(e.to_string()))?;
    if custom_metric.formula.contains('/') && custom_metric.formula.ends_with('/') {
        return Err(MetricsError::FormulaParseError(
            "Invalid formula syntax".to_string()
        ));
    }
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
    use crate::server::types::Trait;

    #[test]
    fn test_schedule_metric_update_logic() {
        let metrics_system = MetricsSystem::new();
        assert!(metrics_system.should_update_metric("population_count", 1));
        assert!(metrics_system.should_update_metric("population_count", 2));
        assert!(metrics_system.should_update_metric("average_health", 30));
        assert!(!metrics_system.should_update_metric("average_health", 31));
        assert!(metrics_system.should_update_metric("average_health", 60));
        assert!(metrics_system.should_update_metric("birth_rate", 365));
        assert!(!metrics_system.should_update_metric("birth_rate", 366));
        assert!(metrics_system.should_update_metric("birth_rate", 730));
    }

    #[test]
    fn test_calculate_predefined_metrics() {
        let mut metrics_system = MetricsSystem::new();
        let individuals = vec![
            Individual {
                id: 1,
                health: 80.0,
                happiness: 70.0,
                wealth: 100.0,
                community_id: 1,
                age: 30,
                children: 2,
                traits: vec![],
            },
            Individual {
                id: 2,
                health: 60.0,
                happiness: 90.0,
                wealth: 50.0,
                community_id: 1,
                age: 25,
                children: 0,
                traits: vec![],
            },
            Individual {
                id: 3,
                health: 100.0,
                happiness: 60.0,
                wealth: 150.0,
                community_id: 2,
                age: 40,
                children: 3,
                traits: vec![],
            },
        ];
        let metrics = metrics_system.calculate_predefined_metrics(&individuals);
        assert_eq!(metrics.get("population_count").unwrap(), &3.0);
        assert!((metrics.get("average_health").unwrap() - 80.0).abs() < 0.001);
        assert!((metrics.get("average_happiness").unwrap() - 73.333).abs() < 0.001);
        assert!((metrics.get("average_wealth").unwrap() - 100.0).abs() < 0.001);
        assert_eq!(metrics.get("community_1_population").unwrap(), &2.0);
        assert_eq!(metrics.get("community_2_population").unwrap(), &1.0);
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
        let invalid_yaml = r#"
            name: "broken_metric"
            formula: "2 * average_health + /"
            update_frequency: "daily"
        "#;
        let result = parse_custom_metric(invalid_yaml);
        assert!(result.is_err());
    }
}
