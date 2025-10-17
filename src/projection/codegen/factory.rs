//! FACTORY: The Second Recursive Descent of Projection (into Storage)
//!
//! This module is a RECURSIVE DESCENT of the Five-Fold Synthesis,
//! applied specifically to the STORAGE domain.
//!
//! It INHERITS the Five-Fold Structure from ../codegen/mod.rs:
//! 1. StorageTransform (Ground) — Schema → Runtime
//! 2. StorageSchema (Identity) — What we know about storage
//! 3. StorageConstraints (Inherence) — What belongs to storage
//! 4. StorageRuntime (Difference) — What we CREATE in storage
//! 5. StorageConsequence (Entailment) — What follows from constraints
//!
//! OPERATION: CREATE
//! Direction: StorageSchema → Create Consequences → StorageRuntime
//! Question: "What shall we bring into Storage being?"
//!
//! factory is the POWER mode: given a Schema (analyzed from a Descriptor),
//! what runtime shall we bring into being?
//!
//! factory realizes the freedom to manifest any runtime from any schema.
//! It is the power that turns knowledge (eval) into actual being.
//!
//! PRINCIPLE: Factory is not eval. Factory does NOT analyze descriptors.
//! Factory MANIFESTS runtimes from the schema that eval provides.
//! Together, eval + factory = complete Projection system.
//!
//! See also: ../eval/mod.rs (first recursive descent)

use std::error::Error;
use std::fmt;

/// Error type for factory failures
#[derive(Debug)]
pub struct FactoryError {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl FactoryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(mut self, err: Box<dyn Error + Send + Sync>) -> Self {
        self.source = Some(err);
        self
    }
}

impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Factory error: {}", self.message)
    }
}

impl Error for FactoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

/// FACTORY TRAIT: Omnipotence
///
/// Given a Schema (derived from a Descriptor via eval), create a Runtime.
/// This is the power to manifest any runtime from any schema.
///
/// S = Schema type (what we know, from eval)
/// R = Runtime type (what we create)
///
/// The runtime is concrete, executable, bound to actual behavior.
pub trait Factory<S, R>: Send + Sync + fmt::Debug
where
    S: Send + Sync,
    R: Send + Sync,
{
    type Error: Error + Send + Sync + 'static;

    /// Create a runtime from the schema.
    /// This is bottom-up manifestation: from abstract to concrete.
    fn create(&self, schema: &S) -> Result<R, Self::Error>;
}

/// GENERIC FACTORY: Function-based creation
/// Captures the simplest case: Schema → Runtime is a pure function.
pub struct FunctionFactory<S, R, F>
where
    S: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&S) -> Result<R, Box<dyn Error + Send + Sync>> + Send + Sync,
{
    create_fn: F,
    _marker: std::marker::PhantomData<(S, R)>,
}

impl<S, R, F> fmt::Debug for FunctionFactory<S, R, F>
where
    S: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&S) -> Result<R, Box<dyn Error + Send + Sync>> + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionFactory").finish()
    }
}

impl<S, R, F> FunctionFactory<S, R, F>
where
    S: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&S) -> Result<R, Box<dyn Error + Send + Sync>> + Send + Sync,
{
    pub fn new(create_fn: F) -> Self {
        Self {
            create_fn,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<S, R, F> Factory<S, R> for FunctionFactory<S, R, F>
where
    S: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&S) -> Result<R, Box<dyn Error + Send + Sync>> + Send + Sync,
{
    type Error = FactoryError;

    fn create(&self, schema: &S) -> Result<R, Self::Error> {
        (self.create_fn)(schema).map_err(|e| FactoryError::new("Creation failed").with_source(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestSchema {
        name: String,
        multiplier: i32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestRuntime {
        instantiated_from: String,
        value: i32,
    }

    #[test]
    fn factory_creates_runtime_from_schema() {
        let factory = FunctionFactory::new(|schema: &TestSchema| {
            Ok(TestRuntime {
                instantiated_from: schema.name.clone(),
                value: schema.multiplier * 10,
            })
        });

        let schema = TestSchema {
            name: "test".to_string(),
            multiplier: 5,
        };

        let runtime = factory.create(&schema).expect("create succeeds");
        assert_eq!(runtime.instantiated_from, "test");
        assert_eq!(runtime.value, 50);
    }

    #[test]
    fn factory_error_propagates() {
        let factory: FunctionFactory<TestSchema, TestRuntime, _> =
            FunctionFactory::new(|_schema: &TestSchema| Err("failed to create".into()));

        let schema = TestSchema {
            name: "test".to_string(),
            multiplier: 5,
        };

        let result = factory.create(&schema);
        assert!(result.is_err());
    }
}
