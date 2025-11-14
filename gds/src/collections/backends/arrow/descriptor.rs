use crate::config::Extension;
use crate::types::ValueType;

/// Static metadata describing an Arrow-backed collection.
#[derive(Debug, Clone)]
pub struct ArrowBackendDescriptor {
    pub name: String,
    pub value_type: ValueType,
    pub nullable: bool,
    pub extensions: Vec<Extension>,
}

impl ArrowBackendDescriptor {
    pub fn new(name: impl Into<String>, value_type: ValueType) -> Self {
        Self {
            name: name.into(),
            value_type,
            nullable: true,
            extensions: Vec::new(),
        }
    }
}
