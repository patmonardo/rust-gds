//! Biased random walk sampling for Node2Vec-style embeddings.
//!
//! Implements random walks with return and in-out biases for learning
//! node embeddings that capture both local and global graph structure.
//!
//! # Algorithm
//!
//! Random walks with two bias parameters:
//! - **Return parameter (p)**: Likelihood of returning to previous node
//! - **In-out parameter (q)**: Likelihood of exploring outward vs staying local
//!
//! # References
//!
//! Node2Vec: Scalable Feature Learning for Networks (Grover & Leskovec, 2016)
//!
//! # Examples
//!
//! ```rust,ignore
//! use rust_gds::ml::core::samplers::RandomWalkSampler;
//!
//! let sampler = RandomWalkSampler::create(
//!     &graph,
//!     |node_id| graph.degree(node_id) as f64,
//!     walk_length: 80,
//!     return_factor: 1.0,  // p parameter
//!     in_out_factor: 1.0,  // q parameter
//!     random_seed: 42,
//! );
//!
//! let walk = sampler.walk(start_node);
//! ```

use crate::types::graph::Graph;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::sync::Arc;

const NO_MORE_NODES: i64 = -1;
const MAX_TRIES: usize = 100;

/// Function that computes cumulative relationship weight for a node.
///
/// Used for weighted random neighbor selection.
pub trait CumulativeWeightSupplier: Fn(u64) -> f64 {}

impl<F> CumulativeWeightSupplier for F where F: Fn(u64) -> f64 {}

/// Biased random walk sampler for Node2Vec-style walks.
pub struct RandomWalkSampler<W: CumulativeWeightSupplier> {
    graph: Arc<dyn Graph>,
    walk_length: usize,
    rng: ChaCha8Rng,
    normalized_return_probability: f64,
    normalized_same_distance_probability: f64,
    normalized_in_out_probability: f64,
    cumulative_weight_supplier: W,
    random_seed: u64,
}

impl<W: CumulativeWeightSupplier> RandomWalkSampler<W> {
    /// Create a new random walk sampler with normalized probabilities.
    ///
    /// # Arguments
    ///
    /// * `graph` - Graph to walk on (will be wrapped in Arc)
    /// * `cumulative_weight_supplier` - Function computing cumulative weights per node
    /// * `walk_length` - Number of steps in each walk
    /// * `return_factor` - Return parameter (p in Node2Vec)
    /// * `in_out_factor` - In-out parameter (q in Node2Vec)
    /// * `random_seed` - Seed for deterministic walks
    ///
    /// # Returns
    ///
    /// Sampler with normalized transition probabilities
    pub fn create(
        graph: Arc<dyn Graph>,
        cumulative_weight_supplier: W,
        walk_length: usize,
        return_factor: f64,
        in_out_factor: f64,
        random_seed: u64,
    ) -> Self {
        // Normalize probabilities by max
        let max_probability = (1.0 / return_factor).max(1.0).max(1.0 / in_out_factor);
        let normalized_return_probability = (1.0 / return_factor) / max_probability;
        let normalized_same_distance_probability = 1.0 / max_probability;
        let normalized_in_out_probability = (1.0 / in_out_factor) / max_probability;

        Self::new(
            graph,
            cumulative_weight_supplier,
            walk_length,
            normalized_return_probability,
            normalized_same_distance_probability,
            normalized_in_out_probability,
            random_seed,
        )
    }

