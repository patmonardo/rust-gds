//! PropertyStore configuration for backend selection

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyBackendKind {
    Vec,
    HugeArray,
    #[cfg(feature = "arrow")] 
    Arrow,
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


