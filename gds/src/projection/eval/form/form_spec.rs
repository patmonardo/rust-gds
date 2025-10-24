//! FormSpec - Form Specification Trait
//!
//! This trait defines the contract for **Form specifications** that execute
//! the **Form infrastructure**. Similar to `AlgorithmSpec` for procedures.
//!
//! ## The Triads of Hegel
//!
//! Each FormSpec implements the **triadic cycle**:
//! - **Thesis** = Procedure (Immediate execution)
//! - **Antithesis** = ML (Mediate processing)
//! - **Synthesis** = Form (Sublates both)

use std::time::Duration;
// use crate::form::core::*;  // Commented out - form module deactivated
use super::triadic_cycle::{Thesis, Antithesis, Synthesis};

/// FormShape - Stub type (form module deactivated)
#[derive(Debug, Clone)]
pub struct FormShape {
    // Stub implementation - form module deactivated
}

impl FormShape {
    pub fn new(_shape: Vec<usize>, _context: String, _morph: Vec<String>) -> Self {
        Self {}
    }
}

/// Form execution result
#[derive(Debug, Clone)]
pub struct FormResult<T> {
    /// The result data
    pub data: T,
    /// Execution time
    pub execution_time: Duration,
    /// Triadic cycle metadata
    pub cycle_metadata: TriadicCycleMetadata,
}

impl<T> FormResult<T> {
    /// Create a new FormResult
    pub fn new(data: T, execution_time: Duration, cycle_metadata: TriadicCycleMetadata) -> Self {
        Self {
            data,
            execution_time,
            cycle_metadata,
        }
    }

    /// Extract the result data
    pub fn into_result(self) -> T {
        self.data
    }
}

/// Triadic cycle metadata
#[derive(Debug, Clone)]
pub struct TriadicCycleMetadata {
    /// Number of cycles executed
    pub cycles_executed: usize,
    /// Thesis execution time
    pub thesis_time: Duration,
    /// Antithesis execution time
    pub antithesis_time: Duration,
    /// Synthesis execution time
    pub synthesis_time: Duration,
}

/// Form execution error
#[derive(Debug, thiserror::Error)]
pub enum FormError {
    /// Configuration error
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
    
    /// Execution error
    #[error("Execution error: {message}")]
    ExecutionError { message: String },
    
    /// Triadic cycle error
    #[error("Triadic cycle error: {message}")]
    TriadicCycleError { message: String },
}

/// FormSpec - The contract for Form specifications
///
/// This trait defines what a Form specification must implement to execute
/// the **Form infrastructure** through the **triadic cycle**.
pub trait FormSpec: Send + Sync {
    /// The output type of this form
    type Output: Send + Sync;

    /// Get the name of this form
    fn name(&self) -> &str;

    /// Get the thesis (Procedure - Immediate)
    fn thesis(&self) -> &Thesis;

    /// Get the antithesis (ML - Mediate)
    fn antithesis(&self) -> &Antithesis;

    /// Get the synthesis (Form - Sublates both)
    fn synthesis(&self) -> &Synthesis;

    /// Execute the form through the triadic cycle
    fn execute<F: FormStore>(
        &self,
        form_store: &F,
        config: &FormConfig,
        context: &ExecutionContext,
    ) -> Result<FormResult<Self::Output>, FormError>;

    /// Parse configuration from JSON
    fn parse_config(&self, input: &serde_json::Value) -> Result<serde_json::Value, FormError>;

    /// Get validation configuration
    fn validation_config(&self, _context: &ExecutionContext) -> ValidationConfiguration {
        ValidationConfiguration::empty()
    }
}

/// FormStore - The storage interface for forms
///
/// This trait defines the interface for accessing form data,
/// similar to GraphStore for procedures.
pub trait FormStore: Send + Sync {
    /// Get the number of forms in the store
    fn form_count(&self) -> usize;

    /// Get a form by name
    fn get_form(&self, name: &str) -> Option<&FormShape>;

    /// Add a form to the store
    fn add_form(&mut self, form: FormShape) -> Result<(), FormError>;

    /// Remove a form from the store
    fn remove_form(&mut self, name: &str) -> Result<Option<FormShape>, FormError>;
}

/// FormConfig - Configuration for form execution
#[derive(Debug, Clone)]
pub struct FormConfig {
    /// The form name
    pub form_name: String,
    /// Execution parameters
    pub parameters: std::collections::HashMap<String, serde_json::Value>,
    /// Triadic cycle configuration
    pub cycle_config: CycleConfig,
}

