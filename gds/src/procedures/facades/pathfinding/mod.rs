//! Path finding algorithm facades
//!
//! Finds paths between nodes with various optimality criteria.
//! Supports single-source, all-pairs, and multi-target queries.

pub mod dijkstra;
pub mod bfs;
pub mod dfs;
pub mod astar;

// Re-export for easy access
pub use dijkstra::{DijkstraBuilder, DijkstraStats};
pub use bfs::{BfsBuilder, BfsStats};
pub use dfs::{DfsBuilder, DfsStats};
pub use astar::{AStarBuilder, AStarStats, Heuristic};

