//! **Mutable Path Result**
//!
//! **Translation Source**: `org.neo4j.gds.paths.yens.MutablePathResult`
//!
//! This module implements a mutable path result for Yen's algorithm manipulation.

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Mutable path result for Yen's algorithm manipulation
///
/// Translation of: `MutablePathResult.java` (lines 31-251)
/// Helper data structure that allows manipulation of path results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutablePathResult {
    /// Index of this path in the result set
    pub index: u32,
    /// Source node ID
    pub source_node: u32,
    /// Target node ID
    pub target_node: u32,
    /// Node IDs along the path
    pub node_ids: Vec<u32>,
    /// Relationship IDs along the path
    pub relationship_ids: Vec<u32>,
    /// Costs accumulated along the path
    pub costs: Vec<f64>,
}

impl MutablePathResult {
    /// Create new mutable path result from individual components
    pub fn new(
        index: u32,
        source_node: u32,
        target_node: u32,
        node_ids: Vec<u32>,
        relationship_ids: Vec<u32>,
        costs: Vec<f64>,
    ) -> Self {
        Self {
            index,
            source_node,
            target_node,
            node_ids,
            relationship_ids,
            costs,
        }
    }

    /// Create from a path result (conversion method)
    pub fn from_path_result(
        index: u32,
        source_node: u32,
        target_node: u32,
        node_ids: Vec<u32>,
        relationship_ids: Vec<u32>,
        costs: Vec<f64>,
    ) -> Self {
        Self::new(index, source_node, target_node, node_ids, relationship_ids, costs)
    }

    /// Convert to immutable path result
    pub fn to_path_result(&self) -> PathResult {
        PathResult {
            index: self.index,
            source_node: self.source_node,
            target_node: self.target_node,
            node_ids: self.node_ids.clone(),
            relationship_ids: self.relationship_ids.clone(),
            costs: self.costs.clone(),
        }
    }

    /// Set the index field
    pub fn with_index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }

    /// Get number of nodes in path
    pub fn node_count(&self) -> usize {
        self.node_ids.len()
    }

    /// Get node at given index
    pub fn node(&self, index: usize) -> Option<u32> {
        self.node_ids.get(index).copied()
    }

    /// Get relationship at given index
    pub fn relationship(&self, index: usize) -> Option<u32> {
        self.relationship_ids.get(index).copied()
    }

    /// Get total cost of the path
    pub fn total_cost(&self) -> f64 {
        self.costs.last().copied().unwrap_or(0.0)
    }

    /// Get sub-path from start to given index (exclusive)
    pub fn sub_path(&self, index: usize) -> Self {
        Self::new(
            index as u32,
            self.source_node,
            self.target_node,
            self.node_ids[..index].to_vec(),
            if index > 0 { self.relationship_ids[..index-1].to_vec() } else { Vec::new() },
            self.costs[..index].to_vec(),
        )
    }

    /// Check if this path matches another path up to given index
    pub fn matches(&self, other: &Self, index: usize) -> bool {
        for i in 0..index {
            if self.node_ids.get(i) != other.node_ids.get(i) {
                return false;
            }
        }
        true
    }

    /// Check if this path matches another path exactly up to given index
    pub fn matches_exactly(&self, other: &Self, index: usize) -> bool {
        if self.relationship_ids.is_empty() || other.relationship_ids.is_empty() {
            return self.matches(other, index);
        }

        for i in 0..index {
            if self.node_ids.get(i) != other.node_ids.get(i) {
                return false;
            }
            if i >= 1 {
                if self.relationship_ids.get(i-1) != other.relationship_ids.get(i-1) {
                    return false;
                }
            }
        }
        true
    }

    /// Append another path to this path
    pub fn append(&mut self, other: &Self) {
        assert_eq!(
            self.node_ids.last(),
            other.node_ids.first(),
            "Last node of first path must match first node of second path"
        );

        let old_length = self.node_ids.len();
        let base_cost = self.costs[old_length - 1];

        // Append node IDs (skip first node of other path)
        self.node_ids.extend_from_slice(&other.node_ids[1..]);

        // Append relationship IDs
        self.relationship_ids.extend_from_slice(&other.relationship_ids);

        // Append costs (add base cost to each)
        for &cost in &other.costs[1..] {
            self.costs.push(base_cost + cost);
        }
    }

    /// Append path without relationship IDs
    pub fn append_without_relationship_ids(&mut self, other: &Self) {
        assert_eq!(
            self.node_ids.last(),
            other.node_ids.first(),
            "Last node of first path must match first node of second path"
        );

        let old_length = self.node_ids.len();
        let base_cost = self.costs[old_length - 1];

        // Append node IDs (skip first node of other path)
        self.node_ids.extend_from_slice(&other.node_ids[1..]);

        // Append costs (add base cost to each)
        for &cost in &other.costs[1..] {
            self.costs.push(base_cost + cost);
        }
    }
}

