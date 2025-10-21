//! Focused Macros for Algorithm Boilerplate
//!
//! These macros generate specific parts of algorithm implementations:
//! - AlgorithmSpec boilerplate
//! - Storage Runtime boilerplate  
//! - Computation Runtime boilerplate
//! - Config structs (already exists)

/// Generate AlgorithmSpec boilerplate
///
/// This macro generates the standard AlgorithmSpec implementation
/// while allowing manual implementation of the execute method.
///
/// ## Usage
///
/// ```rust
/// define_algorithm_spec! {
///     name: "my_algorithm",
///     output_type: MyResult,
///     projection_hint: Dense,
///     modes: [Stream, Stats],
///     
///     // Manual implementation
///     execute: |self, graph_store, config, context| {
///         // Your algorithm logic here
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_algorithm_spec {
    (
        name: $name:literal,
        output_type: $output_type:ty,
        projection_hint: $hint:ident,
        modes: [$($mode:ident),*],
        
        execute: |$self_param:ident, $graph_store_param:ident, $config_param:ident, $context_param:ident| $execute_fn:block
    ) => {
        paste::paste! {
            /// Algorithm Specification for [<$name:upper>]
            ///
            /// This implements the `AlgorithmSpec` trait required by `ProcedureExecutor`.
            pub struct [<$name:upper AlgorithmSpec>] {
                /// Name of the graph to load
                graph_name: String,
            }

            impl [<$name:upper AlgorithmSpec>] {
                /// Create a new algorithm specification
                pub fn new(graph_name: String) -> Self {
                    Self { graph_name }
                }
            }

            impl $crate::projection::eval::procedure::AlgorithmSpec for [<$name:upper AlgorithmSpec>] {
                type Output = $output_type;

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
                    // For now, just pass through the input
                    // Individual algorithms can handle their own config parsing
                    Ok(input.clone())
                }

                fn validation_config(&self, _context: &$crate::projection::eval::procedure::ExecutionContext) -> $crate::projection::eval::procedure::ValidationConfiguration {
                    $crate::projection::eval::procedure::ValidationConfiguration::empty()
                }

                fn execute<G: $crate::types::prelude::GraphStore>(
                    &self,
                    $graph_store_param: &G,
                    $config_param: &serde_json::Value,
                    $context_param: &$crate::projection::eval::procedure::ExecutionContext,
                ) -> Result<$crate::projection::eval::procedure::ComputationResult<Self::Output>, $crate::projection::eval::procedure::AlgorithmError> {
                    let timer = std::time::Instant::now();
                    
                    // Call the manual execute implementation
                    let result = {
                        $execute_fn
                    }?;
                    
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

/// Generate Storage Runtime boilerplate
///
/// This macro generates the standard Storage Runtime structure
/// while allowing manual implementation of the access methods.
///
/// ## Usage
///
/// ```rust
/// define_storage_runtime! {
///     name: "MyStorage",
///     graph_store_type: G,
///     
///     // Manual implementation
///     impl_methods: {
///         pub fn get_node_value(&self, node_id: u32) -> Result<f64, AlgorithmError> {
///             // Your storage access logic here
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_storage_runtime {
    (
        name: $name:ident,
        graph_store_type: $graph_store_type:ident,
        
        impl_methods: {
            $($method:item)*
        }
    ) => {
        paste::paste! {
            /// Storage Runtime for [<$name>]
            ///
            /// This is the **Gross pole** - persistent data structures.
            /// It knows how to access the graph structure and properties.
            pub struct [<$name StorageRuntime>]<'a, $graph_store_type: $crate::types::prelude::GraphStore> {
                /// Reference to the graph store
                graph_store: &'a $graph_store_type,
            }

            impl<'a, $graph_store_type: $crate::types::prelude::GraphStore> [<$name StorageRuntime>]<'a, $graph_store_type> {
                /// Create a new storage runtime
                pub fn new(graph_store: &'a $graph_store_type) -> Result<Self, $crate::projection::eval::procedure::AlgorithmError> {
                    Ok(Self { graph_store })
                }

                /// Get reference to graph store
                pub fn graph_store(&self) -> &'a $graph_store_type {
                    self.graph_store
                }

                $($method)*
            }
        }
    };
}

/// Generate Computation Runtime boilerplate
///
/// This macro generates the standard Computation Runtime structure
/// while allowing manual implementation of the computation methods.
///
/// ## Usage
///
/// ```rust
/// define_computation_runtime! {
///     name: "MyComputation",
///     fields: {
///         sum: f64,
///         count: usize,
///     },
///     
///     // Manual implementation
///     impl_methods: {
///         pub fn add_value(&mut self, value: f64) {
///             // Your computation logic here
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_computation_runtime {
    (
        name: $name:ident,
        fields: {
            $($field:ident : $field_type:ty,)*
        },
        
        impl_methods: {
            $($method:item)*
        }
    ) => {
        paste::paste! {
            /// Computation Runtime for [<$name>]
            ///
            /// This is the **Subtle pole** - ephemeral computation state.
            /// It manages the algorithm's computation logic.
            #[derive(Debug, Clone)]
            pub struct [<$name ComputationRuntime>] {
                $(
                    $field: $field_type,
                )*
            }

            impl [<$name ComputationRuntime>] {
                /// Create a new computation runtime
                pub fn new() -> Self {
                    Self {
                        $(
                            $field: Default::default(),
                        )*
                    }
                }

                $($method)*
            }

            impl Default for [<$name ComputationRuntime>] {
                fn default() -> Self {
                    Self::new()
                }
            }
        }
    };
}
