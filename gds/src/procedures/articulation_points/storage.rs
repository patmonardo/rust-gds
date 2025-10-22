//! Articulation Points Storage Runtime

pub struct ArticulationPointsStorageRuntime {
    concurrency: usize,
}

impl ArticulationPointsStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }
}
