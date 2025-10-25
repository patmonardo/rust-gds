//! Triadic PropertyStore: Three-Level HyperPropertyStore
//!
//! # Architecture Vision
//!
//! The `TriadicPropertyStore` demonstrates a universal pattern for composing
//! multiple storage contexts into a unified system. It's a "HyperPropertyStore"
//! that recognizes nuanced context across three independent levels.
//!
//! ## The Universal Three-Level Pattern
//!
//! ```text
//! MonadicPropertyStore (Universal Standard)
//!     ↓ compose into
//! TriadicPropertyStore (HyperPropertyStore - Context-Aware)
//!     ├── Level 0: meta_properties    (System/Graph metadata)
//!     ├── Level 1: node_properties    (Entity/Element properties)
//!     └── Level 2: link_properties    (Connection/Relationship properties)
//! ```
//!
//! ## Key Insights
//!
//! ### 1. MonadicPropertyStore = Universal Building Block
//!
//! - Collections First backend
//! - Works for ANY domain
//! - Simple, composable, testable
//!
//! ### 2. TriadicPropertyStore = Composition Pattern
//!
//! - Composes THREE independent MonadicPropertyStores
//! - Each level has its own key space (can all have "name")
//! - Each level can use DIFFERENT Collections backends
//! - Different optimization strategies per level
//!
//! ### 3. Universal Pattern (Not Just Graphs!)
//!
//! This Meta/Node/Link pattern transcends graphs:
//! - **File Systems**: volume metadata / file metadata / directory links
//! - **Databases**: database metadata / table metadata / foreign keys
//! - **Networks**: system config / host properties / connection properties
//! - **ML Pipelines**: model metadata / sample features / batch metadata
//!
//! ## Design Principles
//!
//! ### Separate Key Spaces
//!
//! Each level maintains independent keys:
//! ```text
//! meta_properties.get("name")   → "MyGraph"
//! node_properties.get("name")   → vec!["Alice", "Bob", "Carol"]
//! link_properties.get("name")   → vec!["friend", "knows", "likes"]
//! ```
//!
//! ### Independent Backends
//!
//! Each level can optimize independently:
//! ```text
//! meta_properties   → Vec (small, rarely accessed)
//! node_properties   → HugeArray (billions of elements)
//! link_properties   → HugeArray (trillions of edges)
//! ```
//!
//! ### Composition Over Inheritance
//!
//! Rather than building a complex unified store, we compose simple stores:
//! - Each store is independently testable
//! - Each store follows Collections First
//! - The composition is just delegation
//!
//! ## Relationship to GraphStore
//!
//! This is an EXPERIMENTAL module to explore how `GraphStore` could leverage
//! triadic composition. The current `GraphStore` intermingles node, relationship,
//! and graph property access. This explores separating them while maintaining
//! a unified interface.
//!
//! Future GraphStore could either:
//! 1. Use TriadicPropertyStore internally
//! 2. Delegate to three separate MonadicPropertyStores
//! 3. Act as a smart facade over triadic storage
//!
//! ## Example Usage
//!
//! See `examples/triadic_property_store_demo.rs` for a complete demonstration.

pub mod property_store;

pub use property_store::{TriadicPropertyStore, TriadicPropertyStoreBuilder};

