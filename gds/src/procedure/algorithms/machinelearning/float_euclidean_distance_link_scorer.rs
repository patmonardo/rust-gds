//! FloatEuclideanDistanceLinkScorer - Faithful 1:1 translation from Java GDS
//!
//! Translated from: org.neo4j.gds.algorithms.machinelearning.FloatEuclideanDistanceLinkScorer
//!
//! Euclidean distance link scorer implementation using float arrays for embeddings.

use crate::types::properties::node::NodePropertyValues;
use super::link_scorer::LinkScorer;

/// Float Euclidean distance link scorer - translated from Java FloatEuclideanDistanceLinkScorer
/// 
/// Euclidean distance link scorer implementation using float arrays for embeddings.
/// Implements the TransE scoring function: score = sqrt(sum((source[i] + relation[i] - target[i])^2))
#[derive(Debug)]
pub struct FloatEuclideanDistanceLinkScorer {
    /// Embeddings - translated from Java: NodePropertyValues embeddings;
    embeddings: Box<dyn NodePropertyValues>,
    
    /// Relationship type embedding - translated from Java: double[] relationshipTypeEmbedding;
    relationship_type_embedding: Vec<f64>,
    
    /// Current source node - translated from Java: long currentSourceNode;
    current_source_node: u64,
    
    /// Current candidate target - translated from Java: float[] currentCandidateTarget;
    current_candidate_target: Vec<f32>,
}

impl FloatEuclideanDistanceLinkScorer {
    /// Constructor - translated from Java constructor
    /// 
    /// Java constructor:
    /// ```java
    /// FloatEuclideanDistanceLinkScorer(NodePropertyValues embeddings, DoubleArrayList relationshipTypeEmbedding) {
    ///     this.embeddings = embeddings;
    ///     this.relationshipTypeEmbedding = relationshipTypeEmbedding.toArray();
    ///     this.currentCandidateTarget = new float[this.relationshipTypeEmbedding.length];
    /// }
    /// ```
    pub fn new(
        embeddings: Box<dyn NodePropertyValues>,
        relationship_type_embedding: Vec<f64>,
    ) -> Self {
        let current_candidate_target = vec![0.0; relationship_type_embedding.len()];
        
        Self {
            embeddings,
            relationship_type_embedding,
            current_source_node: 0,
            current_candidate_target,
        }
    }
}

impl LinkScorer for FloatEuclideanDistanceLinkScorer {
    /// Initialize scorer for source node - translated from Java init method
    /// 
    /// Java method:
    /// ```java
    /// public void init(long sourceNode) {
    ///     this.currentSourceNode = sourceNode;
    ///     var currentSource = embeddings.floatArrayValue(currentSourceNode);
    ///     for(int i = 0; i < relationshipTypeEmbedding.length; i++){
    ///         this.currentCandidateTarget[i] = (float) (currentSource[i] + relationshipTypeEmbedding[i]);
    ///     }
    /// }
    /// ```
    fn init(&mut self, source_node: u64) {
        self.current_source_node = source_node;
        
        // Get source embedding vector
        let current_source = self.embeddings.float_array_value(source_node)
            .unwrap_or_else(|_| vec![0.0; self.relationship_type_embedding.len()]);
        
        // Compute currentCandidateTarget = (float) (currentSource + relationshipTypeEmbedding)
        for i in 0..self.relationship_type_embedding.len() {
            self.current_candidate_target[i] = (current_source[i] as f64 + self.relationship_type_embedding[i]) as f32;
        }
    }
    
    /// Compute score for target node - translated from Java computeScore method
    /// 
    /// Java method:
    /// ```java
    /// public double computeScore(long targetNode) {
    ///     double res = 0.0;
    ///     var targetVector = embeddings.floatArrayValue(targetNode);
    ///     for (int i = 0; i < currentCandidateTarget.length; i++) {
    ///         double elem = currentCandidateTarget[i] - targetVector[i];
    ///         res += elem * elem;
    ///     }
    ///     return Math.sqrt(res);
    /// }
    /// ```
    fn compute_score(&self, target_node: u64) -> f64 {
        let mut res = 0.0;
        
        // Get target embedding vector
        let target_vector = self.embeddings.float_array_value(target_node)
            .unwrap_or_else(|_| vec![0.0; self.current_candidate_target.len()]);
        
        // Compute Euclidean distance: res = sqrt(sum((currentCandidateTarget[i] - targetVector[i])^2))
        for i in 0..self.current_candidate_target.len() {
            let elem = self.current_candidate_target[i] as f64 - target_vector[i] as f64;
            res += elem * elem;
        }
        
        res.sqrt()
    }
}
