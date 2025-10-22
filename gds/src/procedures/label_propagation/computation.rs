//! Label Propagation Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.labelpropagation.LabelPropagation`
//!
//! Detects communities by iteratively propagating labels through the graph.
//! Each node votes for its neighbor labels and adopts the most popular one.

use std::collections::HashMap;

/// Result of label propagation computation
#[derive(Clone)]
pub struct LabelPropComputationResult {
    pub labels: Vec<u64>,
    pub did_converge: bool,
    pub ran_iterations: u64,
}

/// Label Propagation computation runtime
pub struct LabelPropComputationRuntime {
    labels: Vec<u64>,
    next_labels: Vec<u64>,
    max_iterations: u64,
    seed_labels: Option<Vec<u64>>,
    node_weights: Vec<f64>,
}

impl LabelPropComputationRuntime {
    pub fn new(node_count: usize, max_iterations: u64) -> Self {
        Self {
            labels: vec![0u64; node_count],
            next_labels: vec![0u64; node_count],
            max_iterations,
            seed_labels: None,
            node_weights: vec![1.0f64; node_count],
        }
    }

    pub fn with_seeds(mut self, seed_labels: Vec<u64>) -> Self {
        self.seed_labels = Some(seed_labels);
        self
    }

    pub fn with_weights(mut self, weights: Vec<f64>) -> Self {
        self.node_weights = weights;
        self
    }

    /// Compute label propagation
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<(usize, f64)>, // (neighbor, weight)
    ) -> LabelPropComputationResult {
        // Phase 1: Initialize labels
        self.init_labels(node_count, &get_neighbors);

        // Phase 2: Iteratively propagate labels
        let mut did_converge = false;
        let mut ran_iterations = 0u64;

        for iteration in 0..self.max_iterations {
            ran_iterations = iteration + 1;
            let changed = self.compute_iteration(node_count, &get_neighbors);

            if !changed {
                did_converge = true;
                break;
            }

            // Swap labels for next iteration
            std::mem::swap(&mut self.labels, &mut self.next_labels);
        }

        LabelPropComputationResult {
            labels: self.labels.clone(),
            did_converge,
            ran_iterations,
        }
    }

    /// Initialize labels from seeds or node IDs
    fn init_labels(
        &mut self,
        node_count: usize,
        get_neighbors: &impl Fn(usize) -> Vec<(usize, f64)>,
    ) {
        if let Some(ref seeds) = self.seed_labels {
            // Use provided seed labels
            for i in 0..node_count {
                self.labels[i] = seeds[i];
            }
        } else {
            // Initialize with node IDs
            for i in 0..node_count {
                self.labels[i] = i as u64;
            }
        }

        self.next_labels.copy_from_slice(&self.labels);
    }

    /// Execute one iteration of label propagation
    fn compute_iteration(
        &mut self,
        node_count: usize,
        get_neighbors: &impl Fn(usize) -> Vec<(usize, f64)>,
    ) -> bool {
        let mut changed = false;

        for node_id in 0..node_count {
            let old_label = self.labels[node_id];

            // Collect votes from neighbors
            let mut votes: HashMap<u64, f64> = HashMap::new();
            let neighbors = get_neighbors(node_id);

            for (neighbor_id, edge_weight) in neighbors {
                let neighbor_label = self.labels[neighbor_id];
                let neighbor_weight = self.node_weights[neighbor_id];
                let vote_weight = edge_weight * neighbor_weight;

                *votes.entry(neighbor_label).or_insert(0.0) += vote_weight;
            }

            // Tally votes and find best label
            let new_label = self.tally_votes(old_label, votes);

            if new_label != old_label {
                changed = true;
            }

            self.next_labels[node_id] = new_label;
        }

        changed
    }

    /// Find label with most votes (ties broken by lowest label ID)
    fn tally_votes(&self, current_label: u64, votes: HashMap<u64, f64>) -> u64 {
        if votes.is_empty() {
            return current_label;
        }

        let mut best_label = current_label;
        let mut best_weight = f64::NEG_INFINITY;

        for (label, weight) in votes {
            if weight > best_weight || (weight == best_weight && label < best_label) {
                best_weight = weight;
                best_label = label;
            }
        }

        best_label
    }
}
