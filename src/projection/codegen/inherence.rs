//! INHERENCE: The Third Moment of the Genetic Loop
//!
//! Inherence is the Subsumption relation — it answers "What forms inhere in this consequence?"
//!
//! Given a Runtime (from Consequence), Inherence recognizes:
//! - What structural patterns subsume it
//! - What universal forms it exemplifies
//! - What new Descriptors should be derived
//! - What Transforms are needed for next iteration
//!
//! Inherence is where the loop closes: it generates new Descriptors
//! whose Membership will be extracted in the next iteration.
//!
//! This is the THIRD STEP in the genetic loop:
//! ```
//! Descriptor → Membership → Consequence → [recognize Inherence] → new Descriptors → [next iteration]
//! ```
//!
//! TRANSFORMS ARE INHERENCE MOMENTS:
//! A Transform recognizes a pattern in a Runtime and projects it into a new domain,
//! generating a new Descriptor for the next iteration of the loop.
//!
//! See also:
//! - membership.rs (first moment)
//! - consequence.rs (second moment)
//! - CODEGEN_GENETIC_PROCESS.md (complete architecture)

use std::fmt;

/// Error type for inherence recognition/subsumption
#[derive(Debug)]
pub struct InherenceError {
    message: String,
}

impl InherenceError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for InherenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Inherence error: {}", self.message)
    }
}

impl std::error::Error for InherenceError {}

/// INHERENCE TRAIT: Recognize What Forms Subsume
///
/// Given a Runtime (consequence of the previous moment), recognize
/// what structural forms subsume it and what new Descriptors derive from it.
///
/// This is the subsumption relation: "What universal form does this particular
/// concrete runtime exemplify?"
///
/// R = Runtime type (what we examine)
/// D = Descriptor type (what we derive)
pub trait InherenceRecognizer<R>: Send + Sync + fmt::Debug
where
    R: Send + Sync,
{
    type DerivedDescriptor: Send + Sync + fmt::Debug;

    /// Recognize what forms inhere in this runtime.
    /// Generate new descriptors for the next iteration of the genetic loop.
    fn recognize(&self, runtime: &R) -> Result<Vec<Self::DerivedDescriptor>, InherenceError>;
}

/// GENERIC INHERENCE: Function-based recognition
/// Captures the simplest case: Runtime → DerivedDescriptors is a pure function.
pub struct FunctionInherenceRecognizer<R, D, F>
where
    R: Send + Sync + fmt::Debug,
    D: Send + Sync + fmt::Debug,
    F: Fn(&R) -> Result<Vec<D>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    recognize_fn: F,
    _marker: std::marker::PhantomData<(R, D)>,
}

impl<R, D, F> fmt::Debug for FunctionInherenceRecognizer<R, D, F>
where
    R: Send + Sync + fmt::Debug,
    D: Send + Sync + fmt::Debug,
    F: Fn(&R) -> Result<Vec<D>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionInherenceRecognizer").finish()
    }
}

impl<R, D, F> FunctionInherenceRecognizer<R, D, F>
where
    R: Send + Sync + fmt::Debug,
    D: Send + Sync + fmt::Debug,
    F: Fn(&R) -> Result<Vec<D>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    pub fn new(recognize_fn: F) -> Self {
        Self {
            recognize_fn,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<R, D, F> InherenceRecognizer<R> for FunctionInherenceRecognizer<R, D, F>
where
    R: Send + Sync + fmt::Debug,
    D: Send + Sync + fmt::Debug,
    F: Fn(&R) -> Result<Vec<D>, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    type DerivedDescriptor = D;

    fn recognize(&self, runtime: &R) -> Result<Vec<Self::DerivedDescriptor>, InherenceError> {
        (self.recognize_fn)(runtime)
            .map_err(|e| InherenceError::new("Inherence recognition failed").with_source(e))
    }
}

impl InherenceError {
    pub fn with_source(self, _err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestRuntime {
        name: String,
        data: Vec<i32>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestDescriptor {
        name: String,
        cardinality: usize,
    }

    #[test]
    fn inherence_recognizer_recognizes_forms() {
        let recognizer = FunctionInherenceRecognizer::new(|runtime: &TestRuntime| {
            Ok(vec![TestDescriptor {
                name: format!("Derived_{}", runtime.name),
                cardinality: runtime.data.len(),
            }])
        });

        let runtime = TestRuntime {
            name: "test_runtime".to_string(),
            data: vec![1, 2, 3, 4, 5],
        };

        let descriptors = recognizer.recognize(&runtime).expect("recognize succeeds");
        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].cardinality, 5);
    }

    #[test]
    fn inherence_recognizer_error_propagates() {
        let recognizer: FunctionInherenceRecognizer<TestRuntime, TestDescriptor, _> =
            FunctionInherenceRecognizer::new(|_runtime: &TestRuntime| {
                Err("recognition failed".into())
            });

        let runtime = TestRuntime {
            name: "test".to_string(),
            data: vec![],
        };

        let result = recognizer.recognize(&runtime);
        assert!(result.is_err());
    }
}
