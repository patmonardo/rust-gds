use crate::api::{Graph, GraphStore};
use crate::core::loading::PostLoadValidationHook;
use crate::core::model::MLModel as Model;
use crate::applications::algorithms::node_embeddings::GraphSageModelCatalog;
// use crate::embeddings::{
//     graphsage::{GraphSageBaseConfig, GraphSageTrainConfig},
//     node2vec::Node2VecBaseConfig,
// };
// use crate::core::concurrency::DefaultPool;
// use crate::ml::core::EmbeddingUtils; // Placeholder
// use crate::core::model::Model; // Placeholder - commented out for now
// use crate::embeddings::graphsage::{ModelData, GraphSageModelTrainer, GraphSageTrainConfig as GSGT};

// Placeholder types
pub trait FastRPBaseConfig {}
pub trait Node2VecBaseConfig {}

// Placeholder for DefaultPool
pub struct DefaultPool;

/// Node2Vec validation hook - checks for arithmetic overflow
pub struct Node2VecValidationHook<C: Node2VecBaseConfig> {
    configuration: C,
}

impl<C: Node2VecBaseConfig> Node2VecValidationHook<C> {
    pub fn new(configuration: C) -> Self {
        Self { configuration }
    }
}

impl<C: Node2VecBaseConfig> PostLoadValidationHook for Node2VecValidationHook<C> {
    fn on_graph_store_loaded(&self, _graph_store: &GraphStore) {
        // do nothing
    }

    fn on_graph_loaded(&self, graph: &Graph) {
        // Check for arithmetic overflow in walk generation
        let node_count = graph.node_count();
        let walks_per_node = self.configuration.walks_per_node();
        let walk_length = self.configuration.walk_length();

        // Check if the multiplication would overflow
        if let Some(_) = node_count.checked_mul(walks_per_node).and_then(|x| x.checked_mul(walk_length)) {
            // No overflow, continue
        } else {
            panic!(
                "Aborting execution, running with the configured parameters is likely to overflow: node count: {}, walks per node: {}, walkLength: {}. Try reducing these parameters or run on a smaller graph.",
                node_count, walks_per_node, walk_length
            );
        }
    }
}

/// GraphSage validation hook for inference
pub struct GraphSageValidationHook<C: super::node_embeddings_algorithms::GraphSageBaseConfig> {
    configuration: C,
    model: Model<super::node_embeddings_algorithms::ModelDataImpl, super::node_embeddings_algorithms::GraphSageTrainConfigImpl, super::node_embeddings_algorithms::GraphSageTrainMetrics>,
}

impl<C: super::node_embeddings_algorithms::GraphSageBaseConfig> GraphSageValidationHook<C> {
    pub fn new(configuration: C, model: Model<super::node_embeddings_algorithms::ModelDataImpl, super::node_embeddings_algorithms::GraphSageTrainConfigImpl, super::node_embeddings_algorithms::GraphSageTrainMetrics>) -> Self {
        Self { configuration, model }
    }
}

impl<C: super::node_embeddings_algorithms::GraphSageBaseConfig> PostLoadValidationHook for GraphSageValidationHook<C> {
    fn on_graph_store_loaded(&self, graph_store: &GraphStore) {
        let train_config = self.model.train_config();

        train_config.graph_store_validation(
            graph_store,
            self.configuration.node_label_identifiers(graph_store),
            self.configuration.internal_relationship_types(graph_store),
        );
    }

    fn on_graph_loaded(&self, graph: &Graph) {
        if !graph.has_relationship_property() {
            return;
        }

        EmbeddingUtils::validate_relationship_weight_property_value(
            graph,
            self.configuration.concurrency(),
            DefaultPool::new(),
        );
    }
}

/// GraphSage training validation hook
pub struct GraphSageTrainValidationHook<C: super::node_embeddings_algorithms::GraphSageTrainConfig> {
    configuration: C,
}

impl<C: super::node_embeddings_algorithms::GraphSageTrainConfig> GraphSageTrainValidationHook<C> {
    pub fn new(configuration: C) -> Self {
        Self { configuration }
    }
}

impl<C: super::node_embeddings_algorithms::GraphSageTrainConfig> PostLoadValidationHook for GraphSageTrainValidationHook<C> {
    fn on_graph_store_loaded(&self, _graph_store: &GraphStore) {
        // do nothing, or rather, it is automatic
    }

    fn on_graph_loaded(&self, graph: &Graph) {
        if !graph.has_relationship_property() {
            return;
        }

        EmbeddingUtils::validate_relationship_weight_property_value(
            graph,
            self.configuration.concurrency(),
            DefaultPool::new(),
        );
    }
}

// Placeholder types and implementations
// pub trait PostLoadValidationHook {
//     fn on_graph_store_loaded(&self, graph_store: &GraphStore);
//     fn on_graph_loaded(&self, graph: &Graph);
// }

// Extend configuration traits
// pub trait Node2VecBaseConfig {
//     fn walks_per_node(&self) -> u64;
//     fn walk_length(&self) -> u64;
// }

// pub trait GraphSageBaseConfig {
//     fn concurrency(&self) -> crate::concurrency::Concurrency;
//     fn node_label_identifiers(&self, _graph_store: &GraphStore) -> Vec<String> {
//         vec![]
//     }
//     fn internal_relationship_types(&self, _graph_store: &GraphStore) -> Vec<String> {
//         vec![]
//     }
// }

// pub trait GraphSageTrainConfig {
//     fn concurrency(&self) -> crate::concurrency::Concurrency;
// }

// Placeholder types
// #[derive(Clone)]
// pub struct ModelData;

// impl GraphSageTrainConfig {
//     pub fn graph_store_validation(
//         &self,
//         _graph_store: &GraphStore,
//         _node_label_identifiers: Vec<String>,
//         _internal_relationship_types: Vec<String>,
//     ) {
//         // TODO: Implement graph store validation
//     }
// }

// #[derive(Clone)]
// pub struct GraphSageModelTrainer;

// impl GraphSageModelTrainer {
//     // Placeholder implementation
// }

// Placeholder for EmbeddingUtils
pub struct EmbeddingUtils;

impl EmbeddingUtils {
    pub fn validate_relationship_weight_property_value(
        _graph: &Graph,
        _concurrency: crate::concurrency::Concurrency,
        _executor: DefaultPool,
    ) {
        // TODO: Implement relationship weight property validation
    }
}
