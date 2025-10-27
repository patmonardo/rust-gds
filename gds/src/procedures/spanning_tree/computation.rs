//! Spanning Tree Result Type and Graph Adapter
//!
//! **Translation Source**: `org.neo4j.gds.spanningtree.SpanningTree` and `SpanningGraph`
//!
//! This module implements the result type for spanning tree algorithms and
//! provides a graph adapter for traversing the spanning tree structure.

use serde::{Deserialize, Serialize};

/// Spanning tree result containing parent relationships and costs.
///
/// **Translation Source**: `org.neo4j.gds.spanningtree.SpanningTree`
///
/// Represents a spanning tree with:
/// - Parent array mapping each node to its parent
/// - Cost function for edge weights
/// - Total weight of the spanning tree
/// - Effective node count (nodes actually in the tree)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanningTree {
    /// Root/head node of the spanning tree
    pub head: u32,
    
    /// Total number of nodes in the original graph
    pub node_count: u32,
    
    /// Number of nodes actually included in the spanning tree
    pub effective_node_count: u32,
    
    /// Parent array: parent[node_id] = parent_node_id (-1 if root)
    pub parent: Vec<i32>,
    
    /// Cost to parent for each node
    pub cost_to_parent: Vec<f64>,
    
    /// Total weight of the spanning tree
    pub total_weight: f64,
}

impl SpanningTree {
    /// Creates a new spanning tree result.
    ///
    /// # Arguments
    ///
    /// * `head` - Root node of the spanning tree
    /// * `node_count` - Total number of nodes in the original graph
    /// * `effective_node_count` - Number of nodes in the spanning tree
    /// * `parent` - Parent array (parent[i] = parent of node i, -1 for root)
    /// * `cost_to_parent` - Cost to parent for each node
    /// * `total_weight` - Total weight of the spanning tree
    ///
    /// # Returns
    ///
    /// A new `SpanningTree` instance.
    pub fn new(
        head: u32,
        node_count: u32,
        effective_node_count: u32,
        parent: Vec<i32>,
        cost_to_parent: Vec<f64>,
        total_weight: f64,
    ) -> Self {
        Self {
            head,
            node_count,
            effective_node_count,
            parent,
            cost_to_parent,
            total_weight,
        }
    }
    
    /// Get the effective node count (nodes in the spanning tree).
    ///
    /// # Returns
    ///
    /// The number of nodes included in the spanning tree.
    pub fn effective_node_count(&self) -> u32 {
        self.effective_node_count
    }
    
    /// Get the total weight of the spanning tree.
    ///
    /// # Returns
    ///
    /// The total weight/cost of all edges in the spanning tree.
    pub fn total_weight(&self) -> f64 {
        self.total_weight
    }
    
    /// Get the parent array.
    ///
    /// # Returns
    ///
    /// A reference to the parent array.
    pub fn parent_array(&self) -> &[i32] {
        &self.parent
    }
    
    /// Get the parent of a specific node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node ID to get the parent for
    ///
    /// # Returns
    ///
    /// The parent node ID, or -1 if the node is the root.
    pub fn parent(&self, node_id: u32) -> i32 {
        if node_id < self.parent.len() as u32 {
            self.parent[node_id as usize]
        } else {
            -1
        }
    }
    
    /// Get the cost to parent for a specific node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node ID to get the cost for
    ///
    /// # Returns
    ///
    /// The cost to the parent node.
    pub fn cost_to_parent(&self, node_id: u32) -> f64 {
        if node_id < self.cost_to_parent.len() as u32 {
            self.cost_to_parent[node_id as usize]
        } else {
            0.0
        }
    }
    
    /// Find the head/root node for a given node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to find the head for
    ///
    /// # Returns
    ///
    /// The head/root node of the tree containing the given node.
    pub fn head(&self, node: u32) -> u32 {
        let mut current = node;
        while current < self.parent.len() as u32 && self.parent[current as usize] != -1 {
            current = self.parent[current as usize] as u32;
        }
        current
    }
    
