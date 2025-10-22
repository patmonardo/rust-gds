//! Closeness Centrality Storage Runtime
pub struct ClosenessCentralityStorageRuntime {
    concurrency: usize,
}

impl ClosenessCentralityStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
