// Copyright (c) 2025 Rust-GDS Contributors
//
// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineTrainer.java

use std::error::Error as StdError;

/// Trainer for ML pipelines with termination support.
///
/// This trait represents the core training logic for ML pipelines,
/// including model selection, hyperparameter tuning, and training.
///
/// # Type Parameters
///
/// * `RESULT` - The training result type (e.g., trained model, metrics)
///
/// # Java Source (PipelineTrainer.java)
/// ```java
/// public interface PipelineTrainer<RESULT> {
///     void setTerminationFlag(TerminationFlag terminationFlag);
///     RESULT run();
/// }
/// ```
pub trait PipelineTrainer {
    /// The result type produced by training.
    type Result;

    /// Run the training process.
    ///
    /// This executes model selection, hyperparameter tuning, and training
    /// to produce a trained model and metrics.
    ///
    /// Java: `RESULT run()`
    fn run(&mut self) -> Result<Self::Result, Box<dyn StdError>>;

    /// Check if training has been terminated.
    ///
    /// Training can be stopped early via termination signals.
    /// Implementations should check this periodically during training.
    ///
    /// Java: Uses TerminationFlag for external termination
    fn is_terminated(&self) -> bool {
        false // Default: no termination
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTrainer {
        result: String,
        terminated: bool,
    }

    impl PipelineTrainer for MockTrainer {
        type Result = String;

        fn run(&mut self) -> Result<Self::Result, Box<dyn StdError>> {
            if self.is_terminated() {
                return Err("Training terminated".into());
            }
            Ok(self.result.clone())
        }

        fn is_terminated(&self) -> bool {
            self.terminated
        }
    }

    #[test]
    fn test_pipeline_trainer_run() {
        let mut trainer = MockTrainer {
            result: "trained model".to_string(),
            terminated: false,
        };

        let result = trainer.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "trained model");
    }

    #[test]
    fn test_pipeline_trainer_terminated() {
        let mut trainer = MockTrainer {
            result: "trained model".to_string(),
            terminated: true,
        };

        let result = trainer.run();
        assert!(result.is_err());
    }
}
