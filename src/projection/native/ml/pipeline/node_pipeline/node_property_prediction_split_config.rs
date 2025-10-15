// Copyright (c) "Neo4j"
// Neo4j Sweden AB [http://neo4j.com]
//
// This file is part of Neo4j.
//
// Neo4j is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::projection::native::ml::pipeline::non_empty_set_validation::{
    validate_node_set_size, MIN_SET_SIZE, MIN_TRAIN_SET_SIZE,
};
use std::collections::HashMap;

/// Configuration for splitting node property prediction datasets.
///
/// Defines how to split the graph nodes into training, test, and validation sets
/// for node property prediction pipelines (classification and regression).
#[derive(Debug, Clone, PartialEq)]
pub struct NodePropertyPredictionSplitConfig {
    test_fraction: f64,
    validation_folds: usize,
}

impl NodePropertyPredictionSplitConfig {
    /// Default configuration with 30% test set and 3-fold cross-validation.
    pub const DEFAULT_CONFIG: NodePropertyPredictionSplitConfig =
        NodePropertyPredictionSplitConfig {
            test_fraction: 0.3,
            validation_folds: 3,
        };

    /// Creates a new split configuration.
    ///
    /// # Arguments
    /// * `test_fraction` - Fraction of nodes for test set (0.0 to 1.0)
    /// * `validation_folds` - Number of cross-validation folds (minimum 2)
    pub fn new(test_fraction: f64, validation_folds: usize) -> Result<Self, String> {
        if test_fraction < 0.0 || test_fraction > 1.0 {
            return Err(format!(
                "test_fraction must be between 0.0 and 1.0, got {}",
                test_fraction
            ));
        }
        if validation_folds < 2 {
            return Err(format!(
                "validation_folds must be at least 2, got {}",
                validation_folds
            ));
        }
        Ok(Self {
            test_fraction,
            validation_folds,
        })
    }

    /// Returns the test fraction.
    pub fn test_fraction(&self) -> f64 {
        self.test_fraction
    }

    /// Returns the number of validation folds.
    pub fn validation_folds(&self) -> usize {
        self.validation_folds
    }

    /// Converts configuration to a map representation.
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("testFraction".to_string(), self.test_fraction.to_string());
        map.insert(
            "validationFolds".to_string(),
            self.validation_folds.to_string(),
        );
        map
    }

    /// Validates that the split configuration produces valid set sizes.
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes in the graph
    ///
    /// # Returns
    /// `Err` if any split would produce an invalid set size
    pub fn validate_min_num_nodes_in_split_sets(&self, node_count: usize) -> Result<(), String> {
        let number_nodes_in_test_set = (node_count as f64 * self.test_fraction) as usize;
        let number_nodes_in_train_set = node_count - number_nodes_in_test_set;
        let number_nodes_in_validation_set = number_nodes_in_train_set / self.validation_folds;

        validate_node_set_size(
            number_nodes_in_test_set,
            MIN_SET_SIZE,
            "test",
            "`testFraction` is too low",
        )
        .map_err(|e| e.to_string())?;

        validate_node_set_size(
            number_nodes_in_train_set,
            MIN_TRAIN_SET_SIZE,
            "train",
            "`testFraction` is too high",
        )
        .map_err(|e| e.to_string())?;

        validate_node_set_size(
            number_nodes_in_validation_set,
            MIN_SET_SIZE,
            "validation",
            "`validationFolds` or `testFraction` is too high",
        )
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Returns the test set size for a given node count.
    pub fn test_set_size(&self, node_count: usize) -> usize {
        (self.test_fraction * node_count as f64) as usize
    }

    /// Returns the training set size for a given node count.
    pub fn train_set_size(&self, node_count: usize) -> usize {
        (node_count as f64 * (1.0 - self.test_fraction)) as usize
    }

    /// Returns the training set size for a single fold in cross-validation.
    pub fn fold_train_set_size(&self, node_count: usize) -> usize {
        let train_size = self.train_set_size(node_count);
        train_size * (self.validation_folds - 1) / self.validation_folds
    }

    /// Returns the test set size for a single fold in cross-validation.
    pub fn fold_test_set_size(&self, node_count: usize) -> usize {
        let train_size = self.train_set_size(node_count);
        train_size / self.validation_folds
    }
}

impl Default for NodePropertyPredictionSplitConfig {
    fn default() -> Self {
        Self::DEFAULT_CONFIG
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = NodePropertyPredictionSplitConfig::default();
        assert_eq!(config.test_fraction(), 0.3);
        assert_eq!(config.validation_folds(), 3);
    }

    #[test]
    fn test_new_config() {
        let config = NodePropertyPredictionSplitConfig::new(0.2, 5).unwrap();
        assert_eq!(config.test_fraction(), 0.2);
        assert_eq!(config.validation_folds(), 5);
    }

    #[test]
    fn test_invalid_test_fraction() {
        assert!(NodePropertyPredictionSplitConfig::new(-0.1, 3).is_err());
        assert!(NodePropertyPredictionSplitConfig::new(1.5, 3).is_err());
    }

    #[test]
    fn test_invalid_validation_folds() {
        assert!(NodePropertyPredictionSplitConfig::new(0.3, 1).is_err());
    }

    #[test]
    fn test_to_map() {
        let config = NodePropertyPredictionSplitConfig::new(0.25, 4).unwrap();
        let map = config.to_map();
        assert_eq!(map.get("testFraction"), Some(&"0.25".to_string()));
        assert_eq!(map.get("validationFolds"), Some(&"4".to_string()));
    }

    #[test]
    fn test_test_set_size() {
        let config = NodePropertyPredictionSplitConfig::new(0.3, 3).unwrap();
        assert_eq!(config.test_set_size(1000), 300);
        assert_eq!(config.test_set_size(100), 30);
    }

    #[test]
    fn test_train_set_size() {
        let config = NodePropertyPredictionSplitConfig::new(0.3, 3).unwrap();
        assert_eq!(config.train_set_size(1000), 700);
        assert_eq!(config.train_set_size(100), 70);
    }

    #[test]
    fn test_fold_train_set_size() {
        let config = NodePropertyPredictionSplitConfig::new(0.3, 3).unwrap();
        // Train set = 700, fold train = 700 * 2/3 = 466
        assert_eq!(config.fold_train_set_size(1000), 466);
    }

    #[test]
    fn test_fold_test_set_size() {
        let config = NodePropertyPredictionSplitConfig::new(0.3, 3).unwrap();
        // Train set = 700, fold test = 700 / 3 = 233
        assert_eq!(config.fold_test_set_size(1000), 233);
    }

    #[test]
    fn test_validate_sufficient_nodes() {
        let config = NodePropertyPredictionSplitConfig::new(0.3, 3).unwrap();
        assert!(config.validate_min_num_nodes_in_split_sets(1000).is_ok());
    }

    #[test]
    fn test_validate_insufficient_test_nodes() {
        let config = NodePropertyPredictionSplitConfig::new(0.01, 3).unwrap();
        // With 100 nodes, test set would be 1 node (< MIN_SET_SIZE)
        assert!(config.validate_min_num_nodes_in_split_sets(100).is_err());
    }

    #[test]
    fn test_validate_insufficient_train_nodes() {
        let config = NodePropertyPredictionSplitConfig::new(0.95, 3).unwrap();
        // With 100 nodes, train set would be 5 nodes (< MIN_TRAIN_SET_SIZE)
        assert!(config.validate_min_num_nodes_in_split_sets(100).is_err());
    }
}
