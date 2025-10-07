/// Projection-related types for graph data science.
///
/// This module contains types used for projecting and identifying
/// nodes and relationships in graphs.
pub mod impls;
pub mod node_label;
pub mod orientation;
pub mod relationship_type;
pub mod traits;

// Export only the basic types - no traits or impls for now
pub use impls::*;
pub use node_label::*;
pub use orientation::*;
pub use relationship_type::*;
pub use traits::*;
