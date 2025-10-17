//! Result Consumer - Result processing and consumption helpers
//!
//! Translated from: `org.neo4j.gds.executor.ComputationResultConsumer`
//! Source: ComputationResultConsumer.java (25 lines)
//!
//! **Java GDS Pattern**:
//! - FunctionalInterface: RESULT consume(ComputationResult<...>, ExecutionContext)
//! - Separate consumer implementations for different execution modes
//!
//! **rust-gds Pattern**:
//! - Helper functions for common consumption patterns
//! - Mode-based consumption routing
//! - Simplified without separate consumer trait

use super::{ComputationResult, ConsumerError, ExecutionMode};

/// Stream Results - Return all algorithm results
///
/// **Execution Mode**: `Stream`
///
/// Returns the raw algorithm output for streaming to client.
/// This is the most common consumption pattern.
///
/// **Use case**: PageRank → stream (nodeId, rank) pairs
pub fn stream_results<T>(result: ComputationResult<T>) -> Result<T, ConsumerError> {
    Ok(result.into_result())
}

/// Stats Only - Return summary statistics
///
/// **Execution Mode**: `Stats`
///
/// Returns timing information without the actual algorithm results.
/// Useful for benchmarking and performance analysis.
///
/// **Use case**: Test algorithm performance without full result processing
pub fn stats_only<T>(result: ComputationResult<T>) -> Result<StatsSummary, ConsumerError> {
    Ok(StatsSummary {
        compute_millis: result.compute_millis(),
        preprocess_millis: result.preprocess_millis(),
        total_millis: result.total_millis(),
        is_graph_empty: result.is_graph_empty(),
    })
}

/// Train Mode - Return model metadata
///
/// **Execution Mode**: `Train`
///
/// Returns metadata about the trained model.
/// Actual model is stored in model catalog.
///
/// **Use case**: ML pipeline training → return model info
pub fn train_model<T>(result: ComputationResult<T>) -> Result<TrainSummary, ConsumerError> {
    Ok(TrainSummary {
        compute_millis: result.compute_millis(),
        preprocess_millis: result.preprocess_millis(),
        total_millis: result.total_millis(),
        // Model details would come from result
    })
}

/// Write Node Property - Return write statistics
///
/// **Execution Mode**: `WriteNodeProperty`
///
/// Returns statistics about properties written to database.
///
/// **Use case**: PageRank.write() → return write count and timing
pub fn write_node_property_stats<T>(
    result: ComputationResult<T>,
    properties_written: usize,
) -> Result<WriteSummary, ConsumerError> {
    Ok(WriteSummary {
        compute_millis: result.compute_millis(),
        preprocess_millis: result.preprocess_millis(),
        write_millis: 0, // Would be set by write operation
        total_millis: result.total_millis(),
        properties_written,
        relationships_written: 0,
    })
}

/// Write Relationship - Return write statistics
///
/// **Execution Mode**: `WriteRelationship`
///
/// Returns statistics about relationships written to database.
///
/// **Use case**: GraphSAGE.write() → return relationship write count
pub fn write_relationship_stats<T>(
    result: ComputationResult<T>,
    relationships_written: usize,
) -> Result<WriteSummary, ConsumerError> {
    Ok(WriteSummary {
        compute_millis: result.compute_millis(),
        preprocess_millis: result.preprocess_millis(),
        write_millis: 0, // Would be set by write operation
        total_millis: result.total_millis(),
        properties_written: 0,
        relationships_written,
    })
}

/// Mutate Node Property - Return mutation statistics
///
/// **Execution Mode**: `MutateNodeProperty`
///
/// Returns statistics about in-memory graph mutations.
///
/// **Use case**: PageRank.mutate() → update in-memory graph
pub fn mutate_node_property_stats<T>(
    result: ComputationResult<T>,
    properties_mutated: usize,
) -> Result<MutateSummary, ConsumerError> {
    Ok(MutateSummary {
        compute_millis: result.compute_millis(),
        preprocess_millis: result.preprocess_millis(),
        mutate_millis: 0, // Would be set by mutate operation
        total_millis: result.total_millis(),
        properties_mutated,
        relationships_mutated: 0,
    })
}

/// Mutate Relationship - Return mutation statistics
///
/// **Execution Mode**: `MutateRelationship`
///
/// Returns statistics about relationship mutations in-memory.
///
/// **Use case**: kNN.mutate() → add relationships to in-memory graph
pub fn mutate_relationship_stats<T>(
    result: ComputationResult<T>,
    relationships_mutated: usize,
) -> Result<MutateSummary, ConsumerError> {
    Ok(MutateSummary {
        compute_millis: result.compute_millis(),
        preprocess_millis: result.preprocess_millis(),
        mutate_millis: 0, // Would be set by mutate operation
        total_millis: result.total_millis(),
        properties_mutated: 0,
        relationships_mutated,
    })
}

