//! CentralityAlgorithmResult - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.centrality.CentralityAlgorithmResult
//!
//! This is the base trait for all centrality algorithms, providing both
//! property values and a score provider function.

use crate::types::properties::PropertyValues;

/// Centrality algorithm result - translated from Java CentralityAlgorithmResult
/// 
/// This is the base trait for all centrality algorithms, providing both
/// property values and a score provider function.
pub trait CentralityAlgorithmResult {
    /// Get the node property values containing centrality scores
    /// Translated from: NodePropertyValues nodePropertyValues();
    fn node_property_values(&self) -> &dyn PropertyValues;
    
    /// Get a function that provides centrality scores for nodes
    /// Translated from: LongToDoubleFunction centralityScoreProvider();
    fn centrality_score_provider(&self) -> Box<dyn Fn(u64) -> f64 + Send + Sync>;
}
