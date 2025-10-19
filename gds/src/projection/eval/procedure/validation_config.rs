//! Validation Configuration - Two-phase validation system
//!
//! Translated from: Java GDS validation package
//! Sources:
//! - ValidationConfiguration.java (~40 lines)
//! - BeforeLoadValidation.java (~30 lines)
//! - AfterLoadValidation.java (~30 lines)
//! - Validator.java (~60 lines)
//!
//! Merges all 4 validation files into a single module.

use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::schema::NodeLabel;
use serde_json::Value as JsonValue;

/// Validation Configuration - Two-phase validation
///
/// Manages validators that run:
/// 1. **Before graph load** - Config-only validation (no graph access)
/// 2. **After graph load** - Config + graph validation (graph available)
///
/// **Pattern from Java GDS**:
/// - BeforeLoadValidation = FunctionalInterface (config only)
/// - AfterLoadValidation = FunctionalInterface (config + graph)
/// - ValidationConfiguration = Container for validators
/// - Validator = Orchestrator that runs all validators
///
/// **Rust Simplification**:
/// - Trait objects for dynamic validator dispatch
/// - Builder pattern for fluent API
/// - Result-based error propagation
pub struct ValidationConfiguration {
    before_load: Vec<Box<dyn BeforeLoadValidator>>,
    after_load: Vec<Box<dyn AfterLoadValidator>>,
}

impl ValidationConfiguration {
    /// Create a new empty validation configuration
    pub fn new() -> Self {
        Self {
            before_load: Vec::new(),
            after_load: Vec::new(),
        }
    }

    /// Create an empty validation configuration (alias for new)
    pub fn empty() -> Self {
        Self::new()
    }

    /// Add a before-load validator (builder pattern)
    pub fn add_before_load<V: BeforeLoadValidator + 'static>(mut self, validator: V) -> Self {
        self.before_load.push(Box::new(validator));
        self
    }

    /// Add an after-load validator (builder pattern)
    pub fn add_after_load<V: AfterLoadValidator + 'static>(mut self, validator: V) -> Self {
        self.after_load.push(Box::new(validator));
        self
    }

    /// Validate before graph load (config only)
    ///
    /// Runs all before-load validators in sequence.
    /// Stops at first error.
    pub fn validate_before_load(&self, config: &JsonValue) -> Result<(), ValidationError> {
        for validator in &self.before_load {
            validator.validate(config)?;
        }
        Ok(())
    }

    /// Validate after graph load (config + graph)
    ///
    /// Runs all after-load validators in sequence.
    /// Stops at first error.
    pub fn validate_after_load(
        &self,
        graph_store: &DefaultGraphStore,
        config: &JsonValue,
    ) -> Result<(), ValidationError> {
        for validator in &self.after_load {
            validator.validate(graph_store, config)?;
        }
        Ok(())
    }

    /// Get count of before-load validators
    pub fn before_load_count(&self) -> usize {
        self.before_load.len()
    }

    /// Get count of after-load validators
    pub fn after_load_count(&self) -> usize {
        self.after_load.len()
    }

    /// Check if configuration is empty (no validators)
    pub fn is_empty(&self) -> bool {
        self.before_load.is_empty() && self.after_load.is_empty()
    }
}

impl Default for ValidationConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

/// Validator that runs before graph load
///
/// Translated from: `BeforeLoadValidation<CONFIG>`
///
/// **Use case**: Validate configuration parameters before expensive graph loading
/// - Range checks (min/max iterations, tolerance)
/// - Required parameter checks
/// - Format validation
/// - Cross-parameter consistency
pub trait BeforeLoadValidator: Send + Sync {
    /// Validate configuration before graph load
    fn validate(&self, config: &JsonValue) -> Result<(), ValidationError>;

    /// Optional validator name (for error messages)
    fn name(&self) -> &str {
        "BeforeLoadValidator"
    }
}

/// Validator that runs after graph load
///
/// Translated from: `AfterLoadValidation<CONFIG>`
///
/// **Use case**: Validate configuration against loaded graph
/// - Property existence checks
/// - Node label validation
/// - Relationship type validation
/// - Graph structure requirements
pub trait AfterLoadValidator: Send + Sync {
    /// Validate configuration after graph load
    fn validate(
        &self,
        graph_store: &DefaultGraphStore,
        config: &JsonValue,
    ) -> Result<(), ValidationError>;

    /// Optional validator name (for error messages)
    fn name(&self) -> &str {
        "AfterLoadValidator"
    }
}

/// Validation Error - Errors from validation
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Before-load validation failed ({validator}): {message}")]
    BeforeLoad { validator: String, message: String },

    #[error("After-load validation failed ({validator}): {message}")]
    AfterLoad { validator: String, message: String },

    #[error("Parameter validation failed: {0}")]
    Parameter(String),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("Invalid value for {param}: {message}")]
    InvalidValue { param: String, message: String },

    #[error("Property not found in graph: {0}")]
    PropertyNotFound(String),

    #[error("Node label not found in graph: {0}")]
    NodeLabelNotFound(String),

    #[error("Relationship type not found in graph: {0}")]
    RelationshipTypeNotFound(String),
}

