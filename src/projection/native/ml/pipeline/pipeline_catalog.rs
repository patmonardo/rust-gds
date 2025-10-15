// Copyright (c) 2025 Rust-GDS Contributors
//
// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineCatalog.java

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::projection::native::ml::pipeline::TrainingPipeline;

/// Entry in the pipeline catalog associating a name with a pipeline.
///
/// Java: `PipelineCatalogEntry` value class
///
/// Note: Uses Box<dyn std::any::Any> for type erasure since TrainingPipeline
/// has an associated type (FeatureStep) that prevents direct trait object usage.
#[derive(Clone, Debug)]
pub struct PipelineCatalogEntry {
    pipeline_name: String,
    pipeline_type: String,
    // Store as Any to avoid associated type issues
    // Implementers can downcast to specific pipeline types
    pipeline: Arc<dyn std::any::Any + Send + Sync>,
}

impl PipelineCatalogEntry {
    pub fn new<P: TrainingPipeline + Send + Sync + 'static>(
        pipeline_name: String,
        pipeline: Arc<P>,
    ) -> Self {
        let pipeline_type = pipeline.pipeline_type().to_string();
        Self {
            pipeline_name,
            pipeline_type,
            pipeline: pipeline as Arc<dyn std::any::Any + Send + Sync>,
        }
    }

    pub fn pipeline_name(&self) -> &str {
        &self.pipeline_name
    }

    pub fn pipeline_type(&self) -> &str {
        &self.pipeline_type
    }

    /// Downcast the pipeline to a specific type.
    pub fn pipeline_as<P: TrainingPipeline + Send + Sync + 'static>(&self) -> Option<Arc<P>> {
        self.pipeline.clone().downcast::<P>().ok()
    }
}

/// User-scoped catalog of training pipelines.
///
/// Manages pipeline storage for a single user.
///
/// Java: `PipelineUserCatalog` inner class
struct PipelineUserCatalog {
    pipelines_by_name: HashMap<String, PipelineCatalogEntry>,
}

impl PipelineUserCatalog {
    fn new() -> Self {
        Self {
            pipelines_by_name: HashMap::new(),
        }
    }

    fn set<P: TrainingPipeline + Send + Sync + 'static>(
        &mut self,
        pipeline_name: String,
        pipeline: Arc<P>,
    ) -> Result<(), String> {
        if self.pipelines_by_name.contains_key(&pipeline_name) {
            return Err(format!(
                "Pipeline named `{}` already exists.",
                pipeline_name
            ));
        }

        let entry = PipelineCatalogEntry::new(pipeline_name.clone(), pipeline);
        self.pipelines_by_name.insert(pipeline_name, entry);
        Ok(())
    }

    fn exists(&self, pipeline_name: &str) -> bool {
        self.pipelines_by_name.contains_key(pipeline_name)
    }

    fn get(&self, pipeline_name: &str) -> Option<PipelineCatalogEntry> {
        self.pipelines_by_name.get(pipeline_name).cloned()
    }

    fn drop(&mut self, pipeline_name: &str) -> Option<PipelineCatalogEntry> {
        self.pipelines_by_name.remove(pipeline_name)
    }

    fn iter(&self) -> impl Iterator<Item = PipelineCatalogEntry> + '_ {
        self.pipelines_by_name.values().cloned()
    }
}

/// Global catalog of training pipelines with per-user namespacing.
///
/// Provides thread-safe storage and retrieval of training pipelines.
/// Each user has their own isolated catalog.
///
/// # Java Source (PipelineCatalog.java)
/// ```java
/// public final class PipelineCatalog {
///     private static final ConcurrentHashMap<String, PipelineUserCatalog> userCatalogs = new ConcurrentHashMap<>();
///     
///     public static void set(String user, String pipelineName, TrainingPipeline<?> pipeline) { /* ... */ }
///     public static boolean exists(String user, String pipelineName) { /* ... */ }
///     public static TrainingPipeline<?> get(String user, String pipelineName) { /* ... */ }
///     public static <PIPELINE extends TrainingPipeline<?>> PIPELINE getTyped(...) { /* ... */ }
///     public static TrainingPipeline<?> drop(String user, String pipelineName) { /* ... */ }
///     public static void removeAll() { /* ... */ }
///     public static Stream<PipelineCatalogEntry> getAllPipelines(String user) { /* ... */ }
/// }
/// ```
pub struct PipelineCatalog {
    user_catalogs: Arc<RwLock<HashMap<String, PipelineUserCatalog>>>,
}

