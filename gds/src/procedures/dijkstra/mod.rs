//! Dijkstra Algorithm Module
//!
//! **Translation Source**: `org.neo4j.gds.paths.dijkstra.Dijkstra`
//!
//! This module implements the Dijkstra algorithm as a configurable Algorithmic Virtual Machine
//! with polymorphic target system, traversal state management, and stream-based result handling.
//! The algorithm supports single-target, many-targets, and all-targets modes with composable
//! relationship filters and heuristic functions.

pub mod spec;
pub mod storage;
pub mod computation;
pub mod targets;
pub mod traversal_state;
pub mod path_finding_result;
pub mod integration_tests;

// Re-export main types for easy access
pub use spec::{DIJKSTRAAlgorithmSpec, DijkstraConfig, DijkstraResult};
pub use storage::DijkstraStorageRuntime;
pub use computation::DijkstraComputationRuntime;
pub use targets::{Targets, SingleTarget, ManyTargets, AllTargets};
pub use traversal_state::TraversalState;
pub use path_finding_result::PathFindingResult;
