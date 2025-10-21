//! A* Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.astar.AStar`
//!
//! This module implements the computation runtime for A* algorithm - the "Subtle pole" for ephemeral computation.

/// A* computation result
#[derive(Debug, Clone)]
pub struct AStarComputationResult {
    /// Path from source to target
    pub path: Option<Vec<usize>>,
    /// Total cost of the path
    pub total_cost: f64,
    /// Number of nodes explored
    pub nodes_explored: usize,
}

impl AStarComputationResult {
    /// Create a new A* computation result
    pub fn new(path: Option<Vec<usize>>, total_cost: f64, nodes_explored: usize) -> Self {
        Self {
            path,
            total_cost,
            nodes_explored,
        }
    }
    
    /// Check if a path was found
    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }
    
    /// Get path length
    pub fn path_length(&self) -> usize {
        self.path.as_ref().map_or(0, |p| p.len())
    }
}

/// A* computation runtime for ephemeral computation
///
/// Translation of: A* algorithm computation logic
pub struct AStarComputationRuntime {
    /// Priority queue for A* algorithm (placeholder for now)
    /// TODO: Implement proper priority queue with heuristic function
    open_set: Vec<usize>,
    /// Visited nodes
    visited: std::collections::HashSet<usize>,
    /// Cost from start to each node
    g_cost: std::collections::HashMap<usize, f64>,
    /// Estimated total cost (g + h)
    f_cost: std::collections::HashMap<usize, f64>,
    /// Parent nodes for path reconstruction
    parents: std::collections::HashMap<usize, usize>,
}

impl AStarComputationRuntime {
    /// Create new A* computation runtime
    pub fn new() -> Self {
        Self {
            open_set: Vec::new(),
            visited: std::collections::HashSet::new(),
            g_cost: std::collections::HashMap::new(),
            f_cost: std::collections::HashMap::new(),
            parents: std::collections::HashMap::new(),
        }
    }
    
    /// Initialize A* computation for given source and target
    pub fn initialize(&mut self, source: usize, _target: usize) {
        self.open_set.clear();
        self.visited.clear();
        self.g_cost.clear();
        self.f_cost.clear();
        self.parents.clear();
        
        // Start with source node
        self.open_set.push(source);
        self.g_cost.insert(source, 0.0);
        self.f_cost.insert(source, 0.0); // Will be updated with heuristic
    }
    
    /// Add node to open set
    pub fn add_to_open_set(&mut self, node: usize) {
        if !self.open_set.contains(&node) {
            self.open_set.push(node);
        }
    }
    
    /// Remove node from open set
    pub fn remove_from_open_set(&mut self, node: usize) {
        self.open_set.retain(|&n| n != node);
    }
    
