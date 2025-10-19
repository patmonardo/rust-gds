use crate::types::schema::{
    Direction, PropertySchemaTrait, RelationshipPropertySchema, RelationshipType, SchemaError,
    SchemaResult,
};
use crate::types::ValueType;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Schema entry for a relationship type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipSchemaEntry {
    identifier: RelationshipType,
    direction: Direction,
    properties: HashMap<String, RelationshipPropertySchema>,
}

impl RelationshipSchemaEntry {
    pub fn new(
        identifier: RelationshipType,
        direction: Direction,
        properties: HashMap<String, RelationshipPropertySchema>,
    ) -> Self {
        Self {
            identifier,
            direction,
            properties,
        }
    }

    pub fn empty(identifier: RelationshipType, direction: Direction) -> Self {
        Self {
            identifier,
            direction,
            properties: HashMap::new(),
        }
    }

    pub fn identifier(&self) -> &RelationshipType {
        &self.identifier
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn is_undirected(&self) -> bool {
        self.direction.is_undirected()
    }

    pub fn properties(&self) -> &HashMap<String, RelationshipPropertySchema> {
        &self.properties
    }

    /// Creates a union of this entry with another entry.
    pub fn union(&self, other: &RelationshipSchemaEntry) -> SchemaResult<RelationshipSchemaEntry> {
        if self.identifier != other.identifier {
            return Err(SchemaError::IdentifierMismatch {
                left: self.identifier.name().to_string(),
                right: other.identifier.name().to_string(),
            });
        }

        if self.direction != other.direction {
            return Err(SchemaError::DirectionalityConflict {
                relationship_type: self.identifier.name().to_string(),
            });
        }

        let union_properties = union_relationship_properties(&self.properties, &other.properties)?;
        Ok(RelationshipSchemaEntry::new(
            self.identifier.clone(),
            self.direction,
            union_properties,
        ))
    }
}

/// Mutable relationship schema entry.
#[derive(Clone, Debug)]
pub struct MutableRelationshipSchemaEntry {
    identifier: RelationshipType,
    direction: Direction,
    properties: HashMap<String, RelationshipPropertySchema>,
}

impl MutableRelationshipSchemaEntry {
    pub fn new(identifier: RelationshipType, direction: Direction) -> Self {
        Self {
            identifier,
            direction,
            properties: HashMap::new(),
        }
    }

    pub fn from_entry(entry: &RelationshipSchemaEntry) -> Self {
        Self {
            identifier: entry.identifier.clone(),
            direction: entry.direction,
            properties: entry.properties.clone(),
        }
    }

    pub fn identifier(&self) -> &RelationshipType {
        &self.identifier
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn is_undirected(&self) -> bool {
        self.direction.is_undirected()
    }

    pub fn properties(&self) -> &HashMap<String, RelationshipPropertySchema> {
        &self.properties
    }

    /// Adds a property with the specified name and value type.
    pub fn add_property(&mut self, key: impl Into<String>, value_type: ValueType) -> &mut Self {
        let key = key.into();
        self.properties
            .insert(key.clone(), RelationshipPropertySchema::of(key, value_type));
        self
    }

    /// Adds a property with a schema.
    pub fn add_property_schema(&mut self, schema: RelationshipPropertySchema) -> &mut Self {
        self.properties.insert(schema.key().to_string(), schema);
        self
    }

    /// Removes a property.
    pub fn remove_property(&mut self, key: &str) {
        self.properties.remove(key);
    }

    /// Converts to an immutable entry.
    pub fn build(self) -> RelationshipSchemaEntry {
        RelationshipSchemaEntry::new(self.identifier, self.direction, self.properties)
    }

    /// Creates a union with another entry.
    pub fn union(
        &self,
        other: &RelationshipSchemaEntry,
    ) -> SchemaResult<MutableRelationshipSchemaEntry> {
        if self.identifier != other.identifier {
            return Err(SchemaError::IdentifierMismatch {
                left: self.identifier.name().to_string(),
                right: other.identifier.name().to_string(),
            });
        }

        if self.direction != other.direction {
            return Err(SchemaError::DirectionalityConflict {
                relationship_type: self.identifier.name().to_string(),
            });
        }

        let union_properties = union_relationship_properties(&self.properties, &other.properties)?;
        Ok(MutableRelationshipSchemaEntry {
            identifier: self.identifier.clone(),
            direction: self.direction,
            properties: union_properties,
        })
    }
}

/// Schema for relationships in a graph.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelationshipSchema {
    entries: HashMap<RelationshipType, RelationshipSchemaEntry>,
}

impl RelationshipSchema {
    pub fn new(entries: HashMap<RelationshipType, RelationshipSchemaEntry>) -> Self {
        Self { entries }
    }

