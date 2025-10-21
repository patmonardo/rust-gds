//! A* Pathfinding Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.paths.astar.AStar`
//!
//! This module provides the A* pathfinding algorithm with Haversine heuristic for geographical pathfinding.

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

// Re-export main types
pub use spec::{ASTARAlgorithmSpec, AStarConfig, AStarResult};
pub use storage::AStarStorageRuntime;
pub use computation::AStarComputationRuntime;
