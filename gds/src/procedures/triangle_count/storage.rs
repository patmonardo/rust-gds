pub struct TriangleCountStorageRuntime {
    concurrency: usize,
}

impl TriangleCountStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
