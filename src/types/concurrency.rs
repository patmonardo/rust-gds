/// Simple concurrency configuration placeholder.
///
/// The Java/TypeScript implementation exposes a rich configuration object. For now we
/// only capture the desired level of parallelism so we can mirror the API surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Concurrency {
    pub max_workers: usize,
}

impl Concurrency {
    pub const fn new(max_workers: usize) -> Self {
        Self { max_workers }
    }
}

impl Default for Concurrency {
    fn default() -> Self {
        Self { max_workers: 1 }
    }
}
