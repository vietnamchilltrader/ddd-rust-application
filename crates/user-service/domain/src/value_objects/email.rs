use anyhow::Result;
use regex::Regex;
use tracing::error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A validated email address.
pub struct EmailAddress(String);

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when validating an email address.
pub enum EmailError {
    #[error("Invalid email format")]
    InvalidFormat,
    #[error("Email too long (max 254 characters)")]
    TooLong,
}

impl EmailAddress {
    /// Creates a new `EmailAddress` .
    pub fn new(email: String) -> Result<Self, EmailError> {
        if email.len() > 254 {
            return Err(EmailError::TooLong);
        }

        if !Self::is_valid_email(&email) {
            return Err(EmailError::InvalidFormat);
        }

        // Normalize to lowercase before storing.
        Ok(Self(email.to_lowercase()))
    }
    /// Validates the email format using a regex.
    fn is_valid_email(email: &str) -> bool {
        let regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap_or_else(|_| {
            error!("Failed to compile email regex");
            panic!("Regex compilation should never fail");
        });
        regex.is_match(email)
    }

    /// Returns the inner email string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = "User@Example.COM".to_string();
        let result = EmailAddress::new(email.clone());
        assert!(result.is_ok());
        let address = result.unwrap();
        assert_eq!(address.as_str(), "user@example.com");
    }

    #[test]
    fn test_invalid_email_format() {
        let result = EmailAddress::new("invalid-email".to_string());
        assert!(matches!(result, Err(EmailError::InvalidFormat)));
    }

    #[test]
    fn test_missing_at_symbol() {
        let result = EmailAddress::new("invalid.email.com".to_string());
        assert!(matches!(result, Err(EmailError::InvalidFormat)));
    }

    #[test]
    fn test_missing_tld() {
        let result = EmailAddress::new("user@domain".to_string());
        assert!(matches!(result, Err(EmailError::InvalidFormat)));
    }

    #[test]
    fn test_email_with_spaces() {
        let result = EmailAddress::new("user @example.com".to_string());
        assert!(matches!(result, Err(EmailError::InvalidFormat)));
    }

    #[test]
    fn test_email_too_long() {
        let long_email = "a".repeat(255) + "@example.com";
        let result = EmailAddress::new(long_email);
        assert!(matches!(result, Err(EmailError::TooLong)));
    }
}
