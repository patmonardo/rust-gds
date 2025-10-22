//! WCC Storage Runtime

pub struct WccStorageRuntime {
    concurrency: usize,
}

impl WccStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
