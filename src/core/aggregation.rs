use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

/// Aggregation strategies for merging property values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Aggregation {
    Default,
    None,
    Single,
    Sum,
    Min,
    Max,
    Count,
}

#[derive(Debug, Error)]
pub enum AggregationError {
    #[error("invalid aggregation: {0}")]
    Invalid(String),

    #[error("DEFAULT is a placeholder and cannot be used for merging")]
    DefaultPlaceholder,
}

impl Aggregation {
    /// Resolve DEFAULT to NONE, otherwise return self.
    pub fn resolve(self) -> Self {
        match self {
            Aggregation::Default => Aggregation::None,
            other => other,
        }
    }

    /// Merge a running total with a new value according to the aggregation.
    /// Returns Err for unsupported cases (e.g. DEFAULT / NONE).
    pub fn merge(self, running_total: f64, value: f64) -> Result<f64, AggregationError> {
        match self {
            Aggregation::Default => Err(AggregationError::DefaultPlaceholder),
            Aggregation::None => Err(AggregationError::Invalid(
                "NONE: multiple relationships between same nodes are not expected".into(),
            )),
            Aggregation::Single => Ok(running_total),
            Aggregation::Sum | Aggregation::Count => Ok(running_total + value),
            Aggregation::Min => Ok(running_total.min(value)),
            Aggregation::Max => Ok(running_total.max(value)),
        }
    }

    /// Normalize a property value for a given aggregation (COUNT -> 1.0).
    pub fn normalize_property_value(self, value: f64) -> f64 {
        if self == Aggregation::Count {
            1.0
        } else {
            value
        }
    }

    /// Return the empty (initial) value for this aggregation given a mapping default.
    pub fn empty_value(self, mapping_default_value: f64) -> f64 {
        match self {
            Aggregation::Sum | Aggregation::Count => {
                if mapping_default_value.is_nan() {
                    0.0
                } else {
                    mapping_default_value
                }
            }
            Aggregation::Min => {
                if mapping_default_value.is_nan() {
                    f64::INFINITY
                } else {
                    mapping_default_value
                }
            }
            Aggregation::Max => {
                if mapping_default_value.is_nan() {
                    f64::NEG_INFINITY
                } else {
                    mapping_default_value
                }
            }
            _ => mapping_default_value,
        }
    }

    /// True if aggregation behaves like NONE.
    pub fn equivalent_to_none(self) -> bool {
        self.resolve() == Aggregation::None
    }

    /// Convenience: try to compare with a string or numeric representation.
    pub fn equals_str(&self, other: &str) -> bool {
        match Aggregation::from_str(other) {
            Ok(parsed) => parsed == *self,
            Err(_) => false,
        }
    }

    /// Small numeric hash code (stable mapping).
    pub fn hash_code(self) -> u8 {
        match self {
            Aggregation::Default => 0,
            Aggregation::None => 1,
            Aggregation::Single => 2,
            Aggregation::Sum => 3,
            Aggregation::Min => 4,
            Aggregation::Max => 5,
            Aggregation::Count => 6,
        }
    }
}

impl FromStr for Aggregation {
    type Err = AggregationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "DEFAULT" => Ok(Aggregation::Default),
            "NONE" => Ok(Aggregation::None),
            "SINGLE" => Ok(Aggregation::Single),
            "SUM" => Ok(Aggregation::Sum),
            "MIN" => Ok(Aggregation::Min),
            "MAX" => Ok(Aggregation::Max),
            "COUNT" => Ok(Aggregation::Count),
            other => Err(AggregationError::Invalid(other.to_string())),
        }
    }
}

impl Display for Aggregation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Aggregation::Default => "DEFAULT",
            Aggregation::None => "NONE",
            Aggregation::Single => "SINGLE",
            Aggregation::Sum => "SUM",
            Aggregation::Min => "MIN",
            Aggregation::Max => "MAX",
            Aggregation::Count => "COUNT",
        };
        write!(f, "{}", s)
    }
}
