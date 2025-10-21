//! Stub Traits for Missing Dependencies
//!
//! **Status**: TEMPORARY - Will be replaced when translating core API
//!
//! This module provides stub trait definitions for types that are referenced by
//! algorithm infrastructure but not yet fully translated from Java GDS.
//!
//! ## Why Stubs?
//!
//! The Java GDS `algo` package depends on:
//! - `org.neo4j.gds.api.properties.nodes.*` - Node property value accessors
//! - `org.neo4j.gds.api.Graph` - Graph interface
//! - `org.neo4j.gds.api.GraphStore` - Graph storage
//!
//! These are part of the core API and will be translated separately. For now, we
//! provide minimal stub traits to allow algorithm infrastructure to compile.
//!
//! ## When Will Stubs Be Replaced?
//!
//! These stubs will be replaced when we translate:
//! 1. `core/api/properties/` - Property value accessors
//! 2. `core/api/graph/` - Graph interfaces
//! 3. `core/loading/` - Graph loading and storage
//!
//! ## Usage
//!
//! ```rust,ignore
//! use gds::procedures::algorithms::stubs::*;
//!
//! // Use stub traits in algorithm infrastructure
//! fn process_properties(props: &dyn LongNodePropertyValues) {
//!     for node_id in 0..props.node_count() {
//!         if props.has_value(node_id) {
//!             let value = props.long_value(node_id);
//!             // ... process value
//!         }
//!     }
//! }
//! ```
//!
//! ## DO NOT USE IN PRODUCTION
//!
//! These are **stub implementations only**. They provide the minimal interface
//! needed for algorithm infrastructure to compile. Real implementations will come
//! from the core API translation.

pub mod node_property_values;

pub use node_property_values::*;

