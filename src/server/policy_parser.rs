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
        // Missing community field
        let missing_community = r#"
            name: "Incomplete Policy"
            effects:
              health_multiplier: 1.2
            adoption_rate: 0.6
        "#;

        let result = parse_policy(missing_community);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("community"));
        
        // Missing name field
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
        
        let resource_modifiers = policy.effects.get("resource_modifiers").unwrap()
            .as_mapping().unwrap();
        assert_eq!(resource_modifiers.get("food").unwrap().as_f64().unwrap(), 1.2);
        assert_eq!(resource_modifiers.get("water").unwrap().as_f64().unwrap(), 0.9);
        
        let action_weights = policy.effects.get("action_weights").unwrap()
            .as_mapping().unwrap();
        assert_eq!(action_weights.get("farm").unwrap().as_f64().unwrap(), 1.5);
        assert_eq!(action_weights.get("hunt").unwrap().as_f64().unwrap(), 0.8);
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
        
        // Policy with missing fields that should inherit from template
        let partial_yaml = r#"
            name: "Partial Policy"
            community: "city"
            effects:
              health_multiplier: 1.2
            # Missing adoption_rate
        "#;

        let policy = apply_template(parse_partial_policy(partial_yaml).unwrap(), &template_policy).unwrap();
        
        assert_eq!(policy.name, "Partial Policy");
        assert_eq!(policy.community, "city");
        assert_eq!(policy.effects.get("health_multiplier").unwrap(), &1.2);
        assert_eq!(policy.effects.get("happiness_multiplier").unwrap(), &1.0); // From template
        assert_eq!(policy.effects.get("wealth_multiplier").unwrap(), &1.0);    // From template
        assert_eq!(policy.adoption_rate, 0.5);                                // From template
    }

    #[test]
    fn test_policy_validation_logic() {
        // Test policy with invalid adoption rate (>1.0)
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
        
        // Test policy with negative multipliers
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