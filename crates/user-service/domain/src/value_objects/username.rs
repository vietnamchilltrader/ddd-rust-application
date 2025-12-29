#[derive(Debug, thiserror::Error)]
/// Errors that may occur when creating a `Username`.
pub enum UserNameError {
    #[error("Username too short (min 3 characters)")]
    TooShort,
    #[error("Username too long (max 20 characters)")]
    TooLong,
    #[error("Username contains invalid characters")]
    InvalidCharacters,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A validated username that follows the specified constraints.
pub struct Username(String);

impl Username {
    /// Creates a new `Username`
    pub fn new(name: &str) -> Result<Self, UserNameError> {
        let trimmed = name.trim();
        if trimmed.len() < 3 {
            return Err(UserNameError::TooShort);
        }
        if trimmed.len() > 20 {
            return Err(UserNameError::TooLong);
        }
        if !trimmed
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(UserNameError::InvalidCharacters);
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Returns the inner username as a `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_username_too_short() {
        let result = Username::new("ab");
        assert!(matches!(result, Err(UserNameError::TooShort)));
    }

    #[test]
    fn test_username_too_long() {
        let result = Username::new(&"a".repeat(21));
        assert!(matches!(result, Err(UserNameError::TooLong)));
    }

    #[test]
    fn test_username_invalid_characters() {
        let result_with_period = Username::new("invalid.user");
        let result_with_space = Username::new("invalid user");
        let result_with_special = Username::new("invalid$user");
        assert!(matches!(
            result_with_period,
            Err(UserNameError::InvalidCharacters)
        ));
        assert!(matches!(
            result_with_space,
            Err(UserNameError::InvalidCharacters)
        ));
        assert!(matches!(
            result_with_special,
            Err(UserNameError::InvalidCharacters)
        ));
    }

    #[test]
    fn test_username_with_valid_characters() {
        let result_with_underscore = Username::new("valid_user");
        let result_with_dash = Username::new("valid-user");
        let result_with_both = Username::new("valid_user-name");
        assert!(result_with_underscore.is_ok());
        assert!(result_with_dash.is_ok());
        assert!(result_with_both.is_ok());
    }

    #[test]
    fn test_trimmed_usernames() {
        let result = Username::new("  valid_user  ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "valid_user");
    }

    #[test]
    fn test_username_as_str() {
        let username = Username::new("test_user").unwrap();
        assert_eq!(username.as_str(), "test_user");
    }
}
