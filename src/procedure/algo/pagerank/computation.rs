//! PageRank Computation - Direct 1:1 Translation from Java
//!
//! Implements PregelComputation trait, directly mirroring:
//! org.neo4j.gds.pagerank.PageRankComputation
//!
//! See: /home/pat/GitHub/graph-data-science/algo/src/main/java/org/neo4j/gds/pagerank/PageRankComputation.java

use std::collections::HashSet;

use crate::config::PageRankConfig;
use crate::pregel::{
    ComputeContext, InitContext, MessageIterator, Messages, PregelComputation, PregelSchema,
};
use crate::types::ValueType;

// ============================================================================
// PageRankComputation - Main Implementation
// ============================================================================

/// PageRank Computation (Pregel-based)
///
/// Directly translated from Java GDS PageRankComputation.
/// Implements the Pregel computation model where:
/// - Each node runs `init()` once at the start
/// - Each node runs `compute()` every superstep while active
/// - Message passing propagates rank scores
/// - Convergence checked via delta tracking
///
/// Constructor args (Java):
/// - config: C extends PageRankConfig
/// - sourceNodes: LongSet
/// - degreeFunction: LongToDoubleFunction
///
/// We store config directly; degree is obtained via ComputeContext.
pub struct PageRankComputation {
    /// Damping factor (typically 0.85)
    damping_factor: f64,
    /// Convergence tolerance
    tolerance: f64,
    /// Source nodes for personalized PageRank (empty = all nodes)
    source_nodes: HashSet<u64>,
    /// Flag if source nodes are specified
    has_source_nodes: bool,
    /// Computed alpha = 1 - dampingFactor
    alpha: f64,
}

impl PageRankComputation {
    /// Create a new PageRank computation from PageRankConfig
    /// (Java: constructor taking C config, LongSet sourceNodes, LongToDoubleFunction degreeFunction)
    pub fn new(config: &PageRankConfig, source_nodes: HashSet<u64>) -> Self {
        let damping_factor = config.damping_factor;
        let alpha = 1.0 - damping_factor;
        let has_source_nodes = !source_nodes.is_empty();

        Self {
            damping_factor,
            tolerance: config.tolerance,
            source_nodes,
            has_source_nodes,
            alpha,
        }
    }

    /// Helper: compute initial value for a node
    /// (Java: initialValue(InitContext context))
    fn initial_value(&self, node_id: u64) -> f64 {
        if !self.has_source_nodes || self.source_nodes.contains(&node_id) {
            self.alpha
        } else {
            0.0
        }
    }
}

impl PregelComputation for PageRankComputation {
    type Config = PageRankConfig;

    /// Schema definition
    /// (Java: schema(C config) → PregelSchema.Builder().add(PAGE_RANK, ValueType.DOUBLE).build())
    fn schema(&self, _config: &Self::Config) -> PregelSchema {
        PregelSchema::builder()
            .add_public("pagerank", ValueType::Double)
            .build()
    }

    /// Initialize each node with alpha or 0 depending on source nodes
    /// (Java: init(InitContext context) → context.setNodeValue(PAGE_RANK, initialValue(context)))
    fn init(&mut self, context: &mut InitContext<Self::Config>) {
        let node_id = context.node_id();
        let initial_val = self.initial_value(node_id);
        context.set_node_value("pagerank", initial_val);
    }

    /// Per-node computation for each superstep
    /// (Java: compute(ComputeContext context, Messages messages))
    fn compute<I: MessageIterator>(
        &mut self,
        context: &mut ComputeContext<Self::Config, I>,
        messages: &mut Messages<I>,
    ) {
        // Java: double rank = context.doubleNodeValue(PAGE_RANK);
        let mut rank = context.double_node_value("pagerank");
        let mut delta = rank;

        // Java: if (!context.isInitialSuperstep()) { ... }
        if !context.is_initial_superstep() {
            // Java: double sum = 0; for (var message : messages) { sum += message; }
            let sum: f64 = messages.by_ref().sum();

            // Java: delta = dampingFactor * sum;
            delta = self.damping_factor * sum;

            // Java: context.setNodeValue(PAGE_RANK, rank + delta);
            rank += delta;
            context.set_node_value("pagerank", rank);
        }

        // Java: if (delta > tolerance || context.isInitialSuperstep())
        if delta.abs() > self.tolerance || context.is_initial_superstep() {
            let degree = context.degree();
            if degree > 0 {
                // Java: context.sendToNeighbors(delta / degree);
                context.send_to_neighbors(delta / degree as f64);
            }
        } else {
            // Java: context.voteToHalt();
            context.vote_to_halt();
        }
    }

    /// Apply relationship weight
    /// (Java: applyRelationshipWeight(double nodeValue, double relationshipWeight))
    fn apply_relationship_weight(&self, node_value: f64, relationship_weight: f64) -> f64 {
        node_value * relationship_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagerank_computation_new() {
        let config = PageRankConfig::default();
        let source_nodes = HashSet::new();
        let pr = PageRankComputation::new(&config, source_nodes);

        assert_eq!(pr.damping_factor, 0.85);
        assert!((pr.alpha - 0.15).abs() < 1e-10);
        assert!(!pr.has_source_nodes);
    }

    #[test]
    fn test_initial_value_no_source_nodes() {
        let config = PageRankConfig::default();
        let source_nodes = HashSet::new();
        let pr = PageRankComputation::new(&config, source_nodes);

        assert_eq!(pr.initial_value(0), pr.alpha);
        assert_eq!(pr.initial_value(999), pr.alpha);
    }

    #[test]
    fn test_initial_value_with_source_nodes() {
        let config = PageRankConfig::default();
        let mut source_nodes = HashSet::new();
        source_nodes.insert(1);
        source_nodes.insert(3);

        let pr = PageRankComputation::new(&config, source_nodes);

        assert_eq!(pr.initial_value(1), pr.alpha);
        assert_eq!(pr.initial_value(3), pr.alpha);
        assert_eq!(pr.initial_value(0), 0.0);
        assert_eq!(pr.initial_value(2), 0.0);
    }

    #[test]
    fn test_alpha_calculation() {
        let config = PageRankConfig {
            base: crate::config::base_types::AlgoBaseConfig::default(),
            max_iterations: 20,
            tolerance: 0.0001,
            damping_factor: 0.85,
            source_nodes: None,
        };
        let pr = PageRankComputation::new(&config, HashSet::new());
        assert_eq!(pr.alpha, 0.15);
    }
}
