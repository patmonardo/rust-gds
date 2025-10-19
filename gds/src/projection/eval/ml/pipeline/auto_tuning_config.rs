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

//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/AutoTuningConfig.java
//!
//! Configuration for hyperparameter auto-tuning during model training.
//!
//! Controls how many different parameter combinations should be tried
//! when searching for optimal hyperparameters.

use std::collections::HashMap;

/// Configuration for automatic hyperparameter tuning.
///
/// Specifies how many trials (different parameter combinations) should be
/// evaluated when searching for optimal model hyperparameters.
///
/// # Java Source
/// ```java
/// @Configuration
/// public interface AutoTuningConfig extends ToMapConvertible {
///     int MAX_TRIALS = 10;
///
///     @Configuration.IntegerRange(min = 1)
///     default int maxTrials() {
///         return MAX_TRIALS;
///     }
///
///     @Override
///     @Configuration.ToMap
///     Map<String, Object> toMap();
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutoTuningConfig {
    /// Maximum number of hyperparameter combinations to try (must be >= 1)
    max_trials: usize,
}

impl AutoTuningConfig {
    /// Default maximum number of trials
    pub const MAX_TRIALS: usize = 10;

    /// Create a new auto-tuning configuration.
    ///
    /// # Arguments
    /// * `max_trials` - Maximum number of trials (must be >= 1)
    ///
    /// # Errors
    /// Returns `Err` if `max_trials` is less than 1.
    pub fn new(max_trials: usize) -> Result<Self, AutoTuningConfigError> {
        if max_trials < 1 {
            return Err(AutoTuningConfigError::InvalidMaxTrials {
                value: max_trials,
                min: 1,
            });
        }
        Ok(Self { max_trials })
    }

    /// Get the maximum number of trials.
    pub fn max_trials(&self) -> usize {
        self.max_trials
    }

    /// Convert to a map representation (for serialization/debugging).
    pub fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert(
            "maxTrials".to_string(),
            serde_json::Value::Number(self.max_trials.into()),
        );
        map
    }
}

impl Default for AutoTuningConfig {
    fn default() -> Self {
        Self {
            max_trials: Self::MAX_TRIALS,
        }
    }
}

/// Errors that can occur when creating an `AutoTuningConfig`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AutoTuningConfigError {
    /// Maximum trials value is below the minimum allowed value
    InvalidMaxTrials {
        /// The invalid value provided
        value: usize,
        /// The minimum allowed value
        min: usize,
    },
}

impl std::fmt::Display for AutoTuningConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AutoTuningConfigError::InvalidMaxTrials { value, min } => {
                write!(f, "Invalid maxTrials: {} (must be >= {})", value, min)
            }
        }
    }
}

impl std::error::Error for AutoTuningConfigError {}
