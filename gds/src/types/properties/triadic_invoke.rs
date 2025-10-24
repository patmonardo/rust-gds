// Phase 3: Test Node Long, Double, DoubleArray, FloatArray, and LongArray macro generation
// Guarded behind a feature flag to avoid duplicate definitions with existing defaults.

#![cfg(feature = "triadic_codegen_preview")]

use crate::property_store_manifest;
// Bring traits into scope for macro expansions that use unqualified names
use crate::types::properties::PropertyValues;
use crate::types::properties::PropertyValuesError;
use crate::types::properties::PropertyValuesResult;
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::node::LongNodePropertyValues;
use crate::types::properties::node::DoubleNodePropertyValues;
use crate::types::properties::node::DoubleArrayNodePropertyValues;
use crate::types::properties::node::FloatArrayNodePropertyValues;
use crate::types::properties::node::LongArrayNodePropertyValues;
use crate::types::ValueType;

// Phase 3: Test all 5 Node types generation
property_store_manifest! {
    levels: [Node(Indexed)],
    types: [Long, Double, DoubleArray, FloatArray, LongArray],
    backends: [Vec],
    components: [PropertyValues],
}


