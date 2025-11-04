//! DegreeFunctions - First-pass translation from Java GDS
//!
//! Java Source:
//! - org.neo4j.gds.pagerank.DegreeFunctions
//!
//! Notes:
//! - This is an initial translation intended to compile and provide a usable API surface.
//! - Weighted degree normalization falls back to unweighted `degree()` until
//!   relationship weight accessors are wired (TODO).

use std::sync::Arc;

use crate::types::graph::{Graph, id_map::NodeId};

/// A callable degree function compatible with PageRank/Eigenvector variants.
///
/// **Type Note**: Pregel uses `u64` for node IDs internally, but `Graph::degree()`
/// expects `NodeId = i64`. This function handles the conversion.
///
/// The `get(node_id)` method takes `NodeId` (i64) for Graph compatibility.
/// Pregel contexts automatically convert their internal `u64` node IDs to `NodeId` (i64)
/// when calling Graph methods, so this matches that pattern.
#[derive(Clone)]
pub struct DegreeFunction {
    graph: Arc<dyn Graph>,
    /// When true, intent is to use weighted degree; currently falls back to unweighted
    has_relationship_weight_property: bool,
    /// Mode hint only for semantics (PageRank vs Eigenvector)
    mode: DegreeMode,
}

impl std::fmt::Debug for DegreeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DegreeFunction")
            .field(
                "has_relationship_weight_property",
                &self.has_relationship_weight_property,
            )
            .field("mode", &self.mode)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DegreeMode {
    PageRank,
    Eigenvector,
}

impl DegreeFunction {
    /// Return the (possibly weighted) degree denominator for a node.
    ///
    /// Contract: `node_id` is an internal/mapped ID (NodeId = i64) understood by `Graph`.
    pub fn get(&self, node_id: NodeId) -> f64 {
        if self.has_relationship_weight_property {
            // Sum outgoing relationship weights
            let mut sum = 0.0;
            let stream = self.graph.stream_relationships_weighted(node_id, 1.0);
            for cursor in stream {
                sum += cursor.weight();
            }
            return sum;
        }

        if matches!(self.mode, DegreeMode::Eigenvector) {
            // Java eigenvectorDegreeFunction returns 1 when unweighted
            return 1.0;
        }

        // Unweighted pagerank denominator = out-degree
        self.graph.degree(node_id) as f64
    }
}

/// PageRank degree function.
///
/// Java: DegreeFunctions.pageRankDegreeFunction(graph, hasWeight, concurrency)
pub fn pagerank_degree_function(
    graph: Arc<dyn Graph>,
    has_relationship_weight_property: bool,
) -> DegreeFunction {
    DegreeFunction {
        graph,
        has_relationship_weight_property,
        mode: DegreeMode::PageRank,
    }
}

/// Eigenvector degree function.
///
/// Java: DegreeFunctions.eigenvectorDegreeFunction(graph, hasWeight, concurrency)
pub fn eigenvector_degree_function(
    graph: Arc<dyn Graph>,
    has_relationship_weight_property: bool,
) -> DegreeFunction {
    DegreeFunction {
        graph,
        has_relationship_weight_property,
        mode: DegreeMode::Eigenvector,
    }
}

/// Average degree across all nodes (unweighted for now).
///
/// Java: DegreeFunctions.averageDegree(graph, concurrency)
pub fn average_degree(graph: &Arc<dyn Graph>) -> f64 {
    let node_count = graph.node_count();
    if node_count == 0 {
        return 0.0;
    }
    let mut sum: usize = 0;
    for node_id in 0..node_count as NodeId {
        sum += graph.degree(node_id);
    }
    sum as f64 / node_count as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full tests would require a mock Graph. These are compile-time sanity checks.
    // The constructors are verified to compile - actual graph instantiation happens
    // at runtime with real graph stores.
}


