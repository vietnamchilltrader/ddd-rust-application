use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(Uuid);

impl UserId {
    /// Create a new UserId with a time-ordered UUIDv7
    pub fn new() -> Self {
        let ts = uuid::Timestamp::now(uuid::NoContext);
        Self(Uuid::new_v7(ts))
    }

    /// Parse a UserId from a string representation
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }

    /// Get the inner UUID reference
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Get string representation (convenience method)
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }

    /// Convert to hyphenated string format
    pub fn to_hyphenated(&self) -> String {
        self.0.hyphenated().to_string()
    }

    /// Convert to simple (non-hyphenated) string format
    pub fn to_simple(&self) -> String {
        self.0.simple().to_string()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}

impl From<Uuid> for UserId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserId> for Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_valid_uuid() {
        let id1 = UserId::new();
        let id2 = UserId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_from_string_valid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let user_id = UserId::from_string(uuid_str).unwrap();
        assert_eq!(user_id.to_string(), uuid_str);
    }

    #[test]
    fn test_from_string_invalid() {
        let result = UserId::from_string("not-a-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let user_id = UserId::from(uuid);
        assert_eq!(user_id.to_string(), "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn test_from_str_trait() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let user_id: UserId = uuid_str.parse().unwrap();
        assert_eq!(user_id.to_string(), uuid_str);
    }

    #[test]
    fn test_serde_serialization() {
        let user_id = UserId::new();
        let json = serde_json::to_string(&user_id).unwrap();
        let deserialized: UserId = serde_json::from_str(&json).unwrap();
        assert_eq!(user_id, deserialized);
    }

    #[test]
    fn test_conversions() {
        let ts = uuid::Timestamp::now(uuid::NoContext);
        let uuid = Uuid::new_v7(ts);
        let user_id = UserId::from(uuid);
        let uuid_back: Uuid = user_id.into();
        assert_eq!(uuid, uuid_back);
    }
}