    /// Create sampler with explicit normalized probabilities.
    ///
    /// Use `create()` instead for automatic normalization from p/q parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: Arc<dyn Graph>,
        cumulative_weight_supplier: W,
        walk_length: usize,
        normalized_return_probability: f64,
        normalized_same_distance_probability: f64,
        normalized_in_out_probability: f64,
        random_seed: u64,
    ) -> Self {
        Self {
            graph,
            walk_length,
            rng: ChaCha8Rng::seed_from_u64(random_seed),
            normalized_return_probability,
            normalized_same_distance_probability,
            normalized_in_out_probability,
            cumulative_weight_supplier,
            random_seed,
        }
    }

    /// Perform a random walk starting from the given node.
    ///
    /// # Arguments
    ///
    /// * `start_node` - Node ID to start walk from
    ///
    /// # Returns
    ///
    /// Vector of node IDs visited in the walk (may be shorter than walk_length
    /// if walk terminates early at a node with no outgoing edges)
    pub fn walk(&mut self, start_node: u64) -> Vec<u64> {
        let mut walk = vec![0u64; self.walk_length];
        walk[0] = start_node;

        // Take first step
        let first_step = self.random_neighbour(start_node);
        if first_step == NO_MORE_NODES {
            return vec![walk[0]];
        }
        walk[1] = first_step as u64;

        // Continue walk with bias
        for i in 2..self.walk_length {
            let next_node = self.walk_one_step(walk[i - 2], walk[i - 1]);
            if next_node == NO_MORE_NODES {
                // Walk terminated early, return shortened walk
                return walk[..i].to_vec();
            }
            walk[i] = next_node as u64;
        }

        walk
    }

    /// Prepare the sampler for a new starting node.
    ///
    /// Reseeds the RNG deterministically based on node ID.
    /// Call this before walking from each new node for consistent walks.
    pub fn prepare_for_new_node(&mut self, node_id: u64) {
        self.rng = ChaCha8Rng::seed_from_u64(self.random_seed + node_id);
    }

    /// Take one step in the random walk with Node2Vec bias.
    fn walk_one_step(&mut self, previous_node: u64, current_node: u64) -> i64 {
        let current_node_degree = self.graph.degree(current_node);

        if current_node_degree == 0 {
            // Dead end - no outgoing edges
            return NO_MORE_NODES;
        }

        if current_node_degree == 1 {
            // Only one neighbor, no choice needed
            return self.random_neighbour(current_node);
        }

        // Try to find valid next node with bias
        for _ in 0..MAX_TRIES {
            let new_node = self.random_neighbour(current_node);
            if new_node == NO_MORE_NODES {
                return NO_MORE_NODES;
            }
            let new_node = new_node as u64;

            let r: f64 = self.rng.gen();

            if new_node == previous_node {
                // Return to previous node
                if r < self.normalized_return_probability {
                    return new_node as i64;
                }
            } else {
                // Moving to new node - check same distance vs in-out

                let min_prob = self
                    .normalized_same_distance_probability
                    .min(self.normalized_in_out_probability);
                let max_prob = self
                    .normalized_same_distance_probability
                    .max(self.normalized_in_out_probability);

                // Definitely keep if r < min
                if r < min_prob {
                    return new_node as i64;
                }

                // Definitely reject if r >= max
                if r >= max_prob {
                    continue;
                }

                // Need to check if neighbor of previous node (expensive)
                if self.is_neighbour(previous_node, new_node) {
                    // Same distance from previous
                    if r < self.normalized_same_distance_probability {
                        return new_node as i64;
                    }
                } else {
                    // Moving outward
                    if r < self.normalized_in_out_probability {
                        return new_node as i64;
                    }
                }
            }
        }

        // Failed to find valid neighbor in MAX_TRIES, just pick random one
        self.random_neighbour(current_node)
    }

    /// Select a random neighbor weighted by relationship weights.
    fn random_neighbour(&mut self, node: u64) -> i64 {
        let cumulative_weight = (self.cumulative_weight_supplier)(node);

        if cumulative_weight == 0.0 {
            return NO_MORE_NODES;
        }

        let random_weight: f64 = self.rng.gen();
        let random_weight = cumulative_weight * random_weight;

        let mut current_weight = 0.0;
        let mut selected_neighbor = NO_MORE_NODES;

        // Use stream_relationships for cursor-based iteration
        for cursor in self.graph.stream_relationships(node, 1.0) {
            let weight = cursor.property();
            current_weight += weight;

            if random_weight <= current_weight {
                selected_neighbor = cursor.target_id() as i64;
                break;
            }
        }

        selected_neighbor
    }

    /// Check if target is a neighbor of source.
    fn is_neighbour(&self, source: u64, target: u64) -> bool {
        self.graph.exists(source, target)
    }

    /// Estimate memory usage for random walks.
    ///
    /// # Arguments
    ///
    /// * `walk_length` - Length of walks to estimate
    ///
    /// # Returns
    ///
    /// Memory range in bytes (min, max) accounting for variable walk lengths
    pub fn memory_estimation(walk_length: usize) -> (usize, usize) {
        let base_size = std::mem::size_of::<Self>();
        let walk_size = walk_length * std::mem::size_of::<u64>();

        (
            base_size + walk_size,     // Minimum: one walk
            base_size + 2 * walk_size, // Maximum: two walks in memory
        )
    }
}

// Safety: RandomWalkSampler can be sent between threads if W is Send
unsafe impl<W: CumulativeWeightSupplier + Send> Send for RandomWalkSampler<W> {}

