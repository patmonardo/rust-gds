/// Configuration for writing node labels to the database.
/// 
/// Mirrors Java WriteLabelConfig interface.
/// Extends BaseConfig + multiple trait bounds (ConcurrencyConfig, WriteConfig, JobIdConfig).
/// For now, we'll define it as a simple trait that can be extended later.
pub trait WriteLabelConfig {
    /// Returns the node filter expression as a string.
    /// This filter determines which nodes will have the label written.
    fn node_filter(&self) -> String;
}

/// Builder for creating WriteLabelConfig implementations.
/// 
/// In Java, this uses CypherMapWrapper for configuration parsing.
/// For now, this is a simple struct that can be extended as needed.
#[derive(Clone, Debug)]
pub struct WriteLabelConfigImpl {
    node_filter: String,
}

impl WriteLabelConfigImpl {
    /// Creates a new WriteLabelConfig.
    pub fn new(node_filter: String) -> Self {
        Self { node_filter }
    }
}

impl WriteLabelConfig for WriteLabelConfigImpl {
    fn node_filter(&self) -> String {
        self.node_filter.clone()
    }
}
