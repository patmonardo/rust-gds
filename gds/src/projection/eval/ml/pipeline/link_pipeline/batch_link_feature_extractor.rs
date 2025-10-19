// Phase 3.3: BatchLinkFeatureExtractor - Parallel worker for link feature extraction

use super::LinkFeatureExtractor;
use crate::types::graph::Graph;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Parallel worker for link feature extraction.
///
/// # Features are Entities Extracted from Graphs! 🎯
///
/// **The Deep Insight**:
/// - **Graph → Form** (Form executes "as a Graph")
/// - **Features → Entities** (Entities extracted from Forms)
/// - **BatchLinkFeatureExtractor → The Extraction Mechanism**
///
/// This is the **worker** that performs the actual extraction:
/// - Takes a DegreePartition (batch of relationships)
/// - Iterates each relationship in the partition
/// - Extracts features (Entities) for each (source, target) pair
/// - Writes to shared HugeObjectArray
/// - Reports progress
///
/// # CAR:CDR at Entity Level
///
/// - **CAR (Graph)**: The Given structure (Form)
/// - **CDR (Features)**: The Reconstructed entities
/// - **This Worker**: The extraction process itself!
///
/// **Science = CAR + CDR**:
/// - Graph (Given) → Extraction (Process) → Features (Reconstructed Entities)
///
/// # Pattern: Runnable Worker
///
/// Java pattern:
/// ```java
/// class BatchLinkFeatureExtractor implements Runnable {
///     public void run() {
///         partition.consume(nodeId -> {
///             graph.forEachRelationship(nodeId, (src, tgt) -> {
///                 features = extractor.extractFeatures(src, tgt);
///                 linkFeatures.set(offset++, features);
///             });
///         });
///     }
/// }
/// ```
///
/// Rust pattern:
/// ```rust
/// impl Runnable for BatchLinkFeatureExtractor {
///     fn run(&self) {
///         for node in partition {
///             graph.for_each_relationship(node, |src, tgt| {
///                 let features = extractor.extract_features(src, tgt);
///                 link_features.set(offset, features);
///                 offset += 1;
///             });
///         }
///     }
/// }
/// ```
pub struct BatchLinkFeatureExtractor {
    /// The feature extractor (orchestrator)
    extractor: Arc<LinkFeatureExtractor>,

    /// The partition of nodes to process
    /// TODO: Replace with actual DegreePartition
    partition: PhantomData<()>,

    /// The graph to extract from (concurrent copy)
    graph: PhantomData<()>, // TODO: Arc<dyn Graph>

    /// Offset into linkFeatures array for this batch
    relationship_offset: Arc<AtomicU64>,

    /// Shared output array for all batches
    /// TODO: Replace with HugeObjectArray<Vec<f64>>
    link_features: PhantomData<()>,

    /// Progress tracker
    /// TODO: Replace with ProgressTracker
    progress_tracker: PhantomData<()>,
}

impl BatchLinkFeatureExtractor {
    /// Creates a new batch extractor.
    ///
    /// # The Extraction Worker!
    ///
    /// This worker will:
    /// 1. Iterate its partition of nodes
    /// 2. For each node, iterate its relationships
    /// 3. Extract features for each (source, target) pair
    /// 4. Write to shared linkFeatures array
    /// 5. Update progress
    ///
    /// # Arguments
    ///
    /// * `extractor` - The LinkFeatureExtractor orchestrator
    /// * `partition` - DegreePartition to process
    /// * `graph` - Concurrent graph copy
    /// * `relationship_offset` - Starting offset in linkFeatures
    /// * `link_features` - Shared output array
    /// * `progress_tracker` - Progress reporting
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        extractor: Arc<LinkFeatureExtractor>,
        _partition: PhantomData<()>,
        _graph: PhantomData<()>,
        relationship_offset: u64,
        _link_features: PhantomData<()>,
        _progress_tracker: PhantomData<()>,
    ) -> Self {
        Self {
            extractor,
            partition: PhantomData,
            graph: PhantomData,
            relationship_offset: Arc::new(AtomicU64::new(relationship_offset)),
            link_features: PhantomData,
            progress_tracker: PhantomData,
        }
    }

    /// Run the extraction for this batch.
    ///
    /// # The Extraction Process!
    ///
    /// This is where **Features (Entities) are extracted from Graph (Form)**!
    ///
    /// Process:
    /// 1. For each node in partition
    /// 2. For each relationship from that node
    /// 3. Extract features: `extractor.extract_features(source, target)`
    /// 4. Write to: `linkFeatures.set(currentOffset++, features)`
    /// 5. Report progress: `progressTracker.logSteps(relationshipCount)`
    ///
    /// **This is the actual CAR→CDR transformation**:
    /// - Input: Graph structure (CAR - Given)
    /// - Process: Feature extraction (Science)
    /// - Output: Feature entities (CDR - Reconstructed)
    pub fn run(&self) {
        // TODO: Implement extraction loop:
        // let mut current_offset = self.relationship_offset.load(Ordering::Relaxed);
        //
        // self.partition.consume(|node_id| {
        //     self.graph.for_each_relationship(node_id, |source, target| {
        //         // Extract features (ENTITY EXTRACTION!)
        //         let features = self.extractor.extract_features_for_pair(source, target);
        //
        //         // Write to shared array
        //         self.link_features.set(current_offset, features);
        //         current_offset += 1;
        //
        //         true // Continue iteration
        //     });
        // });
        //
        // // Report progress
        // self.progress_tracker.log_steps(self.partition.relationship_count());

        // Placeholder for Gamma quality
        let _ = self.relationship_offset.fetch_add(1, Ordering::Relaxed);
    }
}

