//! Runtime state container for ML pipeline execution.
//!
//! `PipelineState` holds all intermediate data during pipeline execution:
//! - **Properties**: From graph + computed by node property steps
//! - **Features**: Assembled features ready for ML
//! - **Node IDs**: Dataset scope
//! - **Splits**: Train/validation/test divisions
//!
//! This is the mutable data container that flows through pipeline execution phases.

use std::collections::HashMap;
use std::sync::Arc;

use crate::types::properties::PropertyValues;

/// Runtime state during pipeline execution.
///
/// Pipeline phases update state sequentially:
/// 1. Node property steps → add properties
/// 2. Feature assembly → add features
/// 3. Dataset splitting → set splits
/// 4. Training → read splits + features
#[derive(Debug, Clone)]
pub struct PipelineState {
    /// Computed properties (from node property steps + original graph).
    ///
    /// Maps property name → property values (one value per node).
    /// Example: "pagerank_score" → [0.15, 0.23, 0.18, ...]
    pub properties: HashMap<String, Arc<dyn PropertyValues>>,

    /// Assembled features (from feature steps).
    ///
    /// Maps feature name → feature values (ready for ML consumption).
    /// Example: "normalized_pagerank" → [0.0, 0.5, 0.2, ...]
    pub features: HashMap<String, Arc<dyn PropertyValues>>,

    /// Node IDs in dataset.
    ///
    /// Defines the scope of the ML problem. Usually all nodes or a filtered subset.
    pub node_ids: Vec<u64>,

    /// Dataset splits for training/validation/test.
    pub splits: DatasetSplits,

    /// Current execution phase (for progress tracking).
    pub phase: ExecutionPhase,

    /// Number of steps completed (for progress tracking).
    pub steps_completed: usize,

    /// Total number of steps in pipeline (for progress tracking).
    pub total_steps: usize,
}

/// Dataset splits for training, validation, and testing.
///
/// Splits are disjoint subsets of `node_ids` from PipelineState.
/// train ∪ validation ∪ test = node_ids (usually)
#[derive(Debug, Clone, Default)]
pub struct DatasetSplits {
    /// Training node IDs (used for model fitting).
    pub train: Vec<u64>,

    /// Validation node IDs (used for hyperparameter tuning).
    pub validation: Vec<u64>,

    /// Test node IDs (used for final evaluation).
    pub test: Vec<u64>,
}

/// Execution phase tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionPhase {
    /// Pipeline not yet started.
    NotStarted,

    /// Executing node property steps (computing graph properties).
    NodePropertySteps,

    /// Executing feature steps (assembling features).
    FeatureSteps,

    /// Splitting dataset (train/validation/test).
    DatasetSplitting,

    /// Training models.
    Training,

    /// Evaluating models.
    Evaluation,

    /// Pipeline completed successfully.
    Completed,

    /// Pipeline failed.
    Failed,
}

impl PipelineState {
    /// Create new pipeline state with given node IDs.
    ///
    /// # Arguments
    /// * `node_ids` - Node IDs in the dataset
    /// * `total_steps` - Total number of steps in pipeline (for progress tracking)
    pub fn new(node_ids: Vec<u64>, total_steps: usize) -> Self {
        Self {
            properties: HashMap::new(),
            features: HashMap::new(),
            node_ids,
            splits: DatasetSplits::default(),
            phase: ExecutionPhase::NotStarted,
            steps_completed: 0,
            total_steps,
        }
    }

    /// Add computed property from node property step.
    ///
    /// Properties are raw outputs from graph algorithms (PageRank scores, embeddings, etc.).
    pub fn add_property(&mut self, name: String, values: Arc<dyn PropertyValues>) {
        self.properties.insert(name, values);
    }

    /// Get property by name.
    pub fn get_property(&self, name: &str) -> Option<&Arc<dyn PropertyValues>> {
        self.properties.get(name)
    }

    /// Add assembled feature from feature step.
    ///
    /// Features are transformed/normalized properties ready for ML consumption.
    pub fn add_feature(&mut self, name: String, values: Arc<dyn PropertyValues>) {
        self.features.insert(name, values);
    }

    /// Get feature by name.
    pub fn get_feature(&self, name: &str) -> Option<&Arc<dyn PropertyValues>> {
        self.features.get(name)
    }

    /// Set dataset splits.
    pub fn set_splits(&mut self, splits: DatasetSplits) {
        self.splits = splits;
    }

    /// Update execution phase.
    pub fn set_phase(&mut self, phase: ExecutionPhase) {
        self.phase = phase;
    }

    /// Increment steps completed counter.
    pub fn increment_step(&mut self) {
        self.steps_completed += 1;
    }

    /// Progress indicator (0.0 to 1.0).
    ///
    /// Returns fraction of steps completed.
    pub fn progress(&self) -> f64 {
        if self.total_steps == 0 {
            1.0
        } else {
            self.steps_completed as f64 / self.total_steps as f64
        }
    }

    /// Check if dataset has been split.
    pub fn has_splits(&self) -> bool {
        !self.splits.train.is_empty()
            || !self.splits.validation.is_empty()
            || !self.splits.test.is_empty()
    }

    /// Get total number of nodes in all splits.
    pub fn split_size(&self) -> usize {
        self.splits.train.len() + self.splits.validation.len() + self.splits.test.len()
    }
}

