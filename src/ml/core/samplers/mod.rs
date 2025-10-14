//! Neighborhood sampling for Graph Neural Networks.
//!
//! This module provides efficient neighborhood sampling strategies for GNN training.
//! Samplers enable mini-batch training on large graphs by sampling fixed-size neighborhoods.
//!
//! # Core Samplers
//!
//! - **UniformSampler**: Uniform random sampling without replacement (Algorithm L)
//! - **RandomWalkSampler**: Biased random walks (Node2Vec-style with return/in-out factors)
//!
//! # Usage
//!
//! ```rust,ignore
//! use rust_gds::ml::core::samplers::{UniformSampler, RandomWalkSampler};
//!
//! // Uniform sampling for GraphSAGE aggregation
//! let sampler = UniformSampler::new(42);
//! let neighbors = sampler.sample_from_stream(neighbor_stream, degree, 10);
//!
//! // Random walk sampling for Node2Vec embeddings
//! let walker = RandomWalkSampler::create(
//!     &graph,
//!     cumulative_weights,
//!     walk_length,
//!     return_factor,
//!     in_out_factor,
//!     42,
//! );
//! let walk = walker.walk(start_node);
//! ```

mod random_walk_sampler;
mod uniform_sampler;

pub use random_walk_sampler::{CumulativeWeightSupplier, RandomWalkSampler};
pub use uniform_sampler::UniformSampler;
