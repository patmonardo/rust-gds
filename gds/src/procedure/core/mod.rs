//! Procedure Core - Common utilities from Java GDS algo-common
//!
//! **Translation Source**: `org.neo4j.gds` package in algo-common
//! **Translation Protocol**: TP-006 (Gamma-level architectural translation)
//!
//! This module contains the foundation shared by all graph algorithms:
//! - Result builders and statistics computation
//! - Feature scaling for ML pipelines  
//! - Common algorithm utilities
//!
//! ## Architecture
//!
//! ```text
//! procedure/core/              (Java GDS algo-common)
//! ├── result/                  (org.neo4j.gds.result)
//! │   ├── centrality.rs       (CentralityStatistics, AbstractCentralityResultBuilder)
//! │   ├── community.rs        (CommunityStatistics, AbstractCommunityResultBuilder)
//! │   └── similarity.rs       (SimilarityStatistics)
//! │
//! ├── scaling/                 (org.neo4j.gds.scaling)
//! │   ├── scaler.rs           (Scaler trait, ScalerFactory)
//! │   ├── minmax.rs           (MinMax scaler)
//! │   ├── stdscore.rs         (Standard score / Z-score)
//! │   ├── mean.rs             (Mean normalization)
//! │   ├── max.rs              (Max absolute scaling)
//! │   ├── center.rs           (Center scaling)
//! │   ├── log.rs              (Log scaling)
//! │   └── none.rs             (No-op scaler)
//! │
//! └── prelude.rs              (Common re-exports)
//! ```
//!
//! ## What We're NOT Translating
//!
//! The following Java GDS components are **intentionally skipped**:
//!
//! 1. **Algorithm.java** - Base algorithm class
//!    - **Why**: We use `AlgorithmSpec` trait from executor instead
//!    - **Rust equivalent**: `projection::eval::procedure::AlgorithmSpec`
//!
//! 2. **AlgorithmFactory.java** - Factory pattern with visitor
//!    - **Why**: Rust uses direct construction + builder pattern
//!    - **Rust equivalent**: Direct `PageRank::new()` or builder
//!
//! 3. **GraphAlgorithmFactory.java** - Graph-based factory
//!    - **Why**: Same as above - unnecessary ceremony in Rust
//!
//! 4. **GraphStoreAlgorithmFactory.java** - GraphStore-based factory
//!    - **Why**: Same as above - Rust traits make this simple
//!
//! 5. **Converters.java** - Long/Int conversion utilities
//!    - **Why**: Rust uses u64/usize directly, no conversions needed
//!    - **Note**: Java needs this for legacy int-based code
//!
//! 6. **MemoryEstimationNotImplementedException.java**
//!    - **Why**: We use `AlgorithmError` from executor
//!
//! ## Design Philosophy
//!
//! This is **Gamma-level translation** - we preserve the conceptual architecture
//! but adapt to Rust idioms:
//!
//! - **Replace** Java classes with Rust structs + traits
//! - **Replace** ExecutorService with `rayon` parallelism
//! - **Replace** visitor pattern with trait objects
//! - **Preserve** statistical algorithms and scaling logic
//! - **Preserve** result builder patterns
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use gds::procedure::core::prelude::*;
//!
//! // Use result statistics
//! let stats = CentralityStatistics::compute(
//!     graph.node_count(),
//!     |node_id| pagerank_scores[node_id],
//!     concurrency,
//! )?;
//!
//! // Use feature scaling
//! let scaler = MinMaxScaler::new(property_values, concurrency)?;
//! let scaled_value = scaler.scale(node_id);
//! ```

pub mod prelude;
pub mod result;
pub mod scaling;
