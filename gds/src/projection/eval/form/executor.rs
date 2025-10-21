//! FormExecutor - The Form Executor
//!
//! This module implements the **FormExecutor** as a **fixed singularity** that executes
//! the **Form infrastructure** through the **triadic cycle**.
//!
//! ## Architecture
//!
//! The FormExecutor is similar to ProcedureExecutor but executes **FormSpec** implementations
//! through the **Thesis-Antithesis-Synthesis** cycle.

use std::time::{Duration, Instant};
use super::form_spec::*;
use super::triadic_cycle::{TriadicCycle, TriadicCycleResult};

/// FormExecutor - The main executor for forms
///
/// This struct is the **fixed singularity** that executes **FormSpec** implementations
/// through the **triadic cycle**.
#[derive(Debug)]
pub struct FormExecutor<F: FormStore> {
    /// The form store
    form_store: F,
    /// Execution context
    execution_context: ExecutionContext,
    /// Executor configuration
    config: ExecutorConfig,
}

impl<F: FormStore> FormExecutor<F> {
    /// Create a new FormExecutor
    pub fn new(form_store: F, execution_context: ExecutionContext, config: ExecutorConfig) -> Self {
        Self {
            form_store,
            execution_context,
            config,
        }
    }

    /// Execute a FormSpec through the triadic cycle
    pub fn execute<S: FormSpec>(
        &self,
        form_spec: &S,
        config: &FormConfig,
    ) -> Result<FormResult<S::Output>, FormError> {
        let start = Instant::now();

        // Validate the form specification
        self.validate_form_spec(form_spec, config)?;

        // Create the triadic cycle
        let triadic_cycle = TriadicCycle::new(
            form_spec.thesis().clone(),
            form_spec.antithesis().clone(),
            form_spec.synthesis().clone(),
            config.cycle_config.clone(),
        );

        // Execute the triadic cycle
        let cycle_result = triadic_cycle.execute(&self.execution_context)?;

        // Create the form result
        let execution_time = start.elapsed();
        let cycle_metadata = TriadicCycleMetadata {
            cycles_executed: cycle_result.cycles_executed,
            thesis_time: cycle_result.thesis_time,
            antithesis_time: cycle_result.antithesis_time,
            synthesis_time: cycle_result.synthesis_time,
        };

        // For now, we'll return a simple output
        // In a real implementation, this would be the actual form result
        let output = self.create_form_output(form_spec, &cycle_result)?;

        Ok(FormResult::new(output, execution_time, cycle_metadata))
    }

    /// Validate a form specification
    fn validate_form_spec<S: FormSpec>(
        &self,
        form_spec: &S,
        config: &FormConfig,
    ) -> Result<(), FormError> {
        // Validate form name
        if form_spec.name().is_empty() {
            return Err(FormError::ConfigError {
                message: "Form name cannot be empty".to_string(),
            });
        }

        // Validate configuration
        if config.form_name.is_empty() {
            return Err(FormError::ConfigError {
                message: "Configuration form name cannot be empty".to_string(),
            });
        }

        // Validate cycle configuration
        if config.cycle_config.max_cycles == 0 {
            return Err(FormError::ConfigError {
                message: "Maximum cycles must be greater than 0".to_string(),
            });
        }

        Ok(())
    }

    /// Create the form output
    fn create_form_output<S: FormSpec>(
        &self,
        _form_spec: &S,
        _cycle_result: &TriadicCycleResult,
    ) -> Result<S::Output, FormError> {
        // For now, we'll create a simple output
        // In a real implementation, this would be the actual form result
        // This is a placeholder that will need to be implemented based on the actual FormSpec
        
        // We need to return something that matches S::Output
        // Since we don't know what S::Output is, we'll need to use a different approach
        // For now, we'll return an error indicating this needs to be implemented
        Err(FormError::ExecutionError {
            message: "Form output creation not yet implemented".to_string(),
        })
    }

    /// Get the form store
    pub fn form_store(&self) -> &F {
        &self.form_store
    }

    /// Get the execution context
    pub fn execution_context(&self) -> &ExecutionContext {
        &self.execution_context
    }

    /// Get the executor configuration
    pub fn config(&self) -> &ExecutorConfig {
        &self.config
    }
}

/// ExecutorConfig - Configuration for the FormExecutor
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// Maximum execution time
    pub max_execution_time: Duration,
    /// Enable validation
    pub enable_validation: bool,
    /// Enable logging
    pub enable_logging: bool,
    /// Log level
    pub log_level: LogLevel,
}

/// LogLevel - Logging level
#[derive(Debug, Clone)]
pub enum LogLevel {
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warning,
    /// Error level
    Error,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            max_execution_time: Duration::from_secs(60),
            enable_validation: true,
            enable_logging: true,
            log_level: LogLevel::Info,
        }
    }
}

/// FormExecutorBuilder - Builder for FormExecutor
#[derive(Debug)]
pub struct FormExecutorBuilder<F: FormStore> {
    form_store: Option<F>,
    execution_context: Option<ExecutionContext>,
    config: Option<ExecutorConfig>,
}

impl<F: FormStore> FormExecutorBuilder<F> {
    /// Create a new FormExecutorBuilder
    pub fn new() -> Self {
        Self {
            form_store: None,
            execution_context: None,
            config: None,
        }
    }

    /// Set the form store
    pub fn with_form_store(mut self, form_store: F) -> Self {
        self.form_store = Some(form_store);
        self
    }

    /// Set the execution context
    pub fn with_execution_context(mut self, execution_context: ExecutionContext) -> Self {
        self.execution_context = Some(execution_context);
        self
    }

    /// Set the executor configuration
    pub fn with_config(mut self, config: ExecutorConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Build the FormExecutor
    pub fn build(self) -> Result<FormExecutor<F>, FormError> {
        let form_store = self.form_store.ok_or_else(|| FormError::ConfigError {
            message: "Form store is required".to_string(),
        })?;

        let execution_context = self.execution_context.ok_or_else(|| FormError::ConfigError {
            message: "Execution context is required".to_string(),
        })?;

        let config = self.config.unwrap_or_default();

        Ok(FormExecutor::new(form_store, execution_context, config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::core::FormShape;

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
            self.forms.insert("test_form".to_string(), form);
            Ok(())
        }

        fn remove_form(&mut self, name: &str) -> Result<Option<FormShape>, FormError> {
            Ok(self.forms.remove(name))
        }
    }

    #[test]
    fn test_form_executor_creation() {
        let form_store = TestFormStore::new();
        let execution_context = ExecutionContext::new("exec_1".to_string(), "user_1".to_string());
        let config = ExecutorConfig::default();

        let executor = FormExecutor::new(form_store, execution_context, config);
        assert_eq!(executor.execution_context().execution_id, "exec_1");
        assert_eq!(executor.execution_context().user, "user_1");
    }

    #[test]
    fn test_form_executor_builder() {
        let form_store = TestFormStore::new();
        let execution_context = ExecutionContext::new("exec_1".to_string(), "user_1".to_string());
        let config = ExecutorConfig::default();

        let executor = FormExecutorBuilder::new()
            .with_form_store(form_store)
            .with_execution_context(execution_context)
            .with_config(config)
            .build()
            .unwrap();

        assert_eq!(executor.execution_context().execution_id, "exec_1");
        assert_eq!(executor.execution_context().user, "user_1");
    }

    #[test]
    fn test_executor_config_default() {
        let config = ExecutorConfig::default();
        assert_eq!(config.max_execution_time, Duration::from_secs(60));
        assert!(config.enable_validation);
        assert!(config.enable_logging);
    }
}