    /// Get node with lowest f-cost from open set
    pub fn get_lowest_f_cost_node(&self) -> Option<usize> {
        self.open_set.iter()
            .min_by(|a, b| {
                let f_a = self.f_cost.get(a).unwrap_or(&f64::INFINITY);
                let f_b = self.f_cost.get(b).unwrap_or(&f64::INFINITY);
                f_a.partial_cmp(f_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }
    
    /// Mark node as visited
    pub fn mark_visited(&mut self, node: usize) {
        self.visited.insert(node);
    }
    
    /// Check if node is visited
    pub fn is_visited(&self, node: usize) -> bool {
        self.visited.contains(&node)
    }
    
    /// Update g-cost for a node
    pub fn update_g_cost(&mut self, node: usize, cost: f64) {
        self.g_cost.insert(node, cost);
    }
    
    /// Get g-cost for a node
    pub fn get_g_cost(&self, node: usize) -> f64 {
        self.g_cost.get(&node).copied().unwrap_or(f64::INFINITY)
    }
    
    /// Update f-cost for a node
    pub fn update_f_cost(&mut self, node: usize, cost: f64) {
        self.f_cost.insert(node, cost);
    }
    
    /// Get f-cost for a node
    pub fn get_f_cost(&self, node: usize) -> f64 {
        self.f_cost.get(&node).copied().unwrap_or(f64::INFINITY)
    }
    
    /// Set parent for path reconstruction
    pub fn set_parent(&mut self, child: usize, parent: usize) {
        self.parents.insert(child, parent);
    }
    
    /// Get parent for path reconstruction
    pub fn get_parent(&self, node: usize) -> Option<usize> {
        self.parents.get(&node).copied()
    }
    
    /// Reconstruct path from target to source
    pub fn reconstruct_path(&self, source: usize, target: usize) -> Option<Vec<usize>> {
        if !self.visited.contains(&target) {
            return None;
        }
        
        let mut path = Vec::new();
        let mut current = target;
        
        while current != source {
            path.push(current);
            current = self.get_parent(current)?;
        }
        path.push(source);
        
        path.reverse();
        Some(path)
    }
    
    /// Check if open set is empty
    pub fn is_open_set_empty(&self) -> bool {
        self.open_set.is_empty()
    }
    
    /// Get number of nodes explored
    pub fn nodes_explored(&self) -> usize {
        self.visited.len()
    }
    
    /// Get total cost to target (if found)
    pub fn get_total_cost(&self, target: usize) -> f64 {
        self.get_g_cost(target)
    }
}

impl Default for AStarComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_computation_result() {
        let path = Some(vec![0, 1, 2]);
        let result = AStarComputationResult::new(path.clone(), 10.5, 5);
        
        assert!(result.has_path());
        assert_eq!(result.path_length(), 3);
        assert_eq!(result.total_cost, 10.5);
        assert_eq!(result.nodes_explored, 5);
        
        let no_path_result = AStarComputationResult::new(None, f64::INFINITY, 3);
        assert!(!no_path_result.has_path());
        assert_eq!(no_path_result.path_length(), 0);
    }

    #[test]
    fn test_astar_computation_runtime_initialization() {
        let mut runtime = AStarComputationRuntime::new();
        runtime.initialize(0, 5);
        
        assert_eq!(runtime.open_set.len(), 1);
        assert_eq!(runtime.open_set[0], 0);
        assert_eq!(runtime.get_g_cost(0), 0.0);
        assert_eq!(runtime.get_f_cost(0), 0.0);
        assert!(!runtime.is_visited(0));
    }

    #[test]
    fn test_astar_computation_runtime_operations() {
        let mut runtime = AStarComputationRuntime::new();
        runtime.initialize(0, 5);
        
        // Test adding to open set
        runtime.add_to_open_set(1);
        runtime.add_to_open_set(2);
        assert_eq!(runtime.open_set.len(), 3);
        
        // Test updating costs
        runtime.update_g_cost(1, 5.0);
        runtime.update_f_cost(1, 7.0);
        assert_eq!(runtime.get_g_cost(1), 5.0);
        assert_eq!(runtime.get_f_cost(1), 7.0);
        
        // Test marking as visited
        runtime.mark_visited(1);
        assert!(runtime.is_visited(1));
        assert!(!runtime.is_visited(2));
        
        // Test parent setting
        runtime.set_parent(1, 0);
        assert_eq!(runtime.get_parent(1), Some(0));
        assert_eq!(runtime.get_parent(2), None);
    }

    #[test]
    fn test_astar_computation_runtime_lowest_f_cost() {
        let mut runtime = AStarComputationRuntime::new();
        runtime.initialize(0, 5);
        
        runtime.add_to_open_set(1);
        runtime.add_to_open_set(2);
        runtime.add_to_open_set(3);
        
        runtime.update_f_cost(0, 20.0); // Set high f_cost for source node
        runtime.update_f_cost(1, 10.0);
        runtime.update_f_cost(2, 5.0);
        runtime.update_f_cost(3, 15.0);
        
        assert_eq!(runtime.get_lowest_f_cost_node(), Some(2));
        
        runtime.remove_from_open_set(2);
        assert_eq!(runtime.get_lowest_f_cost_node(), Some(1));
    }

    #[test]
    fn test_astar_computation_runtime_path_reconstruction() {
        let mut runtime = AStarComputationRuntime::new();
        runtime.initialize(0, 5);
        
        // Build a simple path: 0 -> 1 -> 2 -> 5
        runtime.set_parent(1, 0);
        runtime.set_parent(2, 1);
        runtime.set_parent(5, 2);
        
        runtime.mark_visited(5);
        
        let path = runtime.reconstruct_path(0, 5);
        assert_eq!(path, Some(vec![0, 1, 2, 5]));
        
        // Test path reconstruction when target not visited
        runtime.visited.clear();
        let no_path = runtime.reconstruct_path(0, 5);
        assert_eq!(no_path, None);
    }

    #[test]
    fn test_astar_computation_runtime_empty_open_set() {
        let mut runtime = AStarComputationRuntime::new();
        runtime.initialize(0, 5);
        
        assert!(!runtime.is_open_set_empty());
        
        runtime.open_set.clear();
        assert!(runtime.is_open_set_empty());
    }

    #[test]
    fn test_astar_computation_runtime_nodes_explored() {
        let mut runtime = AStarComputationRuntime::new();
        runtime.initialize(0, 5);
        
        assert_eq!(runtime.nodes_explored(), 0);
        
        runtime.mark_visited(0);
        runtime.mark_visited(1);
        runtime.mark_visited(2);
        
        assert_eq!(runtime.nodes_explored(), 3);
    }

    #[test]
    fn test_astar_computation_runtime_total_cost() {
        let mut runtime = AStarComputationRuntime::new();
        runtime.initialize(0, 5);
        
        runtime.update_g_cost(5, 12.5);
        assert_eq!(runtime.get_total_cost(5), 12.5);
        
        // Test with unvisited node
        assert_eq!(runtime.get_total_cost(10), f64::INFINITY);
    }
}
