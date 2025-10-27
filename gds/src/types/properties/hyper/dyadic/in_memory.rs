use super::traits::DyadicHyperStore;
use super::types::{Concepts, DyadId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Default)]
pub struct InMemoryDyadicStore {
    inner: Arc<Mutex<HashMap<DyadId, Concepts>>>,
}

impl InMemoryDyadicStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl DyadicHyperStore for InMemoryDyadicStore {
    fn insert(&self, id: DyadId, concepts: Concepts) {
        let mut m = self.inner.lock().expect("lock");
        m.insert(id, concepts);
    }

    fn get(&self, id: &DyadId) -> Option<Concepts> {
        let m = self.inner.lock().expect("lock");
        m.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::backends::vec::VecLong;
    use crate::types::properties::hyper::monadic::MonadicLongPropertyValues;
    use crate::types::properties::hyper::monadic::MonadicProperty;
    use crate::types::properties::property::Property;
    use std::sync::Arc;

    #[test]
    fn dyadic_roundtrip() {
        let s = InMemoryDyadicStore::new();
        let left = MonadicProperty::of(
            "l",
            Arc::new(MonadicLongPropertyValues::new(VecLong::from(vec![1i64]), 0)),
        );
        let right = MonadicProperty::of(
            "r",
            Arc::new(MonadicLongPropertyValues::new(VecLong::from(vec![2i64]), 0)),
        );
        s.insert("d1".to_string(), (left.clone(), right.clone()));
        let got = s.get(&"d1".to_string()).expect("present");
        assert_eq!(got.0.schema().key(), "l");
        assert_eq!(got.1.schema().key(), "r");
    }
}
