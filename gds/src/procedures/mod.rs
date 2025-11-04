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

// Procedure Facades - User-facing idiomatic Rust API
pub mod facades;

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
// pub mod scc;  // TODO: Fix trait bounds and private module issues
pub mod articulation_points;
pub mod bridges;
pub mod wcc;
pub mod msbfs;
pub mod harmonic;
pub mod closeness;
pub mod betweenness;
pub mod triangle_count;
pub mod louvain;
pub mod label_propagation;
pub mod kcore;
pub mod k1coloring;
pub mod kspanningtree;
pub mod local_clustering_coefficient;
pub mod hits;


// Future modules (to be implemented)
// pub mod facade;      // Public API facades

// Re-export commonly used types
pub use sum::{SumComputationRuntime, SumAlgorithmSpec, SumConfig, SumStorageRuntime};
pub use pagerank::{PageRankAlgorithmSpec, PageRankComputationResult, PageRankPregelComputation, PageRankMemoryEstimation, estimate_pagerank_memory};
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
// pub use scc::{SCCAlgorithmSpec, SccConfig, SccResult, SccStorageRuntime, SccComputationRuntime};
pub use articulation_points::{ArticulationPointsAlgorithmSpec, ArticulationPointsConfig, ArticulationPointsResult, ArticulationPointsStorageRuntime, ArticulationPointsComputationRuntime};
pub use bridges::{BridgesAlgorithmSpec, BridgesConfig, BridgesResult, BridgesStorageRuntime, BridgesComputationRuntime};
pub use wcc::{WccAlgorithmSpec, WccConfig, WccResult, WccStorageRuntime, WccComputationRuntime};
pub use msbfs::SimpleMSBFS;
pub use harmonic::{HarmonicAlgorithmSpec, HarmonicConfig, HarmonicResult, HarmonicStorageRuntime, HarmonicComputationRuntime};
pub use closeness::{ClosenessCentralityAlgorithmSpec, ClosenessCentralityConfig, ClosenessCentralityResult, ClosenessCentralityStorageRuntime, ClosenessCentralityComputationRuntime};
pub use betweenness::{BetweennessCentralityAlgorithmSpec, BetweennessCentralityConfig, BetweennessCentralityResult, BetweennessCentralityStorageRuntime, BetweennessCentralityComputationRuntime};
pub use triangle_count::{TriangleCountAlgorithmSpec, TriangleCountConfig, TriangleCountResult, TriangleCountStorageRuntime, TriangleCountComputationRuntime};
pub use louvain::{LouvainAlgorithmSpec, LouvainConfig, LouvainResult, LouvainStorageRuntime, LouvainComputationRuntime};
pub use label_propagation::{LabelPropAlgorithmSpec, LabelPropConfig, LabelPropResult, LabelPropStorageRuntime, LabelPropComputationRuntime};
pub use kcore::{KCoreAlgorithmSpec, KCoreConfig, KCoreResult, KCoreStorageRuntime, KCoreDecompositionRuntime};
pub use k1coloring::{K1ColoringAlgorithmSpec, K1ColoringConfig, K1ColoringResult, K1ColoringStorageRuntime, K1ColoringComputationRuntime};
pub use kspanningtree::{KSpanningTreeAlgorithmSpec, KSpanningTreeConfig, KSpanningTreeResult, KSpanningTreeStorageRuntime, KSpanningTreeComputationRuntime};
pub use local_clustering_coefficient::{LOCAL_CLUSTERING_COEFFICIENTAlgorithmSpec, LocalClusteringCoefficientConfig, LocalClusteringCoefficientResult, LocalClusteringCoefficientStorageRuntime, LocalClusteringCoefficientComputationRuntime};
pub use hits::{HITSAlgorithmSpec, HitsConfig, HitsResult, HitsStorageRuntime, HitsComputationRuntime};

// pub use algorithms::*;
pub use core::*;
