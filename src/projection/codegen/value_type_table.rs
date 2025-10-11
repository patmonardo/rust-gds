//! Prototype Value Type Table
//!
//! This demonstrates the value_type_table! macro with a few basic types.
//! Real usage would include all primitive types, arrays, lists, and UDTs.

// Invoke the master projector macro
value_type_table! {
    Long {
        id: 1,
        value_type: ValueType::Long,
        storage_hint: StorageHint::FixedWidth,
        rust_type: i64,
    },
    Double {
        id: 2,
        value_type: ValueType::Double,
        storage_hint: StorageHint::FixedWidth,
        rust_type: f64,
    },
    StringProp {
        id: 3,
        value_type: ValueType::String,
        storage_hint: StorageHint::VariableLength,
        rust_type: String,
    },
    LongArray {
        id: 4,
        value_type: ValueType::LongArray,
        storage_hint: StorageHint::ListAsOffsets,
        rust_type: Vec<i64>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::form_processor;
    use crate::types::ValueType;

    #[test]
    fn test_all_descriptors() {
        form_processor::clear_property_registry();

        // Register all
        let count = registry::register_all();
        assert_eq!(count, 4);

        // Verify each one
        let long_desc = form_processor::get_property_descriptor(1).unwrap();
        assert_eq!(long_desc.value_type, ValueType::Long);

        let double_desc = form_processor::get_property_descriptor(2).unwrap();
        assert_eq!(double_desc.value_type, ValueType::Double);

        let string_desc = form_processor::get_property_descriptor(3).unwrap();
        assert_eq!(string_desc.value_type, ValueType::String);

        let array_desc = form_processor::get_property_descriptor(4).unwrap();
        assert_eq!(array_desc.value_type, ValueType::LongArray);
    }

    #[test]
    fn test_property_ids() {
        let ids = registry::all_property_ids();
        assert_eq!(ids, vec![1, 2, 3, 4]);
    }
}
