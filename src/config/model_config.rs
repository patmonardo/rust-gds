//! ModelConfig for ML model configuration in GDS.
//!
//! Translated from Java GDS ModelConfig.java in model-catalog-api.
//! Adapted to Rust config system following repository patterns.

crate::generate_config!(
    ModelConfig, ModelConfigBuilder,
    validate = |cfg: &ModelConfig| {
        crate::config::validation::ConfigValidation::validate_non_empty_string(&cfg.model_name, "modelName")?;
        crate::config::validation::ConfigValidation::validate_non_empty_string(&cfg.model_user, "modelUser")?;
        // Model name cannot contain whitespace
        if cfg.model_name.chars().any(|c| c.is_whitespace()) {
            return Err(crate::config::validation::ConfigError::InvalidParameter {
                parameter: "modelName".to_string(),
                reason: "Model name cannot contain whitespace".to_string(),
            });
        }
        Ok(())
    },
    {
        model_name: String = String::from("default_model");
        model_user: String = String::from("default_user");
        username_override: Option<String> = None;
    }
);

impl ModelConfig {
    /// Get the username, preferring override if present.
    pub fn username(&self) -> &str {
        self.username_override
            .as_deref()
            .unwrap_or(&self.model_user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_config_builder() {
        let config = ModelConfig::builder()
            .model_name("test_model".to_string())
            .model_user("test_user".to_string())
            .build()
            .expect("valid config should build");

        assert_eq!(config.model_name, "test_model");
        assert_eq!(config.model_user, "test_user");
        assert_eq!(config.username(), "test_user");
    }

    #[test]
    fn test_username_override() {
        let config = ModelConfig::builder()
            .model_name("test_model".to_string())
            .model_user("test_user".to_string())
            .username_override(Some("override_user".to_string()))
            .build()
            .expect("valid config should build");

        assert_eq!(config.username(), "override_user");
    }

    #[test]
    fn test_validate_name_empty() {
        let result = ModelConfig::builder()
            .model_name("".to_string())
            .model_user("user".to_string())
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_name_whitespace() {
        let result = ModelConfig::builder()
            .model_name("test model".to_string())
            .model_user("user".to_string())
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_name_valid() {
        let config = ModelConfig::builder()
            .model_name("test_model".to_string())
            .model_user("user".to_string())
            .build()
            .expect("valid config should build");
        assert_eq!(config.model_name, "test_model");
    }
}
