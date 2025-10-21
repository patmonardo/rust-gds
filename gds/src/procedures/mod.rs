//! Procedure Infrastructure - Algorithm Implementations and Catalog
//!
//! **Architecture Note**: This is NOT the executor runtime!
//! The executor runtime lives in `src/projection/eval/procedure/`.
//!
//! ## Key Distinction
//!
//! - **This module** (src/procedure) = Algorithm implementations (extensible content)
//! - **eval/procedure** = Executor runtime (fixed GDSL Runtime)
//!
//! Think of it this way:
//! - **This module** = WHAT to execute (PageRank, Louvain, etc.)
//! - **eval/procedure** = HOW to execute (orchestration machinery)
//!
//! ## What Lives Here
//!
//! This module contains the **Java GDS algo packages** translated to Rust:
//!
//! ```text
//! src/procedure/
//! ├── algo/               ← Algorithm implementations
//! │   ├── pagerank.rs
//! │   ├── louvain.rs
//! │   └── ...
//! ├── common/             ← Shared utilities (algo-common)
//! │   ├── convergence.rs
//! │   ├── tolerance.rs
//! │   └── ...
//! ├── params/             ← Parameter handling (algo-params)
//! └── specifications/     ← Algorithm catalog
//! ```
//!
//! ## The Pattern
//!
//! Each algorithm:
//! 1. **Implements** `AlgorithmSpec` trait (defined in eval/procedure)
//! 2. **Provides** specific computation logic
//! 3. **Registers** in the algorithm catalog
//!
//! Example:
//! ```rust,ignore
//! use crate::projection::eval::procedure::AlgorithmSpec;
//!
//! pub struct PageRankSpec { /* ... */ }
//!
//! impl AlgorithmSpec for PageRankSpec {
//!     // Implement the contract
//!     fn execute(&self, graph, config, context) -> Result<...> {
//!         // PageRank-specific logic here
//!     }
//! }
//! ```
//!
//! ## Relationship to Executor
//!
//! ```text
//! Executor Runtime (eval/procedure):
//!   1. Defines AlgorithmSpec trait (the contract)
//!   2. Provides orchestration (parse → validate → load → execute → consume)
//!   3. Integrates with TypeValidator + AdaptiveProjector
//!
//! Algorithm Implementations (this module):
//!   1. Implement AlgorithmSpec trait
//!   2. Provide specific computation logic
//!   3. Register in catalog
//! ```
//!
//! **The executor USES the algorithms through the trait.**
//!
//!
//! The eval layer RAISES this infrastructure into consciousness through:
//! - AlgorithmSpec (bridges machine to projectors)
//! - TypeValidator (validates forms)
//! - AdaptiveProjector (chooses optimal manifestations)
//!
//! ## Usage
//!
//! Most users will NOT use this module directly. Instead, use the eval/procedure
//! layer which provides projection-aware algorithm specifications.
//!
//! See: `src/projection/eval/procedure/` for the consciousness layer.
/// Core utilities from Java GDS algo-common
/// - Result builders and statistics (centrality, community, similarity)
/// - Feature scaling for ML pipelines
/// - Common algorithm utilities
pub mod core;

/// Algorithm infrastructure (Genera)
/// - Centrality algorithm utilities
/// - Community detection utilities
/// - Algorithm-specific result types and transformations
pub mod algorithms;

// Module structure

pub mod sum;
pub mod pagerank;
pub mod degree_centrality;
pub mod all_shortest_paths;
pub mod astar;
pub mod bellman_ford;
pub mod delta_stepping;
pub mod dijkstra;
pub mod bfs;
pub mod dfs;
pub mod yens;
pub mod traversal;
pub mod spanning_tree;


// Future modules (to be implemented)
// pub mod facade;      // Public API facades

// Re-export commonly used types
pub use sum::{SumComputationRuntime, SumAlgorithmSpec, SumConfig, SumStorageRuntime};
pub use pagerank::{PageRankAlgorithmSpec, PageRankComputationResult, PageRankConfig, PageRankPregelComputation, PageRankMemoryEstimation, estimate_pagerank_memory};
pub use degree_centrality::{DEGREE_CENTRALITYAlgorithmSpec, DegreeCentralityConfig, DegreeCentralityResult, DegreeCentralityStorageRuntime, DegreeCentralityComputationRuntime};
pub use all_shortest_paths::{ALL_SHORTEST_PATHSAlgorithmSpec, AllShortestPathsConfig, AllShortestPathsResult, AllShortestPathsStorageRuntime, AllShortestPathsComputationRuntime};
pub use astar::{ASTARAlgorithmSpec, AStarConfig, AStarResult, AStarStorageRuntime, AStarComputationRuntime};
pub use bellman_ford::{BELLMAN_FORDAlgorithmSpec, BellmanFordConfig, BellmanFordResult, BellmanFordStorageRuntime, BellmanFordComputationRuntime};
pub use delta_stepping::{DELTA_STEPPINGAlgorithmSpec, DeltaSteppingConfig, DeltaSteppingResult, DeltaSteppingStorageRuntime, DeltaSteppingComputationRuntime};
pub use dijkstra::{DIJKSTRAAlgorithmSpec, DijkstraConfig, DijkstraResult, DijkstraStorageRuntime, DijkstraComputationRuntime, Targets, SingleTarget, ManyTargets, AllTargets, TraversalState, PathFindingResult};
pub use bfs::{BFSAlgorithmSpec, BfsConfig, BfsResult, BfsStorageRuntime, BfsComputationRuntime};
pub use dfs::{DFSAlgorithmSpec, DfsConfig, DfsResult, DfsStorageRuntime, DfsComputationRuntime};
pub use yens::{YENSAlgorithmSpec, YensConfig, YensResult, YensStorageRuntime, YensComputationRuntime, MutablePathResult, CandidatePathsPriorityQueue, RelationshipFilterer};
pub use traversal::{ExitPredicate, Aggregator, ExitPredicateResult, FollowExitPredicate, OneHopAggregator, TargetExitPredicate};
pub use spanning_tree::{SPANNING_TREEAlgorithmSpec, SpanningTreeConfig, SpanningTreeResult, SpanningTreeStorageRuntime, SpanningTreeComputationRuntime, SpanningTree, SpanningGraph};

// pub use algorithms::*;
pub use core::*;
