//! Dijkstra Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.dijkstra.Dijkstra`
//!
//! This module implements the "Subtle pole" of the Dijkstra algorithm,
//! handling ephemeral computation state including priority queue management,
//! visited set tracking, and predecessor/relationship ID storage.

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

/// Priority queue item for Dijkstra algorithm
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse ordering for min-heap (lower cost = higher priority)
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// Dijkstra Computation Runtime
///
/// Translation of: Internal state from `Dijkstra` class (lines 54-70)
/// Handles ephemeral computation state for the Dijkstra algorithm
pub struct DijkstraComputationRuntime {
    /// Priority queue for unvisited nodes
    priority_queue: BinaryHeap<QueueItem>,
    
    /// Visited set
    visited: HashSet<u32>,
    
    /// Predecessor map for path reconstruction
    predecessors: HashMap<u32, Option<u32>>,
    
    /// Relationship ID map (if tracking relationships)
    relationship_ids: HashMap<u32, Option<u32>>,
    
    /// Cost map for all nodes
    costs: HashMap<u32, f64>,
    
    /// Source node
    source_node: u32,
    
    /// Whether to track relationship IDs
    track_relationships: bool,
    
    /// Whether to use heuristic function
    use_heuristic: bool,
    
    /// Concurrency level
    concurrency: usize,
}

impl DijkstraComputationRuntime {
    /// Create a new Dijkstra computation runtime
    ///
    /// Translation of: Constructor initialization (lines 118-140)
    pub fn new(
        source_node: u32,
        track_relationships: bool,
        concurrency: usize,
        use_heuristic: bool,
    ) -> Self {
        Self {
            priority_queue: BinaryHeap::new(),
            visited: HashSet::new(),
            predecessors: HashMap::new(),
            relationship_ids: HashMap::new(),
            costs: HashMap::new(),
            source_node,
            track_relationships,
            use_heuristic,
            concurrency,
        }
    }

    /// Initialize the computation runtime
    ///
    /// Translation of: Initialization in `compute()` method (lines 170-173)
    pub fn initialize(&mut self, source_node: u32, track_relationships: bool, use_heuristic: bool) {
        self.source_node = source_node;
        self.track_relationships = track_relationships;
        self.use_heuristic = use_heuristic;
        
        // Clear previous state
        self.priority_queue.clear();
        self.visited.clear();
        self.predecessors.clear();
        self.relationship_ids.clear();
        self.costs.clear();
        
        // Initialize with infinite costs
        for node_id in 0..100 { // TODO: Replace with actual graph node count
            self.costs.insert(node_id, f64::INFINITY);
            self.predecessors.insert(node_id, None);
            if self.track_relationships {
                self.relationship_ids.insert(node_id, None);
            }
        }
    }

    /// Add a node to the priority queue
    ///
    /// Translation of: `queue.add()` calls (lines 173, 228)
    pub fn add_to_queue(&mut self, node_id: u32, cost: f64) {
        self.costs.insert(node_id, cost);
        self.priority_queue.push(QueueItem { node_id, cost });
    }

    /// Pop the node with minimum cost from the queue
    ///
    /// Translation of: `queue.pop()` method (line 189)
    pub fn pop_from_queue(&mut self) -> (u32, f64) {
        if let Some(item) = self.priority_queue.pop() {
            (item.node_id, item.cost)
        } else {
            panic!("Queue is empty");
        }
    }

    /// Check if the queue is empty
    ///
    /// Translation of: `queue.isEmpty()` method (line 188)
    pub fn is_queue_empty(&self) -> bool {
        self.priority_queue.is_empty()
    }

    /// Check if a node is in the queue
    ///
    /// Translation of: `queue.containsElement()` method (line 226)
    pub fn is_in_queue(&self, node_id: u32) -> bool {
        self.costs.get(&node_id).map_or(false, |&cost| cost != f64::INFINITY)
    }

    /// Get the cost of a node
    ///
    /// Translation of: `queue.cost()` method (lines 190, 260)
    pub fn get_cost(&self, node_id: u32) -> f64 {
        self.costs.get(&node_id).copied().unwrap_or(f64::INFINITY)
    }

    /// Update the cost of a node in the queue
    ///
    /// Translation of: `queue.set()` method (line 235)
    pub fn update_queue_cost(&mut self, node_id: u32, new_cost: f64) {
        self.costs.insert(node_id, new_cost);
        // Note: In a real implementation, we'd need to update the priority queue
        // For now, we'll add a new item (the old one will be ignored when visited)
        self.priority_queue.push(QueueItem { node_id, cost: new_cost });
    }

    /// Mark a node as visited
    ///
    /// Translation of: `visited.set()` method (line 191)
    pub fn mark_visited(&mut self, node_id: u32) {
        self.visited.insert(node_id);
    }

    /// Check if a node is visited
    ///
    /// Translation of: `visited.get()` method (line 222)
    pub fn is_visited(&self, node_id: u32) -> bool {
        self.visited.contains(&node_id)
    }

    /// Set the predecessor of a node
    ///
    /// Translation of: `predecessors.put()` method (lines 229, 236)
    pub fn set_predecessor(&mut self, node_id: u32, predecessor: Option<u32>) {
        self.predecessors.insert(node_id, predecessor);
    }

    /// Get the predecessor of a node
    ///
    /// Translation of: `predecessors.getOrDefault()` method (line 271)
    pub fn get_predecessor(&self, node_id: u32) -> Option<u32> {
        self.predecessors.get(&node_id).copied().flatten()
    }

