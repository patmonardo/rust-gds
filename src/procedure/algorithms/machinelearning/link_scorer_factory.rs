//! LinkScorerFactory - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.machinelearning.LinkScorerFactory
//!
//! Factory for creating LinkScorer instances based on score function and embeddings type.

use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use super::link_scorer::LinkScorer;

/// Score function enum - translated from Java ScoreFunction
#[derive(Debug, Clone, PartialEq)]
pub enum ScoreFunction {
    /// TransE scoring function - translated from Java TRANSE
    TransE,
    /// DistMult scoring function - translated from Java DISTMULT
    DistMult,
}

/// Link scorer factory - translated from Java LinkScorerFactory
/// 
/// Factory for creating LinkScorer instances based on score function and embeddings type.
pub struct LinkScorerFactory;

impl LinkScorerFactory {
    /// Create LinkScorer instance - translated from Java static method
    /// 
    /// Java method:
    /// ```java
    /// public static LinkScorer create(
    ///     ScoreFunction scoreFunction,
    ///     NodePropertyValues embeddings,
    ///     DoubleArrayList relationshipTypeEmbedding
    /// )
    /// ```
    pub fn create(
        score_function: ScoreFunction,
        embeddings: &dyn NodePropertyValues,
        relationship_type_embedding: Vec<f64>,
    ) -> Box<dyn LinkScorer> {
        match score_function {
            ScoreFunction::TransE => {
                match embeddings.value_type() {
                    ValueType::FloatArray => {
                        // TODO: Implement FloatEuclideanDistanceLinkScorer
                        todo!("Implement FloatEuclideanDistanceLinkScorer")
                    }
                    ValueType::DoubleArray => {
                        // TODO: Implement DoubleEuclideanDistanceLinkScorer
                        todo!("Implement DoubleEuclideanDistanceLinkScorer")
                    }
                    _ => {
                        panic!("Unsupported embeddings value type: {:?}", embeddings.value_type());
                    }
                }
            }
            ScoreFunction::DistMult => {
                match embeddings.value_type() {
                    ValueType::FloatArray => {
                        // TODO: Implement FloatDistMultLinkScorer
                        todo!("Implement FloatDistMultLinkScorer")
                    }
                    ValueType::DoubleArray => {
                        // TODO: Implement DoubleDistMultLinkScorer
                        todo!("Implement DoubleDistMultLinkScorer")
                    }
                    _ => {
                        panic!("Unsupported embeddings value type: {:?}", embeddings.value_type());
                    }
                }
            }
        }
    }
}
