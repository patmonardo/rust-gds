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
//! use gds::ml::core::samplers::{UniformSampler, RandomWalkSampler};
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

mod long_uniform_sampler_by_exclusion;
mod long_uniform_sampler_from_range;
mod long_uniform_sampler_with_retries;
mod random_walk_sampler;
mod uniform_sampler;
mod weighted_uniform_sampler;

pub use long_uniform_sampler_by_exclusion::LongUniformSamplerByExclusion;
pub use long_uniform_sampler_from_range::LongUniformSamplerFromRange;
pub use long_uniform_sampler_with_retries::LongUniformSamplerWithRetries;
pub use random_walk_sampler::{CumulativeWeightSupplier, RandomWalkSampler};
pub use uniform_sampler::UniformSampler;
pub use weighted_uniform_sampler::WeightedUniformSampler;
