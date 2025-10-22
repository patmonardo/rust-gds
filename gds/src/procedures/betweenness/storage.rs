//! Betweenness Centrality Storage Runtime
pub struct BetweennessCentralityStorageRuntime {
    concurrency: usize,
}

impl BetweennessCentralityStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
