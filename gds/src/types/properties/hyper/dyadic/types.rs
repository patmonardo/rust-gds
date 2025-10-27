//! Dyadic types: pair of monadic properties

use crate::types::properties::hyper::monadic::MonadicProperty;

/// Canonical `Concepts` alias for Dyadic layer: (left, right)
pub type Concepts = (MonadicProperty, MonadicProperty);

/// Dyad identifier (opaque string for now)
pub type DyadId = String;

/// Small convenience wrapper for a dyadic pair
#[derive(Clone, Debug)]
pub struct DyadicProperty {
    left: MonadicProperty,
    right: MonadicProperty,
}

impl DyadicProperty {
    pub fn new(left: MonadicProperty, right: MonadicProperty) -> Self {
        Self { left, right }
    }

    pub fn left(&self) -> &MonadicProperty {
        &self.left
    }

    pub fn right(&self) -> &MonadicProperty {
        &self.right
    }
}