    pub fn empty() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn entries(&self) -> Vec<&RelationshipSchemaEntry> {
        self.entries.values().collect()
    }

    pub fn get(&self, rel_type: &RelationshipType) -> Option<&RelationshipSchemaEntry> {
        self.entries.get(rel_type)
    }

    pub fn available_types(&self) -> HashSet<RelationshipType> {
        self.entries.keys().cloned().collect()
    }

    pub fn is_undirected(&self) -> bool {
        self.entries.values().all(|entry| entry.is_undirected())
    }

    pub fn is_undirected_for_type(&self, rel_type: &RelationshipType) -> bool {
        self.entries
            .get(rel_type)
            .map(|entry| entry.is_undirected())
            .unwrap_or(false)
    }

    /// Returns a map of relationship types to their directions.
    pub fn directions(&self) -> HashMap<RelationshipType, Direction> {
        self.entries
            .iter()
            .map(|(rel_type, entry)| (rel_type.clone(), entry.direction()))
            .collect()
    }

    /// Filters the schema to only include specified types.
    pub fn filter(&self, types_to_keep: &HashSet<RelationshipType>) -> RelationshipSchema {
        let filtered: HashMap<_, _> = self
            .entries
            .iter()
            .filter(|(rel_type, _)| types_to_keep.contains(rel_type))
            .map(|(rel_type, entry)| (rel_type.clone(), entry.clone()))
            .collect();
        RelationshipSchema::new(filtered)
    }

    /// Creates a union of this schema with another.
    pub fn union(&self, other: &RelationshipSchema) -> SchemaResult<RelationshipSchema> {
        let mut result = self.entries.clone();

        for (rel_type, other_entry) in &other.entries {
            if let Some(existing) = result.get(rel_type) {
                let merged = existing.union(other_entry)?;
                result.insert(rel_type.clone(), merged);
            } else {
                result.insert(rel_type.clone(), other_entry.clone());
            }
        }

        Ok(RelationshipSchema::new(result))
    }

    /// Returns all property keys across all relationship types.
    pub fn all_properties(&self) -> HashSet<String> {
        let mut props = HashSet::new();
        for entry in self.entries.values() {
            props.extend(entry.properties.keys().cloned());
        }
        props
    }
}

/// Mutable relationship schema.
#[derive(Clone, Debug)]
pub struct MutableRelationshipSchema {
    entries: HashMap<RelationshipType, MutableRelationshipSchemaEntry>,
}

impl MutableRelationshipSchema {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self::new()
    }

    pub fn from_schema(schema: &RelationshipSchema) -> Self {
        let entries = schema
            .entries
            .iter()
            .map(|(rel_type, entry)| {
                (
                    rel_type.clone(),
                    MutableRelationshipSchemaEntry::from_entry(entry),
                )
            })
            .collect();
        Self { entries }
    }

    pub fn entries(&self) -> Vec<&MutableRelationshipSchemaEntry> {
        self.entries.values().collect()
    }

    pub fn get(&self, rel_type: &RelationshipType) -> Option<&MutableRelationshipSchemaEntry> {
        self.entries.get(rel_type)
    }

    pub fn available_types(&self) -> HashSet<RelationshipType> {
        self.entries.keys().cloned().collect()
    }

    /// Gets or creates a relationship type entry.
    pub fn get_or_create_type(
        &mut self,
        rel_type: RelationshipType,
        direction: Direction,
    ) -> &mut MutableRelationshipSchemaEntry {
        self.entries
            .entry(rel_type.clone())
            .or_insert_with(|| MutableRelationshipSchemaEntry::new(rel_type, direction))
    }

    /// Adds a relationship type to the schema.
    pub fn add_relationship_type(
        &mut self,
        rel_type: RelationshipType,
        direction: Direction,
    ) -> &mut Self {
        self.get_or_create_type(rel_type, direction);
        self
    }