impl DatasetSplits {
    /// Create splits from node IDs and fractions.
    ///
    /// # Arguments
    /// * `node_ids` - All node IDs to split
    /// * `train_fraction` - Fraction for training (e.g., 0.7)
    /// * `val_fraction` - Fraction for validation (e.g., 0.15)
    /// * `test_fraction` - Fraction for test (e.g., 0.15)
    /// * `seed` - Random seed for reproducibility
    ///
    /// # Returns
    /// DatasetSplits with train/validation/test node IDs
    pub fn from_fractions(
        node_ids: &[u64],
        train_fraction: f64,
        val_fraction: f64,
        test_fraction: f64,
        seed: u64,
    ) -> Self {
        use rand::seq::SliceRandom;
        use rand::SeedableRng;

        // Validate fractions
        assert!(
            (train_fraction + val_fraction + test_fraction - 1.0).abs() < 1e-6,
            "Fractions must sum to 1.0"
        );
        assert!(
            train_fraction >= 0.0 && val_fraction >= 0.0 && test_fraction >= 0.0,
            "Fractions must be non-negative"
        );

        let n = node_ids.len();
        let n_train = (n as f64 * train_fraction).round() as usize;
        let n_val = (n as f64 * val_fraction).round() as usize;

        // Shuffle node IDs with seed
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut shuffled = node_ids.to_vec();
        shuffled.shuffle(&mut rng);

        // Split into train/validation/test
        let train = shuffled[..n_train].to_vec();
        let validation = shuffled[n_train..n_train + n_val].to_vec();
        let test = shuffled[n_train + n_val..].to_vec();

        Self {
            train,
            validation,
            test,
        }
    }

    /// Create empty splits.
    pub fn empty() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_state_creation() {
        let node_ids = vec![0, 1, 2, 3, 4];
        let state = PipelineState::new(node_ids.clone(), 10);

        assert_eq!(state.node_ids, node_ids);
        assert_eq!(state.total_steps, 10);
        assert_eq!(state.steps_completed, 0);
        assert_eq!(state.progress(), 0.0);
        assert_eq!(state.phase, ExecutionPhase::NotStarted);
        assert!(!state.has_splits());
    }

    #[test]
    fn test_pipeline_state_progress() {
        let mut state = PipelineState::new(vec![0, 1, 2], 4);

        assert_eq!(state.progress(), 0.0);

        state.increment_step();
        assert_eq!(state.progress(), 0.25);

        state.increment_step();
        assert_eq!(state.progress(), 0.5);

        state.increment_step();
        state.increment_step();
        assert_eq!(state.progress(), 1.0);
    }

    #[test]
    fn test_dataset_splits_from_fractions() {
        let node_ids = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let splits = DatasetSplits::from_fractions(&node_ids, 0.7, 0.15, 0.15, 42);

        // Check sizes (7 train, 1-2 val, 1-2 test)
        assert_eq!(splits.train.len(), 7);
        assert!(splits.validation.len() >= 1 && splits.validation.len() <= 2);
        assert!(splits.test.len() >= 1 && splits.test.len() <= 2);

        // Check total
        assert_eq!(
            splits.train.len() + splits.validation.len() + splits.test.len(),
            node_ids.len()
        );

        // Check disjoint
        let mut all_ids = splits.train.clone();
        all_ids.extend(&splits.validation);
        all_ids.extend(&splits.test);
        all_ids.sort();
        let mut expected = node_ids.clone();
        expected.sort();
        assert_eq!(all_ids, expected);
    }

    #[test]
    fn test_dataset_splits_deterministic() {
        let node_ids = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let splits1 = DatasetSplits::from_fractions(&node_ids, 0.7, 0.15, 0.15, 42);
        let splits2 = DatasetSplits::from_fractions(&node_ids, 0.7, 0.15, 0.15, 42);

        // Same seed should produce same splits
        assert_eq!(splits1.train, splits2.train);
        assert_eq!(splits1.validation, splits2.validation);
        assert_eq!(splits1.test, splits2.test);
    }

    #[test]
    fn test_pipeline_state_has_splits() {
        let mut state = PipelineState::new(vec![0, 1, 2, 3, 4], 5);
        assert!(!state.has_splits());

        let splits = DatasetSplits {
            train: vec![0, 1, 2],
            validation: vec![3],
            test: vec![4],
        };
        state.set_splits(splits);

        assert!(state.has_splits());
        assert_eq!(state.split_size(), 5);
    }

    #[test]
    fn test_execution_phase_transitions() {
        let mut state = PipelineState::new(vec![0, 1], 5);

        assert_eq!(state.phase, ExecutionPhase::NotStarted);

        state.set_phase(ExecutionPhase::NodePropertySteps);
        assert_eq!(state.phase, ExecutionPhase::NodePropertySteps);

        state.set_phase(ExecutionPhase::FeatureSteps);
        assert_eq!(state.phase, ExecutionPhase::FeatureSteps);

        state.set_phase(ExecutionPhase::Completed);
        assert_eq!(state.phase, ExecutionPhase::Completed);
    }

    #[test]
    #[should_panic(expected = "Fractions must sum to 1.0")]
    fn test_dataset_splits_invalid_fractions_sum() {
        let node_ids = vec![0, 1, 2, 3, 4];
        DatasetSplits::from_fractions(&node_ids, 0.5, 0.3, 0.1, 42);
    }

    #[test]
    #[should_panic(expected = "Fractions must be non-negative")]
    fn test_dataset_splits_negative_fraction() {
        let node_ids = vec![0, 1, 2, 3, 4];
        DatasetSplits::from_fractions(&node_ids, 0.8, -0.1, 0.3, 42);
    }
}
