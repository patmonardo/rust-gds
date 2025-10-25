//! PropertyStore configuration for backend selection
//! 
//! DEPRECATED: Use CollectionsConfig instead for Collections First approach

use crate::config::collections_config::{CollectionsBackend, CollectionsConfig, CollectionsConfigBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyBackendKind {
    Vec,
    HugeArray,
    #[cfg(feature = "arrow")] 
    Arrow,
}

impl From<PropertyBackendKind> for CollectionsBackend {
    fn from(kind: PropertyBackendKind) -> Self {
        match kind {
            PropertyBackendKind::Vec => CollectionsBackend::Vec,
            PropertyBackendKind::HugeArray => CollectionsBackend::Huge,
            #[cfg(feature = "arrow")]
            PropertyBackendKind::Arrow => CollectionsBackend::Arrow,
        }
    }
}

impl From<CollectionsBackend> for PropertyBackendKind {
    fn from(backend: CollectionsBackend) -> Self {
        match backend {
            CollectionsBackend::Vec => PropertyBackendKind::Vec,
            CollectionsBackend::Huge => PropertyBackendKind::HugeArray,
            #[cfg(feature = "arrow")]
            CollectionsBackend::Arrow => PropertyBackendKind::Arrow,
            CollectionsBackend::Std => PropertyBackendKind::Vec, // Default fallback
            #[cfg(not(feature = "arrow"))]
            CollectionsBackend::Arrow => PropertyBackendKind::Vec, // Fallback when arrow not available
            // Handle all other backends by defaulting to Vec
            _ => PropertyBackendKind::Vec,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PropertyStoreConfig {
    pub default_backend: PropertyBackendKind,
    pub node_backend: Option<PropertyBackendKind>,
    pub relationship_backend: Option<PropertyBackendKind>,
    pub graph_backend: Option<PropertyBackendKind>,
}

impl Default for PropertyStoreConfig {
    fn default() -> Self {
        Self {
            default_backend: PropertyBackendKind::Vec,
            node_backend: None,
            relationship_backend: None,
            graph_backend: None,
        }
    }
}

impl<T> From<PropertyStoreConfig> for CollectionsConfig<T> {
    fn from(config: PropertyStoreConfig) -> Self {
        CollectionsConfigBuilder::<T>::new()
            .with_backend(config.default_backend.into())
            .build()
    }
}
