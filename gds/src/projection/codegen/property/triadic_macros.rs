//! Universal Property Value Adapter Macros
//!
//! This module provides the macro system for generating property value adapters
//! from the ValueType table. All adapters are generic over Collections backends,
//! enabling runtime backend selection (Vec, Huge, Arrow).
//!
//! Architecture:
//! ```
//! ValueType Table (Master Controller)
//!     ↓
//! Universal Adapter Macros (this file)
//!     ↓
//! Trait Implementation Helpers (property_values.rs)
//!     ↓
//! Generated Adapters (DefaultLongNodePropertyValues<C>, etc.)
//! ```

// ============================================================================
// NODE PROPERTY ADAPTERS
// ============================================================================

/// Universal Node Property Adapter Generator
///
/// Generates DefaultNodePropertyValues<C> structs for each ValueType.
/// All generated types are generic over Collections backend C.
#[macro_export]
macro_rules! node_universal_adapter {
    ($value_type:ident, $rust_type:ty, $category:ident, $default_value:expr) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<Default $value_type NodePropertyValues>]<C>
            where
                C: $crate::collections::traits::Collections<$rust_type> 
                    + $crate::collections::traits::PropertyValuesAdapter<$rust_type> 
                    + Send + Sync + std::fmt::Debug,
            {
                #[allow(dead_code)]  // Used via trait methods, not direct field access
                universal: $crate::collections::adapter::UniversalPropertyValues<$rust_type, C>,
                node_count: usize,
            }

            impl<C> [<Default $value_type NodePropertyValues>]<C>
            where
                C: $crate::collections::traits::Collections<$rust_type> 
                    + $crate::collections::traits::PropertyValuesAdapter<$rust_type> 
                    + Send + Sync + std::fmt::Debug,
            {
                pub fn new(
                    universal: $crate::collections::adapter::UniversalPropertyValues<$rust_type, C>, 
                    node_count: usize
                ) -> Self {
                    Self { universal, node_count }
                }

                pub fn from_collection(collection: C, node_count: usize) -> Self {
                    let universal = $crate::collections::adapter::UniversalPropertyValues::new(
                        collection,
                        $crate::types::ValueType::$value_type,
                        $default_value,
                    );
                    Self { universal, node_count }
                }

                pub fn node_count(&self) -> usize {
                    self.node_count
                }
            }
            
            // Generate trait implementations
            $crate::impl_property_values_universal!(
                [<Default $value_type NodePropertyValues>]<C>,
                $value_type,
                node_count,
                $rust_type
            );
            
            // Note: impl_typed_node_property_values_universal will provide the full
            // NodePropertyValues implementation for the specific value type
            $crate::impl_typed_node_property_values_universal!(
                [<Default $value_type NodePropertyValues>]<C>,
                $value_type,
                $category,
                $rust_type
            );
        }
    };
}

/// Batch Generator: Generate all node adapters from ValueType table
#[macro_export]
macro_rules! generate_all_node_adapters {
    () => {
        macro_rules! gen_node_adapter {
            ($vt:ident, $rt:ty, IntegralScalar, $dv:expr) => {
                $crate::node_universal_adapter!($vt, $rt, IntegralScalar, $dv);
            };
            ($vt:ident, $rt:ty, FloatingPointScalar, $dv:expr) => {
                $crate::node_universal_adapter!($vt, $rt, FloatingPointScalar, $dv);
            };
            ($vt:ident, $rt:ty, OtherScalar, $dv:expr) => {
                $crate::node_universal_adapter!($vt, $rt, OtherScalar, $dv);
            };
            // Skip array types for now
            ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {};
        }
        
        $crate::value_type_table!(gen_node_adapter);
    };
}

/// Batch Node Array Adapter Generator
#[macro_export]
macro_rules! generate_all_node_array_adapters {
    () => {
        macro_rules! gen_node_array_adapter {
            ($vt:ident, $rt:ty, IntegralArray, $dv:expr) => {
                $crate::node_universal_adapter!($vt, $rt, IntegralArray, $dv);
            };
            ($vt:ident, $rt:ty, FloatingPointArray, $dv:expr) => {
                $crate::node_universal_adapter!($vt, $rt, FloatingPointArray, $dv);
            };
            ($vt:ident, $rt:ty, OtherArray, $dv:expr) => {
                $crate::node_universal_adapter!($vt, $rt, OtherArray, $dv);
            };
            // Skip scalars
            ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {};
        }
        
        $crate::value_type_table!(gen_node_array_adapter);
    };
}

