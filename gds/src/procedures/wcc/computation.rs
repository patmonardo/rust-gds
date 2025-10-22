//! WCC Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.wcc.Wcc`
//!
//! This module implements Weakly Connected Components using union-find (disjoint-set).

use crate::collections::HugeLongArray;

/// Union-Find data structure for WCC
pub struct UnionFind {
    parent: HugeLongArray,
}

impl UnionFind {
    pub fn new(node_count: usize) -> Self {
        let mut parent = HugeLongArray::new(node_count);
        for i in 0..node_count {
            parent.set(i, i as i64);
        }
        Self { parent }
    }

    fn find_root(&self, mut x: usize) -> usize {
        loop {
            let p = self.parent.get(x) as usize;
            if p == x {
                return x;
            }
            x = p;
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find_root(x);
        let root_y = self.find_root(y);
        if root_x != root_y {
            self.parent.set(root_x, root_y as i64);
        }
    }

    pub fn get_components(&self, node_count: usize) -> (Vec<u64>, usize) {
        let mut components = vec![0u64; node_count];
        let mut component_map: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        let mut next_component_id = 0usize;

        for i in 0..node_count {
            let root = self.find_root(i);
            if !component_map.contains_key(&root) {
                component_map.insert(root, next_component_id);
                next_component_id += 1;
            }
            components[i] = *component_map.get(&root).unwrap() as u64;
        }

        (components, next_component_id)
    }
}

/// WCC computation result
#[derive(Clone)]
pub struct WccComputationResult {
    pub components: Vec<u64>,
    pub component_count: usize,
}

/// WCC computation runtime
pub struct WccComputationRuntime {
    // Placeholder for runtime state if needed
}

impl WccComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    /// Compute weakly connected components for a graph
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> WccComputationResult {
        let mut union_find = UnionFind::new(node_count);

        // Process all edges
        for node in 0..node_count {
            let neighbors = get_neighbors(node);
            for neighbor in neighbors {
                union_find.union(node, neighbor);
            }
        }

        let (components, component_count) = union_find.get_components(node_count);

        WccComputationResult {
            components,
            component_count,
        }
    }
}
