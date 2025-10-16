//! Execution Mode - How to return procedure results
//!
//! Translated from: `org.neo4j.gds.executor.ExecutionMode`
//! Source: ExecutionMode.java (30 lines)
//!
//! This enum defines how the executor should return results from algorithm execution.

/// Execution Mode - How to return procedure results
///
/// Maps to Neo4j GDS procedure modes:
/// - `Stream`: Return all results to client
/// - `Stats`: Return summary statistics only
/// - `Train`: Train ML model (for ML procedures)
/// - `WriteNodeProperty`: Write node property to database
/// - `WriteRelationship`: Write relationship to database
/// - `MutateNodeProperty`: Add node property to in-memory graph
/// - `MutateRelationship`: Add relationship to in-memory graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExecutionMode {
    /// Stream all results to client
    Stream,

    /// Return summary statistics only
    Stats,

    /// Train ML model (for ML procedures)
    Train,

    /// Write node property to database
    WriteNodeProperty,

    /// Write relationship to database
    WriteRelationship,

    /// Add node property to in-memory graph
    MutateNodeProperty,

    /// Add relationship to in-memory graph
    MutateRelationship,
}

impl ExecutionMode {
    /// Does this mode return full results?
    ///
    /// Only `Stream` mode returns all results. Other modes return
    /// summaries, metadata, or void.
    pub fn returns_results(&self) -> bool {
        matches!(self, ExecutionMode::Stream)
    }

    /// Does this mode modify the graph?
    ///
    /// Mutating modes add properties or relationships to the in-memory graph.
    /// Writing modes persist changes to the database.
    pub fn is_mutating(&self) -> bool {
        matches!(
            self,
            ExecutionMode::MutateNodeProperty
                | ExecutionMode::MutateRelationship
                | ExecutionMode::WriteNodeProperty
                | ExecutionMode::WriteRelationship
        )
    }

    /// Does this mode write to the database?
    ///
    /// Writing modes persist changes back to Neo4j.
    pub fn is_writing(&self) -> bool {
        matches!(
            self,
            ExecutionMode::WriteNodeProperty | ExecutionMode::WriteRelationship
        )
    }

    /// Does this mode only mutate in-memory (not database)?
    pub fn is_in_memory_mutation(&self) -> bool {
        matches!(
            self,
            ExecutionMode::MutateNodeProperty | ExecutionMode::MutateRelationship
        )
    }

    /// Does this mode produce a trained model?
    ///
    /// Train mode is used for ML procedures that produce models.
    pub fn produces_model(&self) -> bool {
        matches!(self, ExecutionMode::Train)
    }

    /// Is this a stats-only mode?
    pub fn is_stats_only(&self) -> bool {
        matches!(self, ExecutionMode::Stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_results() {
        assert!(ExecutionMode::Stream.returns_results());
        assert!(!ExecutionMode::Stats.returns_results());
        assert!(!ExecutionMode::Train.returns_results());
        assert!(!ExecutionMode::WriteNodeProperty.returns_results());
        assert!(!ExecutionMode::MutateNodeProperty.returns_results());
    }

    #[test]
    fn test_is_mutating() {
        assert!(ExecutionMode::MutateNodeProperty.is_mutating());
        assert!(ExecutionMode::MutateRelationship.is_mutating());
        assert!(ExecutionMode::WriteNodeProperty.is_mutating());
        assert!(ExecutionMode::WriteRelationship.is_mutating());

        assert!(!ExecutionMode::Stream.is_mutating());
        assert!(!ExecutionMode::Stats.is_mutating());
        assert!(!ExecutionMode::Train.is_mutating());
    }

    #[test]
    fn test_is_writing() {
        assert!(ExecutionMode::WriteNodeProperty.is_writing());
        assert!(ExecutionMode::WriteRelationship.is_writing());

        assert!(!ExecutionMode::MutateNodeProperty.is_writing());
        assert!(!ExecutionMode::MutateRelationship.is_writing());
        assert!(!ExecutionMode::Stream.is_writing());
        assert!(!ExecutionMode::Stats.is_writing());
    }

    #[test]
    fn test_is_in_memory_mutation() {
        assert!(ExecutionMode::MutateNodeProperty.is_in_memory_mutation());
        assert!(ExecutionMode::MutateRelationship.is_in_memory_mutation());

        assert!(!ExecutionMode::WriteNodeProperty.is_in_memory_mutation());
        assert!(!ExecutionMode::WriteRelationship.is_in_memory_mutation());
        assert!(!ExecutionMode::Stream.is_in_memory_mutation());
    }

    #[test]
    fn test_produces_model() {
        assert!(ExecutionMode::Train.produces_model());

        assert!(!ExecutionMode::Stream.produces_model());
        assert!(!ExecutionMode::Stats.produces_model());
        assert!(!ExecutionMode::MutateNodeProperty.produces_model());
    }

    #[test]
    fn test_is_stats_only() {
        assert!(ExecutionMode::Stats.is_stats_only());

        assert!(!ExecutionMode::Stream.is_stats_only());
        assert!(!ExecutionMode::Train.is_stats_only());
        assert!(!ExecutionMode::WriteNodeProperty.is_stats_only());
    }

    #[test]
    fn test_equality() {
        assert_eq!(ExecutionMode::Stream, ExecutionMode::Stream);
        assert_ne!(ExecutionMode::Stream, ExecutionMode::Stats);
    }

    #[test]
    fn test_clone() {
        let mode = ExecutionMode::Stream;
        let cloned = mode.clone();
        assert_eq!(mode, cloned);
    }
}
