//! Result Processing - Statistics and result builders for algorithm outputs
//!
//! **Translation Source**: `org.neo4j.gds.result` package
//!
//! This module provides statistical analysis and result building for algorithm outputs:
//! - Histogram generation
//! - Percentile calculations
//! - Community size distributions
//! - Centrality score distributions
//! - Similarity score distributions
//!
//! ## Modules
//!
//! - `centrality` - Centrality algorithm result processing
//! - `community` - Community detection result processing
//! - `similarity` - Similarity algorithm result processing
//!
//! ## Translations
//!
//! - ✅ `CentralityStatistics.java` → `centrality.rs` (Phase 2)
//! - ✅ `CommunityStatistics.java` → `community.rs` (Phase 2)
//! - ✅ `SimilarityStatistics.java` → `similarity.rs` (Phase 2)

pub mod centrality;
pub mod community;
pub mod similarity;