    /// Iterate over all edges in the spanning tree.
    ///
    /// # Arguments
    ///
    /// * `mut consumer` - Closure that receives (source, target, cost) for each edge
    ///
    /// # Returns
    ///
    /// `true` if iteration completed, `false` if consumer requested early termination.
    pub fn for_each_edge<F>(&self, mut consumer: F) -> bool
    where
        F: FnMut(u32, u32, f64) -> bool,
    {
        for i in 0..self.node_count {
            let parent = self.parent[i as usize];
            if parent == -1 {
                continue; // Skip root nodes
            }
            
            let cost = self.cost_to_parent[i as usize];
            if !consumer(parent as u32, i, cost) {
                return false;
            }
        }
        true
    }
}

impl Default for SpanningTree {
    fn default() -> Self {
        Self::new(0, 0, 0, Vec::new(), Vec::new(), 0.0)
    }
}

/// Graph adapter for spanning tree traversal.
///
/// **Translation Source**: `org.neo4j.gds.spanningtree.SpanningGraph`
///
/// Provides a graph-like interface for traversing a spanning tree structure.
/// This adapter implements graph traversal methods that only follow edges
/// present in the spanning tree.
#[derive(Debug, Clone)]
pub struct SpanningGraph<'a> {
    /// Reference to the underlying spanning tree
    spanning_tree: &'a SpanningTree,
    
    /// Original graph node count
    node_count: u32,
}

impl<'a> SpanningGraph<'a> {
    /// Creates a new spanning graph adapter.
    ///
    /// # Arguments
    ///
    /// * `spanning_tree` - Reference to the spanning tree
    /// * `node_count` - Original graph node count
    ///
    /// # Returns
    ///
    /// A new `SpanningGraph` adapter.
    pub fn new(spanning_tree: &'a SpanningTree, node_count: u32) -> Self {
        Self {
            spanning_tree,
            node_count,
        }
    }
    
    /// Get the degree of a node in the spanning tree.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node ID
    ///
    /// # Returns
    ///
    /// The degree of the node in the spanning tree.
    pub fn degree(&self, node_id: u32) -> usize {
        if node_id >= self.node_count {
            return 0;
        }
        
        let parent = self.spanning_tree.parent(node_id);
        if parent == -1 {
            // Root node - count children
            let mut children = 0;
            for i in 0..self.node_count {
                if self.spanning_tree.parent(i) == node_id as i32 {
                    children += 1;
                }
            }
            children
        } else {
            // Non-root node - has exactly one parent
            1
        }
    }
    
    /// Iterate over relationships of a node in the spanning tree.
    ///
    /// # Arguments
    ///
    /// * `node_id` - The node ID
    /// * `mut consumer` - Closure that receives (source, target, cost) for each relationship
    ///
    /// # Returns
    ///
    /// `true` if iteration completed, `false` if consumer requested early termination.
    pub fn for_each_relationship<F>(&self, node_id: u32, mut consumer: F) -> bool
    where
        F: FnMut(u32, u32, f64) -> bool,
    {
        if node_id >= self.node_count {
            return true;
        }
        
        let parent = self.spanning_tree.parent(node_id);
        if parent >= 0 {
            let cost = self.spanning_tree.cost_to_parent(node_id);
            if !consumer(parent as u32, node_id, cost) {
                return false;
            }
        }
        
        // Also iterate over children
        for i in 0..self.node_count {
            if self.spanning_tree.parent(i) == node_id as i32 {
                let cost = self.spanning_tree.cost_to_parent(i);
                if !consumer(node_id, i, cost) {
                    return false;
                }
            }
        }
        
        true
    }
    
