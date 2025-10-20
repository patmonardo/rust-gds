/// Configuration for mutating node labels in the graph store.
/// 
/// Mirrors Java MutateLabelConfig interface.
/// Simple trait with a single method for specifying the node filter.
pub trait MutateLabelConfig {
    /// Returns the node filter expression as a string.
    /// This filter determines which nodes will have the label applied.
    fn node_filter(&self) -> String;
}

/// Builder for creating MutateLabelConfig implementations.
/// 
/// In Java, this uses CypherMapWrapper for configuration parsing.
/// For now, this is a simple struct that can be extended as needed.
#[derive(Clone, Debug)]
pub struct MutateLabelConfigImpl {
    node_filter: String,
}

impl MutateLabelConfigImpl {
    /// Creates a new MutateLabelConfig.
    pub fn new(node_filter: String) -> Self {
        Self { node_filter }
    }
}

impl MutateLabelConfig for MutateLabelConfigImpl {
    fn node_filter(&self) -> String {
        self.node_filter.clone()
    }
}
