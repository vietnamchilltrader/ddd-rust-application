#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FullName(String);

impl FullName {
    pub fn new(name: String) -> Result<Self, String> {
        let strimmed = name.trim();
        if strimmed.is_empty() {
            return Err("Full name cannot be empty".to_string());
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
