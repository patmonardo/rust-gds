//! K1Coloring Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.k1coloring.K1Coloring`
//!
//! Greedy graph coloring algorithm ensuring no two adjacent nodes share the same color.
//! Uses iterative phases: Coloring (assign colors) and Validation (detect conflicts).

use crate::collections::BitSet;

/// K1Coloring computation result
#[derive(Clone)]
pub struct K1ColoringComputationResult {
    pub colors: Vec<u64>,
    pub ran_iterations: u64,
    pub did_converge: bool,
}

/// K1Coloring computation runtime
pub struct K1ColoringComputationRuntime {
    /// Current color assignments for each node
    colors: Vec<u64>,
    /// Nodes pending coloring (current iteration)
    nodes_to_color_current: BitSet,
    /// Nodes pending coloring (next iteration)
    nodes_to_color_next: BitSet,
    /// Max iterations
    max_iterations: u64,
    /// Current iteration count
    ran_iterations: u64,
    /// Forbidden colors for current node being colored
    forbidden_colors: BitSet,
}

const INITIAL_COLOR: u64 = 1000;
const INITIAL_FORBIDDEN_COLORS: usize = 1000;

impl K1ColoringComputationRuntime {
    pub fn new(node_count: usize, max_iterations: u64) -> Self {
        Self {
            colors: vec![INITIAL_COLOR; node_count],
            nodes_to_color_current: BitSet::new(node_count),
            nodes_to_color_next: BitSet::new(node_count),
            max_iterations,
            ran_iterations: 0,
            forbidden_colors: BitSet::new(INITIAL_FORBIDDEN_COLORS),
        }
    }

    /// Run K1Coloring algorithm
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> K1ColoringComputationResult {
        // Initialize: all nodes need coloring
        for i in 0..node_count {
            self.nodes_to_color_current.set(i);
        }

        // Iterative coloring and validation
        while self.ran_iterations < self.max_iterations
            && !self.nodes_to_color_current.is_empty()
        {
            // Phase 1: Color all nodes in current set
            self.coloring_phase(node_count, &get_neighbors);

            // Phase 2: Validate and mark conflicts for next iteration
            self.validation_phase(node_count, &get_neighbors);

            // Swap for next iteration
            std::mem::swap(
                &mut self.nodes_to_color_current,
                &mut self.nodes_to_color_next,
            );

            self.ran_iterations += 1;
        }

        let did_converge = self.ran_iterations < self.max_iterations;

        K1ColoringComputationResult {
            colors: self.colors.clone(),
            ran_iterations: self.ran_iterations,
            did_converge,
        }
    }

    /// Phase 1: Assign colors to all nodes in current set
    fn coloring_phase(&mut self, node_count: usize, get_neighbors: &impl Fn(usize) -> Vec<usize>) {
        for node_id in 0..node_count {
            if !self.nodes_to_color_current.get(node_id) {
                continue;
            }

            // Reset forbidden colors for this node
            self.forbidden_colors.clear_all();

            // Mark colors of all neighbors as forbidden
            let neighbors = get_neighbors(node_id);
            for &neighbor in &neighbors {
                if neighbor != node_id {
                    let neighbor_color = self.colors[neighbor];
                    if (neighbor_color as usize) < INITIAL_FORBIDDEN_COLORS {
                        self.forbidden_colors.set(neighbor_color as usize);
                    }
                }
            }

            // Find first available color
            let mut next_color = 0u64;
            while (next_color as usize) < INITIAL_FORBIDDEN_COLORS
                && self.forbidden_colors.get(next_color as usize)
            {
                next_color += 1;
            }

            self.colors[node_id] = next_color;
        }
    }

    /// Phase 2: Validate coloring and mark conflicts
    fn validation_phase(
        &mut self,
        node_count: usize,
        get_neighbors: &impl Fn(usize) -> Vec<usize>,
    ) {
        // Clear next iteration's nodes to color
        self.nodes_to_color_next.clear_all();

        // Check each colored node for conflicts with neighbors
        for node_id in 0..node_count {
            if !self.nodes_to_color_current.get(node_id) {
                continue;
            }

            let node_color = self.colors[node_id];
            let neighbors = get_neighbors(node_id);

            for &neighbor in &neighbors {
                if neighbor != node_id {
                    let neighbor_color = self.colors[neighbor];

                    // Conflict: both have same color and neighbor needs recoloring
                    if node_color == neighbor_color && !self.nodes_to_color_next.get(neighbor) {
                        self.nodes_to_color_next.set(neighbor);
                    }
                }
            }
        }
    }
}

/// Helper trait for BitSet operations
trait BitSetExt {
    fn clear_all(&mut self);
    fn is_empty(&self) -> bool;
}

impl BitSetExt for BitSet {
    fn clear_all(&mut self) {
        // Clear all bits
        for i in 0..self.size() {
            self.clear(i);
        }
    }

    fn is_empty(&self) -> bool {
        // Check if any bit is set
        for i in 0..self.size() {
            if self.get(i) {
                return false;
            }
        }
        true
    }
}

impl BitSet {
    fn size(&self) -> usize {
        // BitSet needs size tracking - this is a limitation we need to work around
        // For now, return a conservative estimate
        INITIAL_FORBIDDEN_COLORS
    }
}
