//! Subgraph module for ML-Core in GDS.
//!
//! This module provides utilities for working with subgraphs and neighborhoods
//! in machine learning contexts, particularly for graph neural networks.

pub mod batch_neighbors;
pub mod local_id_map;
pub mod neighborhood_sampler;
pub mod subgraph;

pub use batch_neighbors::BatchNeighbors;
pub use local_id_map::LocalIdMap;
pub use neighborhood_sampler::NeighborhoodSampler;
pub use subgraph::SubGraph;
