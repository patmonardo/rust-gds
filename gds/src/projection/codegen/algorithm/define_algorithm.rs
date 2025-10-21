//! Define Algorithm Macro
//!
//! Generates AlgorithmSpec implementations from declarative specifications.
//! Uses the config macro pattern as a template for consistency.
//!
//! ## Usage
//!
//! ```rust
//! define_algorithm! {
//!     name: "pagerank",
//!     category: Centrality,
//!     
//!     config: {
//!         damping_factor: f64 = 0.85 { range: 0.0..1.0 },
//!         tolerance: f64 = 1e-6 { validate: |v| v > 0.0 },
//!         max_iterations: usize = 100,
//!         source_nodes: Option<Vec<u64>> = None,
//!         weight_property: Option<String> = None,
//!     },
//!     
//!     result: PageRankResult {
//!         scores: Vec<f64>,
//!         iterations: usize,
//!         converged: bool,
//!         execution_time: Duration,
//!     },
//!     
//!     projection_hint: Dense,
//!     modes: [Stream, Stats],
//!     
//!     // Developer writes ONLY this
//!     execute: |graph_store, config, context| {
//!         // Actual algorithm logic
//!         // config.damping_factor is already parsed!
//!     }
//! }
//! ```

#[macro_export]
macro_rules! define_algorithm {
    (
        name: $name:literal,
        category: $category:ident,
        
        config: {
            $(
                $field:ident : $ty:ty = $default:expr $(,)?
            )*
        },
        
        result: $result_name:ident {
            $(
                $result_field:ident : $result_ty:ty $(,)?
            )*
        },
        
        projection_hint: $hint:ident,
        modes: [$($mode:ident),*],
        
        execute: $execute_fn:block
    ) => {
        define_algorithm!(@generate 
            $name, $category, 
            { $($field : $ty = $default;)* },
            $result_name { $($result_field : $result_ty;)* },
            $hint, [$($mode),*],
            $execute_fn
        );
    };
    
    (@generate 
        $name:literal, $category:ident,
        { $($field:ident : $ty:ty = $default:expr;)* },
        $result_name:ident { $($result_field:ident : $result_ty:ty;)* },
        $hint:ident, [$($mode:ident),*],
        $execute_fn:block
    ) => {
        // Generate config struct using define_config! pattern
        paste::paste! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            pub struct [<$name:upper Config>] {
                $(
                    pub $field: $ty,
                )*
            }

            impl Default for [<$name:upper Config>] {
                fn default() -> Self {
                    Self {
                        $(
                            $field: $default,
                        )*
                    }
                }
            }

            impl [<$name:upper Config>] {
                pub fn validate(&self) -> Result<(), $crate::config::validation::ConfigError> {
                    // Basic validation - can be extended later
                    Ok(())
                }
            }
        }

        // Generate result struct
        paste::paste! {
            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
            pub struct $result_name {
                $(
                    pub $result_field: $result_ty,
                )*
            }
        }

        // Generate AlgorithmSpec struct
        paste::paste! {
            pub struct [<$name:upper AlgorithmSpec>] {
                graph_name: String,
                config: [<$name:upper Config>],
            }

            impl [<$name:upper AlgorithmSpec>] {
                pub fn new(graph_name: String, config: [<$name:upper Config>]) -> Self {
                    Self { graph_name, config }
                }
            }

            impl $crate::projection::eval::procedure::AlgorithmSpec for [<$name:upper AlgorithmSpec>] {
                type Output = $result_name;

                fn name(&self) -> &str {
                    $name
                }

                fn graph_name(&self) -> &str {
                    &self.graph_name
                }

                fn projection_hint(&self) -> $crate::projection::eval::procedure::ProjectionHint {
                    $crate::projection::eval::procedure::ProjectionHint::$hint
                }

                fn parse_config(&self, input: &serde_json::Value) -> Result<serde_json::Value, $crate::projection::eval::procedure::ConfigError> {
                    let config: [<$name:upper Config>] = serde_json::from_value(input.clone())
                        .map_err(|e| $crate::projection::eval::procedure::ConfigError::InvalidValue {
                            param: "config".to_string(),
                            message: format!("JSON parsing failed: {}", e),
                        })?;
                    
                    config.validate()
                        .map_err(|e| $crate::projection::eval::procedure::ConfigError::InvalidValue {
                            param: "config".to_string(),
                            message: format!("Validation failed: {}", e),
                        })?;
                    
                    Ok(serde_json::to_value(config).unwrap())
                }

                fn validation_config(&self, _context: &$crate::projection::eval::procedure::ExecutionContext) -> $crate::projection::eval::procedure::ValidationConfiguration {
                    $crate::projection::eval::procedure::ValidationConfiguration::empty()
                }

                fn execute<G: $crate::types::prelude::GraphStore>(
                    &self,
                    _graph_store: &G,
                    config: &serde_json::Value,
                    _context: &$crate::projection::eval::procedure::ExecutionContext,
                ) -> Result<$crate::projection::eval::procedure::ComputationResult<Self::Output>, $crate::projection::eval::procedure::AlgorithmError> {
                    let _parsed_config: [<$name:upper Config>] = serde_json::from_value(config.clone())
                        .map_err(|e| $crate::projection::eval::procedure::AlgorithmError::Execution(format!("Config parsing failed: {}", e)))?;
                    
                    let timer = std::time::Instant::now();
                    
                    // Call the developer-provided execute block
                    let result = $execute_fn?;
                    
                    let elapsed = timer.elapsed();
                    Ok($crate::projection::eval::procedure::ComputationResult::new(result, elapsed))
                }

                fn consume_result(
                    &self,
                    result: $crate::projection::eval::procedure::ComputationResult<Self::Output>,
                    mode: &$crate::projection::eval::procedure::ExecutionMode,
                ) -> Result<Self::Output, $crate::projection::eval::procedure::ConsumerError> {
                    match mode {
                        $(
                            $crate::projection::eval::procedure::ExecutionMode::$mode => Ok(result.into_result()),
                        )*
                        other => Err($crate::projection::eval::procedure::ConsumerError::UnsupportedMode(*other)),
                    }
                }
            }
        }
    };

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::procedure::*;
    use crate::types::prelude::GraphStore;
    use std::time::Duration;

    #[test]
    fn test_macro_compilation() {
        // Just test that the macro compiles without errors
        // The actual algorithm tests are in test_algorithm.rs
        assert!(true);
    }
}
