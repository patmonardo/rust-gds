// Configuration traits for GraphStore catalog operations

pub mod graph_access_graph_properties_config;
pub mod graph_remove_graph_properties_config;
pub mod graph_stream_graph_properties_config;
pub mod mutate_label_config;
pub mod write_label_config;
pub mod write_relationship_properties_config;
pub mod graph_node_properties_config;
pub mod graph_export_node_properties_config;
pub mod graph_stream_node_properties_config;
pub mod graph_write_node_properties_config;
pub mod graph_stream_relationships_config;
pub mod graph_stream_relationship_properties_config;
pub mod graph_write_relationship_config;
pub mod graph_drop_node_properties_config;

pub use graph_access_graph_properties_config::*;
pub use graph_remove_graph_properties_config::*;
pub use graph_stream_graph_properties_config::*;
pub use mutate_label_config::*;
pub use write_label_config::*;
pub use write_relationship_properties_config::*;
pub use graph_node_properties_config::*;
pub use graph_export_node_properties_config::*;
pub use graph_stream_node_properties_config::*;
pub use graph_write_node_properties_config::*;
pub use graph_stream_relationships_config::*;
pub use graph_stream_relationship_properties_config::*;
pub use graph_write_relationship_config::*;
pub use graph_drop_node_properties_config::*;

