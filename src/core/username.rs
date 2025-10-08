use std::fmt;

/// Represents a username in the system.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Username {
    value: String,
}

impl Username {
    /// The empty username singleton instance.
    pub const EMPTY_USERNAME: Username = Username {
        value: String::new(),
    };

    /// Factory method to create a new Username instance.
    pub fn of(username: impl Into<String>) -> Self {
        Username {
            value: username.into(),
        }
    }

    /// Returns the username as a string.
    pub fn username(&self) -> &str {
        &self.value
    }

    /// String representation of this object.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Default for Username {
    fn default() -> Self {
        Self::EMPTY_USERNAME
    }
}
