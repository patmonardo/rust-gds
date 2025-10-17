//! Sum Aggregation - The Trivial Algorithm
//!
//! A minimal but complete implementation of the Storage:Procedure:Algorithm triad.
//! This demonstrates how Membership→Consequence→Inherence generates concrete algorithms.
//!
//! # The Triad Manifested
//!
//! ```text
//! STORAGE RUNTIME (Being There - Persistent):
//!   Input:  PropertyValues (column-oriented storage of values)
//!   Output: Sum (single aggregated value)
//!   Nature: What PERSISTS
//!
//! COMPUTATION RUNTIME (Ephemeral Nothing - Rules/Transformations):
//!   Process: Iterate through values, accumulate
//!   Return:  Intermediate state or final result
//!   Nature:  What TRANSFORMS
//!
//! ALGORITHM (Concept that subsumes both):
//!   SumAggregation: Orchestrates Storage + Computation
//!   Knows how to extract values from storage
//!   Knows how to combine them via computation
//!   Returns unified result
//! ```
//!
//! # Philosophical Position
//!
//! - **Membership:** What must belong to an aggregation?
//!   - Input must be numeric (Long, Double, not String/Boolean)
//!   - Output must be same type as input
//!   - Must handle nullability consistently
//!
//! - **Consequence:** What logically follows?
//!   - If input is numeric → output is numeric
//!   - If input is nullable → output nullable OR error
//!   - If input is empty → output is 0 (or None)
//!
//! - **Inherence:** What forms subsume this?
//!   - TypedSumAggregation (Long-specific)
//!   - TypedSumAggregation (Double-specific)
//!   - DistributedSumAggregation (for parallel reduction)

use std::sync::Arc;

/// The Aggregation Input trait - what sources can be aggregated
pub trait AggregationSource: Send + Sync {
    /// Get the i-th value, or None if null/missing
    fn get_long(&self, index: usize) -> Option<i64>;
    fn get_double(&self, index: usize) -> Option<f64>;
    /// Total number of values
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Result of aggregation
#[derive(Debug, Clone, PartialEq)]
pub enum AggregationResult {
    Long(i64),
    Double(f64),
    None,
}

/// Error in aggregation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AggregationError {
    IncompatibleSource(String),
    NullableConflict(String),
    Empty,
}

impl std::fmt::Display for AggregationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompatibleSource(msg) => write!(f, "Incompatible source: {}", msg),
            Self::NullableConflict(msg) => write!(f, "Nullable conflict: {}", msg),
            Self::Empty => write!(f, "Empty source"),
        }
    }
}

impl std::error::Error for AggregationError {}

/// Sum Aggregation - The Algorithm (Concept that subsumes Storage + Procedure)
///
/// This IS the unified manifestation of:
/// - Storage Runtime: The input source (persistent data)
/// - Computation Runtime: The summation process (ephemeral transformation)
///
/// The Algorithm knows how to combine them into a meaningful result.
pub struct SumAggregation {
    source: Arc<dyn AggregationSource>,
    value_type: AggregationType,
    nullable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggregationType {
    Long,
    Double,
}

impl SumAggregation {
    /// Create a sum aggregation from a source
    pub fn new(
        source: Arc<dyn AggregationSource>,
        value_type: AggregationType,
        nullable: bool,
    ) -> Self {
        Self {
            source,
            value_type,
            nullable,
        }
    }

    /// MEMBERSHIP: Extract what constraints belong to this aggregation
    ///
    /// Returns constraints that must be satisfied:
    /// - Input values must be of the declared type
    /// - Nullability must be consistent
    /// - Storage must be non-empty or support empty aggregation
    pub fn extract_membership(&self) -> Result<SumAggregationMembership, AggregationError> {
        if self.source.is_empty() && !self.nullable {
            return Err(AggregationError::Empty);
        }

        // Validate constraints
        match self.value_type {
            AggregationType::Long => {
                // Check that values are actually longs
                for i in 0..self.source.len().min(100) {
                    // Sample check first 100 or all
                    if let Some(_val) = self.source.get_long(i) {
                        // ✓ Value exists and is long
                    } else if !self.nullable {
                        return Err(AggregationError::NullableConflict(
                            "Non-nullable source contains null".to_string(),
                        ));
                    }
                }
            }
            AggregationType::Double => {
                // Similar for doubles
                for i in 0..self.source.len().min(100) {
                    if let Some(_val) = self.source.get_double(i) {
                        // ✓ Value exists and is double
                    } else if !self.nullable {
                        return Err(AggregationError::NullableConflict(
                            "Non-nullable source contains null".to_string(),
                        ));
                    }
                }
            }
        }

        Ok(SumAggregationMembership {
            value_type: self.value_type,
            nullable: self.nullable,
            source_len: self.source.len(),
        })
    }

    /// CONSEQUENCE: Derive what logically follows from membership
    ///
    /// Given the constraints (membership), what computation must occur?
    /// This returns the procedure that will execute.
    pub fn derive_consequence(
        &self,
        _membership: &SumAggregationMembership,
    ) -> Result<SumAggregationProcedure, AggregationError> {
        // The consequence is: "iterate and accumulate"
        // This is procedural, not yet executed
        Ok(SumAggregationProcedure {
            value_type: self.value_type,
            accumulation_strategy: AccumulationStrategy::Sequential,
        })
    }

