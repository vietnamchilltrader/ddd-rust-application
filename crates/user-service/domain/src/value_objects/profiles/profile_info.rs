use super::full_name::FullName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProfileInfo {
    pub full_name: Option<FullName>,
}
