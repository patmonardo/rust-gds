//! Execution Context - Runtime environment for procedure execution
//!
//! Translated from: `org.neo4j.gds.executor.ExecutionContext`
//! Source: ExecutionContext.java (187 lines)
//!
//! Simplified for rust-gds:
//! - No Neo4j-specific types (DatabaseId, DependencyResolver, NodeLookup)
//! - No model catalog yet (future work)
//! - Simple HashMap-based graph catalog
//! - Basic logging and metrics

use crate::types::catalog::GraphCatalog;
use crate::types::prelude::DefaultGraphStore;
use std::collections::HashMap;
use std::sync::Arc;

/// Execution Context - Runtime environment for procedure execution
///
/// Provides:
/// - Graph catalog (name → GraphStore)
/// - User context (username, admin flag)
/// - Logging infrastructure
/// - Metrics collection
/// - Configuration overrides
///
/// **Simplified from Java GDS**:
/// - No dependency injection (direct ownership)
/// - No Neo4j database integration
/// - No model catalog yet (future work)
pub struct ExecutionContext {
    /// Graph catalog (name → GraphStore)
    graph_catalog: HashMap<String, Arc<DefaultGraphStore>>,

    /// Optional injected catalog handle (preferred for production)
    catalog_handle: Option<Arc<dyn GraphCatalog>>, 

    /// Current user (for auditing)
    username: String,

    /// Is user a GDS admin?
    is_admin: bool,

    /// Log level threshold
    log_level: LogLevel,

    /// Metrics collector
    metrics: MetricsCollector,

    /// Configuration overrides (key → value)
    config_overrides: HashMap<String, String>,
}

/// Log Level - Logging severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

/// Metrics Collector - Records operation timings
pub struct MetricsCollector {
    /// Operation name → list of timings (milliseconds)
    timings: HashMap<String, Vec<u64>>,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            graph_catalog: HashMap::new(),
            catalog_handle: None,
            username: username.into(),
            is_admin: false,
            log_level: LogLevel::Info,
            metrics: MetricsCollector::new(),
            config_overrides: HashMap::new(),
        }
    }

    /// Create an empty/default context (for testing)
    pub fn empty() -> Self {
        Self::new("")
    }

    /// Load a graph from the catalog by name (prefers injected catalog)
    pub fn load_graph(&self, name: &str) -> Result<Arc<DefaultGraphStore>, ContextError> {
        if let Some(catalog) = &self.catalog_handle {
            if let Some(store) = catalog.get(name) {
                return Ok(store);
            }
            return Err(ContextError::GraphNotFound(name.to_string()));
        }

        self.graph_catalog
            .get(name)
            .cloned()
            .ok_or_else(|| ContextError::GraphNotFound(name.to_string()))
    }

    /// Add graph to catalog
    pub fn add_graph(&mut self, name: impl Into<String>, graph: Arc<DefaultGraphStore>) {
        self.graph_catalog.insert(name.into(), graph);
    }

    /// Remove graph from catalog
    pub fn remove_graph(&mut self, name: &str) -> Option<Arc<DefaultGraphStore>> {
        self.graph_catalog.remove(name)
    }

    /// Check if graph exists in catalog
    pub fn has_graph(&self, name: &str) -> bool {
        self.graph_catalog.contains_key(name)
    }

    /// List all graph names in catalog
    pub fn list_graphs(&self) -> Vec<&str> {
        self.graph_catalog.keys().map(|s| s.as_str()).collect()
    }

    /// Inject a catalog handle (takes precedence over the internal map)
    pub fn set_catalog(&mut self, catalog: Arc<dyn GraphCatalog>) {
        self.catalog_handle = Some(catalog);
    }

    /// Builder-style injection for convenience in setup/tests
    pub fn with_catalog(mut self, catalog: Arc<dyn GraphCatalog>) -> Self {
        self.catalog_handle = Some(catalog);
        self
    }

    /// Get current username
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Check if user is GDS admin
    pub fn is_gds_admin(&self) -> bool {
        self.is_admin
    }

    /// Set admin flag
    pub fn set_admin(&mut self, is_admin: bool) {
        self.is_admin = is_admin;
    }

    /// Set log level
    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }

    /// Log message at specified level
    pub fn log(&self, level: LogLevel, message: &str) {
        if level >= self.log_level {
            let prefix = match level {
                LogLevel::Debug => "[DEBUG]",
                LogLevel::Info => "[INFO]",
                LogLevel::Warn => "[WARN]",
                LogLevel::Error => "[ERROR]",
            };
            eprintln!("{} {}: {}", prefix, self.username, message);
        }
    }

    /// Record timing metric
    pub fn record_timing(&mut self, operation: &str, duration_ms: u64) {
        self.metrics.record(operation, duration_ms);
    }

    /// Get metrics collector (immutable reference)
    pub fn metrics(&self) -> &MetricsCollector {
        &self.metrics
    }

    /// Get metrics collector (mutable reference)
    pub fn metrics_mut(&mut self) -> &mut MetricsCollector {
        &mut self.metrics
    }

    /// Set configuration override
    pub fn set_config_override(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.config_overrides.insert(key.into(), value.into());
    }

    /// Get configuration override
    pub fn get_config_override(&self, key: &str) -> Option<&str> {
        self.config_overrides.get(key).map(|s| s.as_str())
    }
}

