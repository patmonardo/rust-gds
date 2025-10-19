//! EmptySimilaritySummaryBuilder - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.similarity.EmptySimilaritySummaryBuilder
//!
//! No-op implementation of similarity summary builder.

use super::similarity_summary_builder::{SimilaritySummaryBuilder, RelationshipWithPropertyConsumer};
use std::collections::HashMap;

/// Empty similarity summary builder - translated from Java EmptySimilaritySummaryBuilder
/// 
/// No-op implementation of similarity summary builder.
/// 
/// Java class:
/// ```java
/// public class EmptySimilaritySummaryBuilder implements SimilaritySummaryBuilder {
///     @Override
///     public RelationshipWithPropertyConsumer similarityConsumer() {
///         return (node1, node2, similarity) -> true;
///     }
/// 
///     @Override
///     public Map<String, Object> similaritySummary() {
///         return Map.of();
///     }
/// }
/// ```
#[derive(Debug)]
pub struct EmptySimilaritySummaryBuilder;

impl EmptySimilaritySummaryBuilder {
    /// Create new empty builder
    pub fn new() -> Self {
        Self
    }
}

impl SimilaritySummaryBuilder for EmptySimilaritySummaryBuilder {
    /// Get similarity consumer - translated from Java similarityConsumer method
    /// 
    /// Java method:
    /// ```java
    /// public RelationshipWithPropertyConsumer similarityConsumer() {
    ///     return (node1, node2, similarity) -> true;
    /// }
    /// ```
    fn similarity_consumer(&self) -> RelationshipWithPropertyConsumer {
        Box::new(|_node1, _node2, _similarity| true)
    }
    
    /// Get similarity summary - translated from Java similaritySummary method
    /// 
    /// Java method:
    /// ```java
    /// public Map<String, Object> similaritySummary() {
    ///     return Map.of();
    /// }
    /// ```
    fn similarity_summary(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}
