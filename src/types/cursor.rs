//! Advanced Cursor Technology for Rich Property Traversal
//!
//! This module provides cursors that can traverse and access rich property types
//! including lists and maps, going beyond Java GDS's scalar limitations.
//! Cursors support Arrow2-backed properties for efficient columnar access.

use crate::types::property_value::PropertyValue;
use std::collections::HashMap;

/// A cursor that can traverse rich property values including lists and maps.
/// Supports both scalar access (like Java GDS) and advanced list/map operations.
#[derive(Debug)]
pub struct RichPropertyCursor<'a> {
    value: &'a PropertyValue,
}

impl<'a> RichPropertyCursor<'a> {
    /// Create a cursor for a property value.
    pub fn new(value: &'a PropertyValue) -> Self {
        Self { value }
    }

    /// Get the scalar long value if this is a Long.
    pub fn as_long(&self) -> Option<i64> {
        self.value.as_long()
    }

    /// Get the scalar double value if this is a Double.
    pub fn as_double(&self) -> Option<f64> {
        self.value.as_double()
    }

    /// Get the scalar string value if this is a String.
    pub fn as_string(&self) -> Option<&str> {
        self.value.as_string()
    }

    /// Get the long array if this is a LongArray.
    pub fn as_long_array(&self) -> Option<&[i64]> {
        self.value.as_long_array()
    }

    /// Get the double array if this is a DoubleArray.
    pub fn as_double_array(&self) -> Option<&[f64]> {
        self.value.as_double_array()
    }

    /// Get the string map if this is a StringMap.
    pub fn as_string_map(&self) -> Option<&HashMap<String, String>> {
        self.value.as_string_map()
    }

    /// Get the long map if this is a LongMap.
    pub fn as_long_map(&self) -> Option<&HashMap<String, i64>> {
        self.value.as_long_map()
    }

    /// Get the double map if this is a DoubleMap.
    pub fn as_double_map(&self) -> Option<&HashMap<String, f64>> {
        self.value.as_double_map()
    }

    /// Get the untyped map if this is an UntypedMap.
    pub fn as_untyped_map(&self) -> Option<&HashMap<String, PropertyValue>> {
        self.value.as_untyped_map()
    }

    /// Check if this value is a scalar.
    pub fn is_scalar(&self) -> bool {
        self.value.is_scalar()
    }

    /// Check if this value is an array.
    pub fn is_array(&self) -> bool {
        self.value.is_array()
    }

    /// Check if this value is a map.
    pub fn is_map(&self) -> bool {
        self.value.is_map()
    }

    /// Get the length of an array, or None if not an array.
    pub fn array_length(&self) -> Option<usize> {
        match self.value {
            PropertyValue::LongArray(arr) => Some(arr.len()),
            PropertyValue::DoubleArray(arr) => Some(arr.len()),
            PropertyValue::FloatArray(arr) => Some(arr.len()),
            PropertyValue::StringMapArray(arr) => Some(arr.len()),
            PropertyValue::LongMapArray(arr) => Some(arr.len()),
            PropertyValue::DoubleMapArray(arr) => Some(arr.len()),
            PropertyValue::UntypedArray(arr) => Some(arr.len()),
            _ => None,
        }
    }

    /// Get the number of keys in a map, or None if not a map.
    pub fn map_size(&self) -> Option<usize> {
        match self.value {
            PropertyValue::StringMap(map) => Some(map.len()),
            PropertyValue::LongMap(map) => Some(map.len()),
            PropertyValue::DoubleMap(map) => Some(map.len()),
            PropertyValue::BooleanMap(map) => Some(map.len()),
            PropertyValue::UntypedMap(map) => Some(map.len()),
            _ => None,
        }
    }

    /// Get a value from a map by key, or None if not a map or key doesn't exist.
    pub fn map_get(&self, key: &str) -> Option<PropertyValue> {
        match self.value {
            PropertyValue::StringMap(map) => map.get(key).map(|s| PropertyValue::String(s.clone())),
            PropertyValue::LongMap(map) => map.get(key).map(|&v| PropertyValue::Long(v)),
            PropertyValue::DoubleMap(map) => map.get(key).map(|&v| PropertyValue::Double(v)),
            PropertyValue::BooleanMap(map) => map.get(key).map(|&v| PropertyValue::Boolean(v)),
            PropertyValue::UntypedMap(map) => map.get(key).cloned(),
            _ => None,
        }
    }

