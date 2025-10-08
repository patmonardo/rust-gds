use crate::types::schema::{NodeLabel, PropertySchema, SchemaError, SchemaResult};
use crate::types::ValueType;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Schema entry for a node label.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeSchemaEntry {
    identifier: NodeLabel,
    properties: HashMap<String, PropertySchema>,
}

impl NodeSchemaEntry {
    pub fn new(identifier: NodeLabel, properties: HashMap<String, PropertySchema>) -> Self {
        Self {
            identifier,
            properties,
        }
    }

    pub fn empty(identifier: NodeLabel) -> Self {
        Self {
            identifier,
            properties: HashMap::new(),
        }
    }

    pub fn identifier(&self) -> &NodeLabel {
        &self.identifier
    }

    pub fn properties(&self) -> &HashMap<String, PropertySchema> {
        &self.properties
    }

    /// Creates a union of this entry with another entry.
    pub fn union(&self, other: &NodeSchemaEntry) -> SchemaResult<NodeSchemaEntry> {
        if self.identifier != other.identifier {
            return Err(SchemaError::IdentifierMismatch {
                left: self.identifier.name().to_string(),
                right: other.identifier.name().to_string(),
            });
        }

        let union_properties = union_properties(&self.properties, &other.properties)?;
        Ok(NodeSchemaEntry::new(
            self.identifier.clone(),
            union_properties,
        ))
    }
}

/// Mutable node schema entry that allows adding and removing properties.
#[derive(Clone, Debug)]
pub struct MutableNodeSchemaEntry {
    identifier: NodeLabel,
    properties: HashMap<String, PropertySchema>,
}

impl MutableNodeSchemaEntry {
    pub fn new(identifier: NodeLabel) -> Self {
        Self {
            identifier,
            properties: HashMap::new(),
        }
    }

    pub fn from_entry(entry: &NodeSchemaEntry) -> Self {
        Self {
            identifier: entry.identifier.clone(),
            properties: entry.properties.clone(),
        }
    }

    pub fn identifier(&self) -> &NodeLabel {
        &self.identifier
    }

    pub fn properties(&self) -> &HashMap<String, PropertySchema> {
        &self.properties
    }

    /// Adds a property with the specified name and value type.
    pub fn add_property(&mut self, key: impl Into<String>, value_type: ValueType) -> &mut Self {
        let key = key.into();
        self.properties
            .insert(key.clone(), PropertySchema::of(key, value_type));
        self
    }

    /// Adds a property with a schema.
    pub fn add_property_schema(&mut self, schema: PropertySchema) -> &mut Self {
        self.properties.insert(schema.key().to_string(), schema);
        self
    }

    /// Removes a property.
    pub fn remove_property(&mut self, key: &str) {
        self.properties.remove(key);
    }

    /// Converts to an immutable entry.
    pub fn build(self) -> NodeSchemaEntry {
        NodeSchemaEntry::new(self.identifier, self.properties)
    }

    /// Creates a union with another entry.
    pub fn union(&self, other: &NodeSchemaEntry) -> SchemaResult<MutableNodeSchemaEntry> {
        if self.identifier != other.identifier {
            return Err(SchemaError::IdentifierMismatch {
                left: self.identifier.name().to_string(),
                right: other.identifier.name().to_string(),
            });
        }

        let union_properties = union_properties(&self.properties, &other.properties)?;
        Ok(MutableNodeSchemaEntry {
            identifier: self.identifier.clone(),
            properties: union_properties,
        })
    }
}

/// Schema for nodes in a graph.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeSchema {
    entries: HashMap<NodeLabel, NodeSchemaEntry>,
}

impl NodeSchema {
    pub fn new(entries: HashMap<NodeLabel, NodeSchemaEntry>) -> Self {
        Self { entries }
    }

    pub fn empty() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn entries(&self) -> Vec<&NodeSchemaEntry> {
        self.entries.values().collect()
    }

    pub fn get(&self, label: &NodeLabel) -> Option<&NodeSchemaEntry> {
        self.entries.get(label)
    }

    pub fn available_labels(&self) -> HashSet<NodeLabel> {
        self.entries.keys().cloned().collect()
    }

    pub fn contains_only_all_nodes_label(&self) -> bool {
        self.entries.len() == 1 && self.entries.contains_key(&NodeLabel::all_nodes())
    }

    /// Filters the schema to only include specified labels.
    pub fn filter(&self, labels_to_keep: &HashSet<NodeLabel>) -> NodeSchema {
        let filtered: HashMap<_, _> = self
            .entries
            .iter()
            .filter(|(label, _)| labels_to_keep.contains(label))
            .map(|(label, entry)| (label.clone(), entry.clone()))
            .collect();
        NodeSchema::new(filtered)
    }

    /// Creates a union of this schema with another.
    pub fn union(&self, other: &NodeSchema) -> SchemaResult<NodeSchema> {
        let mut result = self.entries.clone();

        for (label, other_entry) in &other.entries {
            if let Some(existing) = result.get(label) {
                let merged = existing.union(other_entry)?;
                result.insert(label.clone(), merged);
            } else {
                result.insert(label.clone(), other_entry.clone());
            }
        }

        Ok(NodeSchema::new(result))
    }

    /// Returns all property keys across all labels.
    pub fn all_properties(&self) -> HashSet<String> {
        let mut props = HashSet::new();
        for entry in self.entries.values() {
            props.extend(entry.properties.keys().cloned());
        }
        props
    }