// Safety: RandomWalkSampler can be shared if W is Sync (but needs &mut for walking)
unsafe impl<W: CumulativeWeightSupplier + Sync> Sync for RandomWalkSampler<W> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph::degrees::Degrees;
    use crate::types::graph::id_map::IdMap;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, RandomGraphResult, RandomRelationshipConfig};

    fn create_test_graph() -> RandomGraphResult<DefaultGraphStore> {
        let config = RandomGraphConfig {
            graph_name: "test-walk".into(),
            database_name: "in-memory".into(),
            node_count: 20,
            node_labels: vec!["Node".into()],
            relationships: vec![RandomRelationshipConfig::new("CONNECTED", 0.3)],
            directed: true,
            inverse_indexed: false,
            seed: Some(42),
        };

        DefaultGraphStore::random(&config)
    }

    #[test]
    fn test_walk_basic() {
        let store = create_test_graph().unwrap();
        let graph = store.graph();

        let weight_fn = |node_id: u64| graph.degree(node_id) as f64;
        let mut sampler = RandomWalkSampler::create(graph.clone(), weight_fn, 10, 1.0, 1.0, 42);

        let walk = sampler.walk(0);

        // Walk should have some nodes
        assert!(!walk.is_empty());
        assert!(walk.len() <= 10);

        // First node should be start node
        assert_eq!(walk[0], 0);
    }

    #[test]
    fn test_walk_deterministic() {
        let store = create_test_graph().unwrap();
        let graph = store.graph();

        let weight_fn = |node_id: u64| graph.degree(node_id) as f64;

        let mut sampler1 = RandomWalkSampler::create(graph.clone(), weight_fn, 10, 1.0, 1.0, 42);

        let mut sampler2 = RandomWalkSampler::create(graph.clone(), weight_fn, 10, 1.0, 1.0, 42);

        let walk1 = sampler1.walk(0);
        let walk2 = sampler2.walk(0);

        assert_eq!(walk1, walk2);
    }

    #[test]
    fn test_walk_different_seeds() {
        let store = create_test_graph().unwrap();
        let graph = store.graph();

        let weight_fn = |node_id: u64| graph.degree(node_id) as f64;

        let mut sampler1 = RandomWalkSampler::create(graph.clone(), weight_fn, 10, 1.0, 1.0, 42);

        let mut sampler2 = RandomWalkSampler::create(graph.clone(), weight_fn, 10, 1.0, 1.0, 43);

        let walk1 = sampler1.walk(5);
        let walk2 = sampler2.walk(5);

        // Different seeds should produce different walks (with high probability)
        assert_ne!(walk1, walk2);
    }

    #[test]
    fn test_prepare_for_new_node() {
        let store = create_test_graph().unwrap();
        let graph = store.graph();

        let weight_fn = |node_id: u64| graph.degree(node_id) as f64;

        let mut sampler = RandomWalkSampler::create(graph.clone(), weight_fn, 10, 1.0, 1.0, 42);

        sampler.prepare_for_new_node(5);
        let walk1 = sampler.walk(5);

        sampler.prepare_for_new_node(5);
        let walk2 = sampler.walk(5);

        // Same preparation should give same walk
        assert_eq!(walk1, walk2);
    }

    #[test]
    fn test_walk_with_return_bias() {
        let store = create_test_graph().unwrap();
        let graph = store.graph();

        let weight_fn = |node_id: u64| graph.degree(node_id) as f64;

        // High return factor (low p) encourages staying close
        let mut sampler = RandomWalkSampler::create(
            graph.clone(),
            weight_fn,
            20,
            0.1, // Low return factor = high return probability
            1.0,
            42,
        );

        let walk = sampler.walk(0);
        assert!(!walk.is_empty());
    }

    #[test]
    fn test_walk_with_in_out_bias() {
        let store = create_test_graph().unwrap();
        let graph = store.graph();

        let weight_fn = |node_id: u64| graph.degree(node_id) as f64;

        // Low in-out factor encourages exploration
        let mut sampler = RandomWalkSampler::create(
            graph.clone(),
            weight_fn,
            20,
            1.0,
            0.5, // Low q = encourage outward movement
            42,
        );

        let walk = sampler.walk(0);
        assert!(!walk.is_empty());
    }

    #[test]
    fn test_memory_estimation() {
        let (min_mem, max_mem) = RandomWalkSampler::<fn(u64) -> f64>::memory_estimation(100);

        assert!(min_mem > 0);
        assert!(max_mem >= min_mem);
        assert!(max_mem <= 3 * min_mem); // Reasonable upper bound
    }

    #[test]
    fn test_walk_all_nodes() {
        let store = create_test_graph().unwrap();
        let graph = store.graph();

        let weight_fn = |node_id: u64| graph.degree(node_id) as f64;
        let mut sampler = RandomWalkSampler::create(graph.clone(), weight_fn, 10, 1.0, 1.0, 42);

        // Walk from all nodes - shouldn't panic
        for node_id_idx in 0..graph.node_count() {
            let node_id = node_id_idx as u64;
            let walk = sampler.walk(node_id);
            assert!(!walk.is_empty());
            assert_eq!(walk[0], node_id);
        }
    }
}