    /// Set the relationship ID of a node
    ///
    /// Translation of: `relationships.put()` method (lines 231, 238)
    pub fn set_relationship_id(&mut self, node_id: u32, relationship_id: Option<u32>) {
        if self.track_relationships {
            self.relationship_ids.insert(node_id, relationship_id);
        }
    }

    /// Get the relationship ID of a node
    ///
    /// Translation of: `relationships.getOrDefault()` method (line 273)
    pub fn get_relationship_id(&self, node_id: u32) -> Option<u32> {
        if self.track_relationships {
            self.relationship_ids.get(&node_id).copied().flatten()
        } else {
            None
        }
    }

    /// Get the number of visited nodes
    pub fn visited_count(&self) -> usize {
        self.visited.len()
    }

    /// Get the number of nodes in the queue
    pub fn queue_size(&self) -> usize {
        self.priority_queue.len()
    }

    /// Get source node
    pub fn source_node(&self) -> u32 {
        self.source_node
    }

    /// Check if tracking relationships
    pub fn track_relationships(&self) -> bool {
        self.track_relationships
    }

    /// Check if using heuristic
    pub fn use_heuristic(&self) -> bool {
        self.use_heuristic
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra_computation_runtime_initialization() {
        let mut runtime = DijkstraComputationRuntime::new(0, true, 4, false);
        runtime.initialize(0, true, false);
        
        assert_eq!(runtime.source_node(), 0);
        assert!(runtime.track_relationships());
        assert!(!runtime.use_heuristic());
        assert_eq!(runtime.visited_count(), 0);
        assert_eq!(runtime.queue_size(), 0);
    }

    #[test]
    fn test_dijkstra_computation_runtime_queue_operations() {
        let mut runtime = DijkstraComputationRuntime::new(0, false, 4, false);
        runtime.initialize(0, false, false);
        
        // Test queue operations
        assert!(runtime.is_queue_empty());
        assert!(!runtime.is_in_queue(1));
        
        runtime.add_to_queue(1, 5.0);
        assert!(!runtime.is_queue_empty());
        assert!(runtime.is_in_queue(1));
        assert_eq!(runtime.get_cost(1), 5.0);
        
        let (node, cost) = runtime.pop_from_queue();
        assert_eq!(node, 1);
        assert_eq!(cost, 5.0);
        assert!(runtime.is_queue_empty());
    }

    #[test]
    fn test_dijkstra_computation_runtime_visited_operations() {
        let mut runtime = DijkstraComputationRuntime::new(0, false, 4, false);
        runtime.initialize(0, false, false);
        
        // Test visited operations
        assert!(!runtime.is_visited(1));
        assert_eq!(runtime.visited_count(), 0);
        
        runtime.mark_visited(1);
        assert!(runtime.is_visited(1));
        assert_eq!(runtime.visited_count(), 1);
    }

    #[test]
    fn test_dijkstra_computation_runtime_predecessor_operations() {
        let mut runtime = DijkstraComputationRuntime::new(0, false, 4, false);
        runtime.initialize(0, false, false);
        
        // Test predecessor operations
        assert_eq!(runtime.get_predecessor(1), None);
        
        runtime.set_predecessor(1, Some(0));
        assert_eq!(runtime.get_predecessor(1), Some(0));
        
        runtime.set_predecessor(1, None);
        assert_eq!(runtime.get_predecessor(1), None);
    }

    #[test]
    fn test_dijkstra_computation_runtime_relationship_operations() {
        let mut runtime = DijkstraComputationRuntime::new(0, true, 4, false);
        runtime.initialize(0, true, false);
        
        // Test relationship operations
        assert_eq!(runtime.get_relationship_id(1), None);
        
        runtime.set_relationship_id(1, Some(5));
        assert_eq!(runtime.get_relationship_id(1), Some(5));
        
        runtime.set_relationship_id(1, None);
        assert_eq!(runtime.get_relationship_id(1), None);
    }

    #[test]
    fn test_dijkstra_computation_runtime_cost_operations() {
        let mut runtime = DijkstraComputationRuntime::new(0, false, 4, false);
        runtime.initialize(0, false, false);
        
        // Test cost operations
        assert_eq!(runtime.get_cost(1), f64::INFINITY);
        
        runtime.add_to_queue(1, 5.0);
        assert_eq!(runtime.get_cost(1), 5.0);
        
        runtime.update_queue_cost(1, 3.0);
        assert_eq!(runtime.get_cost(1), 3.0);
    }

    #[test]
    fn test_dijkstra_computation_runtime_priority_queue_order() {
        let mut runtime = DijkstraComputationRuntime::new(0, false, 4, false);
        runtime.initialize(0, false, false);
        
        // Test priority queue ordering (min-heap)
        runtime.add_to_queue(1, 10.0);
        runtime.add_to_queue(2, 5.0);
        runtime.add_to_queue(3, 15.0);
        
        // Should pop in order of increasing cost
        let (_node1, cost1) = runtime.pop_from_queue();
        let (_node2, cost2) = runtime.pop_from_queue();
        let (_node3, cost3) = runtime.pop_from_queue();
        
        assert_eq!(cost1, 5.0);
        assert_eq!(cost2, 10.0);
        assert_eq!(cost3, 15.0);
    }
}
