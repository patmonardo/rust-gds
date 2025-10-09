//! PregelContext - Base context for all Pregel contexts
//!
//! Provides common functionality shared across all context types:
//! configuration access, logging, and graph statistics.

use crate::pregel::PregelConfig;

/// Base context for all Pregel context types.
///
/// This provides the foundation for all specialized contexts:
/// - `NodeCentricContext` (adds node-specific operations)
/// - `MasterComputeContext` (adds master compute operations)
///
/// # Common Functionality
///
/// - **Configuration Access**: Get user-defined algorithm configuration
/// - **Logging**: Debug, info, and warning messages
/// - **Graph Statistics**: Node count, relationship count, multi-graph check
///
/// # TODO
///
/// This is a foundational stub. Full implementation will include:
/// - Config reference
/// - ProgressTracker reference for logging
/// - Graph reference for statistics
pub struct PregelContext<C: PregelConfig> {
    config: std::marker::PhantomData<C>,
    // TODO: Add fields when implementing
    // config: &'a C,
    // progress_tracker: &'a ProgressTracker,
    // graph: &'a Graph, (or just store statistics)
}

impl<C: PregelConfig> PregelContext<C> {
    /// Create a new Pregel context (stub).
    ///
    /// # TODO
    ///
    /// Add actual parameters: config, progress_tracker, graph stats
    pub fn stub() -> Self {
        Self {
            config: std::marker::PhantomData,
        }
    }

    /// Get the algorithm configuration.
    ///
    /// # TODO
    ///
    /// Stub - will return reference to actual config
    pub fn config(&self) -> &C {
        unimplemented!("Stub - needs actual config reference")
    }

    /// Log a debug message.
    ///
    /// # TODO
    ///
    /// Stub - will call progress_tracker.log_debug(message)
    pub fn log_debug(&self, _message: &str) {
        // Stub
    }

    /// Log an info message.
    ///
    /// # TODO
    ///
    /// Stub - will call progress_tracker.log_info(message)
    pub fn log_message(&self, _message: &str) {
        // Stub
    }

    /// Log a warning message.
    ///
    /// # TODO
    ///
    /// Stub - will call progress_tracker.log_warning(message)
    pub fn log_warning(&self, _message: &str) {
        // Stub
    }

    /// Check if the input graph is a multi-graph (allows parallel edges).
    ///
    /// # TODO
    ///
    /// Stub - will return graph.is_multi_graph()
    pub fn is_multi_graph(&self) -> bool {
        false
    }

    /// Get the number of nodes in the input graph.
    ///
    /// # TODO
    ///
    /// Stub - will return graph.node_count()
    pub fn node_count(&self) -> u64 {
        0
    }

    /// Get the number of relationships in the input graph.
    ///
    /// # TODO
    ///
    /// Stub - will return graph.relationship_count()
    pub fn relationship_count(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig;
    impl PregelConfig for TestConfig {
        fn max_iterations(&self) -> usize {
            10
        }
    }

    #[test]
    fn test_pregel_context_creation() {
        let _ctx: PregelContext<TestConfig> = PregelContext::stub();
        // Just verify it compiles
    }

    #[test]
    fn test_graph_statistics() {
        let ctx: PregelContext<TestConfig> = PregelContext::stub();
        assert!(!ctx.is_multi_graph());
        assert_eq!(ctx.node_count(), 0);
        assert_eq!(ctx.relationship_count(), 0);
    }
}
