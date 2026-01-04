use chrono::{DateTime, Utc};

use crate::value_objects::authentication::{
    email::EmailAddress, password::Password, username::Username,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthenticationInfo {
    pub username: Username,
    pub password: Password,
    pub email: EmailAddress,
    pub vefied: bool,
    pub vefified_at: Option<DateTime<Utc>>,
}
