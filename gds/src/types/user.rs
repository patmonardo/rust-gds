//! User - Represents a user in the system.

use std::fmt;

/// Represents a user in the system.
/// 
/// This is used for authentication and authorization in GraphStore operations.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct User {
    username: String,
    is_admin: bool,
}

impl User {
    /// Creates a new User.
    pub fn new(username: String, is_admin: bool) -> Self {
        Self { username, is_admin }
    }
    
    /// Returns the username.
    pub fn username(&self) -> &str {
        &self.username
    }
    
    /// Returns whether the user is an admin.
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.username)
    }
}

impl From<String> for User {
    fn from(username: String) -> Self {
        Self::new(username, false)
    }
}

impl From<&str> for User {
    fn from(username: &str) -> Self {
        Self::new(username.to_string(), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("alice".to_string(), true);
        assert_eq!(user.username(), "alice");
        assert!(user.is_admin());
    }

    #[test]
    fn test_user_from_string() {
        let user: User = "bob".to_string().into();
        assert_eq!(user.username(), "bob");
        assert!(!user.is_admin());
    }

    #[test]
    fn test_user_display() {
        let user = User::new("charlie".to_string(), false);
        assert_eq!(format!("{}", user), "charlie");
    }
}
