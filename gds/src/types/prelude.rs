//! Minimal prelude for gds::types
//! Re-export a small, stable set of commonly-used types.
//! Keep this list intentionally tiny to avoid large codegen surface.

pub use crate::types::graph_store::DefaultGraphStore;
pub use crate::types::graph_store::GraphStore;
pub use crate::types::random::RandomGraphConfig;
