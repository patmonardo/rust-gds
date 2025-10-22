//! Centrality algorithm facades
//!
//! Provides user-friendly APIs for centrality algorithms:
//! - DegreeCentrality: Counts node connections
//! - PageRank: Importance based on link structure
//! - Betweenness: Frequency of appearance in shortest paths
//! - Closeness: Average distance to all other nodes
//! - Harmonic: Reciprocal distances
//! - HITS: Hub and authority scores

pub mod degree_centrality;
pub mod pagerank;
pub mod betweenness;

// Re-export main facades
pub use degree_centrality::DegreeCentralityFacade;
pub use pagerank::PageRankBuilder;
pub use betweenness::BetweenessBuilder;

