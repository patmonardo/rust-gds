use super::traits::MonadicHyperStore;
use super::types::MonadicProperty;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Simple in-memory monadic store mapping a property key to its `MonadicProperty`.
#[derive(Clone, Debug, Default)]
pub struct InMemoryMonadicStore {
    inner: Arc<Mutex<HashMap<String, MonadicProperty>>>,
}

impl InMemoryMonadicStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn put(&self, key: String, prop: MonadicProperty) {
        let mut m = self.inner.lock().expect("lock");
        m.insert(key, prop);
    }

    pub fn get(&self, key: &str) -> Option<MonadicProperty> {
        let m = self.inner.lock().expect("lock");
        m.get(key).cloned()
    }
}

impl MonadicHyperStore for InMemoryMonadicStore {}
