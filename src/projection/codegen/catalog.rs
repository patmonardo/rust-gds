//! CATALOG: The Second Recursive Descent of Projection (into Storage)
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
//! catalog is the POWER mode: given a Schema (analyzed from a Descriptor),
//! what runtime shall we bring into being?
//!
//! catalog realizes the freedom to manifest any runtime from any schema.
//! It is the power that turns knowledge (registry) into actual being.
//!
//! PRINCIPLE: Catalog is not registry. Catalog does NOT analyze descriptors.
//! Catalog MANIFESTS runtimes from the schema that registry provides.
//! Together, registry + catalog = complete Projection system.
//!
//! See also: ../registry/mod.rs (first recursive descent)

use std::error::Error;
use std::fmt;

/// Error type for catalog failures
#[derive(Debug)]
pub struct CatalogError {
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl CatalogError {
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

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Catalog error: {}", self.message)
    }
}

impl Error for CatalogError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

/// CATALOG TRAIT: Omnipotence
///
/// Given a Schema (derived from a Descriptor via registry), create a Runtime.
/// This is the power to manifest any runtime from any schema.
///
/// S = Schema type (what we know, from registry)
/// R = Runtime type (what we create)
///
/// The runtime is concrete, executable, bound to actual behavior.
pub trait Catalog<S, R>: Send + Sync + fmt::Debug
where
    S: Send + Sync,
    R: Send + Sync,
{
    type Error: Error + Send + Sync + 'static;

    /// Create a runtime from the schema.
    /// This is bottom-up manifestation: from abstract to concrete.
    fn create(&self, schema: &S) -> Result<R, Self::Error>;
}

/// GENERIC CATALOG: Function-based creation
/// Captures the simplest case: Schema → Runtime is a pure function.
pub struct FunctionCatalog<S, R, F>
where
    S: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&S) -> Result<R, Box<dyn Error + Send + Sync>> + Send + Sync,
{
    create_fn: F,
    _marker: std::marker::PhantomData<(S, R)>,
}

impl<S, R, F> fmt::Debug for FunctionCatalog<S, R, F>
where
    S: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&S) -> Result<R, Box<dyn Error + Send + Sync>> + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionCatalog").finish()
    }
}

impl<S, R, F> FunctionCatalog<S, R, F>
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

impl<S, R, F> Catalog<S, R> for FunctionCatalog<S, R, F>
where
    S: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&S) -> Result<R, Box<dyn Error + Send + Sync>> + Send + Sync,
{
    type Error = CatalogError;

    fn create(&self, schema: &S) -> Result<R, Self::Error> {
        (self.create_fn)(schema).map_err(|e| CatalogError::new("Creation failed").with_source(e))
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
    fn catalog_creates_runtime_from_schema() {
        let catalog = FunctionCatalog::new(|schema: &TestSchema| {
            Ok(TestRuntime {
                instantiated_from: schema.name.clone(),
                value: schema.multiplier * 10,
            })
        });

        let schema = TestSchema {
            name: "test".to_string(),
            multiplier: 5,
        };

        let runtime = catalog.create(&schema).expect("create succeeds");
        assert_eq!(runtime.instantiated_from, "test");
        assert_eq!(runtime.value, 50);
    }

    #[test]
    fn catalog_error_propagates() {
        let catalog: FunctionCatalog<TestSchema, TestRuntime, _> =
            FunctionCatalog::new(|_schema: &TestSchema| Err("failed to create".into()));

        let schema = TestSchema {
            name: "test".to_string(),
            multiplier: 5,
        };

        let result = catalog.create(&schema);
        assert!(result.is_err());
    }
}