impl PipelineCatalog {
    /// Create a new pipeline catalog.
    pub fn new() -> Self {
        Self {
            user_catalogs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Store a pipeline in the catalog.
    ///
    /// Java: `public static void set(String user, String pipelineName, TrainingPipeline<?> pipeline)`
    ///
    /// # Errors
    ///
    /// Returns an error if a pipeline with the same name already exists for the user.
    pub fn set<P: TrainingPipeline + Send + Sync + 'static>(
        &self,
        user: &str,
        pipeline_name: &str,
        pipeline: Arc<P>,
    ) -> Result<(), String> {
        let mut catalogs = self
            .user_catalogs
            .write()
            .unwrap_or_else(|e| e.into_inner());

        let user_catalog = catalogs
            .entry(user.to_string())
            .or_insert_with(PipelineUserCatalog::new);

        user_catalog.set(pipeline_name.to_string(), pipeline)
    }

    /// Check if a pipeline exists in the catalog.
    ///
    /// Java: `public static boolean exists(String user, String pipelineName)`
    pub fn exists(&self, user: &str, pipeline_name: &str) -> bool {
        let catalogs = self.user_catalogs.read().unwrap_or_else(|e| e.into_inner());

        catalogs
            .get(user)
            .map(|catalog| catalog.exists(pipeline_name))
            .unwrap_or(false)
    }

    /// Retrieve a pipeline entry from the catalog.
    ///
    /// Java: `public static TrainingPipeline<?> get(String user, String pipelineName)`
    ///
    /// # Errors
    ///
    /// Returns an error if the pipeline does not exist.
    pub fn get(&self, user: &str, pipeline_name: &str) -> Result<PipelineCatalogEntry, String> {
        let catalogs = self.user_catalogs.read().unwrap_or_else(|e| e.into_inner());

        catalogs
            .get(user)
            .and_then(|catalog| catalog.get(pipeline_name))
            .ok_or_else(|| {
                format!(
                    "Pipeline with name `{}` does not exist for user `{}`.",
                    pipeline_name, user
                )
            })
    }

    /// Retrieve a typed pipeline from the catalog.
    ///
    /// Java: `public static <PIPELINE extends TrainingPipeline<?>> PIPELINE getTyped(...)`
    ///
    /// # Type Safety
    ///
    /// In Rust, we use `downcast` to verify the pipeline type at runtime.
    /// The caller must handle type checking or use type-specific catalog methods.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The pipeline does not exist
    /// - The pipeline type does not match the expected type
    pub fn get_typed<T: TrainingPipeline + Send + Sync + 'static>(
        &self,
        user: &str,
        pipeline_name: &str,
    ) -> Result<Arc<T>, String> {
        let entry = self.get(user, pipeline_name)?;

        // Attempt to downcast to the expected type
        entry.pipeline_as::<T>().ok_or_else(|| {
            format!(
                "The pipeline `{}` is not of the expected type.",
                pipeline_name
            )
        })
    }

    /// Remove a pipeline from the catalog.
    ///
    /// Java: `public static TrainingPipeline<?> drop(String user, String pipelineName)`
    ///
    /// # Errors
    ///
    /// Returns an error if the pipeline does not exist.
    pub fn drop(&self, user: &str, pipeline_name: &str) -> Result<PipelineCatalogEntry, String> {
        let mut catalogs = self
            .user_catalogs
            .write()
            .unwrap_or_else(|e| e.into_inner());

        catalogs
            .get_mut(user)
            .and_then(|catalog| catalog.drop(pipeline_name))
            .ok_or_else(|| {
                format!(
                    "Pipeline with name `{}` does not exist for user `{}`.",
                    pipeline_name, user
                )
            })
    }

    /// Remove all pipelines from the catalog (all users).
    ///
    /// Java: `public static void removeAll()`
    pub fn remove_all(&self) {
        let mut catalogs = self
            .user_catalogs
            .write()
            .unwrap_or_else(|e| e.into_inner());
        catalogs.clear();
    }

    /// Get all pipelines for a user.
    ///
    /// Java: `public static Stream<PipelineCatalogEntry> getAllPipelines(String user)`
    pub fn get_all_pipelines(&self, user: &str) -> Vec<PipelineCatalogEntry> {
        let catalogs = self.user_catalogs.read().unwrap_or_else(|e| e.into_inner());

        catalogs
            .get(user)
            .map(|catalog| catalog.iter().collect())
            .unwrap_or_else(Vec::new)
    }

