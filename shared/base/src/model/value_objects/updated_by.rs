use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdatedBy(String);

impl UpdatedBy {

    pub fn new<S: Into<String>>(updated_by: S) -> Self {
        Self(updated_by.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UpdatedBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updated_by_display() {
        let updated_by = UpdatedBy::new("user123");
        assert_eq!(updated_by.to_string(), "user123");
    }

    #[test]
    fn updated_by_as_str() {
        let updated_by = UpdatedBy::new("user123");
        assert_eq!(updated_by.as_str(), "user123");
    }
}
