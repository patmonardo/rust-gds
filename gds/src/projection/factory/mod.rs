// Factory Module - "The Absolute Form's Kernel"
//
// This module provides factory abstractions for creating GraphStores from native data sources.
// The factory pattern separates data ingestion (projection/factory/) from data processing
// (projection/native/ for ML/execution).
//
// Design Philosophy:
// - Arrow IS native for rust-gds (like Neo4j is native for Java GDS)
// - Zero-copy optimization where Arrow types map directly to PropertyValues
// - Extensible for multiple native sources (Arrow, Polars, DuckDB, future Neo4j)
// - NOT IO! Assumes data already in memory (Arrow tables, Polars DataFrames, etc.)

use crate::types::graph_store::DefaultGraphStore;

/// Core factory trait for creating GraphStores from native data sources.
///
/// This trait defines the contract for all factory implementations (Arrow, Polars, etc.).
/// Factories are responsible for:
/// - Schema inference from native source
/// - Parallel data import
/// - Property mapping (native types → GDS types)
/// - ID mapping (source IDs → GDS node IDs)
/// - GraphStore construction
///
/// # Design Notes
///
/// Factories handle the **projection** step (native data → GraphStore).
/// They do NOT handle file I/O - that's `io/import/` responsibility.
/// They assume data is already loaded into memory (Arrow tables, Polars frames, etc.).
pub trait GraphStoreFactory: Send + Sync {
    /// Configuration type for this factory
    type Config;

    /// Error type for factory operations
    type Error: std::error::Error + Send + Sync + 'static;

    /// Create a GraphStore from the native data source.
    ///
    /// This is the main entry point. Implementations should:
    /// 1. Validate configuration
    /// 2. Infer schema from source
    /// 3. Import nodes (parallel)
    /// 4. Import edges (parallel)
    /// 5. Construct and return GraphStore
    fn build_graph_store(&self, config: &Self::Config) -> Result<DefaultGraphStore, Self::Error>;

    /// Estimate memory usage for the import operation.
    ///
    /// This helps users pre-allocate resources and avoid OOM.
    /// Returns (during_loading, after_loading) estimates in bytes.
    fn estimate_memory(&self, config: &Self::Config) -> Result<(usize, usize), Self::Error>;

    /// Get the number of nodes in the source.
    ///
    /// Used for progress tracking and pre-allocation.
    fn node_count(&self, config: &Self::Config) -> Result<usize, Self::Error>;

    /// Get the number of edges in the source.
    ///
    /// Used for progress tracking and pre-allocation.
    fn edge_count(&self, config: &Self::Config) -> Result<usize, Self::Error>;
}

/// Typed factory trait: allows factories to declare the concrete `Store` they produce.
///
/// This is the future-proof form of factory: implementations can return different
/// concrete `GraphStore` types while preserving a strong compile-time contract.
pub trait GraphStoreFactoryTyped: Send + Sync {
    /// Configuration type for this factory
    type Config;

    /// Error type for factory operations
    type Error: std::error::Error + Send + Sync + 'static;

    /// Concrete GraphStore type produced by this factory
    type Store: crate::types::graph_store::GraphStore;

    /// Create a GraphStore from the native data source.
    fn build_graph_store(&self, config: &Self::Config) -> Result<Self::Store, Self::Error>;

    /// Estimate memory usage for the import operation.
    fn estimate_memory(&self, config: &Self::Config) -> Result<(usize, usize), Self::Error>;

    /// Get the number of nodes in the source.
    fn node_count(&self, config: &Self::Config) -> Result<usize, Self::Error>;

    /// Get the number of edges in the source.
    fn edge_count(&self, config: &Self::Config) -> Result<usize, Self::Error>;
}

/// Public prelude for factory module.
///
/// Import this to get all factory-related types:
/// ```
/// use gds::projection::factory::prelude::*;
/// ```
pub mod prelude {
    pub use super::GraphStoreFactory;

    // Re-export Arrow factory when available
    #[cfg(feature = "arrow")]
    pub use super::arrow::{ArrowNativeFactory, ArrowProjectionConfig, ArrowProjectionError};
    // Re-export a CSR/Huge-style factory (delegates to Arrow for now)
    #[cfg(feature = "arrow")]
    pub use super::csr_huge::CsrHugeGraphStoreFactory;
}

// Arrow-native factory (PRIORITY - Phase 1-8)
#[cfg(feature = "arrow")]
pub mod arrow;

// Minimal CSR/Huge-style factory module (Phase 1: delegates to ArrowNativeFactory)
#[cfg(feature = "arrow")]
pub mod csr_huge;

// Relationships builder trait (minimal stub for ML negative sampling)
pub mod relationships_builder;
pub use relationships_builder::RelationshipsBuilder;

// Future: Polars-native factory
// #[cfg(feature = "polars")]
// pub mod polars;

// Future: Neo4j-native factory (compatibility layer)
// #[cfg(feature = "neo4j")]
// pub mod neo4j;
