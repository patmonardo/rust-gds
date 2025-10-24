//! Triadic PropertyStore Matrix Macros (scaffolding)
//!
//! These macros define the compile-time generation surface for the
//! 3×3×N matrix (Levels × Components × ValueTypes). Initial scaffolding
//! expands to no code; later phases will emit full implementations.

/// Leaf adapter generator for a typed PropertyValues implementation.
///
/// Usage (future):
/// property_values_adapter!(MyLongNodePV, Vec<i64>, i64, Node, Indexed);
#[macro_export]
macro_rules! property_values_adapter {
    (
        $struct_name:ident,
        $backend:ty,
        $element:ty,
        $level:ident,
        $access:ident
    ) => {
        // Scaffolding: will be implemented in a later phase
    };
}

/// Access pattern hook (Indexed | Stream) – used by property_values_adapter!.
#[macro_export]
macro_rules! property_values_access_impl {
    ($struct_name:ident, $backend:ty, $element:ty, $level:ident, $access:ident) => {
        // Scaffolding: will be implemented in a later phase
    };
}

/// Node-level adapter generator for PropertyValues with backend abstraction.
///
/// Scalar variant usage:
/// node_property_values_adapter!(DefaultLongNodePropertyValues, Vec<i64>, i64, Long, node_long_property_values_impl);
///
/// Array variant usage:
/// node_property_values_adapter!(DefaultDoubleArrayNodePropertyValues, Vec<Option<Vec<f64>>>, Vec<f64>, DoubleArray, node_double_array_property_values_impl, array);
#[macro_export]
macro_rules! node_property_values_adapter {
    // Scalar values (Long, Double)
    (
        $struct_name:ident,
        $backend:ty,
        $element:ty,
        $value_enum:ident,
        $typed_impl_macro:ident
    ) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: $backend,
            node_count: usize,
        }

        impl $struct_name {
            pub fn new(values: $backend, node_count: usize) -> Self {
                Self { values, node_count }
            }
        }

        $crate::property_values_impl!($struct_name, $value_enum);
        $crate::$typed_impl_macro!($struct_name);
    };

    // Array values (DoubleArray, FloatArray, LongArray)
    (
        $struct_name:ident,
        $backend:ty,
        $element:ty,
        $value_enum:ident,
        $typed_impl_macro:ident,
        array
    ) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: $backend,
            node_count: usize,
            dimension: Option<usize>,
        }

        impl $struct_name {
            pub fn new(values: $backend, node_count: usize) -> Self {
                // Try to infer dimension from the first present vector
                let dimension = (
                    || {
                        // best-effort inference when backend is Vec<Option<Vec<_>>>
                        // pattern match using any slice methods available
                        #[allow(unused_mut)]
                        let mut dim: Option<usize> = None;
                        // SAFETY: This relies on backend being Vec-like in default usage
                        // and is a no-op for other backends.
                        dim
                    }
                )();
                Self { values, node_count, dimension }
            }
        }

        $crate::property_values_impl!($struct_name, $value_enum, array);
        $crate::$typed_impl_macro!($struct_name);
    };
}

/// Node Long adapter with specialized unchecked accessor.
#[macro_export]
macro_rules! node_long_adapter {
    ($struct_name:ident, $backend:ty) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: $backend,
            node_count: usize,
        }

        impl $struct_name {
            pub fn new(values: $backend, node_count: usize) -> Self {
                Self { values, node_count }
            }
        }

        $crate::property_values_impl!($struct_name, Long);
        $crate::node_long_property_values_impl!($struct_name);

        impl $crate::types::properties::node::LongNodePropertyValues for $struct_name {
            fn long_value_unchecked(&self, node_id: u64) -> i64 {
                // Vec backend assumption for now
                self.values[node_id as usize]
            }
        }
    };
}

/// Node Double adapter with specialized unchecked accessor.
#[macro_export]
macro_rules! node_double_adapter {
    ($struct_name:ident, $backend:ty) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: $backend,
            node_count: usize,
        }

        impl $struct_name {
            pub fn new(values: $backend, node_count: usize) -> Self {
                Self { values, node_count }
            }
        }

        $crate::property_values_impl!($struct_name, Double);
        $crate::node_double_property_values_impl!($struct_name);

        impl $crate::types::properties::node::DoubleNodePropertyValues for $struct_name {
            fn double_value_unchecked(&self, node_id: u64) -> f64 {
                self.values[node_id as usize]
            }
        }
    };
}

