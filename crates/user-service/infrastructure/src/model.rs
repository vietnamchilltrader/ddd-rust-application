use std::fmt;

use domain::User;
use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct UserModel {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
}
impl UserModel {
    pub fn new(id: String, username: String, password: String, email: String) -> Self {
        Self {
            id,
            username,
            password,
            email,
        }
    }
}
impl From<User> for UserModel {
    fn from(user: User) -> Self {
        Self {
            id: user.id.as_str().to_string(),
            username: user.username.as_str().to_string(),
            password: user.password_hash.as_str().to_string(),
            email: user.email_address.as_str().to_string(),
        }
    }
}
impl fmt::Display for UserModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UserModel {{ id: {}, username: {}, password: {}, email: {} }}",
            self.id, self.username, self.password, self.email
        )
    }
}
