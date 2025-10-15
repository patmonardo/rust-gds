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

//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/NonEmptySetValidation.java
//!
//! Validation utilities for ensuring dataset splits have sufficient elements.
//!
//! Defines minimum size requirements for different dataset splits:
//! - MIN_SET_SIZE: General minimum (1)
//! - MIN_TRAIN_SET_SIZE: Training set (2) - needs splitting for cross-validation
//! - MIN_TEST_COMPLEMENT_SET_SIZE: Test complement (3) - needs splitting into train + feature-input

/// Minimum size for a general dataset (must have at least 1 element).
pub const MIN_SET_SIZE: usize = 1;

/// Minimum size for a training set (must have at least 2 elements).
///
/// Training sets require at least 2 elements because they will be further
/// split during cross-validation.
pub const MIN_TRAIN_SET_SIZE: usize = 2;

/// Minimum size for a test-complement set (must have at least 3 elements).
///
/// Test-complement sets require at least 3 elements because they need to be
/// split into separate train and feature-input sets.
pub const MIN_TEST_COMPLEMENT_SET_SIZE: usize = 3;

/// Validate that a node set has sufficient size.
///
/// # Java Source
/// ```java
/// public static void validateNodeSetSize(
///     long numberNodesInSet,
///     long minNumberNodes,
///     String setName,
///     String parameterName
/// ) {
///     validateElementSetIsNotEmpty(numberNodesInSet, minNumberNodes, setName, parameterName, "node(s)");
/// }
/// ```
pub fn validate_node_set_size(
    number_nodes_in_set: usize,
    min_number_nodes: usize,
    set_name: &str,
    parameter_name: &str,
) -> Result<(), ValidationError> {
    validate_element_set_is_not_empty(
        number_nodes_in_set,
        min_number_nodes,
        set_name,
        parameter_name,
        "node(s)",
    )
}

/// Validate that a relationship set has sufficient size.
///
/// # Java Source
/// ```java
/// public static void validateRelSetSize(
///     long numberNodesInSet,
///     long minNumberNodes,
///     String errorDesc,
///     String parameterName
/// ) {
///     validateElementSetIsNotEmpty(numberNodesInSet, minNumberNodes, errorDesc, parameterName, "relationship(s)");
/// }
/// ```
pub fn validate_rel_set_size(
    number_rels_in_set: usize,
    min_number_rels: usize,
    error_desc: &str,
    parameter_name: &str,
) -> Result<(), ValidationError> {
    validate_element_set_is_not_empty(
        number_rels_in_set,
        min_number_rels,
        error_desc,
        parameter_name,
        "relationship(s)",
    )
}

/// Internal helper to validate element set size.
///
/// # Java Source
/// ```java
/// private static void validateElementSetIsNotEmpty(
///     long elementsInSet,
///     long expectedMinNumberOfElements,
///     String errorDesc,
///     String parameterName,
///     String elementType
/// ) {
///     if (elementsInSet < expectedMinNumberOfElements) {
///         throw new IllegalArgumentException(formatWithLocale(
///             "The specified %s for the current graph. " +
///             "The %s set would have %d %s " +
///             "but it must have at least %d.",
///             parameterName, errorDesc, elementsInSet, elementType, expectedMinNumberOfElements
///         ));
///     }
/// }
/// ```
fn validate_element_set_is_not_empty(
    elements_in_set: usize,
    expected_min_number_of_elements: usize,
    error_desc: &str,
    parameter_name: &str,
    element_type: &str,
) -> Result<(), ValidationError> {
    if elements_in_set < expected_min_number_of_elements {
        return Err(ValidationError::InsufficientSetSize {
            parameter_name: parameter_name.to_string(),
            error_desc: error_desc.to_string(),
            elements_in_set,
            element_type: element_type.to_string(),
            expected_min: expected_min_number_of_elements,
        });
    }
    Ok(())
}

/// Errors that can occur during dataset validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// Dataset split has insufficient elements
    InsufficientSetSize {
        /// Name of the parameter that caused the issue
        parameter_name: String,
        /// Description of the error condition
        error_desc: String,
        /// Actual number of elements in the set
        elements_in_set: usize,
        /// Type of elements (e.g., "node(s)", "relationship(s)")
        element_type: String,
        /// Minimum required number of elements
        expected_min: usize,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InsufficientSetSize {
                parameter_name,
                error_desc,
                elements_in_set,
                element_type,
                expected_min,
            } => {
                write!(
                    f,
                    "The specified {} for the current graph. \
                    The {} set would have {} {} \
                    but it must have at least {}.",
                    parameter_name, error_desc, elements_in_set, element_type, expected_min
                )
            }
        }
    }
}

impl std::error::Error for ValidationError {}