    /// Adds a property to a relationship type.
    pub fn add_property(
        &mut self,
        rel_type: RelationshipType,
        direction: Direction,
        key: impl Into<String>,
        value_type: ValueType,
    ) -> &mut Self {
        self.get_or_create_type(rel_type, direction)
            .add_property(key, value_type);
        self
    }

    /// Removes a relationship type.
    pub fn remove_type(&mut self, rel_type: &RelationshipType) {
        self.entries.remove(rel_type);
    }

    /// Converts to an immutable schema.
    pub fn build(self) -> RelationshipSchema {
        let entries = self
            .entries
            .into_iter()
            .map(|(rel_type, entry)| (rel_type, entry.build()))
            .collect();
        RelationshipSchema::new(entries)
    }

    /// Filters the schema to only include specified types.
    pub fn filter(&self, types_to_keep: &HashSet<RelationshipType>) -> MutableRelationshipSchema {
        let filtered: HashMap<_, _> = self
            .entries
            .iter()
            .filter(|(rel_type, _)| types_to_keep.contains(rel_type))
            .map(|(rel_type, entry)| (rel_type.clone(), entry.clone()))
            .collect();
        MutableRelationshipSchema { entries: filtered }
    }

    /// Creates a union of this schema with another.
    pub fn union(&self, other: &RelationshipSchema) -> SchemaResult<MutableRelationshipSchema> {
        let mut result = self.entries.clone();

        for (rel_type, other_entry) in &other.entries {
            if let Some(existing) = result.get_mut(rel_type) {
                let merged = existing.union(other_entry)?;
                result.insert(rel_type.clone(), merged);
            } else {
                result.insert(
                    rel_type.clone(),
                    MutableRelationshipSchemaEntry::from_entry(other_entry),
                );
            }
        }

        Ok(MutableRelationshipSchema { entries: result })
    }
}

impl Default for MutableRelationshipSchema {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to union two relationship property maps, checking for conflicts.
fn union_relationship_properties(
    left: &HashMap<String, RelationshipPropertySchema>,
    right: &HashMap<String, RelationshipPropertySchema>,
) -> SchemaResult<HashMap<String, RelationshipPropertySchema>> {
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
    fn test_relationship_schema_entry() {
        let rel_type = RelationshipType::of("KNOWS");
        let mut entry = MutableRelationshipSchemaEntry::new(rel_type.clone(), Direction::Directed);

        entry.add_property("since", ValueType::Long);
        entry.add_property("weight", ValueType::Double);

        assert_eq!(entry.properties().len(), 2);
        assert!(entry.properties().contains_key("since"));
        assert!(entry.properties().contains_key("weight"));
    }

    #[test]
    fn test_relationship_schema_union() {
        let rel_type = RelationshipType::of("KNOWS");

        let mut schema1 = MutableRelationshipSchema::new();
        schema1.add_property(
            rel_type.clone(),
            Direction::Directed,
            "since",
            ValueType::Long,
        );

        let mut schema2 = MutableRelationshipSchema::new();
        schema2.add_property(
            rel_type.clone(),
            Direction::Directed,
            "weight",
            ValueType::Double,
        );

        let union = schema1.union(&schema2.build()).unwrap();
        let entry = union.get(&rel_type).unwrap();

        assert_eq!(entry.properties().len(), 2);
    }

    #[test]
    fn test_relationship_schema_direction_conflict() {
        let rel_type = RelationshipType::of("KNOWS");

        let mut schema1 = MutableRelationshipSchema::new();
        schema1.add_relationship_type(rel_type.clone(), Direction::Directed);

        let mut schema2 = MutableRelationshipSchema::new();
        schema2.add_relationship_type(rel_type.clone(), Direction::Undirected);

        let result = schema1.union(&schema2.build());
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchemaError::DirectionalityConflict { .. }
        ));
    }

    #[test]
    fn test_is_undirected() {
        let mut schema = MutableRelationshipSchema::new();
        schema.add_relationship_type(RelationshipType::of("R1"), Direction::Undirected);
        schema.add_relationship_type(RelationshipType::of("R2"), Direction::Undirected);

        let built = schema.build();
        assert!(built.is_undirected());
    }
}
