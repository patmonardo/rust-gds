//! LinkScorer - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.machinelearning.LinkScorer
//!
//! Interface for scoring links between nodes in knowledge graph embeddings.

/// Link scorer - translated from Java LinkScorer interface
/// 
/// Interface for scoring links between nodes in knowledge graph embeddings.
/// Extends AutoCloseable for resource management.
pub trait LinkScorer {
    /// Initialize the scorer for a source node
    /// 
    /// Translated from Java method:
    /// ```java
    /// void init(long sourceNode);
    /// ```
    fn init(&mut self, source_node: u64);
    
    /// Compute score for a target node
    /// 
    /// Translated from Java method:
    /// ```java
    /// double computeScore(long targetNode);
    /// ```
    fn compute_score(&self, target_node: u64) -> f64;
}