    /// Returns property keys for a specific label.
    pub fn properties_for_label(&self, label: &NodeLabel) -> HashSet<String> {
        self.entries
            .get(label)
            .map(|entry| entry.properties.keys().cloned().collect())
            .unwrap_or_default()
    }
}

/// Mutable node schema that allows adding and removing labels and properties.
#[derive(Clone, Debug)]
pub struct MutableNodeSchema {
    entries: HashMap<NodeLabel, MutableNodeSchemaEntry>,
}

impl MutableNodeSchema {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self::new()
    }

    pub fn from_schema(schema: &NodeSchema) -> Self {
        let entries = schema
            .entries
            .iter()
            .map(|(label, entry)| (label.clone(), MutableNodeSchemaEntry::from_entry(entry)))
            .collect();
        Self { entries }
    }

    pub fn entries(&self) -> Vec<&MutableNodeSchemaEntry> {
        self.entries.values().collect()
    }

    pub fn get(&self, label: &NodeLabel) -> Option<&MutableNodeSchemaEntry> {
        self.entries.get(label)
    }

    pub fn available_labels(&self) -> HashSet<NodeLabel> {
        self.entries.keys().cloned().collect()
    }

    /// Gets or creates a label entry.
    pub fn get_or_create_label(&mut self, label: NodeLabel) -> &mut MutableNodeSchemaEntry {
        self.entries
            .entry(label.clone())
            .or_insert_with(|| MutableNodeSchemaEntry::new(label))
    }

    /// Adds a label to the schema.
    pub fn add_label(&mut self, label: NodeLabel) -> &mut Self {
        self.get_or_create_label(label);
        self
    }

    /// Adds a property to a label.
    pub fn add_property(
        &mut self,
        label: NodeLabel,
        key: impl Into<String>,
        value_type: ValueType,
    ) -> &mut Self {
        self.get_or_create_label(label)
            .add_property(key, value_type);
        self
    }

    /// Removes a label.
    pub fn remove_label(&mut self, label: &NodeLabel) {
        self.entries.remove(label);
    }

    /// Converts to an immutable schema.
    pub fn build(self) -> NodeSchema {
        let entries = self
            .entries
            .into_iter()
            .map(|(label, entry)| (label, entry.build()))
            .collect();
        NodeSchema::new(entries)
    }

    /// Filters the schema to only include specified labels.
    pub fn filter(&self, labels_to_keep: &HashSet<NodeLabel>) -> MutableNodeSchema {
        let filtered: HashMap<_, _> = self
            .entries
            .iter()
            .filter(|(label, _)| labels_to_keep.contains(label))
            .map(|(label, entry)| (label.clone(), entry.clone()))
            .collect();
        MutableNodeSchema { entries: filtered }
    }

    /// Creates a union of this schema with another.
    pub fn union(&self, other: &NodeSchema) -> SchemaResult<MutableNodeSchema> {
        let mut result = self.entries.clone();

        for (label, other_entry) in &other.entries {
            if let Some(existing) = result.get_mut(label) {
                let merged = existing.union(other_entry)?;
                result.insert(label.clone(), merged);
            } else {
                result.insert(
                    label.clone(),
                    MutableNodeSchemaEntry::from_entry(other_entry),
                );
            }
        }

        Ok(MutableNodeSchema { entries: result })
    }
}

impl Default for MutableNodeSchema {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to union two property maps, checking for conflicts.
fn union_properties(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_schema_entry() {
        let label = NodeLabel::of("Person");
        let mut entry = MutableNodeSchemaEntry::new(label.clone());

        entry.add_property("name", ValueType::String);
        entry.add_property("age", ValueType::Long);

        assert_eq!(entry.properties().len(), 2);
        assert!(entry.properties().contains_key("name"));
        assert!(entry.properties().contains_key("age"));
    }

    #[test]
    fn test_node_schema_union() {
        let label = NodeLabel::of("Person");

        let mut schema1 = MutableNodeSchema::new();
        schema1.add_property(label.clone(), "name", ValueType::String);

        let mut schema2 = MutableNodeSchema::new();
        schema2.add_property(label.clone(), "age", ValueType::Long);

        let union = schema1.union(&schema2.build()).unwrap();
        let entry = union.get(&label).unwrap();

        assert_eq!(entry.properties().len(), 2);
        assert!(entry.properties().contains_key("name"));
        assert!(entry.properties().contains_key("age"));
    }

    #[test]
    fn test_node_schema_union_conflict() {
        let label = NodeLabel::of("Person");

        let mut schema1 = MutableNodeSchema::new();
        schema1.add_property(label.clone(), "prop", ValueType::String);

        let mut schema2 = MutableNodeSchema::new();
        schema2.add_property(label.clone(), "prop", ValueType::Long);

        let result = schema1.union(&schema2.build());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchemaError::PropertyTypeConflict { .. }
        ));
    }

    #[test]
    fn test_filter() {
        let mut schema = MutableNodeSchema::new();
        schema.add_label(NodeLabel::of("Person"));
        schema.add_label(NodeLabel::of("Company"));

        let mut keep = HashSet::new();
        keep.insert(NodeLabel::of("Person"));

        let filtered = schema.filter(&keep);
        assert_eq!(filtered.available_labels().len(), 1);
        assert!(filtered
            .available_labels()
            .contains(&NodeLabel::of("Person")));
    }
}
