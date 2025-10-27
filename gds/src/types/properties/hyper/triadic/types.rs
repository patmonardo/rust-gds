//! Triadic types (shared aliases)

use crate::types::properties::hyper::monadic::MonadicProperty;

/// Canonical `Concepts` alias for Triadic layer: (meta, node, link)
pub type Concepts = (MonadicProperty, MonadicProperty, MonadicProperty);

/// Identifier for triadic entries (opaque string for now)
pub type TriadId = String;
