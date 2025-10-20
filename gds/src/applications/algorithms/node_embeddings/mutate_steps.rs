use crate::api::{Graph, GraphStore};
use crate::applications::algorithms::machinery::{MutateNodeProperty, MutateStep};
use crate::applications::algorithms::metadata::NodePropertiesWritten;
// use crate::embeddings::{
//     fastrp::FastRPResult,
//     graphsage::GraphSageResult,
//     hashgnn::HashGNNResult,
//     node2vec::Node2VecResult,
// };
use crate::config::base_types::Config;

// Placeholder result types
pub struct FastRPResult;
pub struct GraphSageResult;
pub struct HashGNNResult;
pub struct Node2VecResult;

/// FastRP mutate step implementation
pub struct FastRPMutateStep<C: Config> {
    mutate_node_property: Box<dyn MutateNodeProperty>,
    configuration: C,
}

impl<C: Config> FastRPMutateStep<C> {
    pub fn new(mutate_node_property: Box<dyn MutateNodeProperty>, configuration: C) -> Self {
        Self {
            mutate_node_property,
            configuration,
        }
    }
}

impl<C: Config> MutateStep<FastRPResult, NodePropertiesWritten> for FastRPMutateStep<C> {
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &mut GraphStore,
        result: FastRPResult,
    ) -> NodePropertiesWritten {
        let node_property_values = NodePropertyValuesAdapter::adapt(result.embeddings());

        self.mutate_node_property.mutate_node_properties(
            graph,
            graph_store,
            &self.configuration,
            node_property_values,
        )
    }
}

/// GraphSage mutate step implementation
pub struct GraphSageMutateStep<C: Config> {
    mutate_node_property: Box<dyn MutateNodeProperty>,
    configuration: C,
}

impl<C: Config> GraphSageMutateStep<C> {
    pub fn new(mutate_node_property: Box<dyn MutateNodeProperty>, configuration: C) -> Self {
        Self {
            mutate_node_property,
            configuration,
        }
    }
}

impl<C: Config> MutateStep<GraphSageResult, NodePropertiesWritten> for GraphSageMutateStep<C> {
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &mut GraphStore,
        result: GraphSageResult,
    ) -> NodePropertiesWritten {
        let node_property_values = NodePropertyValuesAdapter::adapt(result.embeddings());

        self.mutate_node_property.mutate_node_properties(
            graph,
            graph_store,
            &self.configuration,
            node_property_values,
        )
    }
}

/// HashGNN mutate step implementation
pub struct HashGnnMutateStep<C: Config> {
    mutate_node_property: Box<dyn MutateNodeProperty>,
    configuration: C,
}

impl<C: Config> HashGnnMutateStep<C> {
    pub fn new(mutate_node_property: Box<dyn MutateNodeProperty>, configuration: C) -> Self {
        Self {
            mutate_node_property,
            configuration,
        }
    }
}

impl<C: Config> MutateStep<HashGNNResult, NodePropertiesWritten> for HashGnnMutateStep<C> {
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &mut GraphStore,
        result: HashGNNResult,
    ) -> NodePropertiesWritten {
        let node_property_values = result.embeddings();

        self.mutate_node_property.mutate_node_properties(
            graph,
            graph_store,
            &self.configuration,
            node_property_values,
        )
    }
}

/// Node2Vec mutate step implementation
pub struct Node2VecMutateStep<C: Config> {
    mutate_node_property: Box<dyn MutateNodeProperty>,
    configuration: C,
}

impl<C: Config> Node2VecMutateStep<C> {
    pub fn new(mutate_node_property: Box<dyn MutateNodeProperty>, configuration: C) -> Self {
        Self {
            mutate_node_property,
            configuration,
        }
    }
}

impl<C: Config> MutateStep<Node2VecResult, NodePropertiesWritten> for Node2VecMutateStep<C> {
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &mut GraphStore,
        result: Node2VecResult,
    ) -> NodePropertiesWritten {
        let node_property_values = FloatEmbeddingNodePropertyValues::new(result.embeddings());

        self.mutate_node_property.mutate_node_properties(
            graph,
            graph_store,
            &self.configuration,
            Box::new(node_property_values),
        )
    }
}

// Placeholder types
pub struct NodePropertyValuesAdapter;

impl NodePropertyValuesAdapter {
    pub fn adapt(_embeddings: Embeddings) -> Box<dyn NodePropertyValues> {
        Box::new(FloatEmbeddingNodePropertyValues::new(vec![]))
    }
}

pub struct Embeddings;

pub trait NodePropertyValues {}

#[derive(Clone)]
pub struct FloatEmbeddingNodePropertyValues {
    embeddings: Vec<f32>,
}

impl FloatEmbeddingNodePropertyValues {
    pub fn new(embeddings: Vec<f32>) -> Self {
        Self { embeddings }
    }
}

impl NodePropertyValues for FloatEmbeddingNodePropertyValues {}

// Extend result types to have embeddings method
impl FastRPResult {
    pub fn embeddings(&self) -> Embeddings {
        Embeddings
    }
}

impl GraphSageResult {
    pub fn embeddings(&self) -> Embeddings {
        Embeddings
    }
}

impl HashGNNResult {
    pub fn embeddings(&self) -> Box<dyn NodePropertyValues> {
        Box::new(FloatEmbeddingNodePropertyValues::new(vec![]))
    }
}

impl Node2VecResult {
    pub fn embeddings(&self) -> Vec<f32> {
        vec![]
    }
}