// ============================================================================
// Example Validators (for demonstration and testing)
// ============================================================================

/// Range Validator - Validates numeric parameter is within range
///
/// **Before-load validator** - Config only
pub struct RangeValidator {
    param: String,
    min: f64,
    max: f64,
}

impl RangeValidator {
    pub fn new(param: impl Into<String>, min: f64, max: f64) -> Self {
        Self {
            param: param.into(),
            min,
            max,
        }
    }
}

impl BeforeLoadValidator for RangeValidator {
    fn validate(&self, config: &JsonValue) -> Result<(), ValidationError> {
        if let Some(value) = config.get(&self.param).and_then(|v| v.as_f64()) {
            if value < self.min || value > self.max {
                return Err(ValidationError::InvalidValue {
                    param: self.param.clone(),
                    message: format!("value {} out of range [{}, {}]", value, self.min, self.max),
                });
            }
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "RangeValidator"
    }
}

/// Required Parameter Validator - Validates required parameter exists
///
/// **Before-load validator** - Config only
pub struct RequiredParameterValidator {
    param: String,
}

impl RequiredParameterValidator {
    pub fn new(param: impl Into<String>) -> Self {
        Self {
            param: param.into(),
        }
    }
}

impl BeforeLoadValidator for RequiredParameterValidator {
    fn validate(&self, config: &JsonValue) -> Result<(), ValidationError> {
        if config.get(&self.param).is_none() {
            return Err(ValidationError::MissingParameter(self.param.clone()));
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "RequiredParameterValidator"
    }
}

/// Property Exists Validator - Validates property exists in graph
///
/// **After-load validator** - Graph + config
pub struct PropertyExistsValidator {
    property: String,
}

impl PropertyExistsValidator {
    pub fn new(property: impl Into<String>) -> Self {
        Self {
            property: property.into(),
        }
    }
}

impl AfterLoadValidator for PropertyExistsValidator {
    fn validate(
        &self,
        graph_store: &DefaultGraphStore,
        _config: &JsonValue,
    ) -> Result<(), ValidationError> {
        if !graph_store.has_node_property(&self.property) {
            return Err(ValidationError::PropertyNotFound(self.property.clone()));
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "PropertyExistsValidator"
    }
}

/// Node Label Exists Validator - Validates node label exists in graph
///
/// **After-load validator** - Graph + config
pub struct NodeLabelExistsValidator {
    label: String,
}

impl NodeLabelExistsValidator {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl AfterLoadValidator for NodeLabelExistsValidator {
    fn validate(
        &self,
        graph_store: &DefaultGraphStore,
        _config: &JsonValue,
    ) -> Result<(), ValidationError> {
        let label = NodeLabel::of(&self.label);
        if !graph_store.has_node_label(&label) {
            return Err(ValidationError::NodeLabelNotFound(self.label.clone()));
        }
        Ok(())
    }

