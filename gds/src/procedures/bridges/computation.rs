//! Bridges Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.bridges.Bridges`
//!
//! This module implements Tarjan's algorithm for finding bridges using iterative DFS.

use crate::collections::{HugeLongArray, BitSet};
use crate::core::utils::paged::HugeLongArrayStack;

/// Stack event for iterative DFS
#[derive(Debug, Clone, Copy)]
pub struct StackEvent {
    pub event_node: i32,
    pub trigger_node: i32,
    pub last_visit: bool,
}

impl StackEvent {
    pub fn upcoming_visit(node: i32, trigger_node: i32) -> Self {
        Self {
            event_node: node,
            trigger_node,
            last_visit: false,
        }
    }

    pub fn last_visit(node: i32, trigger_node: i32) -> Self {
        Self {
            event_node: node,
            trigger_node,
            last_visit: true,
        }
    }
}

/// Bridge edge
#[derive(Debug, Clone)]
pub struct Bridge {
    pub from: u64,
    pub to: u64,
}

/// Bridges computation result
#[derive(Clone)]
pub struct BridgesComputationResult {
    pub bridges: Vec<Bridge>,
}

/// Bridges computation runtime
pub struct BridgesComputationRuntime {
    visited: BitSet,
    tin: HugeLongArray,
    low: HugeLongArray,
    timer: i64,
    bridges: Vec<Bridge>,
}

impl BridgesComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            visited: BitSet::new(node_count),
            tin: HugeLongArray::new(node_count),
            low: HugeLongArray::new(node_count),
            timer: 0,
            bridges: Vec::new(),
        }
    }

    /// Compute bridges for a graph
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> BridgesComputationResult {
        self.timer = 0;
        self.bridges.clear();
        
        // Initialize tin and low to -1
        for i in 0..node_count {
            self.tin.set(i, -1);
            self.low.set(i, -1);
        }

        // Process each unvisited node
        for i in 0..node_count {
            if !self.visited.get(i) {
                self.dfs(i as i32, -1, &get_neighbors);
            }
        }

        BridgesComputationResult {
            bridges: self.bridges.clone(),
        }
    }

    fn dfs(&mut self, start_node: i32, parent: i32, get_neighbors: &impl Fn(usize) -> Vec<usize>) {
        // Use HugeLongArrayStack to store events
        let mut stack = HugeLongArrayStack::new(100000);
        
        // Push initial event
        stack.push(encode_event(StackEvent::upcoming_visit(start_node, parent)));

        while !stack.is_empty() {
            let encoded = stack.pop();
            let event = decode_event(encoded);

            if event.last_visit {
                // Last visit - process backtracking
                let v = event.trigger_node;
                let to = event.event_node;
                
                let low_v = self.low.get(v as usize);
                let low_to = self.low.get(to as usize);
                self.low.set(v as usize, std::cmp::min(low_v, low_to));
                
                let tin_v = self.tin.get(v as usize);
                if low_to > tin_v {
                    // This is a bridge
                    self.bridges.push(Bridge {
                        from: std::cmp::min(v as u64, to as u64),
                        to: std::cmp::max(v as u64, to as u64),
                    });
                }
            } else {
                // First visit - process node
                let v = event.event_node;
                let p = event.trigger_node;
                
                if !self.visited.get(v as usize) {
                    self.visited.set(v as usize);
                    self.tin.set(v as usize, self.timer);
                    self.low.set(v as usize, self.timer);
                    self.timer += 1;

                    // Push post-visit event if not root
                    if p != -1 {
                        stack.push(encode_event(StackEvent::last_visit(v, p)));
                    }

                    // Process neighbors
                    let neighbors = get_neighbors(v as usize);
                    for to in neighbors {
                        let to_i32 = to as i32;
                        if to_i32 == p {
                            continue;
                        }
                        stack.push(encode_event(StackEvent::upcoming_visit(to_i32, v)));
                    }
                } else if p != -1 {
                    // Back edge - update low value
                    let low_v = self.low.get(p as usize);
                    let tin_v = self.tin.get(v as usize);
                    self.low.set(p as usize, std::cmp::min(low_v, tin_v));
                }
            }
        }
    }
}

/// Encode StackEvent into i64 for storage in HugeLongArrayStack
fn encode_event(event: StackEvent) -> i64 {
    let mut encoded: u64 = 0;
    // Use bottom 16 bits for event_node
    encoded |= ((event.event_node as u32) & 0xFFFF) as u64;
    // Use next 16 bits for trigger_node
    encoded |= (((event.trigger_node as u32) & 0xFFFF) as u64) << 16;
    // Use next bit for last_visit flag
    if event.last_visit {
        encoded |= 1u64 << 32;
    }
    encoded as i64
}

/// Decode i64 back into StackEvent
fn decode_event(encoded: i64) -> StackEvent {
    let encoded_u64 = encoded as u64;
    let event_node_bits = (encoded_u64 & 0xFFFF) as u16;
    let trigger_node_bits = ((encoded_u64 >> 16) & 0xFFFF) as u16;
    let last_visit = (encoded_u64 >> 32) & 1 != 0;
    
    // Sign extend from u16 to i32
    let event_node = (event_node_bits as i16) as i32;
    let trigger_node = (trigger_node_bits as i16) as i32;
    
    StackEvent {
        event_node,
        trigger_node,
        last_visit,
    }
}
