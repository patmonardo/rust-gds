// CSR Huge GraphStore Factory (Phase 1)
//
// Minimal, low-risk implementation that demonstrates the factory pattern.
// For now it delegates to `ArrowNativeFactory` to build a `DefaultGraphStore`.

use crate::projection::factory::arrow::{
    ArrowNativeFactory, ArrowProjectionConfig, ArrowProjectionError,
};
use crate::projection::factory::GraphStoreFactory;
use crate::types::graph_store::DefaultGraphStore;

/// Configuration for CsrHugeGraphStoreFactory.
/// For Phase 1 we reuse ArrowProjectionConfig; in future this will include
/// CSR-specific options (mmap paths, buffer sizes, compression flags).
#[derive(Debug, Clone)]
pub struct CsrHugeFactoryConfig {
    pub inner: ArrowProjectionConfig,
}

impl From<ArrowProjectionConfig> for CsrHugeFactoryConfig {
    fn from(c: ArrowProjectionConfig) -> Self {
        Self { inner: c }
    }
}

impl Default for CsrHugeFactoryConfig {
    fn default() -> Self {
        Self {
            inner: ArrowProjectionConfig::default(),
        }
    }
}

/// Error type for CSR factory - reuse ArrowProjectionError for now.
pub type CsrHugeFactoryError = ArrowProjectionError;

/// A minimal CSR/Huge-style factory that delegates to ArrowNativeFactory for Phase 1.
#[derive(Debug, Clone)]
pub struct CsrHugeGraphStoreFactory {
    arrow: ArrowNativeFactory,
}

impl CsrHugeGraphStoreFactory {
    /// Create a new CSR/Huge factory backed by an Arrow factory.
    pub fn new() -> Self {
        Self {
            arrow: ArrowNativeFactory::new(),
        }
    }
}

impl Default for CsrHugeGraphStoreFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphStoreFactory for CsrHugeGraphStoreFactory {
    type Config = CsrHugeFactoryConfig;
    type Error = CsrHugeFactoryError;

    fn build_graph_store(&self, config: &Self::Config) -> Result<DefaultGraphStore, Self::Error> {
        // Phase 1: Delegate to Arrow factory. In future we'll implement CSR-specific
        // building (mmap-backed arrays, on-disk huge arrays, custom topologies).
        self.arrow.build_graph_store(&config.inner)
    }

    fn estimate_memory(&self, config: &Self::Config) -> Result<(usize, usize), Self::Error> {
        self.arrow.estimate_memory(&config.inner)
    }

    fn node_count(&self, config: &Self::Config) -> Result<usize, Self::Error> {
        self.arrow.node_count(&config.inner)
    }

    fn edge_count(&self, config: &Self::Config) -> Result<usize, Self::Error> {
        self.arrow.edge_count(&config.inner)
    }
}

impl crate::projection::factory::GraphStoreFactoryTyped for CsrHugeGraphStoreFactory {
    type Config = CsrHugeFactoryConfig;
    type Error = CsrHugeFactoryError;
    type Store = crate::types::graph_store::DefaultGraphStore;

    fn build_graph_store(&self, config: &Self::Config) -> Result<Self::Store, Self::Error> {
        <CsrHugeGraphStoreFactory as crate::projection::factory::GraphStoreFactory>::build_graph_store(self, config)
    }

    fn estimate_memory(&self, config: &Self::Config) -> Result<(usize, usize), Self::Error> {
        <CsrHugeGraphStoreFactory as crate::projection::factory::GraphStoreFactory>::estimate_memory(
            self, config,
        )
    }

    fn node_count(&self, config: &Self::Config) -> Result<usize, Self::Error> {
        <CsrHugeGraphStoreFactory as crate::projection::factory::GraphStoreFactory>::node_count(
            self, config,
        )
    }

    fn edge_count(&self, config: &Self::Config) -> Result<usize, Self::Error> {
        <CsrHugeGraphStoreFactory as crate::projection::factory::GraphStoreFactory>::edge_count(
            self, config,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::factory::arrow::ArrowProjectionConfig;

    #[test]
    fn csr_huge_factory_delegate_build() {
        let factory = CsrHugeGraphStoreFactory::new();
        let cfg = CsrHugeFactoryConfig::default();
        let res = factory.build_graph_store(&cfg);
        // Since ArrowNativeFactory is a skeleton, we expect a not-implemented Other error
        assert!(res.is_err());
    }
}