/// Mock context for testing
impl ExecutionContext {
    /// Create a mock context with a single graph
    pub fn mock(graph: Arc<DefaultGraphStore>) -> Self {
        let mut ctx = Self::new("test_user");
        ctx.add_graph("test_graph", graph);
        ctx.set_admin(true);
        ctx
    }

    /// Create a mock context with multiple graphs
    pub fn mock_with_graphs(graphs: Vec<(&str, Arc<DefaultGraphStore>)>) -> Self {
        let mut ctx = Self::new("test_user");
        for (name, graph) in graphs {
            ctx.add_graph(name, graph);
        }
        ctx.set_admin(true);
        ctx
    }
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            timings: HashMap::new(),
        }
    }

    /// Record a timing measurement
    pub fn record(&mut self, operation: &str, duration_ms: u64) {
        self.timings
            .entry(operation.to_string())
            .or_default()
            .push(duration_ms);
    }

    /// Get all timings for an operation
    pub fn get_timings(&self, operation: &str) -> Option<&[u64]> {
        self.timings.get(operation).map(|v| v.as_slice())
    }

    /// Get average timing for an operation
    pub fn get_average(&self, operation: &str) -> Option<f64> {
        self.get_timings(operation).map(|timings| {
            let sum: u64 = timings.iter().sum();
            sum as f64 / timings.len() as f64
        })
    }

    /// Get total timing for an operation
    pub fn get_total(&self, operation: &str) -> Option<u64> {
        self.get_timings(operation)
            .map(|timings| timings.iter().sum())
    }

    /// Get count of measurements for an operation
    pub fn get_count(&self, operation: &str) -> usize {
        self.get_timings(operation).map(|t| t.len()).unwrap_or(0)
    }

    /// Clear all metrics
    pub fn clear(&mut self) {
        self.timings.clear();
    }

    /// List all operation names
    pub fn operations(&self) -> Vec<&str> {
        self.timings.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Context Error - Errors related to execution context
#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    #[error("Graph not found in catalog: {0}")]
    GraphNotFound(String),

    #[error("Graph already exists in catalog: {0}")]
    GraphAlreadyExists(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::prelude::{DefaultGraphStore, RandomGraphConfig};

    #[test]
    fn test_new_context() {
        let ctx = ExecutionContext::new("alice");
        assert_eq!(ctx.username(), "alice");
        assert!(!ctx.is_gds_admin());
        assert_eq!(ctx.list_graphs().len(), 0);
    }

    #[test]
    fn test_graph_catalog_operations() {
        let mut ctx = ExecutionContext::new("bob");
        let config = RandomGraphConfig::default().with_seed(42);
        let graph = Arc::new(DefaultGraphStore::random(&config).unwrap());

        // Add graph
        ctx.add_graph("test", graph);
        assert!(ctx.has_graph("test"));
        assert_eq!(ctx.list_graphs(), vec!["test"]);

        // Load graph
        let loaded = ctx.load_graph("test");
        assert!(loaded.is_ok());

        // Remove graph
        let removed = ctx.remove_graph("test");
        assert!(removed.is_some());
        assert!(!ctx.has_graph("test"));
    }

    #[test]
    fn test_load_graph_not_found() {
        let ctx = ExecutionContext::new("charlie");
        let result = ctx.load_graph("nonexistent");
        assert!(result.is_err());
        match result {
            Err(ContextError::GraphNotFound(name)) => assert_eq!(name, "nonexistent"),
            _ => panic!("Expected GraphNotFound error"),
        }
    }

    #[test]
    fn test_admin_flag() {
        let mut ctx = ExecutionContext::new("dave");
        assert!(!ctx.is_gds_admin());

        ctx.set_admin(true);
        assert!(ctx.is_gds_admin());

        ctx.set_admin(false);
        assert!(!ctx.is_gds_admin());
    }

    #[test]
    fn test_log_levels() {
        let mut ctx = ExecutionContext::new("eve");

        // Default is Info
        ctx.set_log_level(LogLevel::Info);
        ctx.log(LogLevel::Debug, "Should not appear");
        ctx.log(LogLevel::Info, "Should appear");
        ctx.log(LogLevel::Warn, "Should appear");
        ctx.log(LogLevel::Error, "Should appear");

        // Set to Error only
        ctx.set_log_level(LogLevel::Error);
        ctx.log(LogLevel::Info, "Should not appear");
        ctx.log(LogLevel::Error, "Should appear");
    }

    #[test]
    fn test_metrics_collection() {
        let mut ctx = ExecutionContext::new("frank");

        ctx.record_timing("pagerank", 100);
        ctx.record_timing("pagerank", 150);
        ctx.record_timing("pagerank", 200);

        let metrics = ctx.metrics();
        assert_eq!(metrics.get_count("pagerank"), 3);
        assert_eq!(metrics.get_total("pagerank"), Some(450));
        assert_eq!(metrics.get_average("pagerank"), Some(150.0));
    }

    #[test]
    fn test_metrics_operations() {
        let mut collector = MetricsCollector::new();

        collector.record("op1", 100);
        collector.record("op2", 200);
        collector.record("op1", 150);

        let ops = collector.operations();
        assert_eq!(ops.len(), 2);
        assert!(ops.contains(&"op1"));
        assert!(ops.contains(&"op2"));

        collector.clear();
        assert_eq!(collector.operations().len(), 0);
    }

    #[test]
    fn test_config_overrides() {
        let mut ctx = ExecutionContext::new("grace");

        ctx.set_config_override("maxIterations", "20");
        ctx.set_config_override("tolerance", "0.001");

        assert_eq!(ctx.get_config_override("maxIterations"), Some("20"));
        assert_eq!(ctx.get_config_override("tolerance"), Some("0.001"));
        assert_eq!(ctx.get_config_override("unknown"), None);
    }

    #[test]
    fn test_mock_context() {
        let config = RandomGraphConfig::default().with_seed(42);
        let graph = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let ctx = ExecutionContext::mock(graph);
        assert_eq!(ctx.username(), "test_user");
        assert!(ctx.is_gds_admin());
        assert!(ctx.has_graph("test_graph"));
    }

    #[test]
    fn test_mock_with_multiple_graphs() {
        let config = RandomGraphConfig::default().with_seed(42);
        let graph1 = Arc::new(DefaultGraphStore::random(&config).unwrap());
        let graph2 = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let graphs = vec![("g1", graph1), ("g2", graph2)];
        let ctx = ExecutionContext::mock_with_graphs(graphs);

        assert!(ctx.has_graph("g1"));
        assert!(ctx.has_graph("g2"));
        assert_eq!(ctx.list_graphs().len(), 2);
    }
}