    /// Check if a relationship exists in the spanning tree.
    ///
    /// # Arguments
    ///
    /// * `source_node_id` - Source node ID
    /// * `target_node_id` - Target node ID
    ///
    /// # Returns
    ///
    /// `true` if the relationship exists in the spanning tree.
    pub fn exists(&self, source_node_id: u32, target_node_id: u32) -> bool {
        if source_node_id >= self.node_count || target_node_id >= self.node_count {
            return false;
        }
        
        // Check if target is parent of source
        if self.spanning_tree.parent(source_node_id) == target_node_id as i32 {
            return true;
        }
        
        // Check if source is parent of target
        if self.spanning_tree.parent(target_node_id) == source_node_id as i32 {
            return true;
        }
        
        false
    }
    
    /// Get the node count.
    ///
    /// # Returns
    ///
    /// The number of nodes in the original graph.
    pub fn node_count(&self) -> u32 {
        self.node_count
    }
}

/// Priority queue item for Prim's algorithm
#[derive(Debug, Clone)]
struct QueueItem {
    node_id: u32,
    cost: f64,
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.node_id == other.node_id
    }
}

impl Eq for QueueItem {}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reverse ordering for min-heap (lower cost = higher priority)
        other.cost.partial_cmp(&self.cost)
    }
}

impl std::cmp::Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// Spanning Tree Computation Runtime
///
/// **Translation Source**: `org.neo4j.gds.spanningtree.Prim`
///
/// Implements Prim's algorithm for computing minimum or maximum spanning trees.
/// This is the "Subtle pole" handling ephemeral computation state.
pub struct SpanningTreeComputationRuntime {
    /// Start node for the spanning tree
    start_node_id: u32,
    
    /// Whether to compute minimum (true) or maximum (false) spanning tree
    compute_minimum: bool,
    
    /// Priority queue for unvisited nodes
    priority_queue: std::collections::BinaryHeap<QueueItem>,
    
    /// Visited set
    visited: std::collections::HashSet<u32>,
    
    /// Parent array for the spanning tree
    parent: Vec<i32>,
    
    /// Cost to parent for each node
    cost_to_parent: Vec<f64>,
    
    /// Total weight of the spanning tree
    total_weight: f64,
    
    /// Effective node count (nodes in the spanning tree)
    effective_node_count: u32,
    
    /// Concurrency level
    concurrency: usize,
}

impl SpanningTreeComputationRuntime {
    /// Creates a new SpanningTreeComputationRuntime.
    ///
    /// # Arguments
    ///
    /// * `start_node_id` - Starting node for the spanning tree
    /// * `compute_minimum` - Whether to compute minimum (true) or maximum (false) spanning tree
    /// * `node_count` - Total number of nodes in the graph
    /// * `concurrency` - Concurrency level
    ///
    /// # Returns
    ///
    /// A new `SpanningTreeComputationRuntime` instance.
    pub fn new(
        start_node_id: u32,
        compute_minimum: bool,
        node_count: u32,
        concurrency: usize,
    ) -> Self {
        Self {
            start_node_id,
            compute_minimum,
            priority_queue: std::collections::BinaryHeap::new(),
            visited: std::collections::HashSet::new(),
            parent: vec![-1; node_count as usize],
            cost_to_parent: vec![0.0; node_count as usize],
            total_weight: 0.0,
            effective_node_count: 0,
            concurrency,
        }
    }
    
    /// Initialize the computation runtime for a new run.
    ///
    /// # Arguments
    ///
    /// * `start_node_id` - Starting node for this run
    pub fn initialize(&mut self, start_node_id: u32) {
        self.start_node_id = start_node_id;
        self.priority_queue.clear();
        self.visited.clear();
        self.parent.fill(-1);
        self.cost_to_parent.fill(0.0);
        self.total_weight = 0.0;
        self.effective_node_count = 0;
        
        // Add start node to queue with cost 0
        self.priority_queue.push(QueueItem {
            node_id: start_node_id,
            cost: 0.0,
        });
    }
    
    /// Add a node to the priority queue.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID to add
    /// * `cost` - Cost to reach this node
    /// * `parent_id` - Parent node ID
    pub fn add_to_queue(&mut self, node_id: u32, cost: f64, parent_id: u32) {
        self.priority_queue.push(QueueItem {
            node_id,
            cost,
        });
        self.parent[node_id as usize] = parent_id as i32;
        self.cost_to_parent[node_id as usize] = cost;
    }
    
