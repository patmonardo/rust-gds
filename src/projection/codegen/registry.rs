//! REGISTRY: The First Recursive Descent of Projection (into Computation)
//!
//! This module is a RECURSIVE DESCENT of the Five-Fold Synthesis,
//! applied specifically to the COMPUTATION domain.
//!
//! It INHERITS the Five-Fold Structure from ../codegen/mod.rs:
//! 1. ComputationTransform (Ground) — Descriptor → Schema
//! 2. ComputationDescriptor (Identity) — What computation IS
//! 3. ComputationMembership (Inherence) — What belongs to computation
//! 4. ComputationSchema (Difference) — What we KNOW about computation
//! 5. ComputationConsequence (Entailment) — What follows from membership
//!
//! OPERATION: ANALYZE
//! Direction: ComputationDescriptor → Analyze → ComputationSchema
//! Question: "What can we KNOW about this computation?"
//!
//! Registry is the KNOWLEDGE pole: given a Descriptor, what can we deduce?
//! What are its inherent constraints? What consequences follow?
//!
//! Registry overcomes the illusion of Maya by revealing the unity beneath multiplicity:
//! Many descriptors, ONE underlying principle of constraints and relations.
//!
//! PRINCIPLE: Registry is not catalog. Registry does NOT create runtimes.
//! Registry ANALYZES descriptors and extracts their inherent schema.
//! Catalog will use this schema to create runtimes.
//!
//! See also: ../catalog/mod.rs (second recursive descent)

use std::fmt;

/// Error type for registry failures
#[derive(Debug)]
pub struct RegistryError {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl RegistryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(mut self, err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        self.source = Some(err);
        self
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Registry error: {}", self.message)
    }
}

impl std::error::Error for RegistryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

/// REGISTRY TRAIT: Omniscience
///
/// Given a Descriptor, analyze its inherent structure (membership, constraints).
/// Extract a Schema that represents what we know about this descriptor.
///
/// D = Descriptor type (what we analyze)
/// S = Schema type (what we know)
///
/// The schema is pure information—no runtime behavior yet.
pub trait Registry<D>: Send + Sync + fmt::Debug
where
    D: Send + Sync,
{
    type Schema: Send + Sync + fmt::Debug;

    /// Analyze the descriptor: extract its inherent schema.
    /// This is top-down knowledge: from abstract to concrete.
    fn analyze(&self, descriptor: &D) -> Result<Self::Schema, RegistryError>;
}

/// GENERIC REGISTRY: Function-based analysis
/// Captures the simplest case: Descriptor → Schema is a pure function.
pub struct FunctionEval<D, S, F>
where
    D: Send + Sync + fmt::Debug,
    S: Send + Sync + fmt::Debug,
    F: Fn(&D) -> Result<S, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    analyze_fn: F,
    _marker: std::marker::PhantomData<(D, S)>,
}

impl<D, S, F> fmt::Debug for FunctionEval<D, S, F>
where
    D: Send + Sync + fmt::Debug,
    S: Send + Sync + fmt::Debug,
    F: Fn(&D) -> Result<S, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionEval").finish()
    }
}

impl<D, S, F> FunctionEval<D, S, F>
where
    D: Send + Sync + fmt::Debug,
    S: Send + Sync + fmt::Debug,
    F: Fn(&D) -> Result<S, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    pub fn new(analyze_fn: F) -> Self {
        Self {
            analyze_fn,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<D, S, F> Registry<D> for FunctionEval<D, S, F>
where
    D: Send + Sync + fmt::Debug,
    S: Send + Sync + fmt::Debug,
    F: Fn(&D) -> Result<S, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    type Schema = S;

    fn analyze(&self, descriptor: &D) -> Result<Self::Schema, RegistryError> {
        (self.analyze_fn)(descriptor)
            .map_err(|e| RegistryError::new("Analysis failed").with_source(e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestDescriptor {
        name: String,
        complexity: i32,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestSchema {
        analyzed_name: String,
        is_complex: bool,
    }

    #[test]
    fn registry_analyzes_descriptor_to_schema() {
        let registry = FunctionEval::new(|desc: &TestDescriptor| {
            Ok(TestSchema {
                analyzed_name: desc.name.clone(),
                is_complex: desc.complexity > 5,
            })
        });

        let desc = TestDescriptor {
            name: "test".to_string(),
            complexity: 10,
        };

        let schema = registry.analyze(&desc).expect("analyze succeeds");
        assert_eq!(schema.analyzed_name, "test");
        assert!(schema.is_complex);
    }

    #[test]
    fn registry_error_propagates() {
        let registry: FunctionEval<TestDescriptor, TestSchema, _> =
            FunctionEval::new(|_desc: &TestDescriptor| Err("analysis failed".into()));

        let desc = TestDescriptor {
            name: "test".to_string(),
            complexity: 10,
        };

        let result = registry.analyze(&desc);
        assert!(result.is_err());
    }
}
