use crate::types::schema::{
    Direction, MutableNodeSchema, MutableRelationshipSchema, NodeLabel, NodeSchema, PropertySchema,
    PropertySchemaTrait, RelationshipSchema, RelationshipType, SchemaError, SchemaResult,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Schema representation for a graph, including node and relationship schemas.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphSchema {
    node_schema: NodeSchema,
    relationship_schema: RelationshipSchema,
    graph_properties: HashMap<String, PropertySchema>,
}

impl GraphSchema {
    pub fn new(
        node_schema: NodeSchema,
        relationship_schema: RelationshipSchema,
        graph_properties: HashMap<String, PropertySchema>,
    ) -> Self {
        Self {
            node_schema,
            relationship_schema,
            graph_properties,
        }
    }

    pub fn empty() -> Self {
        Self {
            node_schema: NodeSchema::empty(),
            relationship_schema: RelationshipSchema::empty(),
            graph_properties: HashMap::new(),
        }
    }

    pub fn node_schema(&self) -> &NodeSchema {
        &self.node_schema
    }

    pub fn relationship_schema(&self) -> &RelationshipSchema {
        &self.relationship_schema
    }

    pub fn graph_properties(&self) -> &HashMap<String, PropertySchema> {
        &self.graph_properties
    }

    /// Creates a filtered version of this schema containing only the specified node labels.
    pub fn filter_node_labels(&self, labels_to_keep: &HashSet<NodeLabel>) -> GraphSchema {
        GraphSchema {
            node_schema: self.node_schema.filter(labels_to_keep),
            relationship_schema: self.relationship_schema.clone(),
            graph_properties: self.graph_properties.clone(),
        }
    }

    /// Creates a filtered version of this schema containing only the specified relationship types.
    pub fn filter_relationship_types(
        &self,
        types_to_keep: &HashSet<RelationshipType>,
    ) -> GraphSchema {
        GraphSchema {
            node_schema: self.node_schema.clone(),
            relationship_schema: self.relationship_schema.filter(types_to_keep),
            graph_properties: self.graph_properties.clone(),
        }
    }

    /// Combines this schema with another schema.
    pub fn union(&self, other: &GraphSchema) -> SchemaResult<GraphSchema> {
        let node_schema = self.node_schema.union(&other.node_schema)?;
        let relationship_schema = self.relationship_schema.union(&other.relationship_schema)?;
        let graph_properties =
            merge_graph_properties(&self.graph_properties, &other.graph_properties)?;

        Ok(GraphSchema {
            node_schema,
            relationship_schema,
            graph_properties,
        })
    }

    /// Checks if the graph has undirected relationships.
    pub fn is_undirected(&self) -> bool {
        self.relationship_schema.is_undirected()
    }

    /// Returns the direction of relationships in this graph.
    pub fn direction(&self) -> Direction {
        if self.is_undirected() {
            Direction::Undirected
        } else {
            Direction::Directed
        }
    }

    /// Converts the schema to a JSON-friendly map representation.
    pub fn to_map(&self) -> serde_json::Value {
        serde_json::json!({
            "nodes": node_schema_to_map(&self.node_schema),
            "relationships": relationship_schema_to_map(&self.relationship_schema),
            "graphProperties": properties_to_map(&self.graph_properties),
        })
    }
}

/// Mutable graph schema that supports building and modifying graph schemas.
#[derive(Clone, Debug)]
pub struct MutableGraphSchema {
    node_schema: MutableNodeSchema,
    relationship_schema: MutableRelationshipSchema,
    graph_properties: HashMap<String, PropertySchema>,
}

impl MutableGraphSchema {
    pub fn new(
        node_schema: MutableNodeSchema,
        relationship_schema: MutableRelationshipSchema,
        graph_properties: HashMap<String, PropertySchema>,
    ) -> Self {
        Self {
            node_schema,
            relationship_schema,
            graph_properties,
        }
    }

    pub fn empty() -> Self {
        Self {
            node_schema: MutableNodeSchema::empty(),
            relationship_schema: MutableRelationshipSchema::empty(),
            graph_properties: HashMap::new(),
        }
    }

    pub fn from_schema(schema: &GraphSchema) -> Self {
        Self {
            node_schema: MutableNodeSchema::from_schema(&schema.node_schema),
            relationship_schema: MutableRelationshipSchema::from_schema(
                &schema.relationship_schema,
            ),
            graph_properties: schema.graph_properties.clone(),
        }
    }

    pub fn node_schema(&self) -> &MutableNodeSchema {
        &self.node_schema
    }

    pub fn node_schema_mut(&mut self) -> &mut MutableNodeSchema {
        &mut self.node_schema
    }

    pub fn relationship_schema(&self) -> &MutableRelationshipSchema {
        &self.relationship_schema
    }

    pub fn relationship_schema_mut(&mut self) -> &mut MutableRelationshipSchema {
        &mut self.relationship_schema
    }

    pub fn graph_properties(&self) -> &HashMap<String, PropertySchema> {
        &self.graph_properties
    }

    /// Adds a property to the graph schema.
    pub fn put_graph_property(
        &mut self,
        key: impl Into<String>,
        schema: PropertySchema,
    ) -> &mut Self {
        self.graph_properties.insert(key.into(), schema);
        self
    }

    /// Removes a property from the graph schema.
    pub fn remove_graph_property(&mut self, key: &str) -> &mut Self {
        self.graph_properties.remove(key);
        self
    }

    /// Creates a filtered version of this schema containing only the specified node labels.
    pub fn filter_node_labels(&self, labels_to_keep: &HashSet<NodeLabel>) -> MutableGraphSchema {
        MutableGraphSchema {
            node_schema: self.node_schema.filter(labels_to_keep),
            relationship_schema: self.relationship_schema.clone(),
            graph_properties: self.graph_properties.clone(),
        }
    }

    /// Creates a filtered version of this schema containing only the specified relationship types.
    pub fn filter_relationship_types(
        &self,
        types_to_keep: &HashSet<RelationshipType>,
    ) -> MutableGraphSchema {
        MutableGraphSchema {
            node_schema: self.node_schema.clone(),
            relationship_schema: self.relationship_schema.filter(types_to_keep),
            graph_properties: self.graph_properties.clone(),
        }
    }

    /// Combines this schema with another schema.
    pub fn union(&self, other: &GraphSchema) -> SchemaResult<MutableGraphSchema> {
        let node_schema = self.node_schema.union(&other.node_schema)?;
        let relationship_schema = self.relationship_schema.union(&other.relationship_schema)?;
        let graph_properties =
            merge_graph_properties(&self.graph_properties, &other.graph_properties)?;

        Ok(MutableGraphSchema {
            node_schema,
            relationship_schema,
            graph_properties,
        })
    }

    /// Converts to an immutable schema.
    pub fn build(self) -> GraphSchema {
        GraphSchema {
            node_schema: self.node_schema.build(),
            relationship_schema: self.relationship_schema.build(),
            graph_properties: self.graph_properties,
        }
    }
}

impl Default for MutableGraphSchema {
    fn default() -> Self {
        Self::empty()
    }
}

/// Helper to merge graph properties from two schemas.
fn merge_graph_properties(
    left: &HashMap<String, PropertySchema>,
    right: &HashMap<String, PropertySchema>,
) -> SchemaResult<HashMap<String, PropertySchema>> {
    let mut result = left.clone();

    for (key, right_schema) in right {
        if let Some(left_schema) = result.get(key) {
            if left_schema.value_type() != right_schema.value_type() {
                return Err(SchemaError::PropertyTypeConflict {
                    key: key.clone(),
                    left: left_schema.value_type(),
                    right: right_schema.value_type(),
                });
            }
            // Keep left schema if types match
        } else {
            result.insert(key.clone(), right_schema.clone());
        }
    }

    Ok(result)
}

/// Helper to convert node schema to a map.
fn node_schema_to_map(schema: &NodeSchema) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for entry in schema.entries() {
        map.insert(
            entry.identifier().name().to_string(),
            serde_json::json!({
                "properties": properties_to_map(entry.properties()),
            }),
        );
    }
    serde_json::Value::Object(map)
}

