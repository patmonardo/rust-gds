//! Spanning Tree Algorithms - Minimum/Maximum Spanning Tree Implementation
//!
//! **Translation Source**: `org.neo4j.gds.spanningtree.*`
//!
//! This module implements spanning tree algorithms, primarily Prim's algorithm
//! for computing minimum or maximum spanning trees from a given start node.
//!
//! ## Architecture
//!
//! Following our established pattern:
//! - **Storage Runtime**: Graph data access and algorithm orchestration
//! - **Computation Runtime**: Priority queue management and tree construction
//! - **Algorithm Spec**: Configuration and execution using focused macros
//!
//! ## Key Components
//!
//! - **Prim**: Core Prim's algorithm implementation
//! - **SpanningTree**: Result type with parent array and cost tracking
//! - **SpanningGraph**: Graph adapter for tree traversal
//! - **Memory Estimation**: Memory requirements calculation
//!
//! ## Usage
//!
//! ```rust,ignore
//! use crate::procedures::spanning_tree::*;
//!
//! let config = SpanningTreeConfig::default();
//! let mut algorithm = SPANNING_TREEAlgorithmSpec::new("my_graph".to_string());
//! let result = algorithm.execute(&graph_store, &config, &context)?;
//! ```

pub mod spec;
pub mod storage;
pub mod computation;
pub mod integration_tests;

// Re-export main types
pub use spec::{SPANNING_TREEAlgorithmSpec, SpanningTreeConfig, SpanningTreeResult};
pub use storage::SpanningTreeStorageRuntime;
pub use computation::SpanningTreeComputationRuntime;
pub use computation::{SpanningTree, SpanningGraph};

