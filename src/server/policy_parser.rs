use serde_yaml::Value;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Policy {
    pub id: u32,
    pub name: String,
    pub community: String,
    pub effects: HashMap<String, f64>,
    pub adoption_rate: f64,
}

#[derive(Debug, Clone)]
pub struct PartialPolicy {
    pub name: Option<String>,
    pub community: Option<String>,
    pub effects: Option<HashMap<String, Value>>,
    pub adoption_rate: Option<f64>,
}

#[derive(Error, Debug)]
pub enum PolicyError {
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid value for field: {0}")]
    InvalidValue(String),
}

pub fn parse_policy(yaml: &str) -> Result<Policy, PolicyError> {
    let value: Value = serde_yaml::from_str(yaml)?;
    let name = extract_string_field(&value, "name")?;
    let community = extract_string_field(&value, "community")?;
    let effects_value = value
        .get("effects")
        .ok_or_else(|| PolicyError::MissingField("effects".to_string()))?;
    let effects = extract_effects_map(effects_value)?;
    let adoption_rate = value
        .get("adoption_rate")
        .ok_or_else(|| PolicyError::MissingField("adoption_rate".to_string()))?
        .as_f64()
        .ok_or_else(|| PolicyError::InvalidValue("adoption_rate must be a number".to_string()))?;
    Ok(Policy {
        id: 0,
        name,
        community,
        effects,
        adoption_rate,
    })
}

pub fn parse_policy_template(yaml: &str) -> Result<Policy, PolicyError> {
    parse_policy(yaml)
}

pub fn parse_partial_policy(yaml: &str) -> Result<PartialPolicy, PolicyError> {
    let value: Value = serde_yaml::from_str(yaml)?;
    let name = value.get("name").and_then(|v| v.as_str()).map(String::from);
    let community = value.get("community").and_then(|v| v.as_str()).map(String::from);
    let effects = if let Some(effects_value) = value.get("effects") {
        if let Some(mapping) = effects_value.as_mapping() {
            let mut effects_map = HashMap::new();
            for (k, v) in mapping {
                if let Some(key) = k.as_str() {
                    effects_map.insert(key.to_string(), v.clone());
                }
            }
            Some(effects_map)
        } else {
            None
        }
    } else {
        None
    };
    let adoption_rate = value.get("adoption_rate").and_then(|v| v.as_f64());
    Ok(PartialPolicy {
        name,
        community,
        effects,
        adoption_rate,
    })
}

pub fn apply_template(partial: PartialPolicy, template: &Policy) -> Result<Policy, PolicyError> {
    let name = partial.name.unwrap_or_else(|| template.name.clone());
    let community = partial.community.unwrap_or_else(|| template.community.clone());
    let mut effects = template.effects.clone();
    if let Some(partial_effects) = partial.effects {
        for (key, value) in partial_effects {
            if let Some(val) = value.as_f64() {
                effects.insert(key, val);
            } else if let Some(mapping) = value.as_mapping() {
                for (nested_key, nested_value) in mapping {
                    if let (Some(k_str), Some(v_f64)) = (nested_key.as_str(), nested_value.as_f64()) {
                        let combined_key = format!("{}.{}", key, k_str);
                        effects.insert(combined_key, v_f64);
                    }
                }
            }
        }
    }
    let adoption_rate = partial.adoption_rate.unwrap_or(template.adoption_rate);
    Ok(Policy {
        id: template.id,
        name,
        community,
        effects,
        adoption_rate,
    })
}

pub fn validate_policy(policy: Policy) -> Result<Policy, PolicyError> {
    if policy.adoption_rate < 0.0 || policy.adoption_rate > 1.0 {
        return Err(PolicyError::InvalidValue(
            "adoption_rate must be between 0.0 and 1.0".to_string()
        ));
    }
    for (key, value) in &policy.effects {
        if key.contains("multiplier") && *value < 0.0 {
            return Err(PolicyError::InvalidValue(
                format!("{} must be positive", key)
            ));
        }
    }
    Ok(policy)
}

fn extract_string_field(value: &Value, field_name: &str) -> Result<String, PolicyError> {
    value.get(field_name)
        .ok_or_else(|| PolicyError::MissingField(field_name.to_string()))?
        .as_str()
        .ok_or_else(|| PolicyError::InvalidValue(format!("{} must be a string", field_name)))
        .map(String::from)
}

fn extract_effects_map(effects_value: &Value) -> Result<HashMap<String, f64>, PolicyError> {
    if let Some(mapping) = effects_value.as_mapping() {
        let mut effects_map = HashMap::new();
        for (key, value) in mapping {
            if let Some(key_str) = key.as_str() {
                if let Some(val) = value.as_f64() {
                    effects_map.insert(key_str.to_string(), val);
                } else if let Some(nested_mapping) = value.as_mapping() {
                    process_nested_mapping(&mut effects_map, key_str, nested_mapping);
                }
            }
        }
        Ok(effects_map)
    } else {
        Err(PolicyError::InvalidValue("effects must be a mapping".to_string()))
    }
}

