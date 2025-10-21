//! Degree Centrality Storage Runtime
//!
//! This module implements the **Gross pole** of the Functor machinery for Degree Centrality.
//! It represents persistent data structures (GraphStore and graph topology).
//!
//! **Translation Source**: `org.neo4j.gds.degree.DegreeCentrality.java`
//! **Key Features**: Orientation handling, weighted/unweighted, parallel execution

use crate::projection::eval::procedure::AlgorithmError;
use crate::types::prelude::GraphStore;

/// Edge orientation for degree computation
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Orientation {
    /// Natural orientation (outgoing edges)
    Natural,
    /// Reverse orientation (incoming edges)  
    Reverse,
    /// Undirected (both incoming and outgoing)
    Undirected,
}

/// Storage Runtime for Degree Centrality
///
/// This is the **Gross pole** - persistent data structures.
/// It knows how to access the graph structure and compute node degrees.
///
/// ## The Pole's Role
///
/// In the Functor machinery:
/// - **Storage Runtime** (Gross) = persistent GraphStore and graph topology
/// - **Computation Runtime** (Subtle) = ephemeral degree scores and statistics
/// - **Functor** = the mapping between them via degree computation
pub struct DegreeCentralityStorageRuntime<'a, G: GraphStore> {
    /// Reference to the graph store
    graph_store: &'a G,
    /// Edge orientation for computation
    orientation: Orientation,
    /// Whether to use relationship weights
    has_relationship_weight_property: bool,
}

impl<'a, G: GraphStore> DegreeCentralityStorageRuntime<'a, G> {
    /// Create a new storage runtime
    pub fn new(graph_store: &'a G) -> Result<Self, AlgorithmError> {
        Ok(Self { 
            graph_store,
            orientation: Orientation::Natural,
            has_relationship_weight_property: false,
        })
    }

    /// Create with specific orientation and weight settings
    pub fn with_settings(
        graph_store: &'a G, 
        orientation: Orientation,
        has_relationship_weight_property: bool,
    ) -> Result<Self, AlgorithmError> {
        Ok(Self { 
            graph_store,
            orientation,
            has_relationship_weight_property,
        })
    }

    /// Get reference to graph store
    pub fn graph_store(&self) -> &'a G {
        self.graph_store
    }

    /// Get node degree from storage
    ///
    /// This projects from GraphStore (Gross - persistent topology)
    /// to f64 (Subtle - degree count/weight).
    ///
    /// **This is where the Functor machinery actually works**:
    /// GraphStore (Gross) → f64 (Subtle)
    ///
    /// **Translation of Java logic**:
    /// - NATURAL: Use graph.degree() directly
    /// - REVERSE: Count incoming edges  
    /// - UNDIRECTED: Count both incoming and outgoing
    /// - Weighted: Sum relationship weights
    /// - Unweighted: Count relationship count
    pub fn get_node_degree(&self, node_id: u32) -> Result<f64, AlgorithmError> {
        match self.orientation {
            Orientation::Natural => {
                if self.has_relationship_weight_property {
                    self.compute_weighted_natural_degree(node_id)
                } else {
                    self.compute_unweighted_natural_degree(node_id)
                }
            }
            Orientation::Reverse => {
                if self.has_relationship_weight_property {
                    self.compute_weighted_reverse_degree(node_id)
                } else {
                    self.compute_unweighted_reverse_degree(node_id)
                }
            }
            Orientation::Undirected => {
                if self.has_relationship_weight_property {
                    self.compute_weighted_undirected_degree(node_id)
                } else {
                    self.compute_unweighted_undirected_degree(node_id)
                }
            }
        }
    }

    /// Compute unweighted natural (outgoing) degree
    fn compute_unweighted_natural_degree(&self, node_id: u32) -> Result<f64, AlgorithmError> {
        // For NATURAL orientation, just count outgoing edges
        // TODO: Replace with actual GraphStore API call
        // This is a placeholder that simulates the Java graph.degree() call
        let mock_degree = (node_id % 15) as f64;
        Ok(mock_degree)
    }

    /// Compute weighted natural (outgoing) degree  
    fn compute_weighted_natural_degree(&self, node_id: u32) -> Result<f64, AlgorithmError> {
        // For NATURAL orientation with weights, sum outgoing edge weights
        // TODO: Replace with actual GraphStore API call
        // This simulates the Java NaturalWeightedDegreeTask logic
        let mock_weighted_degree = (node_id % 15) as f64 * 1.5;
        Ok(mock_weighted_degree)
    }

    /// Compute unweighted reverse (incoming) degree
    fn compute_unweighted_reverse_degree(&self, node_id: u32) -> Result<f64, AlgorithmError> {
        // For REVERSE orientation, count incoming edges
        // TODO: Replace with actual GraphStore API call
        // This simulates the Java ReverseDegreeTask logic
        let mock_incoming_degree = (node_id % 12) as f64;
        Ok(mock_incoming_degree)
    }

    /// Compute weighted reverse (incoming) degree
    fn compute_weighted_reverse_degree(&self, node_id: u32) -> Result<f64, AlgorithmError> {
        // For REVERSE orientation with weights, sum incoming edge weights
        // TODO: Replace with actual GraphStore API call
        let mock_weighted_incoming_degree = (node_id % 12) as f64 * 1.2;
        Ok(mock_weighted_incoming_degree)
    }

    /// Compute unweighted undirected degree
    fn compute_unweighted_undirected_degree(&self, node_id: u32) -> Result<f64, AlgorithmError> {
        // For UNDIRECTED orientation, count both incoming and outgoing edges
        // TODO: Replace with actual GraphStore API call
        // This simulates the Java UndirectedDegreeTask logic
        let outgoing = (node_id % 15) as f64;
        let incoming = (node_id % 12) as f64;
        Ok(outgoing + incoming)
    }

    /// Compute weighted undirected degree
    fn compute_weighted_undirected_degree(&self, node_id: u32) -> Result<f64, AlgorithmError> {
        // For UNDIRECTED orientation with weights, sum both incoming and outgoing edge weights
        // TODO: Replace with actual GraphStore API call
        // This simulates the Java UndirectedWeightedDegreeTask logic
        let outgoing_weighted = (node_id % 15) as f64 * 1.5;
        let incoming_weighted = (node_id % 12) as f64 * 1.2;
        Ok(outgoing_weighted + incoming_weighted)
    }

    /// Get total number of nodes
    pub fn node_count(&self) -> usize {
        self.graph_store.node_count()
    }

    /// Get orientation setting
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    /// Check if using relationship weights
    pub fn has_relationship_weight_property(&self) -> bool {
        self.has_relationship_weight_property
    }
}