    fn name(&self) -> &str {
        "NodeLabelExistsValidator"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::prelude::{DefaultGraphStore, RandomGraphConfig};
    use serde_json::json;

    #[test]
    fn test_empty_validation_config() {
        let config = ValidationConfiguration::empty();
        assert!(config.is_empty());
        assert_eq!(config.before_load_count(), 0);
        assert_eq!(config.after_load_count(), 0);
    }

    #[test]
    fn test_validation_config_builder() {
        let config = ValidationConfiguration::new()
            .add_before_load(RangeValidator::new("maxIterations", 1.0, 100.0))
            .add_before_load(RequiredParameterValidator::new("tolerance"))
            .add_after_load(PropertyExistsValidator::new("pagerank"));

        assert_eq!(config.before_load_count(), 2);
        assert_eq!(config.after_load_count(), 1);
        assert!(!config.is_empty());
    }

    #[test]
    fn test_range_validator_success() {
        let validator = RangeValidator::new("maxIterations", 1.0, 100.0);
        let config = json!({"maxIterations": 20});

        assert!(validator.validate(&config).is_ok());
    }

    #[test]
    fn test_range_validator_too_low() {
        let validator = RangeValidator::new("maxIterations", 1.0, 100.0);
        let config = json!({"maxIterations": 0});

        let result = validator.validate(&config);
        assert!(result.is_err());
        match result {
            Err(ValidationError::InvalidValue { param, .. }) => {
                assert_eq!(param, "maxIterations");
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[test]
    fn test_range_validator_too_high() {
        let validator = RangeValidator::new("maxIterations", 1.0, 100.0);
        let config = json!({"maxIterations": 101});

        let result = validator.validate(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_range_validator_missing_param_ok() {
        // Missing param is OK for range validator (it's optional)
        let validator = RangeValidator::new("maxIterations", 1.0, 100.0);
        let config = json!({"other": 42});

        assert!(validator.validate(&config).is_ok());
    }

    #[test]
    fn test_required_parameter_validator_success() {
        let validator = RequiredParameterValidator::new("tolerance");
        let config = json!({"tolerance": 0.001});

        assert!(validator.validate(&config).is_ok());
    }

    #[test]
    fn test_required_parameter_validator_missing() {
        let validator = RequiredParameterValidator::new("tolerance");
        let config = json!({"other": 42});

        let result = validator.validate(&config);
        assert!(result.is_err());
        match result {
            Err(ValidationError::MissingParameter(param)) => {
                assert_eq!(param, "tolerance");
            }
            _ => panic!("Expected MissingParameter error"),
        }
    }

    // TODO: Re-enable when we have a way to create graphs with properties
    // #[test]
    // fn test_property_exists_validator_success() {
    //     // Need to create graph with actual properties for this test
    //     let config = RandomGraphConfig::default().with_seed(42);
    //     let graph = DefaultGraphStore::random(&config).unwrap();
    //     let validator = PropertyExistsValidator::new("pagerank");
    //     assert!(validator.validate(&graph, &json!({})).is_ok());
    // }

    #[test]
    fn test_property_exists_validator_missing() {
        let rand_config = RandomGraphConfig::default().with_seed(42);
        let graph = DefaultGraphStore::random(&rand_config).unwrap();

        let validator = PropertyExistsValidator::new("nonexistent");
        let config = json!({});

        let result = validator.validate(&graph, &config);
        assert!(result.is_err());
        match result {
            Err(ValidationError::PropertyNotFound(prop)) => {
                assert_eq!(prop, "nonexistent");
            }
            _ => panic!("Expected PropertyNotFound error"),
        }
    }

    #[test]
    fn test_node_label_exists_validator() {
        let rand_config = RandomGraphConfig::default().with_seed(42);
        let graph = DefaultGraphStore::random(&rand_config).unwrap();

        // Test with non-existing label (random graph doesn't create specific labels by default)
        let validator = NodeLabelExistsValidator::new("NonExistent");
        let config = json!({});
        let result = validator.validate(&graph, &config);
        assert!(result.is_err());
        match result {
            Err(ValidationError::NodeLabelNotFound(label)) => {
                assert_eq!(label, "NonExistent");
            }
            _ => panic!("Expected NodeLabelNotFound error"),
        }
    }

    #[test]
    fn test_validation_config_before_load_chain() {
        let validation = ValidationConfiguration::new()
            .add_before_load(RequiredParameterValidator::new("tolerance"))
            .add_before_load(RangeValidator::new("maxIterations", 1.0, 100.0));

        // Valid config
        let config = json!({"tolerance": 0.001, "maxIterations": 20});
        assert!(validation.validate_before_load(&config).is_ok());

        // Missing required parameter
        let config = json!({"maxIterations": 20});
        assert!(validation.validate_before_load(&config).is_err());

        // Out of range
        let config = json!({"tolerance": 0.001, "maxIterations": 200});
        assert!(validation.validate_before_load(&config).is_err());
    }

    #[test]
    fn test_validation_config_after_load_chain() {
        let validation = ValidationConfiguration::new()
            .add_after_load(PropertyExistsValidator::new("prop1"))
            .add_after_load(PropertyExistsValidator::new("prop2"));

        // Create graph without properties
        let config = RandomGraphConfig::default().with_seed(42);
        let graph = DefaultGraphStore::random(&config).unwrap();

        let result = validation.validate_after_load(&graph, &json!({}));
        // Should fail on first missing property
        assert!(result.is_err());
        match result {
            Err(ValidationError::PropertyNotFound(_)) => {} // Expected
            _ => panic!("Expected PropertyNotFound error"),
        }
    }

    #[test]
    fn test_validation_config_two_phase() {
        let validation = ValidationConfiguration::new()
            .add_before_load(RequiredParameterValidator::new("maxIterations"))
            .add_after_load(PropertyExistsValidator::new("weight"));

        // Phase 1: Before load should fail
        let result = validation.validate_before_load(&json!({}));
        assert!(result.is_err());

        // Phase 2: After load with valid config but missing property
        let config = RandomGraphConfig::default().with_seed(42);
        let graph = DefaultGraphStore::random(&config).unwrap();

        let result = validation.validate_after_load(&graph, &json!({"maxIterations": 20}));
        assert!(result.is_err()); // Should fail on missing property
    }

    #[test]
    fn test_validator_names() {
        let range_validator = RangeValidator::new("x", 0.0, 1.0);
        assert_eq!(range_validator.name(), "RangeValidator");

        let required_validator = RequiredParameterValidator::new("x");
        assert_eq!(required_validator.name(), "RequiredParameterValidator");

        let property_validator = PropertyExistsValidator::new("x");
        assert_eq!(property_validator.name(), "PropertyExistsValidator");

        let label_validator = NodeLabelExistsValidator::new("x");
        assert_eq!(label_validator.name(), "NodeLabelExistsValidator");
    }
}