    /// INHERENCE: Execute the algorithm, recognizing the form it manifests
    ///
    /// This IS the manifestation: Storage + Procedure = Result
    /// The result is what subsumes both.
    pub fn compute(&self) -> Result<AggregationResult, AggregationError> {
        match self.value_type {
            AggregationType::Long => {
                let mut sum: i64 = 0;
                let mut count = 0;

                for i in 0..self.source.len() {
                    if let Some(value) = self.source.get_long(i) {
                        sum = sum.saturating_add(value);
                        count += 1;
                    } else if self.nullable {
                        // Skip nulls if nullable
                    } else {
                        return Err(AggregationError::NullableConflict(
                            "Non-nullable source contains null".to_string(),
                        ));
                    }
                }

                if count == 0 && !self.nullable {
                    return Err(AggregationError::Empty);
                }

                Ok(if count > 0 {
                    AggregationResult::Long(sum)
                } else {
                    AggregationResult::None
                })
            }
            AggregationType::Double => {
                let mut sum: f64 = 0.0;
                let mut count = 0;

                for i in 0..self.source.len() {
                    if let Some(value) = self.source.get_double(i) {
                        sum += value;
                        count += 1;
                    } else if self.nullable {
                        // Skip nulls if nullable
                    } else {
                        return Err(AggregationError::NullableConflict(
                            "Non-nullable source contains null".to_string(),
                        ));
                    }
                }

                if count == 0 && !self.nullable {
                    return Err(AggregationError::Empty);
                }

                Ok(if count > 0 {
                    AggregationResult::Double(sum)
                } else {
                    AggregationResult::None
                })
            }
        }
    }
}

/// MEMBERSHIP: Constraints that belong to this aggregation
///
/// This captures "what must be true" for the aggregation to be valid.
/// It IS the first moment of the genetic process.
#[derive(Debug, Clone)]
pub struct SumAggregationMembership {
    pub value_type: AggregationType,
    pub nullable: bool,
    pub source_len: usize,
}

/// CONSEQUENCE / PROCEDURE: What computation must follow
///
/// This captures "what will happen" given the constraints.
/// It IS the second and third moments of the genetic process.
#[derive(Debug, Clone)]
pub struct SumAggregationProcedure {
    pub value_type: AggregationType,
    pub accumulation_strategy: AccumulationStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccumulationStrategy {
    Sequential,
    // Future: Parallel, Distributed
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock source for testing
    struct MockLongSource {
        values: Vec<Option<i64>>,
    }

    impl AggregationSource for MockLongSource {
        fn get_long(&self, index: usize) -> Option<i64> {
            self.values.get(index).copied().flatten()
        }

        fn get_double(&self, _index: usize) -> Option<f64> {
            None // Not a double source
        }

        fn len(&self) -> usize {
            self.values.len()
        }
    }

    #[test]
    fn test_sum_aggregation_long_values() {
        let source = Arc::new(MockLongSource {
            values: vec![Some(10), Some(20), Some(30)],
        });

        let agg = SumAggregation::new(source, AggregationType::Long, false);

        // Test Membership extraction
        let membership = agg.extract_membership().unwrap();
        assert_eq!(membership.value_type, AggregationType::Long);
        assert!(!membership.nullable);
        assert_eq!(membership.source_len, 3);

        // Test Consequence derivation
        let procedure = agg.derive_consequence(&membership).unwrap();
        assert_eq!(procedure.value_type, AggregationType::Long);
        assert_eq!(
            procedure.accumulation_strategy,
            AccumulationStrategy::Sequential
        );

        // Test Inherence / Computation
        let result = agg.compute().unwrap();
        assert_eq!(result, AggregationResult::Long(60));
    }

    #[test]
    fn test_sum_aggregation_double_values() {
        let source = Arc::new(MockLongSource {
            values: vec![], // Empty for this test
        });

        // For doubles, we'd need a different mock
        // This shows the pattern: each value type can have specialized procedure
        let agg = SumAggregation::new(source, AggregationType::Double, true);
        let membership = agg.extract_membership().unwrap();
        assert_eq!(membership.value_type, AggregationType::Double);
        assert!(membership.nullable);
    }

    #[test]
    fn test_sum_aggregation_with_nulls_nullable() {
        let source = Arc::new(MockLongSource {
            values: vec![Some(10), None, Some(20)],
        });

        let agg = SumAggregation::new(source, AggregationType::Long, true);
        let result = agg.compute().unwrap();
        assert_eq!(result, AggregationResult::Long(30)); // Nulls skipped
    }

    #[test]
    fn test_sum_aggregation_with_nulls_non_nullable_fails() {
        let source = Arc::new(MockLongSource {
            values: vec![Some(10), None, Some(20)],
        });

        let agg = SumAggregation::new(source, AggregationType::Long, false);
        let result = agg.compute();
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_aggregation_empty_non_nullable_fails() {
        let source = Arc::new(MockLongSource { values: vec![] });

        let agg = SumAggregation::new(source, AggregationType::Long, false);
        let result = agg.compute();
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_aggregation_empty_nullable_returns_none() {
        let source = Arc::new(MockLongSource { values: vec![] });

        let agg = SumAggregation::new(source, AggregationType::Long, true);
        let result = agg.compute().unwrap();
        assert_eq!(result, AggregationResult::None);
    }

    #[test]
    fn test_membership_consequence_inherence_flow() {
        // This test demonstrates the complete genetic flow
        let source = Arc::new(MockLongSource {
            values: vec![Some(5), Some(15), Some(25)],
        });

        let agg = SumAggregation::new(source, AggregationType::Long, false);

        // MEMBERSHIP: Extract constraints
        let membership = agg.extract_membership().unwrap();
        assert_eq!(membership.source_len, 3);

        // CONSEQUENCE: Derive procedure
        let procedure = agg.derive_consequence(&membership).unwrap();
        assert_eq!(
            procedure.accumulation_strategy,
            AccumulationStrategy::Sequential
        );

        // INHERENCE: Execute and recognize result
        let result = agg.compute().unwrap();
        assert_eq!(result, AggregationResult::Long(45));

        // The result IS the manifestation of Membership:Consequence:Inherence
    }
}