    /// Get an element from an array by index, or None if not an array or out of bounds.
    pub fn array_get(&self, index: usize) -> Option<PropertyValue> {
        match self.value {
            PropertyValue::LongArray(arr) => arr.get(index).map(|&v| PropertyValue::Long(v)),
            PropertyValue::DoubleArray(arr) => arr.get(index).map(|&v| PropertyValue::Double(v)),
            PropertyValue::FloatArray(arr) => {
                arr.get(index).map(|&v| PropertyValue::Double(v as f64))
            } // convert
            PropertyValue::UntypedArray(arr) => arr.get(index).cloned(),
            _ => None,
        }
    }

    /// Iterate over array elements.
    pub fn array_iter(&self) -> Option<ArrayIterator<'_>> {
        match self.value {
            PropertyValue::LongArray(arr) => Some(ArrayIterator::Long(arr.iter())),
            PropertyValue::DoubleArray(arr) => Some(ArrayIterator::Double(arr.iter())),
            PropertyValue::FloatArray(arr) => Some(ArrayIterator::Float(arr.iter())),
            PropertyValue::UntypedArray(arr) => Some(ArrayIterator::Untyped(arr.iter())),
            _ => None,
        }
    }

    /// Iterate over map entries.
    pub fn map_iter(&self) -> Option<MapIterator<'_>> {
        match self.value {
            PropertyValue::StringMap(map) => Some(MapIterator::String(map.iter())),
            PropertyValue::LongMap(map) => Some(MapIterator::Long(map.iter())),
            PropertyValue::DoubleMap(map) => Some(MapIterator::Double(map.iter())),
            PropertyValue::BooleanMap(map) => Some(MapIterator::Boolean(map.iter())),
            PropertyValue::UntypedMap(map) => Some(MapIterator::Untyped(map.iter())),
            _ => None,
        }
    }
}

