//! HyperStore: Pure Being (Generic Being)
//! Part 1 of Objective Logic
//!
//! HyperStore is the foundation of the Pure A Priori Knowledge System,
//! representing Pure Being as Generic Being via Collections.

use crate::collections::traits::Collections;

/// HyperStore: Pure Being (Generic Being)
///
/// This is Part 1 of Objective Logic - the monadic foundation.
/// HyperStore wraps Collections and provides access to the underlying
/// Generic Being structure.
pub trait HyperStore<T> {
    /// Get the underlying Collections backend
    fn as_backend(&self) -> &dyn Collections<T>;

    /// Get the capacity of the store
    fn capacity(&self) -> usize;

    /// Get the size of the store
    fn size(&self) -> usize;
}

/// HyperAdapter: Adapts Collections to HyperStore
///
/// This adapter allows Collections to be used as HyperStores,
/// enabling the Generic Being to be accessed as Pure Being.
pub trait HyperAdapter<T>: HyperStore<T> {
    /// Adapt from a Collections instance
    fn adapt_from<C: Collections<T>>(collection: C) -> Self;
}

/// Factory trait that adapts a Collections backend into higher-level
/// HyperProperty store types (e.g. Tetradic / Pentadic). Implementations
/// should be provided by the Collections integration layer or by the
/// UniversalAdapter/HyperAdapter wiring.
pub trait HyperPropertyAdapterFactory<T> {
    /// Type produced for tetradic use-cases (dyad:dyad plane).
    type TetradicStore;

    /// Type produced for pentadic use-cases (dyadic × triadic composition).
    type PentadicStore;

    /// Adapt the provided collections backend into a tetradic store.
    fn adapt_tetradic_from<C: Collections<T>>(&self, backend: C) -> Self::TetradicStore;

    /// Adapt the provided collections backend into a pentadic store.
    fn adapt_pentadic_from<C: Collections<T>>(&self, backend: C) -> Self::PentadicStore;
}

/// VecHyperStore: HyperStore implementation using Vec backend
///
/// This is the monadic foundation - Pure Being as Generic Being.
#[derive(Clone)]
pub struct VecHyperStore<T> {
    data: Vec<T>,
}

impl<T> VecHyperStore<T> {
    /// Create a new empty VecHyperStore
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Create a VecHyperStore with given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }
}

impl<T> Default for VecHyperStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> HyperStore<T> for VecHyperStore<T> {
    fn as_backend(&self) -> &dyn Collections<T> {
        // Delegate to Vec implementation
        // This is a placeholder - Vec itself implements Collections
        unimplemented!("VecHyperStore::as_backend needs proper implementation")
    }

    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyperstore_creation() {
        let store: VecHyperStore<i64> = VecHyperStore::new();
        assert_eq!(store.size(), 0);
        assert!(store.data.is_empty());
    }

    #[test]
    fn test_hyperstore_with_capacity() {
        let store: VecHyperStore<i64> = VecHyperStore::with_capacity(100);
        assert_eq!(store.capacity(), 100);
        assert_eq!(store.size(), 0);
    }
}
