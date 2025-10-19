//! MEMBERSHIP: The First Moment of the Genetic Loop
//!
//! Membership is the Identity pole — it answers "What belongs to this concept?"
//!
//! Given a Descriptor, Membership extracts and encodes:
//! - What constraints inhere in it
//! - What relations it participates in
//! - What compatibility rules bind it
//! - What structure it requires
//!
//! Membership is deterministic and compile-time verifiable.
//! It contains all necessary information for the next moment (Consequence).
//!
//! This is the FIRST STEP in the genetic loop:
//! ```
//! Descriptor → [extract Membership] → Consequence → Inherence → [next iteration]
//! ```
//!
//! See also:
//! - consequence.rs (second moment)
//! - inherence.rs (third moment)
//! - CODEGEN_GENETIC_PROCESS.md (complete architecture)

use std::fmt;

/// Error type for membership extraction/validation
#[derive(Debug)]
pub struct MembershipError {
    message: String,
}

impl MembershipError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for MembershipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Membership error: {}", self.message)
    }
}

impl std::error::Error for MembershipError {}

/// MEMBERSHIP TRAIT: Extract What Belongs
///
/// Given a Descriptor, extract what constraints and relations inhere in it.
/// This is the Identity pole — what this concept fundamentally IS.
///
/// D = Descriptor type (what we examine)
/// M = Membership type (what we extract)
pub trait MembershipExtractor<D>: Send + Sync + fmt::Debug
where
    D: Send + Sync,
{
    type Membership: Send + Sync + fmt::Debug;

    /// Extract the membership structure from a descriptor.
    /// This is the foundational step: understanding what belongs.
    fn extract(&self, descriptor: &D) -> Result<Self::Membership, MembershipError>;
}

/// GENERIC MEMBERSHIP: Function-based extraction
/// Captures the simplest case: Descriptor → Membership is a pure function.
pub struct FunctionMembershipExtractor<D, M, F>
where
    D: Send + Sync + fmt::Debug,
    M: Send + Sync + fmt::Debug,
    F: Fn(&D) -> Result<M, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    extract_fn: F,
    _marker: std::marker::PhantomData<(D, M)>,
}

impl<D, M, F> fmt::Debug for FunctionMembershipExtractor<D, M, F>
where
    D: Send + Sync + fmt::Debug,
    M: Send + Sync + fmt::Debug,
    F: Fn(&D) -> Result<M, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FunctionMembershipExtractor").finish()
    }
}

impl<D, M, F> FunctionMembershipExtractor<D, M, F>
where
    D: Send + Sync + fmt::Debug,
    M: Send + Sync + fmt::Debug,
    F: Fn(&D) -> Result<M, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    pub fn new(extract_fn: F) -> Self {
        Self {
            extract_fn,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<D, M, F> MembershipExtractor<D> for FunctionMembershipExtractor<D, M, F>
where
    D: Send + Sync + fmt::Debug,
    M: Send + Sync + fmt::Debug,
    F: Fn(&D) -> Result<M, Box<dyn std::error::Error + Send + Sync>> + Send + Sync,
{
    type Membership = M;

    fn extract(&self, descriptor: &D) -> Result<Self::Membership, MembershipError> {
        (self.extract_fn)(descriptor)
            .map_err(|e| MembershipError::new("Membership extraction failed").with_source(e))
    }
}

impl MembershipError {
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
        constraints: Vec<String>,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestMembership {
        descriptor_id: u32,
        extracted_constraints: Vec<String>,
    }

    #[test]
    fn membership_extractor_extracts_from_descriptor() {
        let extractor = FunctionMembershipExtractor::new(|desc: &TestDescriptor| {
            Ok(TestMembership {
                descriptor_id: desc.id,
                extracted_constraints: desc.constraints.clone(),
            })
        });

        let desc = TestDescriptor {
            id: 1,
            constraints: vec!["constraint_a".to_string(), "constraint_b".to_string()],
        };

        let membership = extractor.extract(&desc).expect("extract succeeds");
        assert_eq!(membership.descriptor_id, 1);
        assert_eq!(membership.extracted_constraints.len(), 2);
    }

    #[test]
    fn membership_extractor_error_propagates() {
        let extractor: FunctionMembershipExtractor<TestDescriptor, TestMembership, _> =
            FunctionMembershipExtractor::new(|_desc: &TestDescriptor| {
                Err("extraction failed".into())
            });

        let desc = TestDescriptor {
            id: 1,
            constraints: vec![],
        };

        let result = extractor.extract(&desc);
        assert!(result.is_err());
    }
}