    /// Update the cost for a node already in the queue.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID to update
    /// * `cost` - New cost
    /// * `parent_id` - New parent node ID
    pub fn update_cost(&mut self, node_id: u32, cost: f64, parent_id: u32) {
        // For simplicity, we'll add a new entry to the queue
        // In a more sophisticated implementation, we'd update the existing entry
        self.priority_queue.push(QueueItem {
            node_id,
            cost,
        });
        self.parent[node_id as usize] = parent_id as i32;
        self.cost_to_parent[node_id as usize] = cost;
    }
    
    /// Pop the next node from the priority queue.
    ///
    /// # Returns
    ///
    /// The next node ID and its cost, or None if queue is empty.
    pub fn pop_from_queue(&mut self) -> Option<(u32, f64)> {
        while let Some(item) = self.priority_queue.pop() {
            if !self.visited.contains(&item.node_id) {
                return Some((item.node_id, item.cost));
            }
        }
        None
    }
    
    /// Mark a node as visited.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID to mark as visited
    /// * `cost` - Cost to reach this node
    pub fn mark_visited(&mut self, node_id: u32, cost: f64) {
        self.visited.insert(node_id);
        if self.compute_minimum {
            self.total_weight += cost;
        } else {
            // For maximum spanning tree, costs in the queue are negated.
            // Add the original (positive) edge weight to the total.
            if node_id != self.start_node_id {
                self.total_weight += -cost;
            }
        }
        self.effective_node_count += 1;
    }
    
    /// Check if a node has been visited.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID to check
    ///
    /// # Returns
    ///
    /// `true` if the node has been visited.
    pub fn is_visited(&self, node_id: u32) -> bool {
        self.visited.contains(&node_id)
    }
    
    /// Get the parent of a node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID
    ///
    /// # Returns
    ///
    /// The parent node ID, or -1 if no parent.
    pub fn parent(&self, node_id: u32) -> i32 {
        if node_id < self.parent.len() as u32 {
            self.parent[node_id as usize]
        } else {
            -1
        }
    }
    
    /// Get the cost to parent for a node.
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID
    ///
    /// # Returns
    ///
    /// The cost to the parent node.
    pub fn cost_to_parent(&self, node_id: u32) -> f64 {
        if node_id < self.cost_to_parent.len() as u32 {
            self.cost_to_parent[node_id as usize]
        } else {
            0.0
        }
    }
    
    /// Get the total weight of the spanning tree.
    ///
    /// # Returns
    ///
    /// The total weight of all edges in the spanning tree.
    pub fn total_weight(&self) -> f64 {
        self.total_weight
    }
    
    /// Get the effective node count.
    ///
    /// # Returns
    ///
    /// The number of nodes included in the spanning tree.
    pub fn effective_node_count(&self) -> u32 {
        self.effective_node_count
    }
    
    /// Get the visited nodes.
    ///
    /// # Returns
    ///
    /// A reference to the visited set.
    pub fn visited_nodes(&self) -> &std::collections::HashSet<u32> {
        &self.visited
    }
    
    /// Check if the queue is empty.
    ///
    /// # Returns
    ///
    /// `true` if the priority queue is empty.
    pub fn is_queue_empty(&self) -> bool {
        self.priority_queue.is_empty()
    }
    
    /// Apply weight transformation for min/max spanning tree.
    ///
    /// # Arguments
    ///
    /// * `weight` - Original edge weight
    ///
    /// # Returns
    ///
    /// Transformed weight for the algorithm.
    pub fn transform_weight(&self, weight: f64) -> f64 {
        if self.compute_minimum {
            weight
        } else {
            -weight // For maximum spanning tree, negate weights
        }
    }
    
