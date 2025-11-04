//! Algorithm Infrastructure - Utilities and result types for graph algorithms (Genera)
//!
//! **Translation Source**: `org.neo4j.gds.algorithms` package
//! **Translation Protocol**: TP-006 (Gamma-level architectural translation)
//!
//! This module provides infrastructure for algorithm **types** (Genera), not individual algorithms (Species).
//! Individual algorithm implementations live in `procedures/specs/` (e.g., PageRank, DegreeCentrality).
//!
//! ## Architecture
//!
//! ```text
//! procedures/
//! ├── core/                    Common utilities (statistics, memory, progress, result builders)
//! ├── algorithms/              Algorithm type infrastructure (THIS MODULE - Genera)
//! │   ├── centrality/         Centrality algorithm utilities
//! │   ├── community/          Community detection utilities
//! │   ├── similarity/         Similarity algorithm utilities (future)
//! │   ├── embeddings/         Embedding algorithm utilities (future)
//! │   └── stubs/              Stub traits for missing dependencies
//! └── specs/                   Individual algorithm implementations (Species)
//!     ├── pagerank/
//!     ├── degree_centrality/
//!     └── all_shortest_paths/
//! ```
//!
//! ## Module Organization
//!
//! ### Centrality (`centrality/`)
//! Infrastructure for centrality algorithms (PageRank, Betweenness, Closeness, etc.):
//! - `CentralityAlgorithmResult` - Result trait for centrality algorithms
//! - `PageRankDistribution` - Distribution computation for PageRank results
//! - Integration with `procedures/core/statistics` for result analysis
//!
//! ### Community (`community/`)
//! Infrastructure for community detection algorithms (Louvain, Label Propagation, etc.):
//! - `CommunityCompanion` - Utility functions for community results
//! - `ConsecutiveLongNodePropertyValues` - Remap community IDs to consecutive integers
//! - `LongIfChangedNodePropertyValues` - Filter for incremental community detection
//! - Integration with `procedures/core/result_builders` for community results
//!
//! ### Stubs (`stubs/`)
//! Temporary stub traits for missing dependencies:
//! - `NodePropertyValues` - Will be replaced when we translate the full property system
//! - `LongNodePropertyValues`, `DoubleArrayNodePropertyValues`, etc.
//!
//! ## Integration with `procedures/core`
//!
//! This module **extends** `procedures/core` with algorithm type-specific functionality:
//! - Uses `StatisticsEngine` for distribution computation
//! - Uses `ResultBuilder` patterns for result construction
//! - Uses `MemoryEstimationEngine` for memory requirements
//! - Uses `ProgressTracker` for execution monitoring
//!
//! ## Design Philosophy
//!
//! **Genera vs. Species:**
//! - **Genera** (this module): Shared infrastructure for algorithm **types**
//! - **Species** (`procedures/specs`): Individual algorithm implementations
//!
//! This separation allows us to:
//! 1. Share common result processing across algorithm types
//! 2. Provide type-specific utilities (e.g., consecutive community IDs)
//! 3. Integrate with our enhanced `procedures/core` statistics
//! 4. Maintain clean separation between infrastructure and implementation
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use gds::procedures::algorithms::centrality::*;
//! use gds::procedures::core::*;
//!
//! // Compute PageRank distribution using centrality infrastructure
//! let distribution = PageRankDistributionComputer::compute_distribution(
//!     &pagerank_result,
//!     false, // use_log_scaler
//!     true,  // should_compute_distribution
//!     4,     // concurrency
//! );
//!
//! // Result includes min, max, mean, percentiles (p50, p75, p90, p95, p99, p999)
//! println!("PageRank distribution: {:?}", distribution.centrality_summary);
//! ```
//!
//! ## Future Work
//!
//! - **Similarity** - Infrastructure for similarity algorithms (Node Similarity, KNN, etc.)
//! - **Embeddings** - Infrastructure for embedding algorithms (Node2Vec, GraphSAGE, etc.)
//! - **Machine Learning** - Infrastructure for ML algorithms (Link Prediction, etc.)
//! - **Replace stubs** - Implement full `NodePropertyValues` system when translating core API

pub mod centrality;
pub mod community;
pub mod stubs;
pub mod result;

// Re-export commonly used types
pub use centrality::*;
pub use community::*;

