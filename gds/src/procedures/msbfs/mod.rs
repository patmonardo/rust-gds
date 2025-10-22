//! Multi-Source Breadth-First Search (MSBFS) Framework
//!
//! **Translation Source**: `org.neo4j.gds.msbfs.MultiSourceBFSAccessMethods`
//!
//! Efficiently computes BFS from multiple source nodes simultaneously by bit-packing
//! up to 64 sources into a single u64 mask per node.
//!
//! This enables algorithms like Harmonic Centrality, Closeness Centrality, and
//! Betweenness Centrality to process multiple sources with minimal memory overhead.

pub mod simple;

pub use simple::SimpleMSBFS;
