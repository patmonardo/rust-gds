pub struct K1ColoringStorageRuntime {
    concurrency: usize,
}

impl K1ColoringStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