/// Consume by Mode - Route consumption based on execution mode
///
/// **Central consumption dispatcher**
///
/// Routes to appropriate consumer based on ExecutionMode.
/// This is typically called by AlgorithmSpec.consume_result().
pub fn consume_by_mode<T>(
    result: ComputationResult<T>,
    mode: ExecutionMode,
) -> Result<ConsumerOutput<T>, ConsumerError> {
    match mode {
        ExecutionMode::Stream => {
            let output = stream_results(result)?;
            Ok(ConsumerOutput::Stream(output))
        }
        ExecutionMode::Stats => {
            let summary = stats_only(result)?;
            Ok(ConsumerOutput::Stats(summary))
        }
        ExecutionMode::Train => {
            let summary = train_model(result)?;
            Ok(ConsumerOutput::Train(summary))
        }
        _ => {
            // Write/Mutate modes require additional context
            // (counts of written/mutated elements)
            Err(ConsumerError::UnsupportedMode(mode))
        }
    }
}

// ============================================================================
// Output Types
// ============================================================================

/// Consumer Output - Tagged union of all consumer outputs
#[derive(Debug)]
pub enum ConsumerOutput<T> {
    /// Stream mode - raw algorithm results
    Stream(T),
    /// Stats mode - summary statistics
    Stats(StatsSummary),
    /// Train mode - model training summary
    Train(TrainSummary),
    /// Write mode - write statistics
    Write(WriteSummary),
    /// Mutate mode - mutation statistics
    Mutate(MutateSummary),
}

/// Stats Summary - Statistics without algorithm results
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatsSummary {
    /// Computation time (milliseconds)
    pub compute_millis: u64,
    /// Preprocessing time (milliseconds)
    pub preprocess_millis: u64,
    /// Total time (milliseconds)
    pub total_millis: u64,
    /// Was the graph empty?
    pub is_graph_empty: bool,
}

/// Train Summary - Model training results
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrainSummary {
    /// Computation time (milliseconds)
    pub compute_millis: u64,
    /// Preprocessing time (milliseconds)
    pub preprocess_millis: u64,
    /// Total time (milliseconds)
    pub total_millis: u64,
    // Additional model metadata would go here
}

/// Write Summary - Database write statistics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteSummary {
    /// Computation time (milliseconds)
    pub compute_millis: u64,
    /// Preprocessing time (milliseconds)
    pub preprocess_millis: u64,
    /// Write time (milliseconds)
    pub write_millis: u64,
    /// Total time (milliseconds)
    pub total_millis: u64,
    /// Number of properties written
    pub properties_written: usize,
    /// Number of relationships written
    pub relationships_written: usize,
}

