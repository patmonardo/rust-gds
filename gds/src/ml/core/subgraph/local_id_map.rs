//! Local ID mapping for subgraphs in GDS.
//!
//! Translated from Java GDS ml-core LocalIdMap.java.
//! This is a literal 1:1 translation following repository translation policy.
//!
//! LocalIdMap provides bidirectional mapping between:
//! - Original (long) node IDs from the graph
//! - Local (int/usize) consecutive IDs for subgraph processing
//!
//! This is crucial for:
//! 1. Efficient array indexing (consecutive local IDs)
//! 2. Preserving original node identity (original IDs)
//! 3. Graph neural network batch processing

use std::collections::HashMap;

/// Bidirectional mapping between original graph node IDs and local consecutive IDs.
///
/// Java uses HPPC's LongArrayList + LongIntHashMap for memory efficiency.
/// Rust translation uses Vec<u64> + HashMap<u64, usize> for clarity and correctness.
#[derive(Debug, Clone)]
pub struct LocalIdMap {
    /// Original node IDs in insertion order (local_id -> original_id)
    original_ids: Vec<u64>,

    /// Reverse mapping (original_id -> local_id)
    original_to_internal: HashMap<u64, usize>,
}

impl LocalIdMap {
    /// Create a new empty LocalIdMap.
    pub fn new() -> Self {
        Self {
            original_ids: Vec::new(),
            original_to_internal: HashMap::new(),
        }
    }

    /// Create a LocalIdMap from a slice of original IDs.
    ///
    /// IDs are mapped in the order they appear.
    pub fn of(original_ids: &[u64]) -> Self {
        let mut map = Self::new();
        for &id in original_ids {
            map.to_mapped(id);
        }
        map
    }

    /// Create a LocalIdMap from original IDs, sorted first.
    ///
    /// This ensures deterministic local ID assignment based on original ID order.
    pub fn of_sorted(original_ids: &[u64]) -> Self {
        let mut sorted = original_ids.to_vec();
        sorted.sort_unstable();

        let mut map = Self::new();
        for id in sorted {
            map.to_mapped(id);
        }
        map
    }

    /// Map an original ID to a local ID.
    ///
    /// If the ID already exists, returns the existing local ID.
    /// Otherwise, assigns a new consecutive local ID.
    ///
    /// This is the primary method for building the mapping.
    pub fn to_mapped(&mut self, original_id: u64) -> usize {
        if let Some(&local_id) = self.original_to_internal.get(&original_id) {
            return local_id;
        }

        let local_id = self.original_ids.len();
        self.original_to_internal.insert(original_id, local_id);
        self.original_ids.push(original_id);
        local_id
    }

    /// Map a local ID back to the original ID.
    ///
    /// # Panics
    /// Panics if local_id is out of bounds.
    pub fn to_original(&self, local_id: usize) -> u64 {
        self.original_ids[local_id]
    }

    /// Get a slice of all original IDs in local ID order.
    pub fn original_ids(&self) -> &[u64] {
        &self.original_ids
    }

    /// Get a vector copy of all original IDs.
    pub fn original_ids_vec(&self) -> Vec<u64> {
        self.original_ids.clone()
    }

    /// Get the number of mapped IDs.
    pub fn size(&self) -> usize {
        debug_assert_eq!(self.original_ids.len(), self.original_to_internal.len());
        self.original_ids.len()
    }

    /// Check if an original ID is already mapped.
    pub fn contains(&self, original_id: u64) -> bool {
        self.original_to_internal.contains_key(&original_id)
    }

    /// Get an iterator over (original_id, local_id) mappings.
    ///
    /// Note: Java's getMappings() uses HPPC spliterator which is not order-preserving.
    /// This Rust version also provides no ordering guarantees.
    pub fn mappings(&self) -> impl Iterator<Item = (u64, usize)> + '_ {
        self.original_to_internal
            .iter()
            .map(|(&orig, &local)| (orig, local))
    }
}

impl Default for LocalIdMap {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for LocalIdMap {
    fn eq(&self, other: &Self) -> bool {
        // Java's equals() compares only originalIds arrays
        self.original_ids == other.original_ids
    }
}

impl Eq for LocalIdMap {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_map() {
        let map = LocalIdMap::new();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_to_mapped_new_id() {
        let mut map = LocalIdMap::new();
        let local_id = map.to_mapped(42);
        assert_eq!(local_id, 0);
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_to_mapped_existing_id() {
        let mut map = LocalIdMap::new();
        let local_id1 = map.to_mapped(42);
        let local_id2 = map.to_mapped(42);
        assert_eq!(local_id1, local_id2);
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_to_mapped_consecutive() {
        let mut map = LocalIdMap::new();
        assert_eq!(map.to_mapped(100), 0);
        assert_eq!(map.to_mapped(200), 1);
        assert_eq!(map.to_mapped(300), 2);
        assert_eq!(map.size(), 3);
    }

    #[test]
    fn test_to_original() {
        let mut map = LocalIdMap::new();
        map.to_mapped(100);
        map.to_mapped(200);
        map.to_mapped(300);

        assert_eq!(map.to_original(0), 100);
        assert_eq!(map.to_original(1), 200);
        assert_eq!(map.to_original(2), 300);
    }

    #[test]
    fn test_of() {
        let map = LocalIdMap::of(&[10, 20, 30, 20]); // 20 appears twice
        assert_eq!(map.size(), 3); // Only 3 unique IDs
        assert_eq!(map.to_original(0), 10);
        assert_eq!(map.to_original(1), 20);
        assert_eq!(map.to_original(2), 30);
    }

    #[test]
    fn test_of_sorted() {
        let map = LocalIdMap::of_sorted(&[30, 10, 20]);
        assert_eq!(map.size(), 3);
        // Should be sorted: 10, 20, 30
        assert_eq!(map.to_original(0), 10);
        assert_eq!(map.to_original(1), 20);
        assert_eq!(map.to_original(2), 30);
    }

    #[test]
    fn test_contains() {
        let mut map = LocalIdMap::new();
        map.to_mapped(42);

        assert!(map.contains(42));
        assert!(!map.contains(99));
    }

    #[test]
    fn test_original_ids() {
        let map = LocalIdMap::of(&[10, 20, 30]);
        let ids = map.original_ids();
        assert_eq!(ids, &[10, 20, 30]);
    }

    #[test]
    fn test_mappings() {
        let map = LocalIdMap::of(&[10, 20, 30]);
        let mut mappings: Vec<_> = map.mappings().collect();
        mappings.sort(); // HashMap iteration order is not guaranteed

        assert_eq!(mappings.len(), 3);
        assert!(mappings.contains(&(10, 0)));
        assert!(mappings.contains(&(20, 1)));
        assert!(mappings.contains(&(30, 2)));
    }

    #[test]
    fn test_equality() {
        let map1 = LocalIdMap::of(&[10, 20, 30]);
        let map2 = LocalIdMap::of(&[10, 20, 30]);
        let map3 = LocalIdMap::of(&[10, 30, 20]); // Different order

        assert_eq!(map1, map2);
        assert_ne!(map1, map3); // Order matters for equality
    }

    #[test]
    fn test_bidirectional_consistency() {
        let mut map = LocalIdMap::new();
        let original_ids = vec![100, 200, 300, 400, 500];

        for &id in &original_ids {
            map.to_mapped(id);
        }

        // Round-trip test
        for local_id in 0..map.size() {
            let original = map.to_original(local_id);
            let mapped_back = map.to_mapped(original);
            assert_eq!(mapped_back, local_id);
        }
    }
}