impl PartialEq for MutablePathResult {
    fn eq(&self, other: &Self) -> bool {
        self.node_ids == other.node_ids && self.relationship_ids == other.relationship_ids
    }
}

impl Eq for MutablePathResult {}

impl Hash for MutablePathResult {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_ids.hash(state);
        self.relationship_ids.hash(state);
    }
}

/// Immutable path result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathResult {
    pub index: u32,
    pub source_node: u32,
    pub target_node: u32,
    pub node_ids: Vec<u32>,
    pub relationship_ids: Vec<u32>,
    pub costs: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutable_path_result_creation() {
        let path = MutablePathResult::new(
            0,
            0,
            3,
            vec![0, 1, 2, 3],
            vec![10, 11, 12],
            vec![0.0, 1.0, 2.0, 3.0],
        );

        assert_eq!(path.index, 0);
        assert_eq!(path.source_node, 0);
        assert_eq!(path.target_node, 3);
        assert_eq!(path.node_count(), 4);
        assert_eq!(path.total_cost(), 3.0);
    }

    #[test]
    fn test_mutable_path_result_node_access() {
        let path = MutablePathResult::new(
            0, 0, 3,
            vec![0, 1, 2, 3],
            vec![10, 11, 12],
            vec![0.0, 1.0, 2.0, 3.0],
        );

        assert_eq!(path.node(0), Some(0));
        assert_eq!(path.node(1), Some(1));
        assert_eq!(path.node(3), Some(3));
        assert_eq!(path.node(4), None);
    }

    #[test]
    fn test_mutable_path_result_sub_path() {
        let path = MutablePathResult::new(
            0, 0, 3,
            vec![0, 1, 2, 3],
            vec![10, 11, 12],
            vec![0.0, 1.0, 2.0, 3.0],
        );

        let sub_path = path.sub_path(2);
        assert_eq!(sub_path.node_ids, vec![0, 1]);
        assert_eq!(sub_path.relationship_ids, vec![10]);
        assert_eq!(sub_path.costs, vec![0.0, 1.0]);
    }

    #[test]
    fn test_mutable_path_result_matches() {
        let path1 = MutablePathResult::new(0, 0, 3, vec![0, 1, 2, 3], vec![10, 11, 12], vec![0.0, 1.0, 2.0, 3.0]);
        let path2 = MutablePathResult::new(0, 0, 3, vec![0, 1, 2, 4], vec![10, 11, 13], vec![0.0, 1.0, 2.0, 4.0]);

        assert!(path1.matches(&path2, 2));
        assert!(!path1.matches(&path2, 3));
    }

    #[test]
    fn test_mutable_path_result_append() {
        let mut path1 = MutablePathResult::new(0, 0, 2, vec![0, 1, 2], vec![10, 11], vec![0.0, 1.0, 2.0]);
        let path2 = MutablePathResult::new(0, 2, 4, vec![2, 3, 4], vec![12, 13], vec![0.0, 1.0, 2.0]);

        path1.append(&path2);

        assert_eq!(path1.node_ids, vec![0, 1, 2, 3, 4]);
        assert_eq!(path1.relationship_ids, vec![10, 11, 12, 13]);
        assert_eq!(path1.costs, vec![0.0, 1.0, 2.0, 3.0, 4.0]);
    }
}
