//! INHERENCE: Scientific Concepts (X & Y)
//!
//! Inherence answers "What forms?" using the **conjunctive relation** (X & Y).
//! It's the **synthesis** - what forms inhere in this consequence.
//!
//! ## Practical Applications
//!
//! - **Code generation**: What forms can be synthesized from this runtime?
//! - **Pattern recognition**: What structural patterns subsume this?
//! - **Scientific cognition**: What new descriptors derive from this?
//!
//! ## Semantic Web Relations
//!
//! The **conjunctive relation** (X & Y) is one of the **three fundamental relations**
//! we recognize in Semantic Webs: X | Y, X → Y, X & Y

use std::fmt;

/// Error type for inherence recognition
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

/// INHERENCE RECOGNIZER: Recognize What Forms Subsume
///
/// Given a Runtime, recognize what **forms** inhere in it using the
/// **conjunctive relation** (X & Y) - the synthesis.
///
/// This answers: "What structural patterns subsume this runtime? What new forms can be synthesized?"
pub trait InherenceRecognizer<R>: Send + Sync + fmt::Debug
where
    R: Send + Sync,
{
    type DerivedDescriptor: Send + Sync + fmt::Debug;

    /// Recognize what forms inhere in this runtime.
    /// Uses the **conjunctive relation** (X & Y) to synthesize new forms.
    fn recognize(&self, runtime: &R) -> Result<Vec<Self::DerivedDescriptor>, InherenceError>;
}

/// PRACTICAL INHERENCE: Code generation and pattern recognition
///
/// Recognizes what **forms** inhere in a runtime:
/// - What code can be generated
/// - What patterns can be recognized
/// - What new descriptors can be synthesized
pub struct CodeGenerationInherence {
    /// Generated code structures
    pub generated_code: Vec<String>,
    /// Recognized patterns
    pub patterns: Vec<String>,
    /// Synthesized descriptors
    pub descriptors: Vec<String>,
    /// Transformations applied
    pub transformations: Vec<String>,
}

impl fmt::Debug for CodeGenerationInherence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CodeGenerationInherence")
            .field("generated_code", &self.generated_code)
            .field("patterns", &self.patterns)
            .field("descriptors", &self.descriptors)
            .field("transformations", &self.transformations)
            .finish()
    }
}

/// FUNCTION-BASED INHERENCE RECOGNIZER
///
/// Captures the simplest case: Runtime → DerivedDescriptors is a pure function
/// using the **conjunctive relation** (X & Y).
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
        patterns: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestDescriptor {
        name: String,
        cardinality: usize,
        generated_code: String,
    }

    #[test]
    fn inherence_recognizer_synthesizes_forms() {
        let recognizer = FunctionInherenceRecognizer::new(|runtime: &TestRuntime| {
            Ok(vec![TestDescriptor {
                name: format!("Synthesized_{}", runtime.name),
                cardinality: runtime.data.len(),
                generated_code: format!("// Generated code for {}", runtime.name),
            }])
        });

        let runtime = TestRuntime {
            name: "test_runtime".to_string(),
            data: vec![1, 2, 3, 4, 5],
            patterns: vec!["pattern1".to_string(), "pattern2".to_string()],
        };

        let descriptors = recognizer.recognize(&runtime).expect("recognize succeeds");
        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].cardinality, 5);
        assert!(descriptors[0].generated_code.contains("Generated code"));
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
            patterns: vec![],
        };

        let result = recognizer.recognize(&runtime);
        assert!(result.is_err());
    }
}