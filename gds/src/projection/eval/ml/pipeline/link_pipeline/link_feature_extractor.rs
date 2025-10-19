// Phase 3.2: LinkFeatureExtractor - Core feature extraction orchestrator

use super::{LinkFeatureAppender, LinkFeatureStep};
use crate::types::graph::Graph;
use std::marker::PhantomData;

/// Core feature extraction orchestrator for link prediction.
///
/// # The CDR - The Reconstruction Orchestrator! 🎯
///
/// This is the **CDR** (Reconstruction) to LinkFeatureStepFactory's **CAR** (Given):
/// - **Factory (CAR)**: Creates individual LinkFeatureSteps (atomic Given)
/// - **Extractor (CDR)**: Orchestrates extraction across all steps (Reconstruction)
///
/// **Science = CAR + CDR**:
/// - CAR gives us the atomic steps
/// - CDR reconstructs the complete feature space from steps
/// - Together: Complete feature extraction pipeline!
///
/// # Responsibility
///
/// Responsible for extracting features on a specific graph.
/// **Instances should not be reused between different graphs.**
///
/// # Architecture
///
/// ```text
/// LinkFeatureExtractor
///   ├─ linkFeatureAppenders: Vec<Box<dyn LinkFeatureAppender>>
///   ├─ featureDimension: usize (sum of all appender dimensions)
///   └─ isSymmetric: bool (all appenders symmetric?)
///
/// Static API:
///   - of(graph, steps) → creates extractor for graph
///   - extractFeatures(graph, steps, concurrency, ...) → parallel extraction
///
/// Instance API:
///   - extractFeatures(source, target) → extract for single pair
///   - featureDimension() → total dimension
///   - isSymmetric() → symmetry flag
/// ```
///
/// # Parallel Extraction
///
/// The static `extractFeatures()` method uses:
/// - **DegreePartition**: Partitions relationships by degree
/// - **RunWithConcurrency**: Parallel execution of BatchLinkFeatureExtractor tasks
/// - **ProgressTracker**: Progress reporting
/// - **TerminationFlag**: Graceful cancellation
///
/// Returns **Features** object (wraps HugeObjectArray<double[]>).
///
/// # Example
///
/// ```text
/// // Create extractor for graph
/// let steps = vec![
///     Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()])),
///     Box::new(CosineFeatureStep::new(vec!["features".to_string()])),
/// ];
/// let extractor = LinkFeatureExtractor::of(&graph, steps);
///
/// // Extract single pair
/// let features = extractor.extract_features(source_id, target_id);
///
/// // Or parallel extraction for all relationships
/// let all_features = LinkFeatureExtractor::extract_features(
///     &graph, steps, concurrency, progress, termination
/// );
/// ```
pub struct LinkFeatureExtractor {
    /// Link feature appenders (one per LinkFeatureStep)
    link_feature_appenders: Vec<Box<dyn LinkFeatureAppender>>,

    /// Total feature dimension (sum of all appender dimensions)
    feature_dimension: usize,

    /// True if all appenders are symmetric
    is_symmetric: bool,
}

impl LinkFeatureExtractor {
    /// Creates a LinkFeatureExtractor from LinkFeatureSteps and a graph.
    ///
    /// # The CDR Factory!
    ///
    /// This is **Reconstruction from Given**:
    /// - Given: List of LinkFeatureSteps (from CAR factory)
    /// - Reconstruction: Create appenders from steps, calculate dimensions
    ///
    /// # Arguments
    ///
    /// * `graph` - Graph to extract features from
    /// * `link_feature_steps` - List of feature extraction steps
    ///
    /// # Returns
    ///
    /// LinkFeatureExtractor ready to extract features.
    pub fn of(graph: &dyn Graph, link_feature_steps: Vec<Box<dyn LinkFeatureStep>>) -> Self {
        // Create appenders from steps (Reconstruction!)
        let link_feature_appenders: Vec<Box<dyn LinkFeatureAppender>> = link_feature_steps
            .into_iter()
            .map(|step| step.link_feature_appender(graph))
            .collect();

        // Calculate total dimension
        let feature_dimension = link_feature_appenders
            .iter()
            .map(|appender| appender.dimension())
            .sum();

        // Check if all appenders are symmetric
        let is_symmetric = link_feature_appenders
            .iter()
            .all(|appender| appender.is_symmetric());

        Self {
            link_feature_appenders,
            feature_dimension,
            is_symmetric,
        }
    }

