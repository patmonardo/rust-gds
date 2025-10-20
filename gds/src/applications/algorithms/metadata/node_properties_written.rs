/// Metadata about node properties that were written.
/// This is a core pattern in the Applications system for tracking
/// what was written during algorithm execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePropertiesWritten {
    value: u64,
}

impl NodePropertiesWritten {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl std::fmt::Display for NodePropertiesWritten {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodePropertiesWritten({})", self.value)
    }
}
