use std::fmt;

use domain::User;
use domain::username::Username;
use domain::value_objects::{EmailAddress, Password, UserId};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
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
impl From<UserModel> for User {
    fn from(user_model: UserModel) -> Self {
        let id = UserId::from_string(&user_model.id).expect("Invalid username");
        let username = Username::new(&user_model.username).expect("Invalid username");
        let password_hash = Password::from_hash(user_model.password);
        let email_address = EmailAddress::new(user_model.email).expect("Invalid email address");
        User {
            id,
            username,
            password_hash,
            email_address,
            audit: None,
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