fn process_nested_mapping(effects_map: &mut HashMap<String, f64>, prefix: &str, mapping: &serde_yaml::Mapping) {
    for (nested_key, nested_value) in mapping {
        if let (Some(key_str), Some(val)) = (nested_key.as_str(), nested_value.as_f64()) {
            effects_map.insert(format!("{}.{}", prefix, key_str), val);
        }
    }
}

#[cfg(test)]
mod policy_parser_tests {
    use super::*;

    #[test]
    fn test_parse_valid_yaml() {
        let yaml = r#"
            name: "Green Energy Initiative"
            community: "nation"
            effects:
              health_multiplier: 1.1
              happiness_multiplier: 1.05
            adoption_rate: 0.75
        "#;
        let policy = parse_policy(yaml).unwrap();
        assert_eq!(policy.name, "Green Energy Initiative");
        assert_eq!(policy.community, "nation");
        assert_eq!(policy.effects.get("health_multiplier").unwrap(), &1.1);
        assert_eq!(policy.effects.get("happiness_multiplier").unwrap(), &1.05);
        assert_eq!(policy.adoption_rate, 0.75);
    }

    #[test]
    fn test_parse_invalid_yaml() {
        let invalid_yaml = r#"
            name: "Broken Policy
            community: nation
            effects: {
              this isn't valid YAML
            adoption_rate: 0.5
        "#;
        let result = parse_policy(invalid_yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_required_fields() {
        let missing_community = r#"
            name: "Incomplete Policy"
            effects:
              health_multiplier: 1.2
            adoption_rate: 0.6
        "#;
        let result = parse_policy(missing_community);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("community"));
        let missing_name = r#"
            community: "nation"
            effects:
              health_multiplier: 1.2
            adoption_rate: 0.6
        "#;
        let result = parse_policy(missing_name);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name"));
    }

    #[test]
    fn test_parse_complex_effects() {
        let complex_yaml = r#"
            name: "Complex Policy"
            community: "nation"
            effects:
              health_multiplier: 1.1
              resource_modifiers:
                food: 1.2
                water: 0.9
              action_weights:
                farm: 1.5
                hunt: 0.8
              population_growth: 1.05
            adoption_rate: 0.65
        "#;
        let policy = parse_policy(complex_yaml).unwrap();
        assert_eq!(policy.name, "Complex Policy");
        assert_eq!(policy.effects.get("resource_modifiers.food").unwrap(), &1.2);
        assert_eq!(policy.effects.get("resource_modifiers.water").unwrap(), &0.9);
        assert_eq!(policy.effects.get("action_weights.farm").unwrap(), &1.5);
        assert_eq!(policy.effects.get("action_weights.hunt").unwrap(), &0.8);
        assert_eq!(policy.effects.get("population_growth").unwrap(), &1.05);
    }

    #[test]
    fn test_policy_template_application() {
        let template = r#"
            name: "Template"
            community: "default"
            effects:
              health_multiplier: 1.0
              happiness_multiplier: 1.0
              wealth_multiplier: 1.0
            adoption_rate: 0.5
        "#;
        let template_policy = parse_policy_template(template).unwrap();
        let partial_yaml = r#"
            name: "Partial Policy"
            community: "city"
            effects:
              health_multiplier: 1.2
        "#;
        let policy = apply_template(parse_partial_policy(partial_yaml).unwrap(), &template_policy).unwrap();
        assert_eq!(policy.name, "Partial Policy");
        assert_eq!(policy.community, "city");
        assert_eq!(policy.effects.get("health_multiplier").unwrap(), &1.2);
        assert_eq!(policy.effects.get("happiness_multiplier").unwrap(), &1.0);
        assert_eq!(policy.effects.get("wealth_multiplier").unwrap(), &1.0);
        assert_eq!(policy.adoption_rate, 0.5);
    }

    #[test]
    fn test_policy_validation_logic() {
        let invalid_adoption_rate = r#"
            name: "Invalid Policy"
            community: "nation"
            effects:
              health_multiplier: 1.1
            adoption_rate: 1.5
        "#;
        let result = validate_policy(parse_policy(invalid_adoption_rate).unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("adoption_rate"));
        let negative_multipliers = r#"
            name: "Negative Policy"
            community: "nation"
            effects:
              health_multiplier: -0.5
            adoption_rate: 0.5
        "#;
        let result = validate_policy(parse_policy(negative_multipliers).unwrap());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("multiplier"));
    }
}