/// Iterator over array elements.
pub enum ArrayIterator<'a> {
    Long(std::slice::Iter<'a, i64>),
    Double(std::slice::Iter<'a, f64>),
    Float(std::slice::Iter<'a, f32>),
    Untyped(std::slice::Iter<'a, PropertyValue>),
}

impl<'a> Iterator for ArrayIterator<'a> {
    type Item = PropertyValue;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ArrayIterator::Long(iter) => iter.next().map(|&v| PropertyValue::Long(v)),
            ArrayIterator::Double(iter) => iter.next().map(|&v| PropertyValue::Double(v)),
            ArrayIterator::Float(iter) => iter.next().map(|&v| PropertyValue::Double(v as f64)),
            ArrayIterator::Untyped(iter) => iter.next().cloned(),
        }
    }
}

/// Iterator over map entries.
pub enum MapIterator<'a> {
    String(std::collections::hash_map::Iter<'a, String, String>),
    Long(std::collections::hash_map::Iter<'a, String, i64>),
    Double(std::collections::hash_map::Iter<'a, String, f64>),
    Boolean(std::collections::hash_map::Iter<'a, String, bool>),
    Untyped(std::collections::hash_map::Iter<'a, String, PropertyValue>),
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = (&'a String, PropertyValue);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MapIterator::String(iter) => iter
                .next()
                .map(|(k, v)| (k, PropertyValue::String(v.clone()))),
            MapIterator::Long(iter) => iter.next().map(|(k, &v)| (k, PropertyValue::Long(v))),
            MapIterator::Double(iter) => iter.next().map(|(k, &v)| (k, PropertyValue::Double(v))),
            MapIterator::Boolean(iter) => iter.next().map(|(k, &v)| (k, PropertyValue::Boolean(v))),
            MapIterator::Untyped(iter) => iter.next().map(|(k, v)| (k, v.clone())),
        }
    }
}

/// A cursor that can traverse relationship properties with rich types.
/// Extends basic relationship traversal with advanced property access.
#[derive(Debug)]
pub struct RichRelationshipCursor<'a> {
    source_id: u64,
    target_id: u64,
    properties: Vec<(&'a str, RichPropertyCursor<'a>)>,
}

impl<'a> RichRelationshipCursor<'a> {
    /// Create a cursor for a relationship with properties.
    pub fn new(
        source_id: u64,
        target_id: u64,
        properties: Vec<(&'a str, &'a PropertyValue)>,
    ) -> Self {
        let properties = properties
            .into_iter()
            .map(|(key, value)| (key, RichPropertyCursor::new(value)))
            .collect();

        Self {
            source_id,
            target_id,
            properties,
        }
    }

    /// Get the source node ID.
    pub fn source_id(&self) -> u64 {
        self.source_id
    }

    /// Get the target node ID.
    pub fn target_id(&self) -> u64 {
        self.target_id
    }

    /// Get a property cursor by key.
    pub fn property(&self, key: &str) -> Option<&RichPropertyCursor<'_>> {
        self.properties
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, cursor)| cursor)
    }

    /// Iterate over all properties.
    pub fn properties(&self) -> impl Iterator<Item = (&str, &RichPropertyCursor<'_>)> {
        self.properties.iter().map(|(k, c)| (*k, c))
    }

    /// Check if this relationship has a property with the given key.
    pub fn has_property(&self, key: &str) -> bool {
        self.properties.iter().any(|(k, _)| *k == key)
    }

    /// Get the weight property (common in GDS) as a double.
    pub fn weight(&self) -> Option<f64> {
        self.property("weight")?.as_double()
    }

    /// Get the cost property (common in GDS) as a double.
    pub fn cost(&self) -> Option<f64> {
        self.property("cost")?.as_double()
    }

    /// Get a property as a long array (e.g., for multi-dimensional weights).
    pub fn weights(&self) -> Option<&[f64]> {
        self.property("weights")?.as_double_array()
    }

    /// Get a property as a map (e.g., for rich relationship metadata).
    pub fn metadata(&self) -> Option<&HashMap<String, PropertyValue>> {
        self.property("metadata")?.as_untyped_map()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_scalar_cursor() {
        let value = PropertyValue::Long(42);
        let cursor = RichPropertyCursor::new(&value);

        assert!(cursor.is_scalar());
        assert!(!cursor.is_array());
        assert!(!cursor.is_map());
        assert_eq!(cursor.as_long(), Some(42));
        assert_eq!(cursor.as_double(), None);
    }

    #[test]
    fn test_array_cursor() {
        let value = PropertyValue::DoubleArray(vec![1.0, 2.0, 3.0]);
        let cursor = RichPropertyCursor::new(&value);

        assert!(!cursor.is_scalar());
        assert!(cursor.is_array());
        assert!(!cursor.is_map());
        assert_eq!(cursor.array_length(), Some(3));
        assert_eq!(cursor.as_double_array(), Some([1.0, 2.0, 3.0].as_slice()));

        let element = cursor.array_get(1);
        assert_eq!(element, Some(PropertyValue::Double(2.0)));
    }

    #[test]
    fn test_map_cursor() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        let value = PropertyValue::StringMap(map);
        let cursor = RichPropertyCursor::new(&value);

        assert!(!cursor.is_scalar());
        assert!(!cursor.is_array());
        assert!(cursor.is_map());
        assert_eq!(cursor.map_size(), Some(2));

        let val = cursor.map_get("key1");
        assert_eq!(val, Some(PropertyValue::String("value1".to_string())));
    }

    #[test]
    fn test_relationship_cursor() {
        let weight = PropertyValue::Double(0.8);
        let metadata = PropertyValue::UntypedMap(HashMap::from([
            (
                "type".to_string(),
                PropertyValue::String("friendship".to_string()),
            ),
            ("strength".to_string(), PropertyValue::Long(5)),
        ]));

        let cursor =
            RichRelationshipCursor::new(0, 1, vec![("weight", &weight), ("metadata", &metadata)]);

        assert_eq!(cursor.source_id(), 0);
        assert_eq!(cursor.target_id(), 1);
        assert!(cursor.has_property("weight"));
        assert_eq!(cursor.weight(), Some(0.8));

        let meta_cursor = cursor.property("metadata").unwrap();
        let meta_map = meta_cursor.as_untyped_map().unwrap();
        assert_eq!(
            meta_map.get("type"),
            Some(&PropertyValue::String("friendship".to_string()))
        );
    }

    #[test]
    fn test_array_iteration() {
        let value = PropertyValue::LongArray(vec![10, 20, 30]);
        let cursor = RichPropertyCursor::new(&value);

        let mut iter = cursor.array_iter().unwrap();
        assert_eq!(iter.next(), Some(PropertyValue::Long(10)));
        assert_eq!(iter.next(), Some(PropertyValue::Long(20)));
        assert_eq!(iter.next(), Some(PropertyValue::Long(30)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_map_iteration() {
        let mut map = HashMap::new();
        map.insert("a".to_string(), 1i64);
        map.insert("b".to_string(), 2i64);
        let value = PropertyValue::LongMap(map);
        let cursor = RichPropertyCursor::new(&value);

        let iter = cursor.map_iter().unwrap();
        let mut entries: Vec<_> = iter.collect();
        entries.sort_by_key(|(k, _)| (*k).clone());

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], (&"a".to_string(), PropertyValue::Long(1)));
        assert_eq!(entries[1], (&"b".to_string(), PropertyValue::Long(2)));
    }
}
