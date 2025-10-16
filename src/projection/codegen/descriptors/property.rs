use crate::types::ValueType;
use serde::{Deserialize, Serialize};

pub type PropertyId = u32;
pub type StructId = u32;

/// Field descriptor inside a Struct/UDT
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldDescriptor {
    pub name: String,
    pub value_type: ValueType,
    pub offset: u16,
}

/// Descriptor for a user-defined struct (UDT)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructDescriptor {
    pub id: StructId,
    pub name: String,
    pub fields: Vec<FieldDescriptor>,
}

/// Storage hint for property backends
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageHint {
    FixedWidth,
    VariableLength,
    ListAsOffsets,
    ColumnarStruct,
    SerializedRow,
}

/// Property descriptor describing richer metadata for a property.
/// This is the compile-time schema emitted by the Eval macro.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyDescriptor {
    pub id: PropertyId,
    pub name: String,
    pub value_type: ValueType,
    pub nullable: bool,
    pub storage_hint: StorageHint,
}

impl PropertyDescriptor {
    pub fn new(id: PropertyId, name: impl Into<String>, value_type: ValueType) -> Self {
        Self {
            id,
            name: name.into(),
            value_type,
            nullable: true,
            storage_hint: StorageHint::VariableLength,
        }
    }

    pub fn with_storage_hint(mut self, hint: StorageHint) -> Self {
        self.storage_hint = hint;
        self
    }

    pub fn with_nullable(mut self, nullable: bool) -> Self {
        self.nullable = nullable;
        self
    }
}