// ============================================================================
// RELATIONSHIP PROPERTY ADAPTERS
// ============================================================================

/// Universal Relationship Property Adapter Generator
///
/// Generates DefaultRelationshipPropertyValues<C> structs for each ValueType.
/// Relationships use element_count instead of node_count.
#[macro_export]
macro_rules! relationship_universal_adapter {
    ($value_type:ident, $rust_type:ty, $category:ident, $default_value:expr) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<Default $value_type RelationshipPropertyValues>]<C>
            where
                C: $crate::collections::traits::Collections<$rust_type> 
                    + $crate::collections::traits::PropertyValuesAdapter<$rust_type> 
                    + Send + Sync + std::fmt::Debug,
            {
                #[allow(dead_code)]  // Used via trait methods, not direct field access
                universal: $crate::collections::adapter::UniversalPropertyValues<$rust_type, C>,
                element_count: usize,
            }

            impl<C> [<Default $value_type RelationshipPropertyValues>]<C>
            where
                C: $crate::collections::traits::Collections<$rust_type> 
                    + $crate::collections::traits::PropertyValuesAdapter<$rust_type> 
                    + Send + Sync + std::fmt::Debug,
            {
                pub fn new(
                    universal: $crate::collections::adapter::UniversalPropertyValues<$rust_type, C>, 
                    element_count: usize
                ) -> Self {
                    Self { universal, element_count }
                }

                pub fn from_collection(collection: C, element_count: usize) -> Self {
                    let universal = $crate::collections::adapter::UniversalPropertyValues::new(
                        collection,
                        $crate::types::ValueType::$value_type,
                        $default_value,
                    );
                    Self { universal, element_count }
                }

                pub fn element_count(&self) -> usize {
                    self.element_count
                }
            }
            
            // Generate trait implementations
            $crate::impl_property_values_universal!(
                [<Default $value_type RelationshipPropertyValues>]<C>,
                $value_type,
                element_count,
                $rust_type
            );
            
            $crate::impl_relationship_property_values_universal!(
                [<Default $value_type RelationshipPropertyValues>]<C>,
                $rust_type
            );
        }
    };
}

/// Batch Generator: Generate all relationship adapters from ValueType table
#[macro_export]
macro_rules! generate_all_relationship_adapters {
    () => {
        macro_rules! gen_rel_adapter {
            ($vt:ident, $rt:ty, IntegralScalar, $dv:expr) => {
                $crate::relationship_universal_adapter!($vt, $rt, IntegralScalar, $dv);
            };
            ($vt:ident, $rt:ty, FloatingPointScalar, $dv:expr) => {
                $crate::relationship_universal_adapter!($vt, $rt, FloatingPointScalar, $dv);
            };
            ($vt:ident, $rt:ty, OtherScalar, $dv:expr) => {
                $crate::relationship_universal_adapter!($vt, $rt, OtherScalar, $dv);
            };
            // Skip arrays for relationships - not typically used
            ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {};
        }
        
        $crate::value_type_table!(gen_rel_adapter);
    };
}

/// Batch Relationship Array Adapter Generator
#[macro_export]
macro_rules! generate_all_relationship_array_adapters {
    () => {
        macro_rules! gen_rel_array_adapter {
            ($vt:ident, $rt:ty, IntegralArray, $dv:expr) => {
                $crate::relationship_universal_adapter!($vt, $rt, IntegralArray, $dv);
            };
            ($vt:ident, $rt:ty, FloatingPointArray, $dv:expr) => {
                $crate::relationship_universal_adapter!($vt, $rt, FloatingPointArray, $dv);
            };
            ($vt:ident, $rt:ty, OtherArray, $dv:expr) => {
                $crate::relationship_universal_adapter!($vt, $rt, OtherArray, $dv);
            };
            // Skip scalars
            ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {};
        }
        
        $crate::value_type_table!(gen_rel_array_adapter);
    };
}

// ============================================================================
// GRAPH PROPERTY ADAPTERS
// ============================================================================

