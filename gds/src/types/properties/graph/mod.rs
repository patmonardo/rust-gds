// Graph properties module
//
// Provides property storage for graph-level properties.
// Graph properties are scalar values that apply to the entire graph.

pub mod graph_property;
pub mod graph_property_store;
pub mod graph_property_values;
pub mod impls;

// Re-export commonly used items
pub use graph_property::*;
pub use graph_property_store::*;
pub use graph_property_values::*;
pub use impls::*;
