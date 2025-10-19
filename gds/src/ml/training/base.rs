/// Base configuration for ML training procedures.
/// This trait combines algorithm and model configuration needs.
/// In Java this extends AlgoBaseConfig & ModelConfig via interface inheritance.
/// In Rust we use composition - implementations must provide these fields.
pub trait TrainBaseConfig: Send + Sync {
    /// Model name identifier
    fn model_name(&self) -> &str;

    /// Username for model ownership
    fn username(&self) -> &str;

    /// Whether to store the model to disk
    fn store_model_to_disk(&self) -> bool {
        false
    }

    /// Concurrency level (from AlgoBaseConfig)
    fn concurrency(&self) -> usize {
        4
    }

    /// Optional job identifier
    fn job_id(&self) -> Option<&str> {
        None
    }
}
