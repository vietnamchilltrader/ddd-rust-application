use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreatedBy(String);

impl CreatedBy {
    pub fn new<S: Into<String>>(created_by: S) -> Self {
        Self(created_by.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for CreatedBy {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for CreatedBy {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Display for CreatedBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_by_display() {
        let created_by = CreatedBy::new("user123");
        assert_eq!(created_by.to_string(), "user123");
    }

    #[test]
    fn created_by_as_str() {
        let created_by = CreatedBy::new("user123");
        assert_eq!(created_by.as_str(), "user123");
    }

    #[test]
    fn created_by_from_string_and_str() {
        let s = String::from("alice");
        let from_string: CreatedBy = s.clone().into();
        let from_str: CreatedBy = "alice".into();
        assert_eq!(from_string.as_str(), "alice");
        assert_eq!(from_str.as_str(), "alice");
    }
}
