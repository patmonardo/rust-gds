use crate::types::property::{Property, PropertySchema, SimpleProperty};
use polars::prelude::{AnyValue, Series};
use std::collections::HashMap;

/// Generic trait for a PropertyStore.
/// NOTE: the trait returns owned maps for simplicity and to mirror the TS API.
/// Implementations may provide more efficient accessors (see NodePropertyStore).
pub trait PropertyStore
where
    Self: Send + Sync,
{
    type P: Property + Clone + Send + Sync + 'static;

    /// Return an owned map of key -> property.
    fn properties(&self) -> HashMap<String, Self::P>;
}

/// Convenience helpers similar to the TS PropertyStore namespace.
pub mod util {
    use super::*;
    /// Map of key -> Series (extracts .values from each property).
    pub fn property_values<P, S>(store: &S) -> HashMap<String, Series>
    where
        S: PropertyStore<P = P>,
        P: Property,
    {
        let mut result = HashMap::new();
        for (k, p) in store.properties().into_iter() {
            result.insert(k, p.values().clone());
        }
        result
    }

    pub fn get<P, S>(store: &S, key: &str) -> Option<P>
    where
        S: PropertyStore<P = P>,
        P: Clone,
    {
        let mut props = store.properties();
        props.remove(key)
    }

    pub fn is_empty<P, S>(store: &S) -> bool
    where
        S: PropertyStore<P = P>,
    {
        store.properties().is_empty()
    }

    pub fn key_set<P, S>(store: &S) -> Vec<String>
    where
        S: PropertyStore<P = P>,
    {
        store.properties().keys().cloned().collect()
    }

    pub fn contains_key<P, S>(store: &S, key: &str) -> bool
    where
        S: PropertyStore<P = P>,
    {
        store.properties().contains_key(key)
    }
}

/// Graph-backed (map) implementation: good for arbitrary / non-columnar stores.
pub mod graph {
    use super::*;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Default)]
    pub struct GraphPropertyStore<P>
    where
        P: Property + Clone + Send + Sync + 'static,
    {
        pub properties: HashMap<String, P>,
    }

    impl<P> GraphPropertyStore<P>
    where
        P: Property + Clone + Send + Sync + 'static,
    {
        pub fn new(properties: HashMap<String, P>) -> Self {
            Self { properties }
        }

        pub fn insert(&mut self, key: impl Into<String>, prop: P) {
            self.properties.insert(key.into(), prop);
        }
    }

    impl<P> super::PropertyStore for GraphPropertyStore<P>
    where
        P: Property + Clone + Send + Sync + 'static,
    {
        type P = P;

        fn properties(&self) -> HashMap<String, Self::P> {
            self.properties.clone()
        }
    }
}

/// Node (columnar) property store: values are vector-indexed by compact integer index.
/// This is optimized for the Node use-case where you have an IdMap -> compact indexes.
pub mod node {
    use super::*;
    use std::collections::HashMap;

    /// A columnar property: holds schema + Series-backed values.
    #[derive(Clone, Debug)]
    pub struct ColumnProperty {
        pub schema: PropertySchema,
        pub values: Series,
    }

    impl ColumnProperty {
        pub fn new(schema: PropertySchema, values: Series) -> Self {
            Self { schema, values }
        }
    }

    /// NodePropertyStore stores columnar properties keyed by property key.
    /// Each property's values are indexed by the compact node index.
    #[derive(Clone, Debug, Default)]
    pub struct NodePropertyStore {
        pub properties: HashMap<String, ColumnProperty>,
    }

    impl NodePropertyStore {
        pub fn new() -> Self {
            Self {
                properties: HashMap::new(),
            }
        }

        pub fn insert_column(
            &mut self,
            key: impl Into<String>,
            schema: PropertySchema,
            values: Series,
        ) {
            let col = ColumnProperty::new(schema, values);
            self.properties.insert(key.into(), col);
        }

        /// Fast indexed access: get a property's value for a compact index.
        pub fn get_value_at(&self, key: &str, compact_index: usize) -> Option<AnyValue<'_>> {
            self.properties
                .get(key)
                .and_then(|col| col.values.get(compact_index).ok())
        }
    }

    impl super::PropertyStore for NodePropertyStore {
        // We expose the crate's concrete SimpleProperty as the property type.
        type P = SimpleProperty;

        fn properties(&self) -> HashMap<String, Self::P> {
            self.properties
                .iter()
                .map(|(k, col)| {
                    (
                        k.clone(),
                        SimpleProperty {
                            values: col.values.clone(),
                            schema: col.schema.clone(),
                        },
                    )
                })
                .collect()
        }
    }
}
