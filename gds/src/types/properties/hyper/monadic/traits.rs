//! Monadic-level traits and markers

/// Marker trait for Monadic HyperStores. Kept intentionally minimal; the true
/// monadic HyperStore contract is `crate::collections::hyper_store::HyperStore`.
pub trait MonadicHyperStore: Send + Sync {}
