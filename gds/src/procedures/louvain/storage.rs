pub struct LouvainStorageRuntime {
    concurrency: usize,
}

impl LouvainStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
