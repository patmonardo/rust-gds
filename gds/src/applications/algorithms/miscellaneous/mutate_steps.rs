use crate::api::{Graph, GraphStore};
use crate::applications::algorithms::machinery::{MutateNodeProperty, MutateStep};
use crate::applications::algorithms::metadata::{NodePropertiesWritten, RelationshipsWritten};
use crate::core::loading::SingleTypeRelationships;
use crate::config::base_types::Config;
use std::collections::HashMap;

/// Collapse Path mutate step implementation
pub struct CollapsePathMutateStep;

impl MutateStep<SingleTypeRelationships, ()> for CollapsePathMutateStep {
    fn execute(
        &self,
        _graph: Graph,
        graph_store: &GraphStore,
        result: SingleTypeRelationships,
    ) -> () {
        graph_store.add_relationship_type(result);
    }
}

/// Index Inverse mutate step implementation
pub struct IndexInverseMutateStep;

impl MutateStep<HashMap<String, SingleTypeRelationships>, ()> for IndexInverseMutateStep {
    fn execute(
        &self,
        _graph: Graph,
        graph_store: &GraphStore,
        result: HashMap<String, SingleTypeRelationships>,
    ) -> () {
        for (relationship_type, inverse_relationships) in result {
            graph_store.add_inverse_index(
                relationship_type,
                inverse_relationships.topology(),
                inverse_relationships.properties()
            );
        }
    }
}

/// Scale Properties mutate step implementation
pub struct ScalePropertiesMutateStep<C: Config> {
    mutate_node_property: Box<dyn MutateNodeProperty>,
    configuration: C,
}

impl<C: Config> ScalePropertiesMutateStep<C> {
    pub fn new(mutate_node_property: Box<dyn MutateNodeProperty>, configuration: C) -> Self {
        Self {
            mutate_node_property,
            configuration,
        }
    }
}

impl<C: Config> MutateStep<crate::applications::algorithms::miscellaneous::ScalePropertiesResult, NodePropertiesWritten> for ScalePropertiesMutateStep<C> {
    fn execute(
        &self,
        graph: Graph,
        graph_store: &GraphStore,
        result: crate::applications::algorithms::miscellaneous::ScalePropertiesResult,
    ) -> NodePropertiesWritten {
        // Create scaled properties node property values
        let node_property_values = ScaledPropertiesNodePropertyValues::new(
            graph.node_count(),
            result.scaled_properties()
        );

        self.mutate_node_property.mutate_node_properties(
            graph,
            graph_store,
            &self.configuration,
            node_property_values
        )
    }
}

/// To Undirected mutate step implementation
pub struct ToUndirectedMutateStep;

impl MutateStep<SingleTypeRelationships, RelationshipsWritten> for ToUndirectedMutateStep {
    fn execute(
        &self,
        _graph: Graph,
        graph_store: &GraphStore,
        result: SingleTypeRelationships,
    ) -> RelationshipsWritten {
        graph_store.add_relationship_type(result.clone());
        RelationshipsWritten::new(result.topology().element_count())
    }
}

// Placeholder for scaled properties node property values
pub struct ScaledPropertiesNodePropertyValues {
    _node_count: u64,
    _scaled_properties: Vec<f64>,
}

impl ScaledPropertiesNodePropertyValues {
    pub fn new(node_count: u64, scaled_properties: Vec<f64>) -> Self {
        Self {
            _node_count: node_count,
            _scaled_properties: scaled_properties,
        }
    }
}