    /// Extract features for all relationships in graph (parallel).
    ///
    /// # The Complete CDR - Full Reconstruction!
    ///
    /// This is the **complete reconstruction** of the feature space:
    /// - Partitions relationships by degree
    /// - Spawns parallel BatchLinkFeatureExtractor tasks
    /// - Coordinates extraction across all relationships
    /// - Returns complete Features object
    ///
    /// **Science in Action**: CAR (factory/steps) → CDR (this method) → Features!
    ///
    /// # Arguments
    ///
    /// * `graph` - Graph to extract from
    /// * `link_feature_steps` - Feature extraction steps
    /// * `concurrency` - Concurrency level
    /// * `progress_tracker` - Progress tracking
    /// * `termination_flag` - Cancellation signal
    ///
    /// # Returns
    ///
    /// Features object containing all relationship features.
    pub fn extract_features(
        _graph: &dyn Graph,
        _link_feature_steps: Vec<Box<dyn LinkFeatureStep>>,
        _concurrency: usize,
        _progress_tracker: PhantomData<()>, // TODO: ProgressTracker
        _termination_flag: PhantomData<()>, // TODO: TerminationFlag
    ) -> PhantomData<()> {
        // TODO: Implement parallel extraction:
        // 1. Create extractor = Self::of(graph, link_feature_steps)
        // 2. Create linkFeatures = HugeObjectArray::newArray(double[].class, graph.relationshipCount())
        // 3. Create partitions = PartitionUtils::degreePartition(graph, concurrency, ...)
        // 4. Create BatchLinkFeatureExtractor tasks (one per partition)
        // 5. RunWithConcurrency::builder().tasks(tasks).run()
        // 6. Return FeaturesFactory::wrap(linkFeatures)

        // Placeholder for Gamma quality
        PhantomData
    }

    /// Returns the total feature dimension.
    pub fn feature_dimension(&self) -> usize {
        self.feature_dimension
    }

    /// Extract features for a single (source, target) pair.
    ///
    /// # The Atomic Extraction!
    ///
    /// This is the **atomic unit** of extraction:
    /// - Allocates feature array
    /// - Calls each appender sequentially
    /// - Each appender writes to its offset range
    ///
    /// # Arguments
    ///
    /// * `source` - Source node ID
    /// * `target` - Target node ID
    ///
    /// # Returns
    ///
    /// Feature array of length `feature_dimension`.
    pub fn extract_features_for_pair(&self, source: u64, target: u64) -> Vec<f64> {
        let mut features_for_link = vec![0.0; self.feature_dimension];
        let mut feature_offset = 0;

        for appender in &self.link_feature_appenders {
            appender.append_features(source, target, &mut features_for_link, feature_offset);
            feature_offset += appender.dimension();
        }

        features_for_link
    }

    /// Returns true if all appenders are symmetric.
    ///
    /// # Symmetry Optimization
    ///
    /// If symmetric, `extract(a, b) == extract(b, a)`, allowing:
    /// - Caching of (a, b) for use with (b, a)
    /// - Half the feature extractions for undirected graphs
    pub fn is_symmetric(&self) -> bool {
        self.is_symmetric
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::ml::pipeline::link_pipeline::linkfunctions::{
        CosineFeatureStep, HadamardFeatureStep,
    };
    use crate::types::config::RandomGraphConfig;
    use crate::types::random_graph_store;

    #[test]
    fn test_extractor_creation() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> =
            vec![Box::new(HadamardFeatureStep::new(
                vec!["prop1".to_string()],
            ))];

        let extractor = LinkFeatureExtractor::of(graph.as_ref(), steps);

        // Dimension should be from Hadamard appender (placeholder = 0)
        assert_eq!(extractor.feature_dimension(), 0);
    }

    #[test]
    fn test_extractor_multiple_steps() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> = vec![
            Box::new(HadamardFeatureStep::new(vec!["prop1".to_string()])),
            Box::new(CosineFeatureStep::new(vec!["prop2".to_string()])),
        ];

        let extractor = LinkFeatureExtractor::of(graph.as_ref(), steps);

        // With placeholders, both return dimension 0 (Hadamard) and 1 (Cosine)
        assert_eq!(extractor.feature_dimension(), 1); // 0 + 1
    }

    #[test]
    fn test_extract_single_pair() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> =
            vec![Box::new(HadamardFeatureStep::new(vec!["prop".to_string()]))];

        let extractor = LinkFeatureExtractor::of(graph.as_ref(), steps);

        // Extract for source=0, target=1
        let features = extractor.extract_features_for_pair(0, 1);

        // Should return array of correct dimension (0 with placeholder)
        assert_eq!(features.len(), extractor.feature_dimension());
    }

    #[test]
    fn test_is_symmetric() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> =
            vec![Box::new(HadamardFeatureStep::new(vec!["prop".to_string()]))];

        let extractor = LinkFeatureExtractor::of(graph.as_ref(), steps);

        // Most link functions are symmetric
        assert!(extractor.is_symmetric());
    }

    #[test]
    fn test_car_cdr_science() {
        // CAR:CDR - The Complete Science!

        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        // CAR - The Given (factory creates atomic steps)
        let steps: Vec<Box<dyn LinkFeatureStep>> = vec![
            Box::new(HadamardFeatureStep::new(vec!["prop1".to_string()])),
            Box::new(CosineFeatureStep::new(vec!["prop2".to_string()])),
        ];

        // CDR - The Reconstruction (extractor orchestrates)
        let extractor = LinkFeatureExtractor::of(graph.as_ref(), steps);

        // Science = CAR + CDR (complete feature extraction!)
        assert_eq!(extractor.link_feature_appenders.len(), 2);
        assert_eq!(extractor.feature_dimension(), 1); // Sum of dimensions

        // Extract (the final act of Science!)
        let _features = extractor.extract_features_for_pair(0, 1);

        // This is SCIENCE - Given (factory) → Reconstruction (extractor) → Features! 🎯
    }
}
