pub struct KCoreStorageRuntime {
    concurrency: usize,
}

impl KCoreStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
