use crate::core::{concurrency::Concurrency, relationship::RelationshipType};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Parameters for split relationships memory estimation
#[derive(Debug, Clone)]
pub struct SplitRelationshipsEstimateParameters {
    pub has_relationship_weight_property: bool,
    pub relationship_types: HashSet<String>,
    pub negative_sampling_ratio: f64,
    pub holdout_fraction: f64,
}

impl SplitRelationshipsEstimateParameters {
    /// Creates new estimation parameters
    pub fn new(
        has_relationship_weight_property: bool,
        relationship_types: HashSet<String>,
        negative_sampling_ratio: f64,
        holdout_fraction: f64,
    ) -> Self {
        Self {
            has_relationship_weight_property,
            relationship_types,
            negative_sampling_ratio,
            holdout_fraction,
        }
    }
}

/// Parameters for split relationships execution
#[derive(Debug, Clone)]
pub struct SplitRelationshipsParameters {
    pub concurrency: Concurrency,
    pub holdout_relationship_type: RelationshipType,
    pub remaining_relationship_type: RelationshipType,
    pub holdout_fraction: f64,
    pub relationship_weight_property: Option<String>,
    pub negative_sampling_ratio: f64,
    pub random_seed: Option<u64>,
}

/// Configuration for split relationships algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitRelationshipsConfig {
    #[serde(default = "default_holdout_fraction")]
    pub holdout_fraction: f64,

    #[serde(default = "default_negative_sampling_ratio")]
    pub negative_sampling_ratio: f64,

    #[serde(default)]
    pub relationship_types: HashSet<String>,

    #[serde(default)]
    pub relationship_weight_property: Option<String>,

    #[serde(default)]
    pub holdout_relationship_type: Option<String>,

    #[serde(default)]
    pub remaining_relationship_type: Option<String>,

    #[serde(default)]
    pub random_seed: Option<u64>,
}

fn default_holdout_fraction() -> f64 {
    0.2
}

fn default_negative_sampling_ratio() -> f64 {
    1.0
}

impl Default for SplitRelationshipsConfig {
    fn default() -> Self {
        Self {
            holdout_fraction: default_holdout_fraction(),
            negative_sampling_ratio: default_negative_sampling_ratio(),
            relationship_types: HashSet::new(),
            relationship_weight_property: None,
            holdout_relationship_type: None,
            remaining_relationship_type: None,
            random_seed: None,
        }
    }
}

impl SplitRelationshipsConfig {
    /// Creates parameters for execution
    pub fn to_parameters(&self, concurrency: Concurrency) -> SplitRelationshipsParameters {
        SplitRelationshipsParameters {
            concurrency,
            holdout_relationship_type: RelationshipType::new(
                self.holdout_relationship_type
                    .clone()
                    .unwrap_or_else(|| "HOLDOUT".to_string()),
            ),
            remaining_relationship_type: RelationshipType::new(
                self.remaining_relationship_type
                    .clone()
                    .unwrap_or_else(|| "REMAINING".to_string()),
            ),
            holdout_fraction: self.holdout_fraction,
            relationship_weight_property: self.relationship_weight_property.clone(),
            negative_sampling_ratio: self.negative_sampling_ratio,
            random_seed: self.random_seed,
        }
    }

    /// Creates parameters for memory estimation
    pub fn to_estimate_parameters(&self) -> SplitRelationshipsEstimateParameters {
        SplitRelationshipsEstimateParameters {
            has_relationship_weight_property: self.relationship_weight_property.is_some(),
            relationship_types: self.relationship_types.clone(),
            negative_sampling_ratio: self.negative_sampling_ratio,
            holdout_fraction: self.holdout_fraction,
        }
    }
}
