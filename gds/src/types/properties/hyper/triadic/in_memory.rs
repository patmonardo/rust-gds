use super::traits::TriadicHyperStore;
use super::types::{Concepts, TriadId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Simple in-memory triadic store using the `Concepts` alias (meta, node, link).
#[derive(Clone, Debug, Default)]
pub struct InMemoryTriadicStore {
    inner: Arc<Mutex<HashMap<TriadId, Concepts>>>,
}

impl InMemoryTriadicStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl TriadicHyperStore for InMemoryTriadicStore {
    fn insert(&self, id: TriadId, concepts: Concepts) {
        let mut map = self.inner.lock().expect("lock");
        map.insert(id, concepts);
    }

    fn get(&self, id: &TriadId) -> Option<Concepts> {
        let map = self.inner.lock().expect("lock");
        map.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::VecLong;
    use crate::types::properties::hyper::monadic::property_values::MonadicLongPropertyValues;
    use crate::types::properties::hyper::monadic::MonadicProperty;
    use crate::types::properties::property::Property;
    use std::sync::Arc;

    #[test]
    fn triadic_insert_get_roundtrip() {
        let store = InMemoryTriadicStore::new();
        let value = VecLong::from(vec![1i64]);
        let pm = MonadicProperty::of("meta", Arc::new(MonadicLongPropertyValues::new(value, 0)));
        // use same monadic property for all three levels in this simple test
        store.insert("t1".to_string(), (pm.clone(), pm.clone(), pm));

        let got = store.get(&"t1".to_string()).expect("present");
        assert_eq!(got.0.schema().key(), "meta");
    }
}
