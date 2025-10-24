use std::collections::{HashMap, HashSet};

use crate::types::schema::NodeLabel;

use super::{
    batch_node_iterable::{BatchNodeIterable, NodeIdBatch},
    id_map::{IdMap, NodeLabelConsumer},
    node_iterator::{NodeIdIterator, NodeIterator},
    partial_id_map::PartialIdMap,
    Concurrency, FilteredIdMap, MappedNodeId, OriginalNodeId,
};

/// Simple in-memory [`IdMap`] implementation built on top of Rust `HashMap`s.
///
/// This mirrors the minimal behaviour that existed in the original `id_map.rs` file and
/// serves as a reference implementation until a more specialised store is introduced.
#[derive(Debug, Clone, Default)]
pub struct SimpleIdMap {
    forward: HashMap<OriginalNodeId, MappedNodeId>,
    reverse: Vec<OriginalNodeId>,
    labels_by_node: HashMap<MappedNodeId, HashSet<NodeLabel>>,
    available_labels: HashSet<NodeLabel>,
}

impl SimpleIdMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_original_ids<I>(ids: I) -> Self
    where
        I: IntoIterator<Item = OriginalNodeId>,
    {
        let mut forward = HashMap::new();
        let mut reverse = Vec::new();
        for (index, original) in ids.into_iter().enumerate() {
            let mapped = index as MappedNodeId;
            forward.insert(original, mapped);
            reverse.push(original);
        }

        Self {
            forward,
            reverse,
            ..Default::default()
        }
    }

    fn mapped_range(&self) -> std::ops::Range<MappedNodeId> {
        0..self.reverse.len() as MappedNodeId
    }
}

impl PartialIdMap for SimpleIdMap {
    fn to_mapped_node_id(&self, original_node_id: OriginalNodeId) -> Option<MappedNodeId> {
        self.forward.get(&original_node_id).copied()
    }

    fn root_node_count(&self) -> Option<usize> {
        Some(self.reverse.len())
    }
}

impl NodeIterator for SimpleIdMap {
    fn for_each_node(&self, consumer: &mut dyn super::NodeConsumer) {
        for mapped in self.mapped_range() {
            if !consumer.accept(mapped) {
                break;
            }
        }
    }

    fn iter(&self) -> NodeIdIterator<'_> {
        Box::new(self.mapped_range())
    }

    fn iter_with_labels<'a>(&'a self, labels: &'a HashSet<NodeLabel>) -> NodeIdIterator<'a> {
        if labels.is_empty() {
            return self.iter();
        }

        Box::new(self.mapped_range().filter(move |node_id| {
            self.labels_by_node
                .get(node_id)
                .map(|node_labels| labels.iter().any(|label| node_labels.contains(label)))
                .unwrap_or(false)
        }))
    }
}

impl BatchNodeIterable for SimpleIdMap {
    fn batch_iterables(&self, batch_size: usize) -> Vec<NodeIdBatch> {
        if batch_size == 0 {
            return vec![];
        }

        let mut batches = Vec::new();
        let mut start = 0u64;
        let total = self.node_count() as u64;
        while start < total {
            let remaining = total - start;
            let length = usize::min(batch_size, remaining as usize);
            batches.push(NodeIdBatch::new(start as i64, length));
            start += length as u64;
        }
        batches
    }
}

impl IdMap for SimpleIdMap {
    fn type_id(&self) -> &str {
        "simple"
    }

    fn safe_to_mapped_node_id(&self, original_node_id: OriginalNodeId) -> Option<MappedNodeId> {
        self.to_mapped_node_id(original_node_id)
    }

    fn to_original_node_id(&self, mapped_node_id: MappedNodeId) -> Option<OriginalNodeId> {
        self.reverse.get(mapped_node_id as usize).copied()
    }

    fn to_root_node_id(&self, mapped_node_id: MappedNodeId) -> Option<MappedNodeId> {
        Some(mapped_node_id)
    }

    fn node_count(&self) -> usize {
        self.reverse.len()
    }

    fn node_count_for_label(&self, node_label: &NodeLabel) -> usize {
        self.labels_by_node
            .values()
            .filter(|labels| labels.contains(node_label))
            .count()
    }

    fn highest_original_id(&self) -> Option<OriginalNodeId> {
        self.forward.keys().copied().max()
    }

    fn node_labels(&self, mapped_node_id: MappedNodeId) -> HashSet<NodeLabel> {
        self.labels_by_node
            .get(&mapped_node_id)
            .cloned()
            .unwrap_or_default()
    }

    fn for_each_node_label(
        &self,
        mapped_node_id: MappedNodeId,
        consumer: &mut dyn NodeLabelConsumer,
    ) {
        if let Some(labels) = self.labels_by_node.get(&mapped_node_id) {
            for label in labels {
                if !consumer.accept(label) {
                    break;
                }
            }
        }
    }

    fn available_node_labels(&self) -> HashSet<NodeLabel> {
        self.available_labels.clone()
    }

    fn has_label(&self, mapped_node_id: MappedNodeId, label: &NodeLabel) -> bool {
        self.labels_by_node
            .get(&mapped_node_id)
            .map(|labels| labels.contains(label))
            .unwrap_or(false)
    }

    fn add_node_label(&mut self, node_label: NodeLabel) {
        self.available_labels.insert(node_label);
    }

    fn add_node_id_to_label(&mut self, node_id: MappedNodeId, node_label: NodeLabel) {
        self.available_labels.insert(node_label.clone());
        self.labels_by_node
            .entry(node_id)
            .or_default()
            .insert(node_label);
    }

    fn root_id_map(&self) -> &dyn IdMap {
        self
    }

    fn with_filtered_labels(
        &self,
        _node_labels: &HashSet<NodeLabel>,
        _concurrency: Concurrency,
    ) -> Option<Box<dyn FilteredIdMap>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_mapping() {
        let map = SimpleIdMap::from_original_ids([10, 20, 30]);
        assert_eq!(map.to_mapped_node_id(10), Some(0));
        assert_eq!(map.to_mapped_node_id(20), Some(1));
        assert_eq!(map.to_mapped_node_id(30), Some(2));
        assert_eq!(map.to_original_node_id(1), Some(20));
        assert_eq!(map.node_count(), 3);
        assert_eq!(map.root_node_count(), Some(3));
    }

    #[test]
    fn node_iteration() {
        let map = SimpleIdMap::from_original_ids([5, 6]);
        let nodes: Vec<_> = map.iter().collect();
        assert_eq!(nodes, vec![0, 1]);

        let mut collected = Vec::new();
        let mut consumer = |node| {
            collected.push(node);
            true
        };
        map.for_each_node(&mut consumer);
        assert_eq!(collected, vec![0, 1]);
    }

    #[test]
    fn batches() {
        let map = SimpleIdMap::from_original_ids([1, 2, 3, 4, 5]);
        let batches = map.batch_iterables(2);
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0], NodeIdBatch::new(0, 2));
        assert_eq!(batches[1], NodeIdBatch::new(2, 2));
        assert_eq!(batches[2], NodeIdBatch::new(4, 1));
    }

    #[test]
    fn labels() {
        let mut map = SimpleIdMap::from_original_ids([1, 2]);
        let label = NodeLabel::of("Person");
        map.add_node_label(label.clone());
        map.add_node_id_to_label(0, label.clone());
        assert!(map.has_label(0, &label));
        assert_eq!(map.node_count_for_label(&label), 1);

        let labels = map.node_labels(0);
        assert!(labels.contains(&label));
    }
}
