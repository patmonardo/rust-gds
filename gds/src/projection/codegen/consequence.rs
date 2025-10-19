//! CONSEQUENCE: The Second Moment of the Genetic Loop
//!
//! Consequence is the Difference pole — it answers "What MUST follow from membership?"
//!
//! Given a Descriptor and its extracted Membership (constraints, relations),
//! Consequence determines what runtime strategy is logically entailed.
//!
//! This is deterministic: same membership → same consequence.
//! No heuristics, no runtime discovery — just pure logical necessity.
//!
//! RUNTIMES ARE MOMENTS OF CONSEQUENCE:
//! A Runtime is the concrete manifestation of what logically follows
//! from a descriptor's membership. It is the "what SHALL BE" of projection.
//!
//! This is the SECOND STEP in the genetic loop:
//! ```
//! Descriptor → Membership → [apply Consequence] → Runtime → Inherence → [next iteration]
//! ```
//!
//! See also:
//! - membership.rs (first moment)
//! - inherence.rs (third moment)
//! - CODEGEN_GENETIC_PROCESS.md (complete architecture)

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

/// CONSEQUENCE TRAIT: Derive What Must Follow
///
/// Given a Descriptor and its Membership, derive what Runtime is logically entailed.
/// This is the manifestation moment: abstract constraints → concrete being.
///
/// D = Descriptor type (what we examine)
/// M = Membership type (what we have extracted)
/// R = Runtime type (what we manifest)
pub trait ConsequenceDeriver<D, M>: Send + Sync + fmt::Debug
where
    D: Send + Sync,
    M: Send + Sync,
{
    type Runtime: Send + Sync + fmt::Debug;

    /// Derive what runtime is logically entailed by descriptor + membership.
    /// This is deterministic: same membership → same runtime.
    fn derive(&self, descriptor: &D, membership: &M) -> Result<Self::Runtime, ConsequenceError>;
}

/// CONSEQUENCE RULE: Placeholder for future implementation
/// This will be specialized per descriptor type (Computation, Storage, etc.)
pub struct ConsequenceRule;

impl ConsequenceRule {
    /// Placeholder for consequence determination.
    /// In full implementation, this examines membership and determines runtime strategy.
    pub fn validate_membership<D: fmt::Debug + Send + Sync>(
        descriptor: &D,
    ) -> Result<String, ConsequenceError> {
        // Placeholder: in full implementation, examine actual membership fields
        Ok(format!("Validated: {:?}", descriptor))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consequence_validates() {
        let test_descriptor = "test";
        let result = ConsequenceRule::validate_membership(&test_descriptor);
        assert!(result.is_ok());
    }
}
