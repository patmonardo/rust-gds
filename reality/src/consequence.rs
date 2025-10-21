//! CONSEQUENCE: Next Step (If X Then Y)
//!
//! Consequence answers "What follows?" using the **implicative relation** (X → Y).
//! It's the **next step** - what logically follows from a given state.
//!
//! ## Practical Applications
//!
//! - **Dependency resolution**: If this field is present, then that field is required
//! - **Execution order**: If this step completes, then that step can begin
//! - **ML Algorithmics**: If this condition is met, then this algorithm applies
//!
//! ## Semantic Web Relations
//!
//! The **implicative relation** (X → Y) is one of the **three fundamental relations**
//! we recognize in Semantic Webs: X | Y, X → Y, X & Y

use std::fmt;

/// Error type for consequence determination
#[derive(Debug)]
pub struct ConsequenceError {
    message: String,
}

impl ConsequenceError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ConsequenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Consequence error: {}", self.message)
    }
}

impl std::error::Error for ConsequenceError {}

/// CONSEQUENCE DERIVER: Derive What Must Follow
///
/// Given a Descriptor and its Membership, derive what **follows** using the
/// **implicative relation** (X → Y) - the next step.
///
/// This answers: "If this membership is true, then what runtime strategy follows?"
pub trait ConsequenceDeriver<D, M>: Send + Sync + fmt::Debug
where
    D: Send + Sync,
    M: Send + Sync,
{
    type Runtime: Send + Sync + fmt::Debug;

    /// Derive what runtime is logically entailed by descriptor + membership.
    /// Uses the **implicative relation** (X → Y) to determine what follows.
    fn derive(&self, descriptor: &D, membership: &M) -> Result<Self::Runtime, ConsequenceError>;
}

/// PRACTICAL CONSEQUENCE: Dependency resolution and execution order
///
/// Derives what **follows** from a membership:
/// - What dependencies must be resolved
/// - What execution order is required
/// - What runtime strategy applies
pub struct ExecutionConsequence {
    /// Dependencies that must be resolved first
    pub dependencies: Vec<String>,
    /// Execution order for this step
    pub execution_order: Vec<String>,
    /// Runtime strategy to apply
    pub runtime_strategy: String,
    /// Conditions that must be met
    pub conditions: Vec<String>,
}

impl fmt::Debug for ExecutionConsequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExecutionConsequence")
            .field("dependencies", &self.dependencies)
            .field("execution_order", &self.execution_order)
            .field("runtime_strategy", &self.runtime_strategy)
            .field("conditions", &self.conditions)
            .finish()
    }
}

/// FUNCTION-BASED CONSEQUENCE DERIVER
///
/// Captures the simplest case: Descriptor + Membership → Runtime is a pure function
/// using the **implicative relation** (X → Y).
pub struct FunctionConsequenceDeriver<D, M, R, F>
where
    D: Send + Sync + fmt::Debug,
    M: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&D, &M) -> Result<R, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    derive_fn: F,
    _marker: std::marker::PhantomData<(D, M, R)>,
}

impl<D, M, R, F> fmt::Debug for FunctionConsequenceDeriver<D, M, R, F>
where
    D: Send + Sync + fmt::Debug,
    M: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&D, &M) -> Result<R, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionConsequenceDeriver").finish()
    }
}

impl<D, M, R, F> FunctionConsequenceDeriver<D, M, R, F>
where
    D: Send + Sync + fmt::Debug,
    M: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&D, &M) -> Result<R, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    pub fn new(derive_fn: F) -> Self {
        Self {
            derive_fn,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<D, M, R, F> ConsequenceDeriver<D, M> for FunctionConsequenceDeriver<D, M, R, F>
where
    D: Send + Sync + fmt::Debug,
    M: Send + Sync + fmt::Debug,
    R: Send + Sync + fmt::Debug,
    F: Fn(&D, &M) -> Result<R, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    type Runtime = R;

    fn derive(&self, descriptor: &D, membership: &M) -> Result<Self::Runtime, ConsequenceError> {
        (self.derive_fn)(descriptor, membership)
            .map_err(|e| ConsequenceError::new("Consequence derivation failed").with_source(e))
    }
}

impl ConsequenceError {
    pub fn with_source(self, _err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestDescriptor {
        id: u32,
        name: String,
    }

    #[derive(Debug, Clone)]
    struct TestMembership {
        required_fields: Vec<String>,
        constraints: Vec<String>,
    }

    #[test]
    fn consequence_deriver_derives_execution_order() {
        let deriver = FunctionConsequenceDeriver::new(|desc: &TestDescriptor, membership: &TestMembership| {
            Ok(ExecutionConsequence {
                dependencies: vec!["dependency1".to_string(), "dependency2".to_string()],
                execution_order: vec!["step1".to_string(), "step2".to_string()],
                runtime_strategy: format!("strategy_for_{}", desc.name),
                conditions: membership.constraints.clone(),
            })
        });

        let desc = TestDescriptor {
            id: 1,
            name: "test".to_string(),
        };

        let membership = TestMembership {
            required_fields: vec!["id".to_string()],
            constraints: vec!["positive".to_string()],
        };

        let consequence = deriver.derive(&desc, &membership).expect("derive succeeds");
        assert_eq!(consequence.dependencies.len(), 2);
        assert_eq!(consequence.execution_order.len(), 2);
        assert_eq!(consequence.runtime_strategy, "strategy_for_test");
        assert_eq!(consequence.conditions.len(), 1);
    }

    #[test]
    fn consequence_deriver_error_propagates() {
        let deriver: FunctionConsequenceDeriver<TestDescriptor, TestMembership, ExecutionConsequence, _> =
            FunctionConsequenceDeriver::new(|_desc: &TestDescriptor, _membership: &TestMembership| {
                Err("derivation failed".into())
            });

        let desc = TestDescriptor {
            id: 1,
            name: "test".to_string(),
        };

        let membership = TestMembership {
            required_fields: vec![],
            constraints: vec![],
        };

        let result = deriver.derive(&desc, &membership);
        assert!(result.is_err());
    }
}