//! KGEPredictParameters - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.machinelearning.KGEPredictParameters
//!
//! Parameters record for KGE prediction algorithm.

use super::link_scorer_factory::ScoreFunction;

/// KGE predict parameters - translated from Java KGEPredictParameters record
/// 
/// Parameters record for KGE prediction algorithm.
/// 
/// Java record:
/// ```java
/// @Parameters
/// public record KGEPredictParameters(
///     Concurrency concurrency,
///     NodeFilterSpec sourceNodeFilter,
///     NodeFilterSpec targetNodeFilter,
///     Collection<RelationshipType> relationshipTypesFilter,
///     List<Double> relationshipTypeEmbedding,
///     String nodeEmbeddingProperty,
///     ScoreFunction scoringFunction,
///     int topK
/// )
/// ```
#[derive(Debug, Clone)]
pub struct KGEPredictParameters {
    /// Concurrency - translated from Java: Concurrency concurrency
    pub concurrency: u32,
    
    /// Source node filter - translated from Java: NodeFilterSpec sourceNodeFilter
    pub source_node_filter: String, // TODO: Implement proper NodeFilterSpec
    
    /// Target node filter - translated from Java: NodeFilterSpec targetNodeFilter
    pub target_node_filter: String, // TODO: Implement proper NodeFilterSpec
    
    /// Relationship types filter - translated from Java: Collection<RelationshipType> relationshipTypesFilter
    pub relationship_types_filter: Vec<String>, // TODO: Implement proper RelationshipType
    
    /// Relationship type embedding - translated from Java: List<Double> relationshipTypeEmbedding
    pub relationship_type_embedding: Vec<f64>,
    
    /// Node embedding property - translated from Java: String nodeEmbeddingProperty
    pub node_embedding_property: String,
    
    /// Scoring function - translated from Java: ScoreFunction scoringFunction
    pub scoring_function: ScoreFunction,
    
    /// Top K - translated from Java: int topK
    pub top_k: i32,
}

impl KGEPredictParameters {
    /// Create new parameters instance
    pub fn new(
        concurrency: u32,
        source_node_filter: String,
        target_node_filter: String,
        relationship_types_filter: Vec<String>,
        relationship_type_embedding: Vec<f64>,
        node_embedding_property: String,
        scoring_function: ScoreFunction,
        top_k: i32,
    ) -> Self {
        Self {
            concurrency,
            source_node_filter,
            target_node_filter,
            relationship_types_filter,
            relationship_type_embedding,
            node_embedding_property,
            scoring_function,
            top_k,
        }
    }
}
