//! Bridges Storage Runtime

pub struct BridgesStorageRuntime {
    concurrency: usize,
}

impl BridgesStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
