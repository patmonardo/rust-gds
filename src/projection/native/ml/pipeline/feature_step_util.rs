/*
 * Copyright (c) "Neo4j"
 * Neo4j Sweden AB [http://neo4j.com]
 *
 * This file is part of Neo4j.
 *
 * Neo4j is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/FeatureStepUtil.java
//!
//! Utility functions for feature step operations in ML pipelines.
//!
//! Provides helpers for:
//! - Computing property dimensions (scalar vs array properties)
//! - Validating computed features for NaN values
//! - Formatting error messages for invalid features

use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;

/// Compute the dimension (feature count) of a node property.
///
/// Returns the number of features that a property contributes to a feature vector:
/// - Scalar properties (Long, Double): dimension = 1
/// - Array properties (DoubleArray, FloatArray, LongArray): dimension = array length
///
/// # Java Source
/// ```java
/// public static int propertyDimension(NodePropertyValues nodeProperties, String propertyName) {
///     int dimension = 0;
///     switch (nodeProperties.valueType()) {
///         case LONG:
///         case DOUBLE:
///             dimension = 1;
///             break;
///         case DOUBLE_ARRAY:
///         case FLOAT_ARRAY:
///             dimension = nodeProperties.doubleArrayValue(0).length;
///             break;
///         case LONG_ARRAY:
///             dimension = nodeProperties.longArrayValue(0).length;
///             break;
///         case UNKNOWN:
///             throw new IllegalStateException(formatWithLocale("Unknown ValueType %s", propertyName));
///     }
///     return dimension;
/// }
/// ```
pub fn property_dimension(
    node_properties: &dyn NodePropertyValues,
    property_name: &str,
) -> Result<usize, FeatureStepError> {
    // Use the dimension() method if available (for array types)
    if let Some(dim) = node_properties.dimension() {
        return Ok(dim);
    }

    // Otherwise, determine dimension from value type
    let dimension = match node_properties.value_type() {
        ValueType::Long | ValueType::Double => 1,
        ValueType::DoubleArray | ValueType::FloatArray | ValueType::LongArray => {
            // For array types, get the first element's array length
            // (all nodes should have same-dimensional properties)
            match node_properties.value_type() {
                ValueType::DoubleArray => node_properties
                    .double_array_value(0)
                    .map(|arr| arr.len())
                    .map_err(|e| FeatureStepError::PropertyAccessError {
                        property: property_name.to_string(),
                        message: e.to_string(),
                    })?,
                ValueType::FloatArray => node_properties
                    .float_array_value(0)
                    .map(|arr| arr.len())
                    .map_err(|e| FeatureStepError::PropertyAccessError {
                        property: property_name.to_string(),
                        message: e.to_string(),
                    })?,
                ValueType::LongArray => node_properties
                    .long_array_value(0)
                    .map(|arr| arr.len())
                    .map_err(|e| FeatureStepError::PropertyAccessError {
                        property: property_name.to_string(),
                        message: e.to_string(),
                    })?,
                _ => unreachable!(),
            }
        }
        _ => {
            return Err(FeatureStepError::UnknownValueType {
                property: property_name.to_string(),
            })
        }
    };

    Ok(dimension)
}

/// Validate that computed features do not contain NaN values.
///
/// Checks a slice of the feature vector for NaN values and runs the provided
/// error callback if any are found.
///
/// # Java Source
/// ```java
/// public static void validateComputedFeatures(
///     double[] linkFeatures,
///     int startOffset,
///     int endOffset,
///     Runnable throwError
/// ) {
///     for (int offset = startOffset; offset < endOffset; offset++) {
///         if (Double.isNaN(linkFeatures[offset])) {
///             throwError.run();
///         }
///     }
/// }
/// ```
pub fn validate_computed_features<F>(
    link_features: &[f64],
    start_offset: usize,
    end_offset: usize,
    throw_error: F,
) where
    F: FnOnce(),
{
    for offset in start_offset..end_offset {
        if link_features[offset].is_nan() {
            throw_error();
            return;
        }
    }
}

/// Create an error for NaN values in computed features.
///
/// # Java Source
/// ```java
/// public static void throwNanError(
///     String featureStep,
///     Collection<String> nodeProperties,
///     long source,
///     long target
/// ) {
///     throw new IllegalArgumentException(formatWithLocale(
///         "Encountered NaN when combining the nodeProperties %s for the node pair (%d, %d) when computing the %s feature vector. " +
///         "Either define a default value if its a stored property or check the nodePropertyStep.",
///         StringJoining.join(nodeProperties),
///         source,
///         target,
///         featureStep
///     ));
/// }
/// ```
pub fn throw_nan_error(
    feature_step: &str,
    node_properties: &[String],
    source: u64,
    target: u64,
) -> FeatureStepError {
    FeatureStepError::NanInFeatures {
        feature_step: feature_step.to_string(),
        node_properties: node_properties.to_vec(),
        source,
        target,
    }
}

/// Errors that can occur during feature step operations.
#[derive(Debug, Clone, PartialEq)]
pub enum FeatureStepError {
    /// Unknown or unsupported value type for a property
    UnknownValueType {
        /// Name of the property with unknown type
        property: String,
    },

    /// Error accessing property values
    PropertyAccessError {
        /// Name of the property that failed to access
        property: String,
        /// Error message from the underlying property system
        message: String,
    },

    /// NaN values encountered in computed features
    NanInFeatures {
        /// Name of the feature step that produced NaN
        feature_step: String,
        /// Node properties that were combined
        node_properties: Vec<String>,
        /// Source node ID
        source: u64,
        /// Target node ID
        target: u64,
    },
}

impl std::fmt::Display for FeatureStepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureStepError::UnknownValueType { property } => {
                write!(f, "Unknown ValueType for property: {}", property)
            }
            FeatureStepError::PropertyAccessError { property, message } => {
                write!(f, "Failed to access property '{}': {}", property, message)
            }
            FeatureStepError::NanInFeatures {
                feature_step,
                node_properties,
                source,
                target,
            } => {
                write!(
                    f,
                    "Encountered NaN when combining the nodeProperties [{}] for the node pair ({}, {}) when computing the {} feature vector. \
                    Either define a default value if its a stored property or check the nodePropertyStep.",
                    node_properties.join(", "),
                    source,
                    target,
                    feature_step
                )
            }
        }
    }
}

impl std::error::Error for FeatureStepError {}
