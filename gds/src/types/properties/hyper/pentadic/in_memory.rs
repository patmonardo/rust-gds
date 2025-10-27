use super::traits::PentadicHyperStore;
use super::types::{AssertionId, Concepts};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct InMemoryPentadicStore {
    inner: Arc<Mutex<HashMap<AssertionId, Concepts>>>,
}

impl InMemoryPentadicStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Helper to create a new auto-generated id and insert the concepts.
    pub fn assert_new(&self, concepts: Concepts) -> AssertionId {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let id = format!("assertion-{}", COUNTER.fetch_add(1, Ordering::SeqCst));
        let mut map = self.inner.lock().expect("lock");
        map.insert(id.clone(), concepts);
        id
    }
}

impl PentadicHyperStore for InMemoryPentadicStore {
    fn insert(&self, id: AssertionId, concepts: Concepts) {
        let mut map = self.inner.lock().expect("lock");
        map.insert(id, concepts);
    }

    fn get(&self, id: &AssertionId) -> Option<Concepts> {
        let map = self.inner.lock().expect("lock");
        map.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_assert() {
        let s = InMemoryPentadicStore::new();
        let id = s.assert_new(("a".to_string(), "b".to_string(), 0.5));
        let got = s.get(&id).expect("present");
        assert_eq!(got.0, "a");
        assert_eq!(got.1, "b");
        assert!((got.2 - 0.5).abs() < 1e-6);
    }
}
