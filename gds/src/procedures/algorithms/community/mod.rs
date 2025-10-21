//! Community Detection Algorithm Infrastructure
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.community` package
//!
//! This module provides infrastructure for community detection algorithms:
//! - Community result utilities
//! - Node property value transformations
//! - Consecutive ID mapping
//! - Incremental detection support
//! - Integration with `procedures/core` statistics
//!
//! ## Community Detection Algorithms (Genera)
//!
//! This infrastructure supports community detection algorithm types:
//! - **Louvain** - Modularity optimization
//! - **Label Propagation** - Label spreading
//! - **Leiden** - Improved modularity optimization
//! - **Weakly Connected Components** - Graph component detection
//! - **Strongly Connected Components** - Directed graph components
//! - **Triangle Count** - Local clustering coefficient
//! - **Local Clustering Coefficient** - Node clustering metric
//!
//! Individual implementations live in `procedures/specs/` (e.g., `specs/louvain/`).
//!
//! ## Key Features
//!
//! ### Consecutive ID Mapping
//! Community IDs from algorithms may not be consecutive. `ConsecutiveLongNodePropertyValues`
//! remaps them to 0, 1, 2, ... for cleaner output.
//!
//! ### Incremental Detection
//! `LongIfChangedNodePropertyValues` filters results to only show nodes whose community
//! changed, useful for incremental/iterative algorithms.
//!
//! ### Community Filtering
//! `CommunityCompanion` provides utilities like minimum community size filtering.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::procedures::algorithms::community::*;
//!
//! // Remap community IDs to consecutive integers
//! let consecutive = ConsecutiveLongNodePropertyValues::new(community_results);
//!
//! // Filter to only changed communities (for incremental algorithms)
//! let changed_only = LongIfChangedNodePropertyValues::new(seed, new_results);
//!
//! // Apply community companion utilities
//! let filtered = CommunityCompanion::node_property_values_with_filter(
//!     true,  // consecutive_ids
//!     results,
//!     Some(5), // min_community_size
//!     4,     // concurrency
//! );
//! ```

pub mod companion;
pub mod consecutive_values;
pub mod incremental_values;

pub use companion::*;
pub use consecutive_values::*;
pub use incremental_values::*;