/// Universal Graph Property Adapter Generator
///
/// Generates DefaultGraphPropertyValues<C> structs for each ValueType.
/// Graph properties don't have a count field - they use the collection size directly.
#[macro_export]
macro_rules! graph_universal_adapter {
    ($value_type:ident, $rust_type:ty, $category:ident, $default_value:expr) => {
        paste::paste! {
            #[derive(Debug)]
            pub struct [<Default $value_type GraphPropertyValues>]<C>
            where
                C: $crate::collections::traits::Collections<$rust_type> 
                    + $crate::collections::traits::PropertyValuesAdapter<$rust_type> 
                    + Send + Sync + std::fmt::Debug,
            {
                #[allow(dead_code)]  // Used via trait methods, not direct field access
                universal: $crate::collections::adapter::UniversalPropertyValues<$rust_type, C>,
            }

            impl<C> [<Default $value_type GraphPropertyValues>]<C>
            where
                C: $crate::collections::traits::Collections<$rust_type> 
                    + $crate::collections::traits::PropertyValuesAdapter<$rust_type> 
                    + Send + Sync + std::fmt::Debug,
            {
                pub fn new(
                    universal: $crate::collections::adapter::UniversalPropertyValues<$rust_type, C>
                ) -> Self {
                    Self { universal }
                }

                pub fn from_collection(collection: C) -> Self {
                    let universal = $crate::collections::adapter::UniversalPropertyValues::new(
                        collection,
                        $crate::types::ValueType::$value_type,
                        $default_value,
                    );
                    Self { universal }
                }
                
                pub fn singleton(value: $rust_type) -> Self 
                where
                    C: From<Vec<$rust_type>>,
                {
                    let collection = C::from(vec![value]);
                    Self::from_collection(collection)
                }
            }
            
            // Implement PropertyValues manually for graph (no count field)
            impl<C> $crate::types::properties::PropertyValues for [<Default $value_type GraphPropertyValues>]<C>
            where
                C: $crate::collections::traits::Collections<$rust_type> 
                    + $crate::collections::traits::PropertyValuesAdapter<$rust_type> 
                    + Send + Sync + std::fmt::Debug,
            {
                fn value_type(&self) -> $crate::types::ValueType {
                    $crate::types::ValueType::$value_type
                }

                fn element_count(&self) -> usize {
                    #[allow(unused_imports)]
                    use $crate::collections::traits::Collections;
                    self.universal.collection().len()
                }
            }
            
            $crate::impl_graph_property_values_universal!(
                [<Default $value_type GraphPropertyValues>]<C>,
                $rust_type
            );
        }
    };
}

/// Batch Generator: Generate all graph adapters from ValueType table
#[macro_export]
macro_rules! generate_all_graph_adapters {
    () => {
        macro_rules! gen_graph_adapter {
            ($vt:ident, $rt:ty, IntegralScalar, $dv:expr) => {
                $crate::graph_universal_adapter!($vt, $rt, IntegralScalar, $dv);
            };
            ($vt:ident, $rt:ty, FloatingPointScalar, $dv:expr) => {
                $crate::graph_universal_adapter!($vt, $rt, FloatingPointScalar, $dv);
            };
            // Skip OtherScalar (Boolean, Char, String) for graphs - require specialized impls
            ($vt:ident, $rt:ty, OtherScalar, $dv:expr) => {};
            // Skip arrays
            ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {};
        }
        
        $crate::value_type_table!(gen_graph_adapter);
    };
}

/// Batch Graph Array Adapter Generator
#[macro_export]
macro_rules! generate_all_graph_array_adapters {
    () => {
        macro_rules! gen_graph_array_adapter {
            ($vt:ident, $rt:ty, IntegralArray, $dv:expr) => {
                $crate::graph_universal_adapter!($vt, $rt, IntegralArray, $dv);
            };
            ($vt:ident, $rt:ty, FloatingPointArray, $dv:expr) => {
                $crate::graph_universal_adapter!($vt, $rt, FloatingPointArray, $dv);
            };
            // Skip OtherArray (BooleanArray, CharArray, StringArray) for graphs - require specialized impls
            ($vt:ident, $rt:ty, OtherArray, $dv:expr) => {};
            // Skip scalars
            ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {};
        }
        
        $crate::value_type_table!(gen_graph_array_adapter);
    };
}