/// Helper to convert relationship schema to a map.
fn relationship_schema_to_map(schema: &RelationshipSchema) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for entry in schema.entries() {
        map.insert(
            entry.identifier().name().to_string(),
            serde_json::json!({
                "direction": entry.direction().to_string(),
                "properties": relationship_properties_to_map(entry.properties()),
            }),
        );
    }
    serde_json::Value::Object(map)
}

/// Helper to convert properties to a map.
fn properties_to_map(properties: &HashMap<String, PropertySchema>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (key, schema) in properties {
        map.insert(
            key.clone(),
            serde_json::json!({
                "valueType": format!("{:?}", schema.value_type()),
                "defaultValue": schema.default_value().to_string(),
                "state": format!("{:?}", schema.state()),
            }),
        );
    }
    serde_json::Value::Object(map)
}

/// Helper to convert relationship properties to a map.
fn relationship_properties_to_map(
    properties: &HashMap<String, crate::types::schema::RelationshipPropertySchema>,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (key, schema) in properties {
        map.insert(
            key.clone(),
            serde_json::json!({
                "valueType": format!("{:?}", schema.value_type()),
                "defaultValue": schema.default_value().to_string(),
                "state": format!("{:?}", schema.state()),
                "aggregation": format!("{:?}", schema.aggregation()),
            }),
        );
    }
    serde_json::Value::Object(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::property::ValueType;

    #[test]
    fn test_empty_graph_schema() {
        let schema = GraphSchema::empty();
        assert!(schema.node_schema().entries().is_empty());
        assert!(schema.relationship_schema().entries().is_empty());
        assert!(schema.graph_properties().is_empty());
    }

    #[test]
    fn test_graph_schema_union() {
        let mut schema1 = MutableGraphSchema::empty();
        schema1
            .node_schema_mut()
            .add_property(NodeLabel::new("Person"), "name", ValueType::String);

        let mut schema2 = MutableGraphSchema::empty();
        schema2
            .node_schema_mut()
            .add_property(NodeLabel::new("Person"), "age", ValueType::Long);

        let union = schema1.union(&schema2.build()).unwrap();
        let person_entry = union.node_schema().get(&NodeLabel::new("Person")).unwrap();

        assert_eq!(person_entry.properties().len(), 2);
    }

    #[test]
    fn test_filter_node_labels() {
        let mut schema = MutableGraphSchema::empty();
        schema.node_schema_mut().add_label(NodeLabel::new("Person"));
        schema
            .node_schema_mut()
            .add_label(NodeLabel::new("Company"));

        let mut keep = HashSet::new();
        keep.insert(NodeLabel::new("Person"));

        let filtered = schema.filter_node_labels(&keep);
        assert_eq!(filtered.node_schema().available_labels().len(), 1);
    }

    #[test]
    fn test_graph_properties() {
        let mut schema = MutableGraphSchema::empty();
        schema.put_graph_property("version", PropertySchema::of("version", ValueType::Long));

        assert!(schema.graph_properties().contains_key("version"));
    }

    #[test]
    fn test_to_map() {
        let mut schema = MutableGraphSchema::empty();
        schema
            .node_schema_mut()
            .add_property(NodeLabel::new("Person"), "name", ValueType::String);

        let built = schema.build();
        let map = built.to_map();

        assert!(map.is_object());
        assert!(map["nodes"].is_object());
        assert!(map["relationships"].is_object());
    }
}
