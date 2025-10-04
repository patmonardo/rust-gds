#![allow(clippy::module_inception)]

pub mod adj_list;
pub mod characteristics;
pub mod default_graph;
pub mod degrees;
pub mod graph;
pub mod id_map;
pub mod topology;

pub use characteristics::{GraphCharacteristics, GraphCharacteristicsBuilder};
pub use default_graph::DefaultGraph;
pub use degrees::Degrees;
pub use graph::{Graph, GraphExt, GraphResult};
pub use topology::RelationshipTopology;
