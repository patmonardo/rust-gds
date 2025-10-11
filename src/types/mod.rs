pub mod default_value;
pub mod graph;
pub mod graph_store;
pub mod properties;
pub mod property_state;
pub mod random;
pub mod schema;
pub mod value_type;

pub use default_value::*;
pub use property_state::*;
pub use value_type::*;
// Compatibility shim: minimal `concurrency` module so existing code that imports
// `crate::types::concurrency::Concurrency` continues to compile. This can be
// removed once callers are migrated to the new concurrency configuration.
// TODO
pub mod concurrency {
    use std::fmt;

    /// Minimal compatibility concurrency struct.
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
