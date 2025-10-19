use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the state/origin of a property in the graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PropertyState {
    /// The property is projected from a source graph.
    Persistent,

    /// The property is only present in the in-memory graph,
    /// e.g. as a result of a mutate operation.
    Transient,

    /// The property is projected from a remote source graph.
    Remote,
}

impl PropertyState {
    /// Returns the name of the property state.
    pub fn name(self) -> &'static str {
        match self {
            PropertyState::Persistent => "PERSISTENT",
            PropertyState::Transient => "TRANSIENT",
            PropertyState::Remote => "REMOTE",
        }
    }
}

impl fmt::Display for PropertyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