/// CycleConfig - Configuration for the triadic cycle
#[derive(Debug, Clone)]
pub struct CycleConfig {
    /// Maximum number of cycles
    pub max_cycles: usize,
    /// Cycle timeout
    pub cycle_timeout: Duration,
    /// Enable thesis execution
    pub enable_thesis: bool,
    /// Enable antithesis execution
    pub enable_antithesis: bool,
    /// Enable synthesis execution
    pub enable_synthesis: bool,
}

impl Default for CycleConfig {
    fn default() -> Self {
        Self {
            max_cycles: 100,
            cycle_timeout: Duration::from_secs(30),
            enable_thesis: true,
            enable_antithesis: true,
            enable_synthesis: true,
        }
    }
}

/// ExecutionContext - Context for form execution
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Execution ID
    pub execution_id: String,
    /// User context
    pub user: String,
    /// Execution metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl ExecutionContext {
    /// Create a new ExecutionContext
    pub fn new(execution_id: String, user: String) -> Self {
        Self {
            execution_id,
            user,
            metadata: std::collections::HashMap::new(),
        }
    }
}

/// ValidationConfiguration - Configuration for validation
#[derive(Debug, Clone)]
pub struct ValidationConfiguration {
    /// Validation rules
    pub rules: Vec<String>,
}

impl ValidationConfiguration {
    /// Create an empty validation configuration
    pub fn empty() -> Self {
        Self {
            rules: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestFormStore {
        forms: std::collections::HashMap<String, FormShape>,
    }

    impl TestFormStore {
        fn new() -> Self {
            Self {
                forms: std::collections::HashMap::new(),
            }
        }
    }

    impl FormStore for TestFormStore {
        fn form_count(&self) -> usize {
            self.forms.len()
        }

        fn get_form(&self, name: &str) -> Option<&FormShape> {
            self.forms.get(name)
        }

        fn add_form(&mut self, form: FormShape) -> Result<(), FormError> {
            // For testing, we'll use a simple name
            self.forms.insert("test_form".to_string(), form);
            Ok(())
        }

        fn remove_form(&mut self, name: &str) -> Result<Option<FormShape>, FormError> {
            Ok(self.forms.remove(name))
        }
    }

    #[test]
    fn test_form_result_creation() {
        let metadata = TriadicCycleMetadata {
            cycles_executed: 1,
            thesis_time: Duration::from_millis(100),
            antithesis_time: Duration::from_millis(200),
            synthesis_time: Duration::from_millis(300),
        };

        let result = FormResult::new("test_data", Duration::from_millis(600), metadata);
        assert_eq!(result.data, "test_data");
        assert_eq!(result.execution_time, Duration::from_millis(600));
        assert_eq!(result.cycle_metadata.cycles_executed, 1);
    }

    #[test]
    fn test_form_store_operations() {
        let mut store = TestFormStore::new();
        assert_eq!(store.form_count(), 0);

        let shape = Shape::new(
            vec!["id".to_string()],
            vec![],
            std::collections::HashMap::new(),
            std::collections::HashMap::new(),
        );

        let context = Context::new(
            vec![],
            vec![],
            "strategy".to_string(),
            vec![],
        );

        let morph = crate::form::core::shape::Morph::new(
            vec![],
            vec![],
            vec![],
            vec![],
        );

        let form_shape = FormShape::new(shape, context, morph);
        store.add_form(form_shape).unwrap();
        assert_eq!(store.form_count(), 1);

        let retrieved = store.get_form("test_form");
        assert!(retrieved.is_some());

        let removed = store.remove_form("test_form").unwrap();
        assert!(removed.is_some());
        assert_eq!(store.form_count(), 0);
    }

    #[test]
    fn test_cycle_config_default() {
        let config = CycleConfig::default();
        assert_eq!(config.max_cycles, 100);
        assert_eq!(config.cycle_timeout, Duration::from_secs(30));
        assert!(config.enable_thesis);
        assert!(config.enable_antithesis);
        assert!(config.enable_synthesis);
    }

    #[test]
    fn test_execution_context() {
        let context = ExecutionContext::new("exec_1".to_string(), "user_1".to_string());
        assert_eq!(context.execution_id, "exec_1");
        assert_eq!(context.user, "user_1");
        assert_eq!(context.metadata.len(), 0);
    }
}
