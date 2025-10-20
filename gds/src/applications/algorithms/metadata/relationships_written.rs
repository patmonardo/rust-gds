/// Metadata about relationships that were written.
/// This is a core pattern in the Applications system for tracking
/// what was written during algorithm execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationshipsWritten {
    value: u64,
}

impl RelationshipsWritten {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl std::fmt::Display for RelationshipsWritten {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RelationshipsWritten({})", self.value)
    }
}
