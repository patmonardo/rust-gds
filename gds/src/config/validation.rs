//! Validation utilities for configurations

use thiserror::Error;

/// Configuration validation errors
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration parameter '{name}' is required")]
    RequiredParameter { name: String },

    #[error("Configuration parameter '{name}' must be positive, got: {value}")]
    MustBePositive { name: String, value: f64 },

    #[error("Configuration parameter '{name}' must be between {min} and {max}, got: {value}")]
    OutOfRange {
        name: String,
        min: f64,
        max: f64,
        value: f64,
    },

    #[error("Path cannot be empty")]
    EmptyPath,

    #[error("Database name cannot be empty")]
    EmptyDatabaseName,

    #[error("Database name must start with letter and contain only alphanumeric characters and underscores, got: {name}")]
    InvalidDatabaseName { name: String },

    #[error("Feature properties must contain at least one property")]
    EmptyFeatureProperties,

    #[error("Property key cannot be empty")]
    EmptyPropertyKey,

    #[error("Configuration parameter '{parameter}' is invalid: {reason}")]
    InvalidParameter { parameter: String, reason: String },

    #[error("Configuration parameter '{0}' is missing")]
    MissingParameter(String),
}

/// Validation utilities
pub struct ConfigValidation;

impl ConfigValidation {
    /// Validate that a value is present
    pub fn validate_required<T>(value: Option<T>, name: &str) -> Result<T, ConfigError> {
        value.ok_or_else(|| ConfigError::RequiredParameter {
            name: name.to_string(),
        })
    }

    /// Validate that a number is positive
    pub fn validate_positive(value: f64, name: &str) -> Result<(), ConfigError> {
        if value <= 0.0 {
            Err(ConfigError::MustBePositive {
                name: name.to_string(),
                value,
            })
        } else {
            Ok(())
        }
    }

    /// Validate that a number is within a range
    pub fn validate_range(value: f64, min: f64, max: f64, name: &str) -> Result<(), ConfigError> {
        if value < min || value > max {
            Err(ConfigError::OutOfRange {
                name: name.to_string(),
                min,
                max,
                value,
            })
        } else {
            Ok(())
        }
    }

    /// Validate that a path is not empty
    pub fn validate_path(path: &str) -> Result<(), ConfigError> {
        if path.trim().is_empty() {
            Err(ConfigError::EmptyPath)
        } else {
            Ok(())
        }
    }

    /// Validate database name format
    pub fn validate_database_name(name: &str) -> Result<(), ConfigError> {
        if name.trim().is_empty() {
            return Err(ConfigError::EmptyDatabaseName);
        }

        let mut chars = name.chars();
        let first = chars.next().unwrap();

        if !first.is_ascii_alphabetic() {
            return Err(ConfigError::InvalidDatabaseName {
                name: name.to_string(),
            });
        }

        for c in chars {
            if !c.is_ascii_alphanumeric() && c != '_' {
                return Err(ConfigError::InvalidDatabaseName {
                    name: name.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Validate that feature properties is not empty
    pub fn validate_node_properties(properties: &[String]) -> Result<(), ConfigError> {
        if properties.is_empty() {
            Err(ConfigError::EmptyFeatureProperties)
        } else {
            Ok(())
        }
    }

    /// Validate that property key is not empty
    pub fn validate_property_key(key: &str) -> Result<(), ConfigError> {
        if key.trim().is_empty() {
            Err(ConfigError::EmptyPropertyKey)
        } else {
            Ok(())
        }
    }

    /// Validate that a string is non-empty
    pub fn validate_non_empty_string(value: &str, parameter: &str) -> Result<(), ConfigError> {
        if value.trim().is_empty() {
            Err(ConfigError::InvalidParameter {
                parameter: parameter.to_string(),
                reason: format!("{} cannot be empty", parameter),
            })
        } else {
            Ok(())
        }
    }

    /// Validate model name: non-empty, no whitespace
    pub fn validate_model_name(name: &str) -> Result<(), ConfigError> {
        let trimmed = name.trim();
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
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_positive() {
        assert!(ConfigValidation::validate_positive(1.0, "test").is_ok());
        assert!(ConfigValidation::validate_positive(0.0, "test").is_err());
        assert!(ConfigValidation::validate_positive(-1.0, "test").is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(ConfigValidation::validate_range(0.5, 0.0, 1.0, "test").is_ok());
        assert!(ConfigValidation::validate_range(0.0, 0.0, 1.0, "test").is_ok());
        assert!(ConfigValidation::validate_range(1.0, 0.0, 1.0, "test").is_ok());
        assert!(ConfigValidation::validate_range(-0.1, 0.0, 1.0, "test").is_err());
        assert!(ConfigValidation::validate_range(1.1, 0.0, 1.0, "test").is_err());
    }

    #[test]
    fn test_validate_path() {
        assert!(ConfigValidation::validate_path("/tmp/test").is_ok());
        assert!(ConfigValidation::validate_path("").is_err());
        assert!(ConfigValidation::validate_path("   ").is_err());
    }

    #[test]
    fn test_validate_database_name() {
        assert!(ConfigValidation::validate_database_name("mydb").is_ok());
        assert!(ConfigValidation::validate_database_name("my_db_123").is_ok());
        assert!(ConfigValidation::validate_database_name("").is_err());
        assert!(ConfigValidation::validate_database_name("123db").is_err());
        assert!(ConfigValidation::validate_database_name("my-db").is_err());
    }

    #[test]
    fn test_validate_node_properties() {
        assert!(ConfigValidation::validate_node_properties(&["prop1".to_string()]).is_ok());
        assert!(ConfigValidation::validate_node_properties(&[]).is_err());
    }

    #[test]
    fn test_validate_property_key() {
        assert!(ConfigValidation::validate_property_key("key").is_ok());
        assert!(ConfigValidation::validate_property_key("").is_err());
        assert!(ConfigValidation::validate_property_key("   ").is_err());
    }

    #[test]
    fn test_invalid_parameter() {
        let error = ConfigError::InvalidParameter {
            parameter: "modelName".to_string(),
            reason: "Model name cannot be empty".to_string(),
        };
        assert_eq!(
            error.to_string(),
            "Configuration parameter 'modelName' is invalid: Model name cannot be empty"
        );
    }

    #[test]
    fn test_missing_parameter() {
        let error = ConfigError::MissingParameter("modelName".to_string());
        assert_eq!(
            error.to_string(),
            "Configuration parameter 'modelName' is missing"
        );
    }
}
