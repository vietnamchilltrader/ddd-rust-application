use crate::value_objects::{EmailAddress, Password, UserId, Username};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub password_hash: Password,
    pub email_address: EmailAddress,
}
impl User {
    pub fn new(username: Username, password_hash: Password, email_address: EmailAddress) -> Self {
        let id = UserId::new();
        Self {
            id,
            username,
            password_hash,
            email_address,
        }
    }
}