    /// Build the final spanning tree result.
    ///
    /// # Arguments
    ///
    /// * `node_count` - Total number of nodes in the graph
    ///
    /// # Returns
    ///
    /// The completed `SpanningTree` result.
    pub fn build_result(&self, node_count: u32) -> SpanningTree {
        SpanningTree::new(
            self.start_node_id,
            node_count,
            self.effective_node_count,
            self.parent.clone(),
            self.cost_to_parent.clone(),
            self.total_weight,
        )
    }
}

#[cfg(test)]
mod computation_tests {
    use super::*;
    
    #[test]
    fn test_computation_runtime_creation() {
        let runtime = SpanningTreeComputationRuntime::new(0, true, 4, 1);
        
        assert_eq!(runtime.start_node_id, 0);
        assert!(runtime.compute_minimum);
        assert_eq!(runtime.parent.len(), 4);
        assert_eq!(runtime.cost_to_parent.len(), 4);
        assert_eq!(runtime.total_weight(), 0.0);
        assert_eq!(runtime.effective_node_count(), 0);
    }
    
    #[test]
    fn test_computation_runtime_initialization() {
        let mut runtime = SpanningTreeComputationRuntime::new(0, true, 4, 1);
        runtime.initialize(1);
        
        assert_eq!(runtime.start_node_id, 1);
        assert!(!runtime.is_queue_empty());
        assert_eq!(runtime.visited.len(), 0);
        assert_eq!(runtime.total_weight(), 0.0);
    }
    
    #[test]
    fn test_computation_runtime_queue_operations() {
        let mut runtime = SpanningTreeComputationRuntime::new(0, true, 4, 1);
        runtime.initialize(0);
        
        // Add nodes to queue
        runtime.add_to_queue(1, 1.0, 0);
        runtime.add_to_queue(2, 2.0, 0);
        
        // Pop nodes
        let (node1, cost1) = runtime.pop_from_queue().unwrap();
        assert_eq!(node1, 0); // Start node should be first
        assert_eq!(cost1, 0.0);
        
        let (node2, cost2) = runtime.pop_from_queue().unwrap();
        assert_eq!(node2, 1); // Lower cost should be next
        assert_eq!(cost2, 1.0);
        
        let (node3, cost3) = runtime.pop_from_queue().unwrap();
        assert_eq!(node3, 2); // Higher cost should be last
        assert_eq!(cost3, 2.0);
    }
    
    #[test]
    fn test_computation_runtime_visited_tracking() {
        let mut runtime = SpanningTreeComputationRuntime::new(0, true, 4, 1);
        runtime.initialize(0);
        
        assert!(!runtime.is_visited(0));
        runtime.mark_visited(0, 0.0);
        assert!(runtime.is_visited(0));
        assert_eq!(runtime.effective_node_count(), 1);
        assert_eq!(runtime.total_weight(), 0.0);
        
        runtime.mark_visited(1, 1.0);
        assert!(runtime.is_visited(1));
        assert_eq!(runtime.effective_node_count(), 2);
        assert_eq!(runtime.total_weight(), 1.0);
    }
    
    #[test]
    fn test_computation_runtime_weight_transformation() {
        let runtime_min = SpanningTreeComputationRuntime::new(0, true, 4, 1);
        let runtime_max = SpanningTreeComputationRuntime::new(0, false, 4, 1);
        
        assert_eq!(runtime_min.transform_weight(5.0), 5.0);
        assert_eq!(runtime_max.transform_weight(5.0), -5.0);
    }
    
