// pub mod cursor;  // TODO: Disabled - needs redesign after PropertyValue enum removal
pub mod default_value;
pub mod graph;
pub mod graph_store;
pub mod properties;
pub mod property_state;
pub mod random;
pub mod schema;
pub mod value_type;

/// Simple concurrency configuration placeholder.
///
/// The Java/TypeScript implementation exposes a rich configuration object. For now we
/// only capture the desired level of parallelism so we can mirror the API surface.
pub mod concurrency {
    use std::fmt;

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

    impl fmt::Display for Concurrency {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Concurrency(max_workers: {})", self.max_workers)
        }
    }
}
