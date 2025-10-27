use super::traits::TetradicHyperStore;
use super::types::{Concepts, DyadId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Simple in-memory reference implementation of the Tetradic contract.
#[derive(Clone, Debug, Default)]
pub struct InMemoryTetradicStore {
    store: Arc<Mutex<HashMap<DyadId, Concepts>>>,
}

impl InMemoryTetradicStore {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Helper: find ids where the left concept contains `needle` bytes.
    pub fn find_by_left_contains(&self, needle: &[u8]) -> Vec<DyadId> {
        let map = self.store.lock().expect("lock");
        map.iter()
            .filter_map(|(k, (left, _right))| {
                if left.windows(needle.len()).any(|w| w == needle) {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl TetradicHyperStore for InMemoryTetradicStore {
    fn insert(&self, id: DyadId, concepts: Concepts) {
        let mut map = self.store.lock().expect("lock");
        map.insert(id, concepts);
    }

    fn get(&self, id: &DyadId) -> Option<Concepts> {
        let map = self.store.lock().expect("lock");
        map.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get_roundtrip() {
        let s = InMemoryTetradicStore::new();
        s.insert("r1".to_string(), (b"f".to_vec(), b"l".to_vec()));
        let got = s.get(&"r1".to_string()).expect("present");
        assert_eq!(got.0, b"f".to_vec());
        assert_eq!(got.1, b"l".to_vec());
    }

    #[test]
    fn find_by_left_works() {
        let s = InMemoryTetradicStore::new();
        s.insert("a".to_string(), (b"abcd".to_vec(), b"x".to_vec()));
        let ids = s.find_by_left_contains(b"bc");
        assert_eq!(ids, vec!["a".to_string()]);
    }
}
