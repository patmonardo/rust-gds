use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::{CatalogError, Dropped, GraphCatalog, GraphMemoryUsage, ListEntry};
use crate::types::graph::{degrees::Degrees, id_map::IdMap};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

#[derive(Default)]
pub struct InMemoryGraphCatalog {
    entries: RwLock<HashMap<String, Arc<DefaultGraphStore>>>,
}

impl InMemoryGraphCatalog {
    pub fn new() -> Self {
        Self { entries: RwLock::new(HashMap::new()) }
    }
}

impl GraphCatalog for InMemoryGraphCatalog {
    fn set(&self, name: &str, store: Arc<DefaultGraphStore>) {
        let mut map = self.entries.write().expect("catalog poisoned");
        map.insert(name.to_string(), store);
    }

    fn get(&self, name: &str) -> Option<Arc<DefaultGraphStore>> {
        let map = self.entries.read().ok()?;
        map.get(name).cloned()
    }

    fn drop(&self, names: &[&str], fail_if_missing: bool) -> Result<Vec<Dropped>, CatalogError> {
        let mut map = self.entries.write().expect("catalog poisoned");
        let mut dropped = Vec::with_capacity(names.len());
        for n in names {
            match map.remove(*n) {
                Some(store) => dropped.push(Dropped {
                    name: n.to_string(),
                    node_count: GraphStore::node_count(store.as_ref()) as u64,
                    relationship_count: GraphStore::relationship_count(store.as_ref()) as u64,
                }),
                None if fail_if_missing => return Err(CatalogError::NotFound((*n).to_string())),
                None => {}
            }
        }
        Ok(dropped)
    }

    fn list(&self, filter: Option<&str>, include_degree_dist: bool) -> Vec<ListEntry> {
        let map = self.entries.read().expect("catalog poisoned");
        let iter = map.iter().filter(|(name, _)| match filter {
            Some(f) => name.as_str() == f,
            None => true,
        });
        iter.map(|(name, store)| ListEntry {
            name: name.clone(),
            node_count: GraphStore::node_count(store.as_ref()) as u64,
            relationship_count: GraphStore::relationship_count(store.as_ref()) as u64,
            degree_distribution: if include_degree_dist { Some(simple_degree_histogram(store)) } else { None },
        }).collect()
    }

    fn size_of(&self, name: &str) -> Result<GraphMemoryUsage, CatalogError> {
        let map = self.entries.read().expect("catalog poisoned");
        let store = map.get(name).ok_or_else(|| CatalogError::NotFound(name.to_string()))?;
        // Placeholder memory accounting; can be replaced with real tracker later
        Ok(GraphMemoryUsage {
            bytes: 0,
            nodes: GraphStore::node_count(store.as_ref()) as u64,
            relationships: GraphStore::relationship_count(store.as_ref()) as u64,
        })
    }
}

fn simple_degree_histogram(store: &DefaultGraphStore) -> HashMap<u32, u64> {
    let mut hist = HashMap::new();
    let graph = store.graph();
    let n = IdMap::node_count(graph.as_ref());
    for node_id in 0..n {
        let deg = Degrees::degree(graph.as_ref(), node_id as i64) as u32;
        *hist.entry(deg).or_insert(0) += 1;
    }
    hist
}


