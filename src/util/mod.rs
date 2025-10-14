//! Utility modules for rust-gds
//!
//! Common utilities: string formatting, joining, optional values, logging, exception handling,
//! checked functional interfaces, thread-local resource management, and feature toggles.

pub mod checked;
pub mod exception_util;
pub mod feature_toggles;
pub mod log;
pub mod optional;
pub mod string_formatting;
pub mod string_joining;
pub mod thread_local;

// Re-export commonly used items
pub use checked::{
    checked_consumer, checked_function, checked_runnable, checked_supplier, CheckedConsumer,
    CheckedFunction, CheckedRunnable, CheckedSupplier,
};
pub use exception_util::{AutoCloseable, ExceptionUtil};
pub use feature_toggles::{AdjacencyPackingStrategy, FeatureConfig, FeatureToggle};
pub use optional::Optional;
pub use string_formatting::{format_number, format_with_locale};
pub use string_joining::{format_list, join, join_with_delimiter};
pub use thread_local::{AutoCloseableThreadLocal, CloseableThreadLocal};