    #[test]
    fn test_computation_runtime_result_building() {
        let mut runtime = SpanningTreeComputationRuntime::new(0, true, 4, 1);
        runtime.initialize(0);
        
        runtime.mark_visited(0, 0.0);
        runtime.mark_visited(1, 1.0);
        runtime.parent[1] = 0;
        runtime.cost_to_parent[1] = 1.0;
        
        let result = runtime.build_result(4);
        
        assert_eq!(result.head(0), 0);
        assert_eq!(result.effective_node_count(), 2);
        assert_eq!(result.total_weight(), 1.0);
        assert_eq!(result.parent(1), 0);
        assert_eq!(result.cost_to_parent(1), 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spanning_tree_creation() {
        let parent = vec![-1, 0, 1, 0]; // Node 0 is root, 1->0, 2->1, 3->0
        let cost_to_parent = vec![0.0, 1.0, 2.0, 1.5];
        
        let spanning_tree = SpanningTree::new(
            0, // head
            4, // node_count
            4, // effective_node_count
            parent,
            cost_to_parent,
            4.5, // total_weight
        );
        
        assert_eq!(spanning_tree.head(0), 0);
        assert_eq!(spanning_tree.effective_node_count(), 4);
        assert_eq!(spanning_tree.total_weight(), 4.5);
        assert_eq!(spanning_tree.parent(1), 0);
        assert_eq!(spanning_tree.parent(2), 1);
        assert_eq!(spanning_tree.parent(3), 0);
        assert_eq!(spanning_tree.cost_to_parent(1), 1.0);
        assert_eq!(spanning_tree.cost_to_parent(2), 2.0);
        assert_eq!(spanning_tree.cost_to_parent(3), 1.5);
    }
    
    #[test]
    fn test_spanning_tree_head_finding() {
        let parent = vec![-1, 0, 1, 0];
        let cost_to_parent = vec![0.0, 1.0, 2.0, 1.5];
        
        let spanning_tree = SpanningTree::new(0, 4, 4, parent, cost_to_parent, 4.5);
        
        assert_eq!(spanning_tree.head(0), 0);
        assert_eq!(spanning_tree.head(1), 0);
        assert_eq!(spanning_tree.head(2), 0);
        assert_eq!(spanning_tree.head(3), 0);
    }
    
    #[test]
    fn test_spanning_tree_edge_iteration() {
        let parent = vec![-1, 0, 1, 0];
        let cost_to_parent = vec![0.0, 1.0, 2.0, 1.5];
        
        let spanning_tree = SpanningTree::new(0, 4, 4, parent, cost_to_parent, 4.5);
        
        let mut edges = Vec::new();
        spanning_tree.for_each_edge(|source, target, cost| {
            edges.push((source, target, cost));
            true
        });
        
        assert_eq!(edges.len(), 3);
        assert!(edges.contains(&(0, 1, 1.0)));
        assert!(edges.contains(&(1, 2, 2.0)));
        assert!(edges.contains(&(0, 3, 1.5)));
    }
    
    #[test]
    fn test_spanning_graph_adapter() {
        let parent = vec![-1, 0, 1, 0];
        let cost_to_parent = vec![0.0, 1.0, 2.0, 1.5];
        
        let spanning_tree = SpanningTree::new(0, 4, 4, parent, cost_to_parent, 4.5);
        let graph = SpanningGraph::new(&spanning_tree, 4);
        
        assert_eq!(graph.degree(0), 2); // Root has 2 children
        assert_eq!(graph.degree(1), 1); // Node 1 has 1 parent
        assert_eq!(graph.degree(2), 1); // Node 2 has 1 parent
        assert_eq!(graph.degree(3), 1); // Node 3 has 1 parent
        
        assert!(graph.exists(0, 1));
        assert!(graph.exists(1, 2));
        assert!(graph.exists(0, 3));
        assert!(!graph.exists(1, 3));
    }
    
    #[test]
    fn test_spanning_graph_relationship_iteration() {
        let parent = vec![-1, 0, 1, 0];
        let cost_to_parent = vec![0.0, 1.0, 2.0, 1.5];
        
        let spanning_tree = SpanningTree::new(0, 4, 4, parent, cost_to_parent, 4.5);
        let graph = SpanningGraph::new(&spanning_tree, 4);
        
        let mut relationships = Vec::new();
        graph.for_each_relationship(0, |source, target, cost| {
            relationships.push((source, target, cost));
            true
        });
        
        assert_eq!(relationships.len(), 2);
        assert!(relationships.contains(&(0, 1, 1.0)));
        assert!(relationships.contains(&(0, 3, 1.5)));
    }
}
