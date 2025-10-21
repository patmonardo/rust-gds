//! Centrality Algorithm Infrastructure
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.centrality` package
//!
//! This module provides infrastructure for centrality algorithms:
//! - Result traits and types
//! - Distribution computation
//! - Integration with `procedures/core` statistics
//!
//! ## Centrality Algorithms (Genera)
//!
//! This infrastructure supports centrality algorithm types:
//! - **PageRank** - Importance based on incoming links
//! - **Betweenness Centrality** - Importance based on shortest paths through node
//! - **Closeness Centrality** - Importance based on distance to all nodes
//! - **Degree Centrality** - Importance based on number of connections
//! - **Eigenvector Centrality** - Importance based on connections to important nodes
//!
//! Individual implementations live in `procedures/specs/` (e.g., `specs/pagerank/`).
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::procedures::algorithms::centrality::*;
//!
//! // Compute distribution for any centrality algorithm result
//! let distribution = PageRankDistributionComputer::compute_distribution(
//!     &result,
//!     false, // use_log_scaler
//!     true,  // should_compute_distribution
//!     4,     // concurrency
//! );
//! ```

pub mod result;
pub mod distribution;

pub use result::*;
pub use distribution::*;

