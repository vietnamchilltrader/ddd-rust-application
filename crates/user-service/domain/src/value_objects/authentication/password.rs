use argon2::password_hash::{PasswordHash, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHasher};
use base::web::error::AppError;
use rand_core::OsRng;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Password(String);

impl Password {
    /// Create from plain password (hash it)
    pub fn from_plain(password: &str) -> Result<Self, AppError> {
        if password.len() < 8 {
            return Err(AppError::BadRequest("Password too short".into()));
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::BadRequest(e.to_string()))?
            .to_string();

        Ok(Self(hash))
    }

    pub fn from_hash(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn verify(&self, password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.0).unwrap();
        let argon2 = Argon2::default();

        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_too_short() {
        let result = Password::from_plain("short");
        assert!(matches!(result, Err(AppError::BadRequest(_))));
    }

    #[test]
    fn test_password_to_long() {
        let result = Password::from_plain("thisisalongpassword");
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_verify() {
        let password = "securepassword";
        let pwd_obj = Password::from_plain(password).unwrap();
        assert!(pwd_obj.verify(password));
        assert!(!pwd_obj.verify("wrongpassword"));
    }
}
