/// Configuration for accessing graph properties from the graph store.
/// 
/// Mirrors Java GraphAccessGraphPropertiesConfig interface.
/// This is a simple trait with two methods for specifying which graph and property to access.
pub trait GraphAccessGraphPropertiesConfig {
    /// Returns the optional graph name.
    /// If None, the graph name should be provided through other means.
    fn graph_name(&self) -> Option<String>;
    
    /// Returns the graph property key to access.
    fn graph_property(&self) -> String;
}

