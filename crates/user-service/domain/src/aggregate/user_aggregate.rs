use crate::value_objects::AuthenticationInfo;
use crate::value_objects::ProfileInfo;
use crate::value_objects::UserId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: UserId,
    pub profile_info: Option<ProfileInfo>,
    pub authentication_info: AuthenticationInfo,
}