/// Node DoubleArray adapter with specialized unchecked accessor.
#[macro_export]
macro_rules! node_double_array_adapter {
    ($struct_name:ident, $backend:ty) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: $backend,
            node_count: usize,
            dimension: Option<usize>,
        }

        impl $struct_name {
            pub fn new(values: $backend, node_count: usize) -> Self {
                let dimension = values.first().and_then(|v| v.as_ref().map(|arr| arr.len()));
                Self { values, node_count, dimension }
            }
        }

        $crate::property_values_impl!($struct_name, DoubleArray, array);
        $crate::node_double_array_property_values_impl!($struct_name);

        impl $crate::types::properties::node::DoubleArrayNodePropertyValues for $struct_name {
            fn double_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f64>> {
                self.values[node_id as usize].clone()
            }
        }
    };
}

/// Node FloatArray adapter with specialized unchecked accessor.
#[macro_export]
macro_rules! node_float_array_adapter {
    ($struct_name:ident, $backend:ty) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: $backend,
            node_count: usize,
            dimension: Option<usize>,
        }

        impl $struct_name {
            pub fn new(values: $backend, node_count: usize) -> Self {
                let dimension = values.first().and_then(|v| v.as_ref().map(|arr| arr.len()));
                Self { values, node_count, dimension }
            }
        }

        $crate::property_values_impl!($struct_name, FloatArray, array);
        $crate::node_float_array_property_values_impl!($struct_name);

        impl $crate::types::properties::node::FloatArrayNodePropertyValues for $struct_name {
            fn float_array_value_unchecked(&self, node_id: u64) -> Option<Vec<f32>> {
                self.values[node_id as usize].clone()
            }
        }
    };
}

/// Node LongArray adapter with specialized unchecked accessor.
#[macro_export]
macro_rules! node_long_array_adapter {
    ($struct_name:ident, $backend:ty) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: $backend,
            node_count: usize,
            dimension: Option<usize>,
        }

        impl $struct_name {
            pub fn new(values: $backend, node_count: usize) -> Self {
                let dimension = values.first().and_then(|v| v.as_ref().map(|arr| arr.len()));
                Self { values, node_count, dimension }
            }
        }

        $crate::property_values_impl!($struct_name, LongArray, array);
        $crate::node_long_array_property_values_impl!($struct_name);

        impl $crate::types::properties::node::LongArrayNodePropertyValues for $struct_name {
            fn long_array_value_unchecked(&self, node_id: u64) -> Option<Vec<i64>> {
                self.values[node_id as usize].clone()
            }
        }
    };
}

/// Relationship-level adapter generator (Double-only parity for now).
///
/// Example:
/// relationship_property_values_adapter!(DefaultRelationshipPropertyValues, Vec<f64>, f64, Double);
#[macro_export]
macro_rules! relationship_property_values_adapter {
    (
        $struct_name:ident,
        $backend:ty,
        $element:ty,
        $value_enum:ident
    ) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: $backend,
            default_value: $element,
            element_count: usize,
        }

        impl $struct_name {
            pub fn new(values: $backend, default_value: $element, element_count: usize) -> Self {
                Self { values, default_value, element_count }
            }
        }

        $crate::property_values_impl!($struct_name, $value_enum, relationship);
        // Note: RelationshipPropertyValues trait impl remains backend-specific and
        // should be provided by the caller or a dedicated typed macro in a later phase.
    };
}

/// Graph-level adapter generator (Stream/aggregate access patterns).
///
/// Example:
/// graph_property_values_adapter!(DefaultLongGraphPropertyValues, Vec<i64>, i64, Long);
#[macro_export]
macro_rules! graph_property_values_adapter {
    (
        $struct_name:ident,
        $backend:ty,
        $element:ty,
        $value_enum:ident
    ) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            values: $backend,
        }

        impl $struct_name {
            pub fn new(values: $backend) -> Self {
                Self { values }
            }
        }

        // For graph, element_count is values.len(); use graph variant
        $crate::property_values_impl!($struct_name, $value_enum, graph);
        // Note: GraphPropertyValues trait impl should be provided per typed adapter
        // in a later phase, to expose stream accessors.
    };
}

