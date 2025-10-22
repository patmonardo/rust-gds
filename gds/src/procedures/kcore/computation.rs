//! KCore Decomposition Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.kcore.KCoreDecomposition`
//!
//! K-core decomposition finds maximal subgraphs where every node has degree >= k.
//! Uses iterative SCAN/ACT phases to progressively remove low-degree nodes.

use crate::core::utils::paged::HugeLongArrayStack;

const UNASSIGNED: i32 = -1;
const REBUILD_CONSTANT: f64 = 0.02;

/// Phase of k-core decomposition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Scan,
    Act,
}

/// K-core decomposition result
#[derive(Clone)]
pub struct KCoreDecompositionResult {
    pub core_values: Vec<i32>,
    pub degeneracy: i32,
}

/// K-core decomposition computation runtime
pub struct KCoreDecompositionRuntime {
    /// Current degree of each node (mutable during computation)
    current_degrees: Vec<i32>,
    /// Core value assignment for each node
    core_values: Vec<i32>,
    /// Stack for processing nodes
    examination_stack: HugeLongArrayStack,
    /// Scanning degree (k in k-core)
    scanning_degree: i32,
    /// Maximum k value found (degeneracy)
    degeneracy: i32,
    /// Number of nodes still unassigned
    remaining_nodes: u64,
}

impl KCoreDecompositionRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            current_degrees: vec![0; node_count],
            core_values: vec![UNASSIGNED; node_count],
            examination_stack: HugeLongArrayStack::new(node_count),
            scanning_degree: 1,
            degeneracy: 0,
            remaining_nodes: node_count as u64,
        }
    }

    /// Compute k-core decomposition
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> KCoreDecompositionResult {
        // Phase 1: Initialize degrees and find zero-degree nodes
        let mut degree_zero_count = 0u64;
        for node_id in 0..node_count {
            let neighbors = get_neighbors(node_id);
            let degree = neighbors.len() as i32;
            self.current_degrees[node_id] = degree;

            if degree == 0 {
                self.core_values[node_id] = 0;
                degree_zero_count += 1;
            }
        }

        self.remaining_nodes = node_count as u64 - degree_zero_count;

        // Phase 2: Iteratively find k-cores
        self.scanning_degree = 1;
        let rebuild_limit = ((REBUILD_CONSTANT * node_count as f64).ceil()) as u64;
        let mut has_rebuild = false;

        while self.remaining_nodes > 0 {
            // Optimization: rebuild node list when few nodes remain
            if !has_rebuild && self.remaining_nodes < rebuild_limit {
                // In sequential version, we skip rebuild but could optimize later
                has_rebuild = true;
            }

            // SCAN Phase: Find all nodes with degree >= scanning_degree
            let mut smallest_active_degree = i32::MAX;
            self.examination_stack.clear();

            for node_id in 0..node_count {
                if self.core_values[node_id] == UNASSIGNED {
                    let node_degree = self.current_degrees[node_id];
                    if node_degree >= self.scanning_degree {
                        if node_degree == self.scanning_degree {
                            self.examination_stack.push(node_id as i64);
                        }
                        smallest_active_degree = smallest_active_degree.min(node_degree);
                    }
                }
            }

            // If no nodes found with scanning_degree, jump to next degree or exit
            if smallest_active_degree == i32::MAX {
                // All remaining nodes have lower degree - shouldn't happen
                break;
            }

            if smallest_active_degree == self.scanning_degree {
                // ACT Phase: Remove nodes with degree == scanning_degree
                let mut nodes_examined = 0u64;

                while !self.examination_stack.is_empty() {
                    let node_id = self.examination_stack.pop() as usize;
                    
                    // Skip already assigned nodes
                    if self.core_values[node_id] != UNASSIGNED {
                        continue;
                    }

                    self.core_values[node_id] = self.scanning_degree;
                    nodes_examined += 1;

                    // Relax neighbors: decrement their degrees
                    let neighbors = get_neighbors(node_id);
                    for &neighbor_id in &neighbors {
                        if self.core_values[neighbor_id] == UNASSIGNED {
                            self.current_degrees[neighbor_id] -= 1;

                            // If neighbor reaches scanning_degree, add to stack
                            if self.current_degrees[neighbor_id] == self.scanning_degree {
                                self.examination_stack.push(neighbor_id as i64);
                            }
                        }
                    }
                }

                self.remaining_nodes -= nodes_examined;
                self.degeneracy = self.scanning_degree;
                self.scanning_degree += 1;
            } else {
                // Skip to next smallest active degree
                self.scanning_degree = smallest_active_degree;
            }
        }

        // Assign remaining unassigned nodes (should not happen in valid graphs)
        for node_id in 0..node_count {
            if self.core_values[node_id] == UNASSIGNED {
                self.core_values[node_id] = self.degeneracy;
            }
        }

        KCoreDecompositionResult {
            core_values: self.core_values.clone(),
            degeneracy: self.degeneracy,
        }
    }
}
