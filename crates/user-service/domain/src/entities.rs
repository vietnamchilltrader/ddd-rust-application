use crate::value_objects::{EmailAddress, Password, UserId, Username};
use base::model::{Audit, value_objects::CreatedAt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: UserId,
    pub username: Username,
    pub password_hash: Password,
    pub email_address: EmailAddress,
    pub audit: Option<Audit>,
}
impl User {
    pub fn new(username: Username, password_hash: Password, email_address: EmailAddress) -> Self {
        let created_at = CreatedAt::now();
        let id = UserId::new();
        let audit = Audit::with_created_at(created_at);
        Self {
            id,
            username,
            password_hash,
            email_address,
            audit: Some(audit),
        }
    }
}