/// Mutate Summary - In-memory graph mutation statistics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutateSummary {
    /// Computation time (milliseconds)
    pub compute_millis: u64,
    /// Preprocessing time (milliseconds)
    pub preprocess_millis: u64,
    /// Mutation time (milliseconds)
    pub mutate_millis: u64,
    /// Total time (milliseconds)
    pub total_millis: u64,
    /// Number of properties mutated
    pub properties_mutated: usize,
    /// Number of relationships mutated
    pub relationships_mutated: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn create_test_result() -> ComputationResult<Vec<u64>> {
        ComputationResult::new(vec![1, 2, 3, 4, 5], Duration::from_millis(100))
            .with_preprocess_time(Duration::from_millis(50))
    }

    #[test]
    fn test_stream_results() {
        let result = create_test_result();
        let output = stream_results(result);

        assert!(output.is_ok());
        assert_eq!(output.unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_stats_only() {
        let result = create_test_result();
        let summary = stats_only(result);

        assert!(summary.is_ok());
        let summary = summary.unwrap();
        assert_eq!(summary.compute_millis, 100);
        assert_eq!(summary.preprocess_millis, 50);
        assert_eq!(summary.total_millis, 150);
        assert!(!summary.is_graph_empty);
    }

    #[test]
    fn test_train_model() {
        let result = create_test_result();
        let summary = train_model(result);

        assert!(summary.is_ok());
        let summary = summary.unwrap();
        assert_eq!(summary.compute_millis, 100);
        assert_eq!(summary.preprocess_millis, 50);
        assert_eq!(summary.total_millis, 150);
    }

    #[test]
    fn test_write_node_property_stats() {
        let result = create_test_result();
        let summary = write_node_property_stats(result, 1000);

        assert!(summary.is_ok());
        let summary = summary.unwrap();
        assert_eq!(summary.compute_millis, 100);
        assert_eq!(summary.properties_written, 1000);
        assert_eq!(summary.relationships_written, 0);
    }

    #[test]
    fn test_write_relationship_stats() {
        let result = create_test_result();
        let summary = write_relationship_stats(result, 500);

        assert!(summary.is_ok());
        let summary = summary.unwrap();
        assert_eq!(summary.compute_millis, 100);
        assert_eq!(summary.properties_written, 0);
        assert_eq!(summary.relationships_written, 500);
    }

    #[test]
    fn test_mutate_node_property_stats() {
        let result = create_test_result();
        let summary = mutate_node_property_stats(result, 800);

        assert!(summary.is_ok());
        let summary = summary.unwrap();
        assert_eq!(summary.compute_millis, 100);
        assert_eq!(summary.properties_mutated, 800);
        assert_eq!(summary.relationships_mutated, 0);
    }

    #[test]
    fn test_mutate_relationship_stats() {
        let result = create_test_result();
        let summary = mutate_relationship_stats(result, 300);

        assert!(summary.is_ok());
        let summary = summary.unwrap();
        assert_eq!(summary.compute_millis, 100);
        assert_eq!(summary.properties_mutated, 0);
        assert_eq!(summary.relationships_mutated, 300);
    }

    #[test]
    fn test_consume_by_mode_stream() {
        let result = create_test_result();
        let output = consume_by_mode(result, ExecutionMode::Stream);

        assert!(output.is_ok());
        match output.unwrap() {
            ConsumerOutput::Stream(data) => assert_eq!(data, vec![1, 2, 3, 4, 5]),
            _ => panic!("Expected Stream output"),
        }
    }

    #[test]
    fn test_consume_by_mode_stats() {
        let result = create_test_result();
        let output = consume_by_mode(result, ExecutionMode::Stats);

        assert!(output.is_ok());
        match output.unwrap() {
            ConsumerOutput::Stats(summary) => {
                assert_eq!(summary.compute_millis, 100);
                assert_eq!(summary.total_millis, 150);
            }
            _ => panic!("Expected Stats output"),
        }
    }

    #[test]
    fn test_consume_by_mode_train() {
        let result = create_test_result();
        let output = consume_by_mode(result, ExecutionMode::Train);

        assert!(output.is_ok());
        match output.unwrap() {
            ConsumerOutput::Train(summary) => {
                assert_eq!(summary.compute_millis, 100);
            }
            _ => panic!("Expected Train output"),
        }
    }

    #[test]
    fn test_consume_by_mode_write_unsupported() {
        let result = create_test_result();
        let output = consume_by_mode(result, ExecutionMode::WriteNodeProperty);

        assert!(output.is_err());
        match output {
            Err(ConsumerError::UnsupportedMode(mode)) => {
                assert_eq!(mode, ExecutionMode::WriteNodeProperty);
            }
            _ => panic!("Expected UnsupportedMode error"),
        }
    }

    #[test]
    fn test_consume_by_mode_mutate_unsupported() {
        let result = create_test_result();
        let output = consume_by_mode(result, ExecutionMode::MutateRelationship);

        assert!(output.is_err());
    }

    #[test]
    fn test_stats_summary_equality() {
        let summary1 = StatsSummary {
            compute_millis: 100,
            preprocess_millis: 50,
            total_millis: 150,
            is_graph_empty: false,
        };

        let summary2 = StatsSummary {
            compute_millis: 100,
            preprocess_millis: 50,
            total_millis: 150,
            is_graph_empty: false,
        };

        assert_eq!(summary1, summary2);
    }

    #[test]
    fn test_write_summary_totals() {
        let summary = WriteSummary {
            compute_millis: 100,
            preprocess_millis: 50,
            write_millis: 25,
            total_millis: 175,
            properties_written: 1000,
            relationships_written: 0,
        };

        assert_eq!(summary.total_millis, 175);
        assert_eq!(summary.properties_written, 1000);
    }

    #[test]
    fn test_mutate_summary_totals() {
        let summary = MutateSummary {
            compute_millis: 100,
            preprocess_millis: 50,
            mutate_millis: 30,
            total_millis: 180,
            properties_mutated: 500,
            relationships_mutated: 200,
        };

        assert_eq!(summary.total_millis, 180);
        assert_eq!(summary.properties_mutated, 500);
        assert_eq!(summary.relationships_mutated, 200);
    }
}
