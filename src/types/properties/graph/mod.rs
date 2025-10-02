// Graph properties module
//
// Provides property storage for graph-level properties.
// Graph properties are scalar values that apply to the entire graph.

pub mod graph_property;
pub mod graph_property_store;
pub mod graph_property_values;
pub mod impls;
pub mod traits;

// Re-export public API
pub use graph_property::GraphProperty;
pub use graph_property_store::{GraphPropertyStore, GraphPropertyStoreBuilder};
pub use graph_property_values::{
    DefaultDoubleArrayGraphPropertyValues, DefaultDoubleGraphPropertyValues,
    DefaultFloatArrayGraphPropertyValues, DefaultLongArrayGraphPropertyValues,
    DefaultLongGraphPropertyValues, DoubleArrayGraphPropertyValues, DoubleGraphPropertyValues,
    FloatArrayGraphPropertyValues, GraphPropertyValues, LongArrayGraphPropertyValues,
    LongGraphPropertyValues,
};
