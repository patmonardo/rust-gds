use crate::api::{Graph, GraphStore, ResultStore};
use crate::applications::algorithms::machinery::{WriteStep, WriteToDatabase, AlgorithmLabel};
use crate::applications::algorithms::metadata::NodePropertiesWritten;
use crate::applications::algorithms::node_embeddings::GraphSageModelCatalog;
use crate::core::utils::progress::JobId;
// use crate::embeddings::{
//     fastrp::FastRPResult,
//     graphsage::GraphSageResult,
//     node2vec::Node2VecResult,
// };
use crate::config::base_types::Config;

// Placeholder result types
pub struct FastRPResult;
pub struct GraphSageResult;
pub struct Node2VecResult;

/// FastRP write step implementation
pub struct FastRPWriteStep<C: Config> {
    write_to_database: Box<dyn WriteToDatabase>,
    configuration: C,
}

impl<C: Config> FastRPWriteStep<C> {
    pub fn new(write_to_database: Box<dyn WriteToDatabase>, configuration: C) -> Self {
        Self {
            write_to_database,
            configuration,
        }
    }
}

impl<C: Config> WriteStep<FastRPResult, NodePropertiesWritten> for FastRPWriteStep<C> {
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &GraphStore,
        result_store: &mut ResultStore,
        result: FastRPResult,
        job_id: JobId,
    ) -> NodePropertiesWritten {
        let node_property_values = NodePropertyValuesAdapter::adapt(result.embeddings());

        self.write_to_database.perform(
            graph,
            graph_store,
            result_store,
            &self.configuration,
            &self.configuration,
            AlgorithmLabel::FastRP,
            job_id,
            node_property_values,
        )
    }
}

/// GraphSage write step implementation
pub struct GraphSageWriteStep<C: Config> {
    write_to_database: Box<dyn WriteToDatabase>,
    configuration: C,
}

impl<C: Config> GraphSageWriteStep<C> {
    pub fn new(write_to_database: Box<dyn WriteToDatabase>, configuration: C) -> Self {
        Self {
            write_to_database,
            configuration,
        }
    }
}

impl<C: Config> WriteStep<GraphSageResult, NodePropertiesWritten> for GraphSageWriteStep<C> {
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &GraphStore,
        result_store: &mut ResultStore,
        result: GraphSageResult,
        job_id: JobId,
    ) -> NodePropertiesWritten {
        let node_property_values = NodePropertyValuesAdapter::adapt(result.embeddings());

        self.write_to_database.perform(
            graph,
            graph_store,
            result_store,
            &self.configuration,
            &self.configuration,
            AlgorithmLabel::GraphSage,
            job_id,
            node_property_values,
        )
    }
}

/// Node2Vec write step implementation
pub struct Node2VecWriteStep<C: Config> {
    write_to_database: Box<dyn WriteToDatabase>,
    configuration: C,
}

impl<C: Config> Node2VecWriteStep<C> {
    pub fn new(write_to_database: Box<dyn WriteToDatabase>, configuration: C) -> Self {
        Self {
            write_to_database,
            configuration,
        }
    }
}

impl<C: Config> WriteStep<Node2VecResult, NodePropertiesWritten> for Node2VecWriteStep<C> {
    fn execute(
        &self,
        graph: &Graph,
        graph_store: &GraphStore,
        result_store: &mut ResultStore,
        result: Node2VecResult,
        job_id: JobId,
    ) -> NodePropertiesWritten {
        let node_property_values = FloatEmbeddingNodePropertyValues::new(result.embeddings());

        self.write_to_database.perform(
            graph,
            graph_store,
            result_store,
            &self.configuration,
            &self.configuration,
            AlgorithmLabel::Node2Vec,
            job_id,
            Box::new(node_property_values),
        )
    }
}

/// GraphSage training write-to-disk step implementation
pub struct GraphSageTrainWriteToDiskStep<C: Config> {
    graph_sage_model_catalog: GraphSageModelCatalog,
    model_repository: ModelRepository,
    configuration: C,
}

impl<C: Config> GraphSageTrainWriteToDiskStep<C> {
    pub fn new(
        graph_sage_model_catalog: GraphSageModelCatalog,
        model_repository: ModelRepository,
        configuration: C,
    ) -> Self {
        Self {
            graph_sage_model_catalog,
            model_repository,
            configuration,
        }
    }
}

impl<C: Config> WriteStep<Model<super::node_embeddings_algorithms::ModelDataImpl, super::node_embeddings_algorithms::GraphSageTrainConfigImpl, super::node_embeddings_algorithms::GraphSageTrainMetrics>, ()> for GraphSageTrainWriteToDiskStep<C> {
    fn execute(
        &self,
        _graph: &Graph,
        _graph_store: &GraphStore,
        _result_store: &mut ResultStore,
        result: Model<super::node_embeddings_algorithms::ModelDataImpl, super::node_embeddings_algorithms::GraphSageTrainConfigImpl, super::node_embeddings_algorithms::GraphSageTrainMetrics>,
        _job_id: JobId,
    ) -> () {
        self.graph_sage_model_catalog.store(result.clone());

        if self.configuration.store_model_to_disk() {
            self.model_repository.store(result);
        }
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

#[derive(Clone)]
pub struct ModelRepository;

impl ModelRepository {
    pub fn store<M>(&self, _model: M) {
        // TODO: Implement model repository storage
    }
}

#[derive(Clone)]
pub struct ModelData;

#[derive(Clone)]
pub struct GraphSageTrainConfig;

impl GraphSageTrainConfig {
    pub fn store_model_to_disk(&self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct GraphSageModelTrainer;

impl GraphSageModelTrainer {
    // Placeholder implementation
}

#[derive(Clone)]
pub struct Model<D, TC, M> {
    _data: D,
    _train_config: TC,
    _metrics: M,
}

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

impl Node2VecResult {
    pub fn embeddings(&self) -> Vec<f32> {
        vec![]
    }
}