/// Generates the PropertyValues trait hierarchy and default adapters for a level.
#[macro_export]
macro_rules! property_values_matrix {
    // Node level, Indexed access: generate default Vec-backed adapters
    (Node, Indexed, [Long, Double]) => {
        // Generate Long and Double adapters using macros
        $crate::node_long_adapter!(DefaultLongNodePropertyValues, Vec<i64>);
        $crate::node_double_adapter!(DefaultDoubleNodePropertyValues, Vec<f64>);
    };

    // Node level, Indexed access: generate Long, Double, and DoubleArray adapters
    (Node, Indexed, [Long, Double, DoubleArray]) => {
        // Generate Long, Double, and DoubleArray adapters using macros
        $crate::node_long_adapter!(DefaultLongNodePropertyValues, Vec<i64>);
        $crate::node_double_adapter!(DefaultDoubleNodePropertyValues, Vec<f64>);
        $crate::node_double_array_adapter!(DefaultDoubleArrayNodePropertyValues, Vec<Option<Vec<f64>>>);
    };

    // Node level, Indexed access: generate all 5 adapters (complete Node level)
    (Node, Indexed, [Long, Double, DoubleArray, FloatArray, LongArray]) => {
        // Generate all 5 Node adapters using macros
        $crate::node_long_adapter!(DefaultLongNodePropertyValues, Vec<i64>);
        $crate::node_double_adapter!(DefaultDoubleNodePropertyValues, Vec<f64>);
        $crate::node_double_array_adapter!(DefaultDoubleArrayNodePropertyValues, Vec<Option<Vec<f64>>>);
        $crate::node_float_array_adapter!(DefaultFloatArrayNodePropertyValues, Vec<Option<Vec<f32>>>);
        $crate::node_long_array_adapter!(DefaultLongArrayNodePropertyValues, Vec<Option<Vec<i64>>>);
    };

    // Relationship level placeholder
    (Relationship, Indexed, [$($value_type:ident),*]) => { };

    // Graph level placeholder
    (Graph, Stream, [$($value_type:ident),*]) => { };
}

/// Generates the Property trait layer for a level.
#[macro_export]
macro_rules! property_matrix {
    ($level:ident, [$($value_type:ident),*]) => {
        // Scaffolding: will be implemented in a later phase
    };
}

/// Generates the PropertyStore trait and default store for a level.
#[macro_export]
macro_rules! property_store_matrix {
    ($level:ident, [$($value_type:ident),*]) => {
        // Scaffolding: will be implemented in a later phase
    };
}

/// Root macro orchestrating the full 3-component generation for a level.
#[macro_export]
macro_rules! property_store_level {
    (
        level: $level:ident,
        access: $access:ident,
        value_types: [$($vt:ident),*]
    ) => {
        $crate::property_values_matrix!($level, $access, [$($vt),*]);
        $crate::property_matrix!($level, [$($vt),*]);
        $crate::property_store_matrix!($level, [$($vt),*]);
    };
}

/// Manifest macro: declarative generation of Level × Components × Types × Backends.
///
/// Current support (scaffold):
/// - Components: PropertyValues only
/// - Levels: Node(Indexed)
/// - Backends: Vec
#[macro_export]
macro_rules! property_store_manifest {
    // Preview arm: accept exactly one level, Vec backend, and expand to items
    (
        levels: [$level:ident($access:ident)],
        types: [$($value_type:ident),* $(,)?],
        backends: [Vec],
        components: [$($component:ident),* $(,)?] $(,)?
    ) => {
        $crate::property_values_matrix!($level, $access, [$($value_type),*]);
    };

    // Fallback arm: other backends/components are ignored in preview
    (
        levels: [$level:ident($access:ident)],
        types: [$($value_type:ident),* $(,)?],
        backends: [$backend:ident],
        components: [$($component:ident),* $(,)?] $(,)?
    ) => {};
}