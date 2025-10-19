//! KGEPredictConfigTransformer - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.machinelearning.KGEPredictConfigTransformer
//!
//! Config transformer for converting KGE predict base config to parameters.

use super::kge_predict_parameters::KGEPredictParameters;
use super::link_scorer_factory::ScoreFunction;

/// KGE predict base config - placeholder for Java KGEPredictBaseConfig
#[derive(Debug)]
pub struct KGEPredictBaseConfig {
    /// Concurrency - placeholder for Java concurrency()
    pub concurrency: u32,
    
    /// Source node filter - placeholder for Java sourceNodeFilter()
    pub source_node_filter: String,
    
    /// Target node filter - placeholder for Java targetNodeFilter()
    pub target_node_filter: String,
    
    /// Relationship types filter - placeholder for Java relationshipTypesFilter()
    pub relationship_types_filter: Vec<String>,
    
    /// Relationship type embedding - placeholder for Java relationshipTypeEmbedding()
    pub relationship_type_embedding: Vec<f64>,
    
    /// Node embedding property - placeholder for Java nodeEmbeddingProperty()
    pub node_embedding_property: String,
    
    /// Scoring function - placeholder for Java scoringFunction()
    pub scoring_function: ScoreFunction,
    
    /// Top K - placeholder for Java topK()
    pub top_k: i32,
}

impl KGEPredictBaseConfig {
    /// Create new config instance
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

/// KGE predict config transformer - translated from Java KGEPredictConfigTransformer
/// 
/// Config transformer for converting KGE predict base config to parameters.
pub struct KGEPredictConfigTransformer;

impl KGEPredictConfigTransformer {
    /// Convert config to parameters - translated from Java static method
    /// 
    /// Java method:
    /// ```java
    /// public static KGEPredictParameters toParameters(KGEPredictBaseConfig config) {
    ///     return new KGEPredictParameters(
    ///         config.concurrency(),
    ///         config.sourceNodeFilter(),
    ///         config.targetNodeFilter(),
    ///         config.relationshipTypesFilter(),
    ///         config.relationshipTypeEmbedding(),
    ///         config.nodeEmbeddingProperty(),
    ///         config.scoringFunction(),
    ///         config.topK()
    ///     );
    /// }
    /// ```
    pub fn to_parameters(config: KGEPredictBaseConfig) -> KGEPredictParameters {
        KGEPredictParameters::new(
            config.concurrency,
            config.source_node_filter,
            config.target_node_filter,
            config.relationship_types_filter,
            config.relationship_type_embedding,
            config.node_embedding_property,
            config.scoring_function,
            config.top_k,
        )
    }
}
