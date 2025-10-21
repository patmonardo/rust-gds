//! MEMBERSHIP: Protocol of Disjunctive Syllogism
//!
//! Membership answers "What belongs?" using the **disjunctive relation** (X | Y).
//! It's the **method of division** - how we categorize and classify.
//!
//! ## Practical Applications
//!
//! - **Field validation**: Does this value belong to this type?
//! - **Type checking**: Is this compatible with that constraint?
//! - **Constraint extraction**: What rules bind this descriptor?
//!
//! ## Semantic Web Relations
//!
//! The **disjunctive relation** (X | Y) is one of the **three fundamental relations**
//! we recognize in Semantic Webs: X | Y, X → Y, X & Y

use std::fmt;

/// Error type for membership extraction
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

/// MEMBERSHIP EXTRACTOR: Extract What Belongs
///
/// Given a Descriptor, extract what **belongs** to it using the
/// **disjunctive relation** (X | Y) - the method of division.
///
/// This answers: "What constraints, types, and rules belong to this concept?"
pub trait MembershipExtractor<D>: Send + Sync + fmt::Debug
where
    D: Send + Sync,
{
    type Membership: Send + Sync + fmt::Debug;

    /// Extract the membership structure from a descriptor.
    /// Uses the **disjunctive relation** (X | Y) to determine what belongs.
    fn extract(&self, descriptor: &D) -> Result<Self::Membership, MembershipError>;
}

/// PRACTICAL MEMBERSHIP: Field validation and type checking
///
/// Extracts what **belongs** to a descriptor:
/// - What fields are required/optional
/// - What types are compatible
/// - What constraints must be satisfied
pub struct FieldMembership {
    /// Required fields that must be present
    pub required_fields: Vec<String>,
    /// Optional fields that may be present
    pub optional_fields: Vec<String>,
    /// Type constraints for each field
    pub type_constraints: std::collections::HashMap<String, String>,
    /// Validation rules for each field
    pub validation_rules: std::collections::HashMap<String, String>,
}

impl fmt::Debug for FieldMembership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldMembership")
            .field("required_fields", &self.required_fields)
            .field("optional_fields", &self.optional_fields)
            .field("type_constraints", &self.type_constraints)
            .field("validation_rules", &self.validation_rules)
            .finish()
    }
}

/// FUNCTION-BASED MEMBERSHIP EXTRACTOR
///
/// Captures the simplest case: Descriptor → Membership is a pure function
/// using the **disjunctive relation** (X | Y).
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
        fields: Vec<String>,
        constraints: Vec<String>,
    }

    #[test]
    fn membership_extractor_extracts_field_constraints() {
        let extractor = FunctionMembershipExtractor::new(|desc: &TestDescriptor| {
            Ok(FieldMembership {
                required_fields: vec!["id".to_string()],
                optional_fields: desc.fields.clone(),
                type_constraints: std::collections::HashMap::new(),
                validation_rules: desc.constraints.iter()
                    .map(|c| (c.clone(), "validation_rule".to_string()))
                    .collect(),
            })
        });

        let desc = TestDescriptor {
            id: 1,
            fields: vec!["name".to_string(), "value".to_string()],
            constraints: vec!["positive".to_string(), "non_empty".to_string()],
        };

        let membership = extractor.extract(&desc).expect("extract succeeds");
        assert_eq!(membership.required_fields, vec!["id"]);
        assert_eq!(membership.optional_fields.len(), 2);
        assert_eq!(membership.validation_rules.len(), 2);
    }

    #[test]
    fn membership_extractor_error_propagates() {
        let extractor: FunctionMembershipExtractor<TestDescriptor, FieldMembership, _> =
            FunctionMembershipExtractor::new(|_desc: &TestDescriptor| {
                Err("extraction failed".into())
            });

        let desc = TestDescriptor {
            id: 1,
            fields: vec![],
            constraints: vec![],
        };

        let result = extractor.extract(&desc);
        assert!(result.is_err());
    }
}