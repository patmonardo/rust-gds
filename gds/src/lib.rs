//! Rust GDS - Graph Data Science library
//!
//! A modular graph data structure and algorithms library.
#![allow(ambiguous_wide_pointer_comparisons)]

// pub mod applications;
pub mod all_pairs_shortest_path;
pub mod approx_max_k_cut;
pub mod articulationpoints;
pub mod betweenness;
pub mod bridges;
pub mod closeness;
pub mod collections;
pub mod concurrency;
pub mod degree;
pub mod edge_splitter;
pub mod fast_rp;
pub mod graph_coloring;
pub mod graph_sage;
pub mod harmonic;
pub mod hits;
pub mod indirect_exposure;
pub mod influence_maximization;
pub mod k1_coloring;
pub mod k_core;
pub mod k_means;
pub mod kge;
pub mod label_propagation;
pub mod leiden;
pub mod lcc;
pub mod link_prediction;
pub mod logging;
pub mod louvain;
pub mod modularity_optimization;
pub mod node2vec;
pub mod node_similarity;
pub mod pagerank;
pub mod random_walk;
pub mod scc;
pub mod shortest_path;
pub mod single_source_shortest_path;
pub mod speaker_listener_lpa;
pub mod triangle_count;
pub mod wcc;
pub mod yens_k_shortest_paths;

// Ensure projection (codegen macros) is compiled before config so macro_rules! defs
// inside projection::codegen are available to config modules that invoke them.
pub mod config;
pub mod core;
pub mod errors;
pub mod form;
pub mod mem;
pub mod ml;
pub mod pregel;
pub mod procedures;
pub mod projection;
pub mod termination;
pub mod types;
pub mod util;
pub mod values;
pub use core::*;
