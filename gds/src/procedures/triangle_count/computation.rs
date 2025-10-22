//! Triangle Count Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.triangle.IntersectingTriangleCount`
//!
//! Counts triangles using set intersection on adjacency lists.
//! For each node, we check if common neighbors exist to form triangles.

use std::collections::HashMap;

#[derive(Clone)]
pub struct TriangleCountComputationResult {
    pub local_triangles: Vec<u64>,
    pub global_triangles: u64,
}

pub struct TriangleCountComputationRuntime;

impl TriangleCountComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Compute triangle count using intersection approach
    /// For each edge (u, v) where u < v:
    ///   Find all nodes w that are adjacent to both u and v
    ///   Each such w forms a triangle (u, v, w)
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> TriangleCountComputationResult {
        let mut local_triangles = vec![0u64; node_count];
        let mut global_triangles = 0u64;

        // Build adjacency as HashMap for quick intersection
        let mut adjacency: HashMap<usize, std::collections::HashSet<usize>> = HashMap::new();
        for node in 0..node_count {
            let neighbors = get_neighbors(node);
            adjacency.insert(node, neighbors.into_iter().collect());
        }

        // For each pair of nodes (u, v) where u < v
        for u in 0..node_count {
            let u_neighbors = &adjacency[&u];
            for &v in u_neighbors {
                if v <= u {
                    continue; // Skip to ensure u < v
                }

                // Find common neighbors w where u < w and v < w
                let v_neighbors = &adjacency[&v];
                for &w in v_neighbors {
                    if w > v && u_neighbors.contains(&w) {
                        // Found triangle (u, v, w) with u < v < w
                        // Count it for all three nodes
                        local_triangles[u] += 1;
                        local_triangles[v] += 1;
                        local_triangles[w] += 1;
                        global_triangles += 1;
                    }
                }
            }
        }

        TriangleCountComputationResult {
            local_triangles,
            global_triangles,
        }
    }
}
