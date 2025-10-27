//! KSpanningTree Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.kspanningtree.KSpanningTree`
//!
//! Computes k-spanning tree by:
//! 1. Computing MST using Prim's algorithm
//! 2. Progressively cutting k weakest edges

use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Result of k-spanning tree computation
#[derive(Clone)]
pub struct KSpanningTreeResult {
    pub parent: Vec<i64>,
    pub cost_to_parent: Vec<f64>,
    pub total_cost: f64,
    pub root: u64,
}

/// Priority queue element for edge tracking
#[derive(Debug, Clone)]
struct QueueElement {
    node_id: usize,
    cost: f64,
}

impl Eq for QueueElement {}

impl PartialEq for QueueElement {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id
    }
}

impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: reverse comparison
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// KSpanningTree computation runtime
pub struct KSpanningTreeComputationRuntime {
    parent: Vec<i64>,
    cost_to_parent: Vec<f64>,
    total_cost: f64,
}

impl KSpanningTreeComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            parent: vec![-1i64; node_count],
            cost_to_parent: vec![-1.0f64; node_count],
            total_cost: 0.0,
        }
    }

    /// Compute k-spanning tree
    pub fn compute(
        &mut self,
        node_count: usize,
        start_node: usize,
        k: u64,
        objective: &str,
        get_neighbors: impl Fn(usize) -> Vec<(usize, f64)>, // (neighbor, cost)
    ) -> KSpanningTreeResult {
        let is_min = objective == "min";

        // Step 1: Compute initial spanning tree using Prim's algorithm
        self.compute_mst(node_count, start_node, is_min, &get_neighbors);

        // Step 2: If k >= number of nodes, return as-is
        if k as usize >= node_count {
            return KSpanningTreeResult {
                parent: self.parent.clone(),
                cost_to_parent: self.cost_to_parent.clone(),
                total_cost: self.total_cost,
                root: start_node as u64,
            };
        }

        // Step 3: Grow approach - progressively cut k weakest edges
        self.grow_approach(node_count, start_node, k, is_min, &get_neighbors);

        KSpanningTreeResult {
            parent: self.parent.clone(),
            cost_to_parent: self.cost_to_parent.clone(),
            total_cost: self.total_cost,
            root: start_node as u64,
        }
    }

    /// Compute MST using Prim's algorithm
    fn compute_mst(
        &mut self,
        node_count: usize,
        start_node: usize,
        is_min: bool,
        get_neighbors: &impl Fn(usize) -> Vec<(usize, f64)>,
    ) {
        let mut visited = vec![false; node_count];
        let mut pq: BinaryHeap<QueueElement> = BinaryHeap::new();

        pq.push(QueueElement {
            node_id: start_node,
            cost: 0.0,
        });

        self.parent[start_node] = -1;
        self.cost_to_parent[start_node] = 0.0;
        self.total_cost = 0.0;

        while let Some(QueueElement { node_id, cost }) = pq.pop() {
            if visited[node_id] {
                continue;
            }

            visited[node_id] = true;
            self.total_cost += cost;

            for (neighbor, edge_cost) in get_neighbors(node_id) {
                if !visited[neighbor] {
                    if is_min {
                        pq.push(QueueElement {
                            node_id: neighbor,
                            cost: edge_cost,
                        });
                    } else {
                        pq.push(QueueElement {
                            node_id: neighbor,
                            cost: -edge_cost, // For max, use negative
                        });
                    }
                }
            }
        }
    }

    /// Grow approach: progressively add nodes and cut weakest edges
    fn grow_approach(
        &mut self,
        node_count: usize,
        root: usize,
        k: u64,
        is_min: bool,
        get_neighbors: &impl Fn(usize) -> Vec<(usize, f64)>,
    ) {
        let k = k as usize;
        let mut out_degree = vec![0usize; node_count];
        let mut included = vec![false; node_count];
        let mut exterior = vec![false; node_count];
        let mut pq: BinaryHeap<QueueElement> = BinaryHeap::new();
        let mut trim_pq: BinaryHeap<QueueElement> = BinaryHeap::new();

        // Initialize degrees
        for node_id in 0..node_count {
            if self.parent[node_id] != -1 {
                out_degree[self.parent[node_id] as usize] += 1;
            }
        }

        pq.push(QueueElement {
            node_id: root,
            cost: 0.0,
        });

        included[root] = true;
        exterior[root] = true;
        let mut nodes_in_tree = 1usize;

        // Main loop: add nodes or trim edges
        while let Some(QueueElement { node_id, cost }) = pq.pop() {
            if nodes_in_tree >= k {
                // Find worst leaf to trim
                if let Some(QueueElement {
                    node_id: worst_leaf,
                    cost: worst_cost,
                }) = trim_pq.pop()
                {
                    // Check if we should swap
                    let should_swap = if is_min {
                        cost < worst_cost
                    } else {
                        cost > worst_cost
                    };

                    if should_swap && self.parent[node_id] != worst_leaf as i64 {
                        // Remove worst leaf
                        self.parent[worst_leaf] = -1;
                        self.cost_to_parent[worst_leaf] = -1.0;
                        included[worst_leaf] = false;
                        nodes_in_tree -= 1;

                        // Add new node
                        included[node_id] = true;
                        self.cost_to_parent[node_id] = cost;
                        nodes_in_tree += 1;
                        exterior[node_id] = true;
                        trim_pq.push(QueueElement {
                            node_id,
                            cost,
                        });

                        // Relax neighbors
                        for (neighbor, edge_cost) in get_neighbors(node_id) {
                            if included[neighbor] && self.parent[neighbor] == node_id as i64 {
                                pq.push(QueueElement {
                                    node_id: neighbor,
                                    cost: edge_cost,
                                });
                            }
                        }
                    } else {
                        // Put leaf back if not swapping
                        trim_pq.push(QueueElement {
                            node_id: worst_leaf,
                            cost: worst_cost,
                        });
                    }
                } else {
                    break;
                }
            } else {
                // Add node to tree (size < k)
                included[node_id] = true;
                self.cost_to_parent[node_id] = cost;
                nodes_in_tree += 1;
                exterior[node_id] = true;

                trim_pq.push(QueueElement {
                    node_id,
                    cost,
                });

                // Relax neighbors
                for (neighbor, edge_cost) in get_neighbors(node_id) {
                    if !included[neighbor] && self.parent[neighbor] != -1 {
                        pq.push(QueueElement {
                            node_id: neighbor,
                            cost: edge_cost,
                        });
                    }
                }
            }
        }

        // Prune untouched nodes
        for node_id in 0..node_count {
            if !included[node_id] {
                self.parent[node_id] = -1;
                self.cost_to_parent[node_id] = -1.0;
            }
        }
    }
}
