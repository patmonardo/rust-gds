//! ModelConfig for ML model configuration in GDS.
//!
//! Translated from Java GDS ModelConfig.java in model-catalog-api.
//! Adapted to Rust config system following repository patterns:
//! - Builder pattern with ConfigType::builder()...build() API
//! - Validation at construction time via build() method
//! - Located in src/config/

use std::collections::HashMap;

use crate::config::base_types::{BaseConfig, Config};
use crate::config::validation::ConfigError;

/// Configuration for ML models.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelConfig {
    /// The name of the model.
    pub model_name: String,
    /// The user associated with the model.
    pub model_user: String,
    /// Username override (optional).
    pub username_override: Option<String>,
}

impl ModelConfig {
    /// Create a new builder for ModelConfig.
    pub fn builder() -> ModelConfigBuilder {
        ModelConfigBuilder::default()
    }

    /// Get the username, preferring override if present.
    pub fn username(&self) -> &str {
        self.username_override
            .as_deref()
            .unwrap_or(&self.model_user)
    }

    /// Validate the model name.
    pub fn validate_name(input: &str) -> Result<String, ConfigError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "modelName".to_string(),
                reason: "Model name cannot be empty".to_string(),
            });
        }
        if trimmed.chars().any(|c| c.is_whitespace()) {
            return Err(ConfigError::InvalidParameter {
                parameter: "modelName".to_string(),
                reason: "Model name cannot contain whitespace".to_string(),
            });
        }
        Ok(trimmed.to_string())
    }
}

impl Config for ModelConfig {}

impl BaseConfig for ModelConfig {
    fn parameters(&self) -> HashMap<String, serde_json::Value> {
        let mut params = HashMap::new();
        params.insert(
            "modelName".to_string(),
            serde_json::Value::String(self.model_name.clone()),
        );
        params.insert(
            "modelUser".to_string(),
            serde_json::Value::String(self.model_user.clone()),
        );
        if let Some(ref username_override) = self.username_override {
            params.insert(
                "usernameOverride".to_string(),
                serde_json::Value::String(username_override.clone()),
            );
        }
        params
    }
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            model_name: "default_model".to_string(),
            model_user: "default_user".to_string(),
            username_override: None,
        }
    }
}

/// Builder for ModelConfig.
#[derive(Debug, Default)]
pub struct ModelConfigBuilder {
    model_name: Option<String>,
    model_user: Option<String>,
    username_override: Option<String>,
}

impl ModelConfigBuilder {
    /// Set the model name.
    pub fn model_name(mut self, model_name: String) -> Self {
        self.model_name = Some(model_name);
        self
    }

    /// Set the model user.
    pub fn model_user(mut self, model_user: String) -> Self {
        self.model_user = Some(model_user);
        self
    }

    /// Set the username override.
    pub fn username_override(mut self, username_override: String) -> Self {
        self.username_override = Some(username_override);
        self
    }

    /// Build the ModelConfig, validating at construction time.
    pub fn build(self) -> Result<ModelConfig, ConfigError> {
        let model_name = self
            .model_name
            .ok_or_else(|| ConfigError::MissingParameter("modelName".to_string()))?;
        let validated_name = ModelConfig::validate_name(&model_name)?;

        let model_user = self
            .model_user
            .ok_or_else(|| ConfigError::MissingParameter("modelUser".to_string()))?;

        Ok(ModelConfig {
            model_name: validated_name,
            model_user,
            username_override: self.username_override,
        })
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
            .unwrap();

        assert_eq!(config.model_name, "test_model");
        assert_eq!(config.model_user, "test_user");
        assert_eq!(config.username(), "test_user");
    }

    #[test]
    fn test_username_override() {
        let config = ModelConfig::builder()
            .model_name("test_model".to_string())
            .model_user("test_user".to_string())
            .username_override("override_user".to_string())
            .build()
            .unwrap();

        assert_eq!(config.username(), "override_user");
    }

    #[test]
    fn test_validate_name_empty() {
        assert!(ModelConfig::validate_name("").is_err());
    }

    #[test]
    fn test_validate_name_whitespace() {
        assert!(ModelConfig::validate_name("test model").is_err());
    }

    #[test]
    fn test_validate_name_valid() {
        assert_eq!(
            ModelConfig::validate_name("test_model").unwrap(),
            "test_model"
        );
    }
}