    /// Get the number of pipelines for a user.
    pub fn count(&self, user: &str) -> usize {
        let catalogs = self.user_catalogs.read().unwrap_or_else(|e| e.into_inner());

        catalogs
            .get(user)
            .map(|catalog| catalog.pipelines_by_name.len())
            .unwrap_or(0)
    }

    /// Get the total number of users with pipelines.
    pub fn user_count(&self) -> usize {
        let catalogs = self.user_catalogs.read().unwrap_or_else(|e| e.into_inner());
        catalogs.len()
    }
}

impl Default for PipelineCatalog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Mock FeatureStep for testing
    #[derive(Clone)]
    struct MockFeatureStep;

    impl crate::projection::native::ml::pipeline::FeatureStep for MockFeatureStep {
        fn name(&self) -> &str {
            "mock"
        }

        fn input_node_properties(&self) -> &[String] {
            &[]
        }

        fn configuration(&self) -> &std::collections::HashMap<String, serde_json::Value> {
            use std::sync::OnceLock;
            static CONFIG: OnceLock<std::collections::HashMap<String, serde_json::Value>> =
                OnceLock::new();
            CONFIG.get_or_init(std::collections::HashMap::new)
        }

        fn to_map(&self) -> std::collections::HashMap<String, serde_json::Value> {
            std::collections::HashMap::new()
        }
    }

    // Mock TrainingPipeline for testing
    struct MockPipeline {
        name: String,
    }

    impl crate::projection::native::ml::pipeline::Pipeline for MockPipeline {
        type FeatureStep = MockFeatureStep;

        fn node_property_steps(
            &self,
        ) -> &[Box<dyn crate::projection::native::ml::pipeline::ExecutableNodePropertyStep>]
        {
            &[]
        }

        fn feature_steps(&self) -> &[Self::FeatureStep] {
            &[]
        }

        fn specific_validate_before_execution(
            &self,
            _graph_store: &crate::types::graph_store::DefaultGraphStore,
        ) -> Result<(), crate::projection::native::ml::pipeline::PipelineValidationError> {
            Ok(())
        }

        fn to_map(&self) -> std::collections::HashMap<String, serde_json::Value> {
            let mut map = std::collections::HashMap::new();
            map.insert("name".to_string(), serde_json::json!(self.name));
            map
        }
    }

    impl TrainingPipeline for MockPipeline {
        fn pipeline_type(&self) -> &str {
            &self.name
        }

        fn training_parameter_space(
            &self,
        ) -> &HashMap<
            crate::projection::native::ml::pipeline::TrainingMethod,
            Vec<Box<dyn crate::projection::native::ml::pipeline::TunableTrainerConfig>>,
        > {
            use crate::projection::native::ml::pipeline::TrainingMethod;
            use std::sync::OnceLock;
            static EMPTY: OnceLock<
                HashMap<
                    TrainingMethod,
                    Vec<Box<dyn crate::projection::native::ml::pipeline::TunableTrainerConfig>>,
                >,
            > = OnceLock::new();
            EMPTY.get_or_init(HashMap::new)
        }

        fn training_parameter_space_mut(
            &mut self,
        ) -> &mut HashMap<
            crate::projection::native::ml::pipeline::TrainingMethod,
            Vec<Box<dyn crate::projection::native::ml::pipeline::TunableTrainerConfig>>,
        > {
            unreachable!("Not needed for tests")
        }

        fn auto_tuning_config(&self) -> &crate::projection::native::ml::pipeline::AutoTuningConfig {
            use std::sync::OnceLock;
            static CONFIG: OnceLock<crate::projection::native::ml::pipeline::AutoTuningConfig> =
                OnceLock::new();
            CONFIG.get_or_init(|| {
                crate::projection::native::ml::pipeline::AutoTuningConfig::new(1).unwrap()
            })
        }

        fn set_auto_tuning_config(
            &mut self,
            _config: crate::projection::native::ml::pipeline::AutoTuningConfig,
        ) {
            // Not needed for tests
        }
    }

    #[test]
    fn test_set_and_get_pipeline() {
        let catalog = PipelineCatalog::new();
        let pipeline = Arc::new(MockPipeline {
            name: "test_pipeline".to_string(),
        });

        // Set pipeline
        assert!(catalog
            .set("alice", "my_pipeline", Arc::clone(&pipeline))
            .is_ok());

        // Get pipeline
        let entry = catalog.get("alice", "my_pipeline").unwrap();
        assert_eq!(entry.pipeline_type(), "test_pipeline");
        assert_eq!(entry.pipeline_name(), "my_pipeline");
    }

    #[test]
    fn test_duplicate_pipeline_error() {
        let catalog = PipelineCatalog::new();
        let pipeline = Arc::new(MockPipeline {
            name: "test".to_string(),
        });

        catalog
            .set("alice", "my_pipeline", Arc::clone(&pipeline))
            .unwrap();

        // Attempting to set again should fail
        let result = catalog.set("alice", "my_pipeline", pipeline);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_exists() {
        let catalog = PipelineCatalog::new();
        let pipeline = Arc::new(MockPipeline {
            name: "test".to_string(),
        });

        assert!(!catalog.exists("alice", "my_pipeline"));

        catalog.set("alice", "my_pipeline", pipeline).unwrap();
        assert!(catalog.exists("alice", "my_pipeline"));
        assert!(!catalog.exists("bob", "my_pipeline"));
    }

    #[test]
    fn test_drop_pipeline() {
        let catalog = PipelineCatalog::new();
        let pipeline = Arc::new(MockPipeline {
            name: "test".to_string(),
        });

        catalog.set("alice", "my_pipeline", pipeline).unwrap();
        assert!(catalog.exists("alice", "my_pipeline"));

        let entry = catalog.drop("alice", "my_pipeline").unwrap();
        assert_eq!(entry.pipeline_type(), "test");
        assert!(!catalog.exists("alice", "my_pipeline"));
    }

    #[test]
    fn test_drop_nonexistent_pipeline() {
        let catalog = PipelineCatalog::new();
        let result = catalog.drop("alice", "nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("does not exist"));
    }

    #[test]
    fn test_get_all_pipelines() {
        let catalog = PipelineCatalog::new();
        let pipeline1 = Arc::new(MockPipeline {
            name: "pipeline1".to_string(),
        });
        let pipeline2 = Arc::new(MockPipeline {
            name: "pipeline2".to_string(),
        });

        catalog.set("alice", "p1", pipeline1).unwrap();
        catalog.set("alice", "p2", pipeline2).unwrap();

        let all_pipelines = catalog.get_all_pipelines("alice");
        assert_eq!(all_pipelines.len(), 2);

        let names: Vec<&str> = all_pipelines
            .iter()
            .map(|entry| entry.pipeline_name())
            .collect();
        assert!(names.contains(&"p1"));
        assert!(names.contains(&"p2"));
    }

    #[test]
    fn test_remove_all() {
        let catalog = PipelineCatalog::new();
        let pipeline = Arc::new(MockPipeline {
            name: "test".to_string(),
        });

        catalog.set("alice", "p1", Arc::clone(&pipeline)).unwrap();
        catalog.set("bob", "p2", pipeline).unwrap();

        assert_eq!(catalog.user_count(), 2);

        catalog.remove_all();

        assert_eq!(catalog.user_count(), 0);
        assert!(!catalog.exists("alice", "p1"));
        assert!(!catalog.exists("bob", "p2"));
    }

    #[test]
    fn test_user_isolation() {
        let catalog = PipelineCatalog::new();
        let pipeline = Arc::new(MockPipeline {
            name: "test".to_string(),
        });

        catalog
            .set("alice", "my_pipeline", Arc::clone(&pipeline))
            .unwrap();
        catalog.set("bob", "my_pipeline", pipeline).unwrap();

        assert!(catalog.exists("alice", "my_pipeline"));
        assert!(catalog.exists("bob", "my_pipeline"));

        catalog.drop("alice", "my_pipeline").unwrap();
        assert!(!catalog.exists("alice", "my_pipeline"));
        assert!(catalog.exists("bob", "my_pipeline"));
    }

    #[test]
    fn test_count() {
        let catalog = PipelineCatalog::new();
        let pipeline = Arc::new(MockPipeline {
            name: "test".to_string(),
        });

        assert_eq!(catalog.count("alice"), 0);

        catalog.set("alice", "p1", Arc::clone(&pipeline)).unwrap();
        assert_eq!(catalog.count("alice"), 1);

        catalog.set("alice", "p2", pipeline).unwrap();
        assert_eq!(catalog.count("alice"), 2);
    }
}