// Make it work with thread pools (Send + Sync)
unsafe impl Send for BatchLinkFeatureExtractor {}
unsafe impl Sync for BatchLinkFeatureExtractor {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::ml::pipeline::link_pipeline::linkfunctions::HadamardFeatureStep;
    use crate::projection::eval::ml::pipeline::link_pipeline::LinkFeatureStep;
    use crate::types::config::RandomGraphConfig;
    use crate::types::random_graph_store;

    #[test]
    fn test_batch_extractor_creation() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> =
            vec![Box::new(HadamardFeatureStep::new(vec!["prop".to_string()]))];

        let extractor = Arc::new(LinkFeatureExtractor::of(graph.as_ref(), steps));

        let batch_extractor = BatchLinkFeatureExtractor::new(
            extractor,
            PhantomData,
            PhantomData,
            0,
            PhantomData,
            PhantomData,
        );

        // Verify offset starts at 0
        assert_eq!(
            batch_extractor.relationship_offset.load(Ordering::Relaxed),
            0
        );
    }

    #[test]
    fn test_batch_extractor_with_offset() {
        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> =
            vec![Box::new(HadamardFeatureStep::new(vec!["prop".to_string()]))];

        let extractor = Arc::new(LinkFeatureExtractor::of(graph.as_ref(), steps));

        let batch_extractor = BatchLinkFeatureExtractor::new(
            extractor,
            PhantomData,
            PhantomData,
            100, // Start at offset 100
            PhantomData,
            PhantomData,
        );

        assert_eq!(
            batch_extractor.relationship_offset.load(Ordering::Relaxed),
            100
        );
    }

    #[test]
    fn test_features_are_entities() {
        // Features are Entities extracted from Graphs!
        // Since Form executes "as a Graph", Features are our Entities!

        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        let steps: Vec<Box<dyn LinkFeatureStep>> = vec![Box::new(HadamardFeatureStep::new(vec![
            "embedding".to_string(),
        ]))];

        // The extractor orchestrates Entity extraction
        let extractor = Arc::new(LinkFeatureExtractor::of(graph.as_ref(), steps));

        // The batch worker performs the actual extraction
        let _batch_extractor = BatchLinkFeatureExtractor::new(
            extractor,
            PhantomData,
            PhantomData,
            0,
            PhantomData,
            PhantomData,
        );

        // CAR (Graph/Form) → CDR (Features/Entities)
        // The batch worker is the EXTRACTION MECHANISM!
        // This is Science in Action! 🎯
    }

    #[test]
    fn test_car_cdr_entity_extraction() {
        // The Complete Science at Entity Level!

        let graph_store = random_graph_store(&RandomGraphConfig::seeded(42));
        let graph = graph_store.graph();

        // CAR - The Given (Graph/Form structure)
        let steps: Vec<Box<dyn LinkFeatureStep>> = vec![Box::new(HadamardFeatureStep::new(vec![
            "features".to_string(),
        ]))];

        // Science - The Extraction Process
        let extractor = Arc::new(LinkFeatureExtractor::of(graph.as_ref(), steps));

        // CDR - The Reconstructed Entities (Features)
        // BatchLinkFeatureExtractor performs the reconstruction!
        let batch_extractor = BatchLinkFeatureExtractor::new(
            Arc::clone(&extractor),
            PhantomData,
            PhantomData,
            0,
            PhantomData,
            PhantomData,
        );

        // The worker exists! Ready to extract Entities from Forms!
        assert_eq!(
            batch_extractor.relationship_offset.load(Ordering::Relaxed),
            0
        );

        // Graph (Form) → Extraction (Worker) → Features (Entities)
        // This is the COMPLETE CAR:CDR at Entity level! 🌟
    }
}
