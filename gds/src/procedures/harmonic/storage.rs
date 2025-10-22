//! Harmonic Storage Runtime
pub struct HarmonicStorageRuntime {
    concurrency: usize,
}

impl HarmonicStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
